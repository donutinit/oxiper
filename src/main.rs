mod app;
mod color;
mod config;
mod eggs;
mod headless;
mod render;
mod scene;
mod scenes;
mod shell;
mod term;
mod theme;

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};

use config::{Config, Override};

const KEYS: &str = "\
Keys:
  ← →  scene      ↑ ↓  theme      space  pause
  + -  speed      0    reset      r      reseed
  ?    help       q    quit       (any key quits in --saver mode)";

#[derive(Parser)]
#[command(name = "oxiper", version, about = "Animated terminal wallpapers, oxidized.", after_help = KEYS)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
    /// Scene(s) to play: a name, a comma-separated list, or "all"
    #[arg(short, long, value_name = "NAME[,NAME...]", global = true)]
    scene: Option<String>,
    /// Color theme (see `oxiper list themes`)
    #[arg(short, long, global = true)]
    theme: Option<String>,
    /// Target frames per second
    #[arg(long, global = true)]
    fps: Option<u32>,
    /// Animation speed multiplier (motion only; clocks keep real time)
    #[arg(long, global = true)]
    speed: Option<f64>,
    /// Seconds per scene when playing several (0 = no cycling)
    #[arg(short, long, value_name = "SECS", global = true)]
    duration: Option<f64>,
    /// Play scenes in random order
    #[arg(long, global = true)]
    shuffle: bool,
    /// Color depth: auto, truecolor, 256, 16, mono
    #[arg(long, value_name = "MODE", global = true)]
    color: Option<String>,
    /// transparent, theme, or a color like "#101018"
    #[arg(long, value_name = "MODE", global = true)]
    background: Option<String>,
    /// Set any config key for this run, e.g. -o neo.density=0.9 -o fps=60
    #[arg(short = 'o', long = "set", value_name = "KEY=VALUE", global = true)]
    set: Vec<String>,
    /// Config file (default: ~/.config/oxiper/config.toml)
    #[arg(short, long, value_name = "FILE", global = true)]
    config: Option<PathBuf>,
    /// Screensaver mode: any key quits, uses the [saver] config section
    #[arg(long)]
    saver: bool,
    /// Random seed for reproducible runs
    #[arg(long, global = true)]
    seed: Option<u64>,
    /// The shell a saver was started for (lets the idle watcher see it running)
    #[arg(long, hide = true)]
    owner: Option<i32>,
}

#[derive(Subcommand)]
enum Cmd {
    /// List scenes, themes, a scene's options, or the aquarium's species
    List {
        #[arg(value_enum, default_value = "scenes")]
        what: ListWhat,
        /// Scene name (for `options`)
        name: Option<String>,
    },
    /// Manage the config file
    Config {
        #[command(subcommand)]
        action: ConfigCmd,
    },
    /// Print shell integration that starts the saver when the prompt is idle
    ShellInit {
        #[arg(value_enum)]
        shell: ShellKind,
    },
    /// Record a scene to an asciicast file (play with asciinema, convert with agg)
    Record {
        /// Output .cast file
        out: PathBuf,
        #[arg(long, default_value = "100x30")]
        size: String,
        #[arg(long, default_value_t = 8.0)]
        seconds: f64,
        /// Seconds to run before recording starts
        #[arg(long, default_value_t = 0.0)]
        skip: f64,
    },
    /// Print one frame as text
    Snap {
        #[arg(long, default_value = "80x24")]
        size: String,
        /// Seconds into the scene
        #[arg(long, default_value_t = 5.0)]
        at: f64,
        /// Keep colors (ANSI escapes)
        #[arg(long)]
        ansi: bool,
    },
    /// Measure how much time each scene takes per frame
    Bench {
        #[arg(long, default_value = "200x60")]
        size: String,
        #[arg(long, default_value_t = 600)]
        frames: usize,
    },
    /// Print shell completions
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Print the scene option reference as Markdown (for the README)
    #[command(hide = true)]
    Docs,
    /// Show the aquarium's art: everything, a biome, or names containing FILTER
    #[command(hide = true)]
    Gallery {
        #[arg(default_value = "")]
        filter: String,
        /// Keep colors (ANSI escapes)
        #[arg(long)]
        ansi: bool,
    },
    #[command(hide = true)]
    IdleWatch {
        #[arg(long)]
        pid: i32,
        #[arg(long, default_value = "")]
        idle: String,
        #[arg(long)]
        takeover: bool,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum ListWhat {
    Scenes,
    Themes,
    Options,
    /// The aquarium's species, plants, decor and visitors
    Species,
}

#[derive(Subcommand)]
enum ConfigCmd {
    /// Write a commented config file with every setting
    Init {
        #[arg(long)]
        force: bool,
    },
    /// Print the config file path
    Path,
    /// Print a commented template with every setting to stdout
    Template,
    /// Check the config file for mistakes
    Check,
}

#[derive(Clone, Copy, ValueEnum)]
enum ShellKind {
    Bash,
    Zsh,
    Fish,
}

impl Cli {
    fn overrides(&self) -> Result<Vec<Override>, String> {
        let mut v = Vec::new();
        let s = |x: &str| toml::Value::String(x.to_string());
        if let Some(x) = &self.scene {
            v.push(Override::new("scene", s(x)));
        }
        if let Some(x) = &self.theme {
            v.push(Override::new("theme", s(x)));
        }
        if let Some(x) = self.fps {
            v.push(Override::new("fps", toml::Value::Integer(x as i64)));
        }
        if let Some(x) = self.speed {
            v.push(Override::new("speed", toml::Value::Float(x)));
        }
        if let Some(x) = self.duration {
            v.push(Override::new("duration", toml::Value::Float(x)));
        }
        if self.shuffle {
            v.push(Override::new("shuffle", toml::Value::Boolean(true)));
        }
        if let Some(x) = &self.color {
            v.push(Override::new("color", s(x)));
        }
        if let Some(x) = &self.background {
            v.push(Override::new("background", s(x)));
        }
        if let Some(x) = self.seed {
            v.push(Override::new("seed", toml::Value::Integer(x as i64)));
        }
        for o in &self.set {
            v.push(Override::parse(o)?);
        }
        Ok(v)
    }
}

/// Drops ANSI styling when stdout is not a terminal (for `list` piped to a file).
struct Styled<W: Write> {
    inner: W,
    strip: bool,
    in_esc: bool,
}

impl<W: Write> Styled<W> {
    fn new(inner: W) -> Self {
        let strip = unsafe { libc::isatty(1) } != 1
            || std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty());
        Styled {
            inner,
            strip,
            in_esc: false,
        }
    }
}

impl<W: Write> Write for Styled<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if !self.strip {
            return self.inner.write(buf);
        }
        let mut out = Vec::with_capacity(buf.len());
        for &b in buf {
            match (self.in_esc, b) {
                (false, 0x1b) => self.in_esc = true,
                (false, _) => out.push(b),
                (true, b'[') => {}
                (true, b) if b.is_ascii_alphabetic() => self.in_esc = false,
                (true, _) => {}
            }
        }
        self.inner.write_all(&out)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn warn(cfg: &Config) {
    for w in &cfg.warnings {
        eprintln!("oxiper: warning: {w}");
    }
}

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("oxiper: {e}");
            ExitCode::FAILURE
        }
    }
}

fn real_main() -> Result<(), String> {
    // Behave like a normal CLI when piped into `head` and friends.
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let cli = Cli::parse();
    let overrides = cli.overrides()?;
    let path = cli.config.clone();
    let load = |saver: bool| Config::load(path.as_deref(), &overrides, saver);
    let mut stdout = std::io::stdout().lock();
    let io = |e: std::io::Error| e.to_string();

    match cli.cmd {
        None => {
            let cfg = load(cli.saver)?;
            if !cli.saver && !cfg.warnings.is_empty() {
                warn(&cfg);
                std::thread::sleep(std::time::Duration::from_millis(1200));
            }
            app::run(
                cfg,
                app::RunOpts {
                    config_path: path.clone(),
                    overrides: overrides.clone(),
                    saver: cli.saver,
                },
            )
        }
        Some(Cmd::List {
            what: ListWhat::Scenes,
            ..
        }) => {
            let mut stdout = Styled::new(&mut stdout);
            for s in scenes::ALL {
                let aliases = if s.aliases.is_empty() {
                    String::new()
                } else {
                    format!("  ({})", s.aliases.join(", "))
                };
                writeln!(
                    stdout,
                    "  \x1b[1m{:<14}\x1b[0m {}\x1b[2m{aliases}\x1b[0m",
                    s.name, s.about
                )
                .map_err(io)?;
            }
            Ok(())
        }
        Some(Cmd::List {
            what: ListWhat::Themes,
            ..
        }) => {
            let mut stdout = Styled::new(&mut stdout);
            let cfg = load(false)?;
            let tc = theme::TermColors::default();
            let mut names: Vec<String> = theme::names().into_iter().map(String::from).collect();
            names.extend(cfg.custom_theme_names());
            for n in names {
                match app::resolve_theme(&n, &cfg, &tc) {
                    Ok(t) => {
                        let note = match n.as_str() {
                            "classic" => "  each scene's original colors",
                            "terminal" => "  your terminal's own palette",
                            "wal" => "  from pywal",
                            _ => "",
                        };
                        writeln!(stdout, "  {:<12} {}\x1b[2m{note}\x1b[0m", n, t.swatch())
                            .map_err(io)?
                    }
                    Err(_) => {
                        writeln!(stdout, "  {n:<12} \x1b[2m(unavailable)\x1b[0m").map_err(io)?
                    }
                }
            }
            Ok(())
        }
        Some(Cmd::List {
            what: ListWhat::Options,
            name,
        }) => {
            let mut stdout = Styled::new(&mut stdout);
            let infos: Vec<_> = match &name {
                Some(n) => vec![scenes::find(n).ok_or(format!("unknown scene '{n}'"))?],
                None => scenes::ALL.to_vec(),
            };
            for info in infos {
                writeln!(
                    stdout,
                    "\x1b[1m[scenes.{}]\x1b[0m  {}",
                    info.name, info.about
                )
                .map_err(io)?;
                if info.options.is_empty() {
                    writeln!(stdout, "  (no options)").map_err(io)?;
                }
                for o in info.options {
                    let def = o.default.to_toml().to_string();
                    writeln!(stdout, "  {:<16} {:<12} {}", o.key, def, o.help).map_err(io)?;
                }
                writeln!(stdout).map_err(io)?;
            }
            Ok(())
        }
        Some(Cmd::Config { action }) => match action {
            ConfigCmd::Path => {
                let p = path
                    .or_else(config::default_path)
                    .ok_or("cannot determine the config path")?;
                writeln!(stdout, "{}", p.display()).map_err(io)
            }
            ConfigCmd::Template => write!(stdout, "{}", config::template()).map_err(io),
            ConfigCmd::Init { force } => {
                let p = path
                    .or_else(config::default_path)
                    .ok_or("cannot determine the config path")?;
                if p.exists() && !force {
                    return Err(format!(
                        "{} already exists (use --force to overwrite)",
                        p.display()
                    ));
                }
                if let Some(dir) = p.parent() {
                    std::fs::create_dir_all(dir).map_err(io)?;
                }
                std::fs::write(&p, config::template()).map_err(io)?;
                writeln!(stdout, "wrote {}", p.display()).map_err(io)
            }
            ConfigCmd::Check => {
                let cfg = load(false)?;
                let tc = theme::TermColors::default();
                let mut problems = cfg.warnings.clone();
                if let Err(e) = app::resolve_theme(&cfg.theme, &cfg, &tc) {
                    problems.push(e);
                }
                for n in cfg.custom_theme_names() {
                    if let Err(e) = app::resolve_theme(&n, &cfg, &tc) {
                        problems.push(e);
                    }
                }
                let where_ = cfg
                    .path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default();
                if problems.is_empty() {
                    writeln!(stdout, "{where_}: ok").map_err(io)
                } else {
                    for p in &problems {
                        writeln!(stdout, "{where_}: {p}").map_err(io)?;
                    }
                    Err(format!("{} problem(s)", problems.len()))
                }
            }
        },
        Some(Cmd::ShellInit { shell }) => {
            let name = match shell {
                ShellKind::Bash => "bash",
                ShellKind::Zsh => "zsh",
                ShellKind::Fish => "fish",
            };
            write!(stdout, "{}", shell::init_script(name)?).map_err(io)
        }
        Some(Cmd::Record {
            out,
            size,
            seconds,
            skip,
        }) => {
            let (w, h) = headless::parse_size(&size)?;
            let mut cfg = load(false)?;
            warn(&cfg);
            // Several scenes play in turn when a duration is set (a tour).
            if cfg.duration <= 0.0 {
                cfg.scenes.truncate(1);
            }
            let fps = cfg.fps;
            let mut f = std::io::BufWriter::new(std::fs::File::create(&out).map_err(io)?);
            headless::record(cfg, w, h, seconds, skip, fps, &mut f)?;
            f.flush().map_err(io)?;
            eprintln!("wrote {}", out.display());
            Ok(())
        }
        Some(Cmd::Snap { size, at, ansi }) => {
            let (w, h) = headless::parse_size(&size)?;
            let mut cfg = load(false)?;
            cfg.scenes.truncate(1);
            write!(stdout, "{}", headless::snap(cfg, w, h, at, ansi)?).map_err(io)
        }
        Some(Cmd::Bench { size, frames }) => {
            let (w, h) = headless::parse_size(&size)?;
            let base = load(false)?;
            let names: Vec<&'static str> = base.scenes.iter().map(|s| s.name).collect();
            let mk = |name: &'static str| -> Result<Config, String> {
                let mut ov = overrides.clone();
                ov.push(Override::new("scene", toml::Value::String(name.into())));
                Config::load(path.as_deref(), &ov, false)
            };
            let rows = headless::bench(&mk, &names, w, h, frames)?;
            writeln!(stdout, "{w}x{h}, {frames} frames per scene, truecolor\n").map_err(io)?;
            writeln!(
                stdout,
                "  {:<14} {:>10} {:>10} {:>11} {:>9}",
                "scene", "sim µs", "render µs", "bytes/frame", "changed"
            )
            .map_err(io)?;
            for r in rows {
                writeln!(
                    stdout,
                    "  {:<14} {:>10.1} {:>10.1} {:>11.0} {:>8.1}%",
                    r.name, r.sim_us, r.render_us, r.bytes, r.changed
                )
                .map_err(io)?;
            }
            Ok(())
        }
        Some(Cmd::List {
            what: ListWhat::Species,
            ..
        }) => {
            use scenes::aquarium::assets::biome_names;
            let mut stdout = Styled::new(&mut stdout);
            for (title, rows) in scenes::aquarium::roster() {
                writeln!(stdout, "\x1b[1m{title}\x1b[0m ({})", rows.len()).map_err(io)?;
                for (name, b) in rows {
                    writeln!(stdout, "  {name:<26} \x1b[2m{}\x1b[0m", biome_names(b))
                        .map_err(io)?;
                }
                writeln!(stdout).map_err(io)?;
            }
            Ok(())
        }
        Some(Cmd::Gallery { filter, ansi }) => write!(
            stdout,
            "{}",
            scenes::aquarium::assets::gallery(&filter, ansi)
        )
        .map_err(io),
        Some(Cmd::Docs) => {
            let md = |s: &str| s.replace('|', "\\|");
            for info in scenes::ALL {
                let aliases = info
                    .aliases
                    .iter()
                    .map(|a| format!("`{a}`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                writeln!(
                    stdout,
                    "<details>\n<summary><b>{}</b> — {}</summary>\n",
                    info.name, info.about
                )
                .map_err(io)?;
                if !aliases.is_empty() {
                    writeln!(stdout, "Aliases: {aliases}\n").map_err(io)?;
                }
                writeln!(stdout, "| option | default | |\n|---|---|---|").map_err(io)?;
                for o in info.options {
                    writeln!(
                        stdout,
                        "| `{}` | `{}` | {} |",
                        o.key,
                        md(&o.default.to_toml().to_string()),
                        md(o.help)
                    )
                    .map_err(io)?;
                }
                writeln!(stdout, "\n</details>\n").map_err(io)?;
            }
            Ok(())
        }
        Some(Cmd::Completions { shell }) => {
            clap_complete::generate(shell, &mut Cli::command(), "oxiper", &mut stdout);
            Ok(())
        }
        Some(Cmd::IdleWatch {
            pid,
            idle,
            takeover,
        }) => {
            let env_idle: Option<u64> = idle.trim().parse().ok().filter(|v| *v > 0);
            let idle_fn = || {
                env_idle
                    .or_else(|| {
                        Config::load(path.as_deref(), &[], true)
                            .ok()
                            .map(|c| c.idle)
                    })
                    .unwrap_or(120)
            };
            let saver = || {
                let cfg = Config::load(path.as_deref(), &[], true)?;
                app::run(
                    cfg,
                    app::RunOpts {
                        config_path: path.clone(),
                        overrides: vec![],
                        saver: true,
                    },
                )
            };
            shell::idle_watch(pid, &idle_fn, takeover, &saver)
        }
    }
}
