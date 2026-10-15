//! The terminal, done by hand: raw mode through termios, input through poll(2)
//! and a small escape-sequence parser, resize through SIGWINCH. No terminal
//! library means nothing between us and the TTY, and a lost terminal (closed
//! window, killed tmux) is noticed immediately instead of spinning on EOF.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::os::fd::{AsRawFd, FromRawFd, RawFd};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::{Duration, Instant};

use crate::color::Color;
use crate::theme::TermColors;

const ENTER: &[u8] = b"\x1b[?1049h\x1b[?25l\x1b[?7l\x1b[?1004h";
const MOUSE_ON: &[u8] = b"\x1b[?1000h\x1b[?1006h";
const LEAVE: &[u8] =
    b"\x1b[0m\x1b[?1006l\x1b[?1000l\x1b[?1004l\x1b[?2026l\x1b[?7h\x1b[?25h\x1b[?1049l";

/// Set by SIGTERM/SIGHUP/SIGINT; the main loop checks it every frame.
pub static QUIT: AtomicBool = AtomicBool::new(false);
static RESIZED: AtomicBool = AtomicBool::new(false);
/// A shell we stopped to borrow its terminal (bash idle takeover); resumed on any exit path.
pub static STOPPED_SHELL: AtomicI32 = AtomicI32::new(0);
/// The fd and termios to restore, while we own the terminal.
static SAVED: Mutex<Option<(RawFd, libc::termios)>> = Mutex::new(None);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Esc,
    Enter,
    CtrlC,
    CtrlD,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Key(Key),
    FocusIn,
    FocusOut,
    MouseDown,
    Resize,
    /// The terminal went away.
    Hangup,
}

/// The terminal we draw on and read from: stdout/stdin when they are TTYs,
/// otherwise /dev/tty (as when started by the idle watcher).
pub struct Term {
    pub out: File,
    in_fd: RawFd,
    _tty: Option<File>,
    buf: Vec<u8>,
}

fn dup_file(fd: RawFd) -> Option<File> {
    let d = unsafe { libc::dup(fd) };
    (d >= 0).then(|| unsafe { File::from_raw_fd(d) })
}

impl Term {
    pub fn open() -> io::Result<Term> {
        let tty = || OpenOptions::new().read(true).write(true).open("/dev/tty");
        let (out, mut keep) = match unsafe { libc::isatty(1) } == 1 {
            true => (dup_file(1).ok_or_else(io::Error::last_os_error)?, None),
            false => {
                let t = tty()?;
                (t.try_clone()?, Some(t))
            }
        };
        let in_fd = if unsafe { libc::isatty(0) } == 1 {
            0
        } else {
            if keep.is_none() {
                keep = Some(tty()?);
            }
            keep.as_ref().unwrap().as_raw_fd()
        };
        Ok(Term {
            out,
            in_fd,
            _tty: keep,
            buf: Vec::with_capacity(256),
        })
    }

    /// Raw mode, alternate screen, hidden cursor, no autowrap, focus reports.
    pub fn enter(&mut self, mouse: bool) -> io::Result<()> {
        unsafe {
            let mut t: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(self.in_fd, &mut t) != 0 {
                return Err(io::Error::last_os_error());
            }
            *SAVED.lock().unwrap() = Some((self.in_fd, t));
            let mut raw = t;
            raw.c_iflag &= !(libc::IGNBRK
                | libc::BRKINT
                | libc::PARMRK
                | libc::ISTRIP
                | libc::INLCR
                | libc::IGNCR
                | libc::ICRNL
                | libc::IXON);
            raw.c_oflag &= !libc::OPOST;
            raw.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::ISIG | libc::IEXTEN);
            raw.c_cflag = (raw.c_cflag & !(libc::CSIZE | libc::PARENB)) | libc::CS8;
            raw.c_cc[libc::VMIN] = 1;
            raw.c_cc[libc::VTIME] = 0;
            if libc::tcsetattr(self.in_fd, libc::TCSAFLUSH, &raw) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
        self.out.write_all(ENTER)?;
        if mouse {
            self.out.write_all(MOUSE_ON)?;
        }
        self.out.flush()
    }

    pub fn size(&self) -> (u16, u16) {
        let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
        for fd in [self.out.as_raw_fd(), self.in_fd] {
            if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) } == 0
                && ws.ws_col > 0
                && ws.ws_row > 0
            {
                return (ws.ws_col, ws.ws_row);
            }
        }
        (80, 24)
    }

    /// Waits up to `timeout` for input and appends what arrived to `out`.
    /// Returns early on signals (resize, quit).
    pub fn poll(&mut self, timeout: Duration, out: &mut Vec<Event>) {
        if RESIZED.swap(false, Ordering::SeqCst) {
            out.push(Event::Resize);
            return;
        }
        let mut pfd = libc::pollfd {
            fd: self.in_fd,
            events: libc::POLLIN,
            revents: 0,
        };
        // Round up: a 0 ms poll for the last fraction of a millisecond would spin.
        let ms = timeout.as_micros().div_ceil(1000).min(i32::MAX as u128) as i32;
        let r = unsafe { libc::poll(&mut pfd, 1, ms) };
        if r < 0 {
            // EINTR: a signal arrived; the caller re-checks the flags.
            if RESIZED.swap(false, Ordering::SeqCst) {
                out.push(Event::Resize);
            }
            return;
        }
        if r == 0 {
            return;
        }
        if pfd.revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL) != 0
            && pfd.revents & libc::POLLIN == 0
        {
            out.push(Event::Hangup);
            return;
        }
        let mut chunk = [0u8; 1024];
        let n = unsafe { libc::read(self.in_fd, chunk.as_mut_ptr().cast(), chunk.len()) };
        if n == 0
            || (n < 0
                && io::Error::last_os_error().raw_os_error() != Some(libc::EINTR)
                && io::Error::last_os_error().raw_os_error() != Some(libc::EAGAIN))
        {
            out.push(Event::Hangup);
            return;
        }
        if n > 0 {
            self.buf.extend_from_slice(&chunk[..n as usize]);
            parse(&mut self.buf, out, true);
        }
    }

    /// Asks the terminal for its foreground, background and (optionally) its
    /// 16-color palette. Terminals that ignore the color queries still answer
    /// the trailing DA1 request, so we rarely wait out the timeout. Anything
    /// that arrives late is swallowed by the input parser.
    pub fn query_colors(&mut self, palette: bool) -> TermColors {
        let mut res = TermColors::default();
        let mut q = Vec::from(&b"\x1b]10;?\x1b\\\x1b]11;?\x1b\\"[..]);
        if palette {
            for i in 0..16 {
                q.extend_from_slice(format!("\x1b]4;{i};?\x1b\\").as_bytes());
            }
        }
        q.extend_from_slice(b"\x1b[c");
        if self
            .out
            .write_all(&q)
            .and_then(|_| self.out.flush())
            .is_err()
        {
            return res;
        }
        let mut buf = Vec::with_capacity(1024);
        let deadline = Instant::now() + Duration::from_millis(if palette { 400 } else { 250 });
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                break;
            }
            let mut pfd = libc::pollfd {
                fd: self.in_fd,
                events: libc::POLLIN,
                revents: 0,
            };
            if unsafe { libc::poll(&mut pfd, 1, left.as_millis().max(1) as i32) } <= 0 {
                break;
            }
            let mut chunk = [0u8; 512];
            let n = unsafe { libc::read(self.in_fd, chunk.as_mut_ptr().cast(), chunk.len()) };
            if n <= 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n as usize]);
            if let Some(end) = da1_end(&buf) {
                // Keep anything typed after the reply for the input parser.
                self.buf.extend_from_slice(&buf[end..]);
                buf.truncate(end);
                break;
            }
        }
        let text = String::from_utf8_lossy(&buf);
        res.fg = find_osc(&text, "]10;");
        res.bg = find_osc(&text, "]11;");
        if palette {
            let mut p = [Color::Reset; 16];
            let mut all = true;
            for (i, slot) in p.iter_mut().enumerate() {
                match find_osc(&text, &format!("]4;{i};")) {
                    Some(c) => *slot = c,
                    None => all = false,
                }
            }
            if all {
                res.palette = Some(p);
            }
        }
        res
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        restore();
    }
}

/// Puts the terminal back the way we found it. Safe to call more than once,
/// from the panic hook, and after the terminal is gone.
pub fn restore() {
    if let Some((fd, t)) = SAVED.lock().map(|mut s| s.take()).unwrap_or(None) {
        unsafe {
            libc::write(fd, LEAVE.as_ptr().cast(), LEAVE.len());
            if libc::isatty(1) == 1 && fd != 1 {
                // Sequences go to the same terminal; also make sure stdout got them.
                libc::write(1, LEAVE.as_ptr().cast(), LEAVE.len());
            }
            libc::tcsetattr(fd, libc::TCSAFLUSH, &t);
        }
    }
    resume_shell();
}

pub fn resume_shell() {
    let pid = STOPPED_SHELL.swap(0, Ordering::SeqCst);
    if pid > 0 {
        unsafe {
            libc::kill(pid, libc::SIGCONT);
        }
    }
}

extern "C" fn on_quit(_: libc::c_int) {
    QUIT.store(true, Ordering::SeqCst);
}

extern "C" fn on_winch(_: libc::c_int) {
    RESIZED.store(true, Ordering::SeqCst);
}

fn handle(sig: libc::c_int, f: extern "C" fn(libc::c_int)) {
    unsafe {
        let mut sa: libc::sigaction = std::mem::zeroed();
        sa.sa_sigaction = f as *const () as libc::sighandler_t;
        // No SA_RESTART: poll() must return so the loop sees the flag at once.
        sa.sa_flags = 0;
        libc::sigemptyset(&mut sa.sa_mask);
        libc::sigaction(sig, &sa, std::ptr::null_mut());
    }
}

pub fn install_handlers() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore();
        hook(info);
    }));
    for sig in [libc::SIGTERM, libc::SIGHUP, libc::SIGINT, libc::SIGQUIT] {
        handle(sig, on_quit);
    }
    handle(libc::SIGWINCH, on_winch);
}

/// Index just past the DA1 reply (`ESC [ ? digits/; c`), if present.
fn da1_end(buf: &[u8]) -> Option<usize> {
    (0..buf.len().saturating_sub(2)).find_map(|i| {
        if !buf[i..].starts_with(b"\x1b[?") {
            return None;
        }
        let tail = &buf[i + 3..];
        let n = tail
            .iter()
            .take_while(|b| b.is_ascii_digit() || **b == b';')
            .count();
        (tail.get(n) == Some(&b'c')).then_some(i + 3 + n + 1)
    })
}

/// Parses complete input sequences from the front of `buf`, leaving an
/// incomplete trailing escape sequence for the next read (unless `flush_esc`
/// and it is a lone ESC, which is the Escape key).
fn parse(buf: &mut Vec<u8>, out: &mut Vec<Event>, flush_esc: bool) {
    let mut i = 0;
    while i < buf.len() {
        let b = buf[i];
        let rest = &buf[i..];
        let consumed = match b {
            0x1b => match rest.get(1) {
                None if flush_esc => {
                    out.push(Event::Key(Key::Esc));
                    1
                }
                None => break,
                Some(b'[') => match csi(rest) {
                    Some((len, ev)) => {
                        if let Some(ev) = ev {
                            out.push(ev);
                        }
                        len
                    }
                    None => break,
                },
                Some(b'O') => match rest.get(2) {
                    Some(c) => {
                        out.push(Event::Key(arrow(*c)));
                        3
                    }
                    None => break,
                },
                // OSC, DCS, APC, PM, SOS: strings ended by BEL or ST. Swallow them
                // (late replies to our color queries end up here).
                Some(b']' | b'P' | b'_' | b'^' | b'X') => match string_end(rest) {
                    Some(len) => len,
                    None => break,
                },
                Some(b'\x1b') => {
                    out.push(Event::Key(Key::Esc));
                    1
                }
                // Alt+key.
                Some(_) => {
                    out.push(Event::Key(Key::Other));
                    2
                }
            },
            0x03 => {
                out.push(Event::Key(Key::CtrlC));
                1
            }
            0x04 => {
                out.push(Event::Key(Key::CtrlD));
                1
            }
            b'\r' | b'\n' => {
                out.push(Event::Key(Key::Enter));
                1
            }
            0x00..=0x1f | 0x7f => {
                out.push(Event::Key(Key::Other));
                1
            }
            _ => {
                let len = utf8_len(b);
                if rest.len() < len {
                    break;
                }
                let ch = std::str::from_utf8(&rest[..len])
                    .ok()
                    .and_then(|s| s.chars().next());
                out.push(Event::Key(ch.map_or(Key::Other, Key::Char)));
                len
            }
        };
        i += consumed;
    }
    buf.drain(..i);
    // Never let garbage accumulate.
    if buf.len() > 4096 {
        buf.clear();
    }
}

fn utf8_len(b: u8) -> usize {
    match b {
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf7 => 4,
        _ => 1,
    }
}

fn arrow(c: u8) -> Key {
    match c {
        b'A' => Key::Up,
        b'B' => Key::Down,
        b'C' => Key::Right,
        b'D' => Key::Left,
        _ => Key::Other,
    }
}

/// A CSI sequence: `ESC [ params final`. Returns its length and meaning.
fn csi(s: &[u8]) -> Option<(usize, Option<Event>)> {
    let body = &s[2..];
    let end = body.iter().position(|b| (0x40..=0x7e).contains(b))?;
    let fin = body[end];
    let params = &body[..end];
    let ev = match fin {
        b'A' | b'B' | b'C' | b'D' => Some(Event::Key(arrow(fin))),
        b'I' if params.is_empty() => Some(Event::FocusIn),
        b'O' if params.is_empty() => Some(Event::FocusOut),
        b'M' if params.first() == Some(&b'<') => {
            // SGR mouse: ESC [ < button ; x ; y M (press) / m (release).
            let btn: u32 = std::str::from_utf8(&params[1..])
                .ok()?
                .split(';')
                .next()?
                .parse()
                .ok()?;
            (btn & 64 == 0 && btn & 32 == 0).then_some(Event::MouseDown)
        }
        b'm' => None,
        b'~' | b'u' => Some(Event::Key(Key::Other)),
        // Replies (cursor position, DA, modes): ignore.
        _ => None,
    };
    Some((2 + end + 1, ev))
}

/// Length of an escape string terminated by BEL or ST (`ESC \`).
fn string_end(s: &[u8]) -> Option<usize> {
    let mut i = 2;
    while i < s.len() {
        match s[i] {
            0x07 => return Some(i + 1),
            0x1b if s.get(i + 1) == Some(&b'\\') => return Some(i + 2),
            0x1b if s.get(i + 1).is_some() => return Some(i),
            _ => i += 1,
        }
    }
    None
}

/// Finds `<prefix>rgb:RRRR/GGGG/BBBB` and converts it to 8-bit RGB.
fn find_osc(text: &str, prefix: &str) -> Option<Color> {
    let at = text.find(prefix)?;
    let rest = text[at + prefix.len()..].strip_prefix("rgb:")?;
    let end = rest
        .find(|c: char| !(c.is_ascii_hexdigit() || c == '/'))
        .unwrap_or(rest.len());
    let mut parts = rest[..end].split('/');
    let mut comp = || -> Option<u8> {
        let p = parts.next()?;
        let v = u32::from_str_radix(p, 16).ok()?;
        let max = (1u32 << (4 * p.len().clamp(1, 4))) - 1;
        Some((v * 255 / max) as u8)
    };
    Some(Color::Rgb(comp()?, comp()?, comp()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_osc_replies() {
        let r = "\x1b]11;rgb:1e1e/1e1e/2e2e\x1b\\\x1b]4;1;rgb:ff/00/80\x07\x1b[?62;22c";
        assert_eq!(find_osc(r, "]11;"), Some(Color::Rgb(0x1e, 0x1e, 0x2e)));
        assert_eq!(find_osc(r, "]4;1;"), Some(Color::Rgb(255, 0, 128)));
        assert_eq!(da1_end(r.as_bytes()), Some(r.len()));
        assert_eq!(da1_end(b"\x1b]11;rgb:0/0/0\x07"), None);
    }

    fn events(input: &[u8]) -> Vec<Event> {
        let mut buf = input.to_vec();
        let mut out = Vec::new();
        parse(&mut buf, &mut out, true);
        out
    }

    #[test]
    fn parses_keys_and_ignores_replies() {
        use Event::*;
        assert_eq!(events(b"q"), [Key(self::Key::Char('q'))]);
        assert_eq!(
            events(b"\x1b[A\x1bOD"),
            [Key(self::Key::Up), Key(self::Key::Left)]
        );
        assert_eq!(events(b"\x1b"), [Key(self::Key::Esc)]);
        assert_eq!(
            events(b"\x1b]11;rgb:0/0/0\x1b\\x"),
            [Key(self::Key::Char('x'))]
        );
        assert_eq!(events(b"\x1b[?62;22c\x1b[I\x1b[O"), [FocusIn, FocusOut]);
        assert_eq!(events(b"\x1b[<0;10;5M\x1b[<0;10;5m"), [MouseDown]);
        assert_eq!(events("é".as_bytes()), [Key(self::Key::Char('é'))]);
        assert_eq!(events(b"\x03"), [Key(self::Key::CtrlC)]);
    }

    #[test]
    fn keeps_incomplete_sequences() {
        let mut buf = b"\x1b[".to_vec();
        let mut out = Vec::new();
        parse(&mut buf, &mut out, true);
        assert!(out.is_empty());
        assert_eq!(buf, b"\x1b[");
        buf.extend_from_slice(b"B");
        parse(&mut buf, &mut out, true);
        assert_eq!(out, [Event::Key(Key::Down)]);
    }
}
