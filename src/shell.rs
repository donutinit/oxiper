//! Shell integration: start oxiper as a screensaver after the prompt has been idle.
//!
//! A tiny watcher process per shell checks the TTY's access time (updated when
//! the shell reads a keystroke) and whether the shell owns the foreground.
//!
//! - zsh and fish run signal handlers while you type, so the watcher sends them
//!   SIGUSR1 and they launch `oxiper --saver` themselves.
//! - bash cannot. Inside tmux the watcher opens a full-screen tmux popup over
//!   the pane, which leaves the shell untouched.
//! - Plain bash: the watcher stops the shell, borrows the terminal, runs the
//!   saver and hands everything back. (tmux resumes stopped panes on its own,
//!   which is why it gets the popup instead.)

use std::fs;
use std::os::fd::AsRawFd;
use std::sync::atomic::Ordering;
use std::time::{Duration, SystemTime};

use crate::term;

pub fn init_script(shell: &str) -> Result<String, String> {
    Ok(match shell {
        "bash" => r#"# oxiper — idle screensaver for bash. In ~/.bashrc:
#   eval "$(oxiper shell-init bash)"
# Idle seconds: `idle` under [saver] in the oxiper config, or $OXIPER_IDLE.
if [[ $- == *i* && -t 0 && ${_oxiper_watch-} != "$$" ]]; then
  _oxiper_watch=$$
  ( command oxiper idle-watch --pid "$$" --takeover --idle "${OXIPER_IDLE-}" </dev/null >/dev/null 2>&1 & )
fi
"#
        .into(),
        "zsh" => r#"# oxiper — idle screensaver for zsh. In ~/.zshrc:
#   eval "$(oxiper shell-init zsh)"
# Idle seconds: `idle` under [saver] in the oxiper config, or $OXIPER_IDLE.
if [[ -o interactive && -t 0 && ${_oxiper_watch-} != "$$" ]]; then
  _oxiper_watch=$$
  TRAPUSR1() {
    command oxiper --saver --owner $$
    zle && zle reset-prompt
    return 0
  }
  ( command oxiper idle-watch --pid "$$" --idle "${OXIPER_IDLE-}" </dev/null >/dev/null 2>&1 & )
fi
"#
        .into(),
        "fish" => r#"# oxiper — idle screensaver for fish. In ~/.config/fish/config.fish:
#   oxiper shell-init fish | source
# Idle seconds: `idle` under [saver] in the oxiper config, or $OXIPER_IDLE.
if status is-interactive; and test "$_oxiper_watch" != "$fish_pid"
    set -g _oxiper_watch $fish_pid
    function __oxiper_saver --on-signal SIGUSR1
        command oxiper --saver --owner $fish_pid
        commandline -f repaint
    end
    command oxiper idle-watch --pid $fish_pid --idle "$OXIPER_IDLE" </dev/null >/dev/null 2>&1 &
    disown
end
"#
        .into(),
        other => return Err(format!("unsupported shell '{other}' (bash, zsh or fish)")),
    })
}

struct Proc {
    state: char,
    pgrp: i32,
    tpgid: i32,
    start: u64,
}

fn proc_stat(pid: i32) -> Option<Proc> {
    let s = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    // The command name may contain spaces and parentheses; fields resume after the last ')'.
    let rest = &s[s.rfind(')')? + 2..];
    let f: Vec<&str> = rest.split_whitespace().collect();
    Some(Proc {
        state: f.first()?.chars().next()?,
        pgrp: f.get(2)?.parse().ok()?,
        tpgid: f.get(5)?.parse().ok()?,
        start: f.get(19)?.parse().ok()?,
    })
}

/// Sleeps, but wakes early on signals so SIGTERM/SIGHUP are handled promptly.
fn nap(d: Duration) {
    let ts = libc::timespec {
        tv_sec: d.as_secs() as libc::time_t,
        tv_nsec: d.subsec_nanos() as libc::c_long,
    };
    unsafe {
        libc::nanosleep(&ts, std::ptr::null_mut());
    }
}

fn atime(tty: &str) -> Option<SystemTime> {
    fs::metadata(tty).and_then(|m| m.accessed()).ok()
}

/// Is a saver started on behalf of this shell (`--owner <pid>`) still running?
/// Needed because zsh/fish may run it inside their own process group.
fn saver_running(owner: i32) -> bool {
    let want = owner.to_string();
    let Ok(dir) = fs::read_dir("/proc") else {
        return false;
    };
    dir.flatten().any(|e| {
        let name = e.file_name();
        let Some(pid) = name
            .to_str()
            .filter(|n| n.bytes().all(|b| b.is_ascii_digit()))
        else {
            return false;
        };
        let Ok(cmd) = fs::read(format!("/proc/{pid}/cmdline")) else {
            return false;
        };
        let mut args = cmd.split(|b| *b == 0);
        let mut seen_owner = false;
        args.any(|a| {
            let hit = seen_owner && a == want.as_bytes();
            seen_owner = a == b"--owner";
            hit
        })
    })
}

fn tmux(args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("tmux")
        .args(args)
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Version of the tmux server as (major, minor), e.g. "tmux 3.5a" → (3, 5).
fn tmux_version() -> Option<(u32, u32)> {
    let v = tmux(&["-V"])?;
    let num = v.split_whitespace().nth(1)?;
    let mut it = num.split('.');
    let major = it.next()?.parse().ok()?;
    let minor: String = it
        .next()
        .unwrap_or("0")
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    Some((major, minor.parse().unwrap_or(0)))
}

/// Opens a full-screen popup running the saver on the client that is looking
/// at our pane. Returns false when nobody is looking (nothing to do).
fn tmux_popup(owner: i32, pane: &str, borderless: bool) -> bool {
    let Some(clients) = tmux(&["list-clients", "-F", "#{client_tty} #{pane_id}"]) else {
        return false;
    };
    let Some(client) = clients.lines().find_map(|l| {
        l.split_once(' ')
            .filter(|(_, p)| *p == pane)
            .map(|(c, _)| c.to_string())
    }) else {
        return false;
    };
    let exe = std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| "oxiper".into());
    let cmd = format!("'{}' --saver --owner {owner}", exe.replace('\'', "'\\''"));
    let mut args = vec![
        "display-popup",
        "-E",
        "-c",
        &client,
        "-w",
        "100%",
        "-h",
        "100%",
    ];
    if borderless {
        args.push("-B");
    }
    args.push(&cmd);
    tmux(&args).is_some()
}

/// Runs until the shell exits. `idle` is re-read each round so config edits apply.
pub fn idle_watch(
    pid: i32,
    idle: &dyn Fn() -> u64,
    takeover: bool,
    saver: &dyn Fn() -> Result<(), String>,
) -> Result<(), String> {
    let first = proc_stat(pid).ok_or(format!("no such process {pid}"))?;
    let tty = fs::read_link(format!("/proc/{pid}/fd/0"))
        .map_err(|e| format!("cannot find the shell's terminal: {e}"))?;
    let tty = tty.to_string_lossy().to_string();
    if !tty.starts_with("/dev/") {
        return Err(format!("the shell's input is not a terminal ({tty})"));
    }
    term::install_handlers();
    unsafe {
        libc::setpgid(0, 0);
        libc::signal(libc::SIGTTOU, libc::SIG_IGN);
        libc::signal(libc::SIGTTIN, libc::SIG_IGN);
    }
    let tmux_pane = std::env::var("TMUX_PANE")
        .ok()
        .filter(|_| takeover && std::env::var_os("TMUX").is_some());
    let popup = match &tmux_pane {
        Some(_) => match tmux_version() {
            Some(v) if v >= (3, 2) => Some(v >= (3, 3)),
            // Too old for popups, and stopping a pane's shell would kill it.
            _ => return Ok(()),
        },
        None => None,
    };
    // The TTY access time when we last fired: never fire again until the user has
    // typed since (the saver may run inside the shell's own process group, so the
    // foreground check alone cannot tell that it is still showing).
    let mut fired_at: Option<SystemTime> = None;
    loop {
        if term::QUIT.load(Ordering::SeqCst) {
            return Ok(());
        }
        let Some(p) = proc_stat(pid) else {
            return Ok(());
        };
        if p.start != first.start {
            return Ok(());
        }
        if p.tpgid != p.pgrp {
            // Something else (an editor, a build, the saver itself) owns the terminal.
            nap(Duration::from_secs(2));
            continue;
        }
        let Some(last_input) = atime(&tty) else {
            return Ok(());
        };
        if fired_at.is_some_and(|t| last_input <= t) {
            nap(Duration::from_secs(2));
            continue;
        }
        let limit = idle().max(1);
        let idle_for = SystemTime::now()
            .duration_since(last_input)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if idle_for < limit {
            nap(Duration::from_secs((limit - idle_for).clamp(1, 15)));
            continue;
        }
        if saver_running(pid) {
            nap(Duration::from_secs(2));
            continue;
        }
        if let (Some(pane), Some(borderless)) = (&tmux_pane, popup) {
            if !tmux_popup(pid, pane, borderless) {
                nap(Duration::from_secs(5));
                continue;
            }
        } else if takeover {
            take_over(pid, &tty, saver)?;
        } else {
            unsafe {
                libc::kill(pid, libc::SIGUSR1);
            }
        }
        fired_at = Some(last_input);
        nap(Duration::from_secs(2));
    }
}

fn take_over(pid: i32, tty: &str, saver: &dyn Fn() -> Result<(), String>) -> Result<(), String> {
    let f = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(tty)
        .map_err(|e| e.to_string())?;
    let fd = f.as_raw_fd();
    unsafe {
        let shell_pg = libc::tcgetpgrp(fd);
        if shell_pg < 0 {
            return Ok(());
        }
        term::STOPPED_SHELL.store(pid, Ordering::SeqCst);
        libc::kill(pid, libc::SIGSTOP);
        for _ in 0..50 {
            if proc_stat(pid).is_some_and(|p| p.state == 'T') {
                break;
            }
            nap(Duration::from_millis(10));
        }
        if libc::tcsetpgrp(fd, libc::getpgrp()) != 0 {
            term::resume_shell();
            return Ok(());
        }
        let res = saver();
        term::restore();
        libc::tcsetpgrp(fd, shell_pg);
        term::resume_shell();
        libc::kill(pid, libc::SIGWINCH);
        term::QUIT.store(false, Ordering::SeqCst);
        res
    }
}
