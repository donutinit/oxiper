//! Configuration: defaults ← config file ← `[saver]` section (in saver mode) ←
//! command-line flags and `-o key=value` overrides.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::color::{Color, ColorMode};
use crate::scene::{Opt, SceneInfo, Val};
use crate::scenes;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Background {
    /// Never paint a background: the terminal's own (possibly translucent) one shows.
    Transparent,
    /// Paint the theme's background color.
    Theme,
    Color(Color),
}

pub struct Config {
    pub scenes: Vec<&'static SceneInfo>,
    pub duration: f64,
    pub shuffle: bool,
    pub theme: String,
    pub fps: u32,
    pub unfocused_fps: u32,
    pub speed: f64,
    pub color: Option<ColorMode>,
    pub background: Background,
    pub strict: bool,
    pub bg_hint: Option<Color>,
    pub hud: bool,
    pub seed: Option<u64>,
    pub exit_on_mouse: bool,
    /// Easter eggs allowed to play (see `crate::eggs`).
    pub eggs: Vec<&'static crate::eggs::Info>,
    /// Average eggs per hour of real playing time.
    pub egg_rate: f64,
    pub idle: u64,
    /// The merged table, for scene options and user themes.
    pub raw: toml::Table,
    pub path: Option<PathBuf>,
    pub mtime: Option<SystemTime>,
    pub warnings: Vec<String>,
}

/// Top-level keys, in the order `config init` documents them.
pub const GLOBAL: &[Opt] = &[
    Opt {
        key: "scene",
        default: Val::S("all"),
        help: "Scene to show: a name, a list like [\"bonsai\", \"neo\"], or \"all\"",
    },
    Opt {
        key: "duration",
        default: Val::F(60.0),
        help: "Seconds per scene when several are selected (0 = stay on the first)",
    },
    Opt {
        key: "shuffle",
        default: Val::B(false),
        help: "Play the selected scenes in random order",
    },
    Opt {
        key: "theme",
        default: Val::S("classic"),
        help: "Color theme; \"classic\" keeps each scene's original colors, \"terminal\" follows your terminal scheme",
    },
    Opt {
        key: "fps",
        default: Val::I(30),
        help: "Target frames per second",
    },
    Opt {
        key: "unfocused_fps",
        default: Val::I(10),
        help: "Frame rate while the terminal window is unfocused (0 = pause)",
    },
    Opt {
        key: "speed",
        default: Val::F(1.0),
        help: "Global animation speed multiplier (motion only: clocks such as a biome's stay or a day keep real time)",
    },
    Opt {
        key: "color",
        default: Val::S("auto"),
        help: "Color depth: auto, truecolor, 256, 16 or mono (NO_COLOR is honored)",
    },
    Opt {
        key: "background",
        default: Val::S("transparent"),
        help: "transparent (terminal shows through), theme, or a color like \"#101018\"",
    },
    Opt {
        key: "strict_transparency",
        default: Val::B(false),
        help: "Never paint any cell background, not even inside half-block pixel art",
    },
    Opt {
        key: "bg_hint",
        default: Val::S("auto"),
        help: "Color trails fade toward; auto asks the terminal for its background",
    },
    Opt {
        key: "hud",
        default: Val::B(true),
        help: "Briefly show scene/theme names when switching",
    },
    Opt {
        key: "seed",
        default: Val::I(0),
        help: "Random seed (0 = different every run)",
    },
    Opt {
        key: "exit_on_mouse",
        default: Val::B(false),
        help: "In saver mode, also wake up on mouse clicks",
    },
    Opt {
        key: "eggs",
        default: Val::S("all"),
        help: "Easter eggs that may appear now and then: \"all\", \"none\", or names like \"sl, duckpond\"",
    },
    Opt {
        key: "egg_rate",
        default: Val::F(1.2),
        help: "Average easter eggs per hour of playing time (0 = never; never in the first minute)",
    },
];

/// Keys every `[scenes.<name>]` table accepts on top of the scene's own options.
pub const PER_SCENE: &[Opt] = &[
    Opt {
        key: "theme",
        default: Val::S(""),
        help: "Theme for this scene only",
    },
    Opt {
        key: "pace",
        default: Val::F(1.0),
        help: "Speed multiplier for this scene only, on top of the global speed (motion only)",
    },
    Opt {
        key: "duration",
        default: Val::F(60.0),
        help: "Seconds this scene stays on when cycling",
    },
];

pub const SAVER: &[Opt] = &[Opt {
    key: "idle",
    default: Val::I(120),
    help: "Seconds without typing before the shell integration starts the saver",
}];

pub fn default_path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| Path::new(&h).join(".config")))?;
    Some(base.join("oxiper").join("config.toml"))
}

/// A parsed `-o key=value`: where it goes in the table and what it is.
#[derive(Clone, Debug)]
pub struct Override {
    pub path: Vec<String>,
    pub value: toml::Value,
}

impl Override {
    /// `neo.density=0.5` → `scenes.neo.density`, `fps=60` → `fps`,
    /// `saver.idle=60` → `saver.idle`. Values are TOML; bare words become strings.
    pub fn parse(s: &str) -> Result<Override, String> {
        let (key, val) = s
            .split_once('=')
            .ok_or(format!("expected KEY=VALUE, got '{s}'"))?;
        let mut path: Vec<String> = key
            .trim()
            .split('.')
            .map(|p| p.trim().to_string())
            .collect();
        if path.iter().any(|p| p.is_empty()) {
            return Err(format!("bad key '{key}'"));
        }
        if path.len() > 1
            && let Some(info) = scenes::find(&path[0])
        {
            path[0] = info.name.to_string();
            path.insert(0, "scenes".into());
        }
        let value = format!("v = {}", val.trim())
            .parse::<toml::Table>()
            .ok()
            .and_then(|mut t| t.remove("v"))
            .unwrap_or_else(|| toml::Value::String(val.trim().to_string()));
        Ok(Override { path, value })
    }

    pub fn new(key: &str, value: toml::Value) -> Override {
        Override {
            path: vec![key.to_string()],
            value,
        }
    }
}

fn set_path(t: &mut toml::Table, path: &[String], v: toml::Value) {
    if path.len() == 1 {
        t.insert(path[0].clone(), v);
        return;
    }
    let entry = t
        .entry(path[0].clone())
        .or_insert_with(|| toml::Value::Table(toml::Table::new()));
    if !entry.is_table() {
        *entry = toml::Value::Table(toml::Table::new());
    }
    set_path(entry.as_table_mut().unwrap(), &path[1..], v);
}

/// Deep-merges `src` into `dst` (tables merge, everything else replaces).
fn merge(dst: &mut toml::Table, src: &toml::Table) {
    for (k, v) in src {
        match (dst.get_mut(k), v) {
            (Some(toml::Value::Table(d)), toml::Value::Table(s)) => merge(d, s),
            _ => {
                dst.insert(k.clone(), v.clone());
            }
        }
    }
}

impl Config {
    pub fn load(
        path: Option<&Path>,
        overrides: &[Override],
        saver: bool,
    ) -> Result<Config, String> {
        let explicit = path.is_some();
        let path = path.map(Path::to_path_buf).or_else(default_path);
        let mut raw = toml::Table::new();
        let mut mtime = None;
        if let Some(p) = &path {
            match std::fs::read_to_string(p) {
                Ok(text) => {
                    raw = text
                        .parse::<toml::Table>()
                        .map_err(|e| format!("{}: {e}", p.display()))?;
                    mtime = std::fs::metadata(p).and_then(|m| m.modified()).ok();
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound && !explicit => {}
                Err(e) => return Err(format!("{}: {e}", p.display())),
            }
        }
        if saver && let Some(toml::Value::Table(s)) = raw.get("saver").cloned() {
            let mut s = s;
            s.remove("idle");
            merge(&mut raw, &s);
        }
        for o in overrides {
            set_path(&mut raw, &o.path, o.value.clone());
        }
        Config::resolve(raw, path, mtime)
    }

    fn resolve(
        raw: toml::Table,
        path: Option<PathBuf>,
        mtime: Option<SystemTime>,
    ) -> Result<Config, String> {
        let mut warnings = Vec::new();
        let get = |k: &str| raw.get(k);
        let spec = |k: &str| GLOBAL.iter().find(|o| o.key == k).unwrap().default;
        let num = |k: &str, w: &mut Vec<String>| -> f64 {
            match get(k) {
                Some(toml::Value::Float(f)) => *f,
                Some(toml::Value::Integer(i)) => *i as f64,
                Some(v) => {
                    w.push(format!("'{k}' should be a number, got {v}"));
                    default_num(spec(k))
                }
                None => default_num(spec(k)),
            }
        };
        let boolean = |k: &str, w: &mut Vec<String>| -> bool {
            match get(k) {
                Some(toml::Value::Boolean(b)) => *b,
                Some(v) => {
                    w.push(format!("'{k}' should be true or false, got {v}"));
                    matches!(spec(k), Val::B(true))
                }
                None => matches!(spec(k), Val::B(true)),
            }
        };
        let string = |k: &str, w: &mut Vec<String>| -> String {
            match get(k) {
                Some(toml::Value::String(s)) => s.clone(),
                Some(v) => {
                    w.push(format!("'{k}' should be a string, got {v}"));
                    default_str(spec(k))
                }
                None => default_str(spec(k)),
            }
        };

        let names: Vec<String> = match get("scene") {
            None => vec!["all".into()],
            Some(toml::Value::String(s)) => s
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            Some(toml::Value::Array(a)) => a
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect(),
            Some(v) => {
                return Err(format!(
                    "'scene' should be a name or a list of names, got {v}"
                ));
            }
        };
        let mut list: Vec<&'static SceneInfo> = Vec::new();
        for n in &names {
            if n == "all" {
                list.extend(scenes::ALL.iter().copied());
            } else {
                let info = scenes::find(n).ok_or_else(|| {
                    format!(
                        "unknown scene '{n}' (available: {})",
                        scenes::ALL
                            .iter()
                            .map(|s| s.name)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })?;
                list.push(info);
            }
        }
        if list.is_empty() {
            list.extend(scenes::ALL.iter().copied());
        }

        let color = string("color", &mut warnings);
        let color = ColorMode::parse(&color).unwrap_or_else(|| {
            warnings.push(format!("unknown color mode '{color}', using auto"));
            None
        });
        let bg = string("background", &mut warnings);
        let background = match bg.to_ascii_lowercase().as_str() {
            "transparent" | "none" | "default" => Background::Transparent,
            "theme" | "solid" => Background::Theme,
            _ => match Color::parse(&bg) {
                Some(c) => Background::Color(c),
                None => {
                    warnings.push(format!("bad background '{bg}', using transparent"));
                    Background::Transparent
                }
            },
        };
        let hint = string("bg_hint", &mut warnings);
        let bg_hint = match hint.as_str() {
            "auto" | "" => None,
            h => Color::parse(h).or_else(|| {
                warnings.push(format!("bad bg_hint '{h}', using auto"));
                None
            }),
        };
        let seed = num("seed", &mut warnings) as u64;
        let idle = match raw.get("saver").and_then(|s| s.get("idle")) {
            Some(toml::Value::Integer(i)) if *i > 0 => *i as u64,
            Some(v) => {
                warnings.push(format!(
                    "'saver.idle' should be a positive integer, got {v}"
                ));
                120
            }
            None => 120,
        };

        // Point out typos instead of silently ignoring them.
        for (k, v) in &raw {
            match k.as_str() {
                "scenes" => {
                    let Some(t) = v.as_table() else { continue };
                    for (name, opts) in t {
                        let Some(info) = scenes::find(name) else {
                            warnings.push(format!("[scenes.{name}]: no such scene"));
                            continue;
                        };
                        let Some(opts) = opts.as_table() else {
                            continue;
                        };
                        for (ok, ov) in opts {
                            match info.options.iter().chain(PER_SCENE).find(|o| o.key == ok) {
                                None => warnings.push(format!("scenes.{name}.{ok}: unknown option (see `oxiper list options {name}`)")),
                                Some(o) if !o.default.accepts(ov) => {
                                    warnings.push(format!("scenes.{name}.{ok}: expected {}, got {ov}", o.default.type_name()))
                                }
                                _ => {}
                            }
                        }
                    }
                }
                "themes" | "saver" => {}
                _ if GLOBAL.iter().any(|o| o.key == k) => {}
                _ => warnings.push(format!("unknown setting '{k}'")),
            }
        }

        let eggs = crate::eggs::parse(&string("eggs", &mut warnings), &mut warnings);
        Ok(Config {
            scenes: list,
            duration: num("duration", &mut warnings).max(0.0),
            shuffle: boolean("shuffle", &mut warnings),
            theme: string("theme", &mut warnings),
            fps: (num("fps", &mut warnings) as u32).clamp(1, 240),
            unfocused_fps: (num("unfocused_fps", &mut warnings) as u32).min(240),
            speed: num("speed", &mut warnings).clamp(0.01, 50.0),
            color,
            background,
            strict: boolean("strict_transparency", &mut warnings),
            bg_hint,
            hud: boolean("hud", &mut warnings),
            seed: (seed != 0).then_some(seed),
            exit_on_mouse: boolean("exit_on_mouse", &mut warnings),
            eggs,
            egg_rate: num("egg_rate", &mut warnings).max(0.0),
            idle,
            raw,
            path,
            mtime,
            warnings,
        })
    }

    /// The `[scenes.<name>]` table for a scene (empty if absent).
    pub fn scene_table(&self, name: &str) -> toml::Table {
        self.raw
            .get("scenes")
            .and_then(|s| s.get(name))
            .and_then(|t| t.as_table())
            .cloned()
            .unwrap_or_default()
    }

    pub fn theme_table(&self, name: &str) -> Option<&toml::Table> {
        self.raw.get("themes")?.get(name)?.as_table()
    }

    pub fn custom_theme_names(&self) -> Vec<String> {
        self.raw
            .get("themes")
            .and_then(|t| t.as_table())
            .map(|t| t.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Has the config file changed on disk since it was loaded?
    pub fn changed_on_disk(&self) -> bool {
        let Some(p) = &self.path else { return false };
        let now = std::fs::metadata(p).and_then(|m| m.modified()).ok();
        now != self.mtime
    }
}

fn default_num(v: Val) -> f64 {
    match v {
        Val::F(f) => f,
        Val::I(i) => i as f64,
        _ => 0.0,
    }
}

fn default_str(v: Val) -> String {
    match v {
        Val::S(s) => s.to_string(),
        _ => String::new(),
    }
}

fn fmt_val(v: Val) -> String {
    match v {
        Val::F(f) if f.fract() == 0.0 => format!("{f:.1}"),
        Val::F(f) => format!("{f}"),
        Val::I(i) => i.to_string(),
        Val::B(b) => b.to_string(),
        Val::S(s) => format!("{s:?}"),
    }
}

fn opt_lines(out: &mut String, opts: &[Opt], commented: bool) {
    let width = opts
        .iter()
        .map(|o| o.key.len() + fmt_val(o.default).len() + 3)
        .max()
        .unwrap_or(0);
    for o in opts {
        let kv = format!("{} = {}", o.key, fmt_val(o.default));
        let pre = if commented { "# " } else { "" };
        out.push_str(&format!("{pre}{kv:<width$}  # {}\n", o.help));
    }
}

/// A fully commented config file listing every setting and its default.
pub fn template() -> String {
    let mut s = String::from(
        "# oxiper configuration\n\
         # Every setting is optional and shows its default. Changes are picked up\n\
         # live while oxiper runs. Any key can also be set for one run with\n\
         # `oxiper -o key=value` (scene options as `-o neo.density=0.8`).\n\n",
    );
    opt_lines(&mut s, GLOBAL, true);
    s.push_str(
        "\n# Overrides used when the shell integration starts oxiper as a screensaver.\n\
         # Any top-level setting can go here too (scene, theme, duration...).\n# [saver]\n",
    );
    opt_lines(&mut s, SAVER, true);
    s.push_str("# scene = \"all\"\n# shuffle = true\n# duration = 30.0\n");
    s.push_str(
        "\n# Your own themes. `inherit` copies a theme; then override what you like.\n\
         # ramp = colors from dark to bright (gradients); colors = distinct accents.\n\
         # [themes.midnight]\n\
         # inherit = \"tokyonight\"\n\
         # ramp = [\"#0b1021\", \"#1f3b73\", \"#4f7cff\", \"#a9c1ff\"]\n\
         # colors = [\"#4f7cff\", \"#ff6ac1\", \"#5af78e\", \"#f3f99d\"]\n\
         # fg = \"#c0caf5\"\n\
         # bg = \"#0b1021\"\n",
    );
    s.push_str(
        "\n# Scene sections. Besides its own options, every [scenes.<name>] also\n\
         # accepts these, overriding the global value while that scene plays:\n",
    );
    opt_lines(&mut s, PER_SCENE, true);
    for info in scenes::ALL {
        s.push_str(&format!(
            "\n# {} — {}\n# [scenes.{}]\n",
            info.name, info.about, info.name
        ));
        opt_lines(&mut s, info.options, true);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overrides_route_to_scenes() {
        let o = Override::parse("matrix.density=0.5").unwrap();
        assert_eq!(o.path, ["scenes", "neo", "density"]);
        assert_eq!(o.value, toml::Value::Float(0.5));
        let o = Override::parse("theme=nord").unwrap();
        assert_eq!(o.path, ["theme"]);
        assert_eq!(o.value, toml::Value::String("nord".into()));
        assert_eq!(
            Override::parse("saver.idle=30").unwrap().path,
            ["saver", "idle"]
        );
    }

    #[test]
    fn loads_and_layers() {
        let dir = std::env::temp_dir().join(format!("oxiper-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("c.toml");
        std::fs::write(&p, "scene = 'neo'\nfps = 20\nbogus = 1\n[saver]\nidle = 30\nfps = 5\n[scenes.neo]\nnope = 1\n").unwrap();
        let c = Config::load(Some(&p), &[], false).unwrap();
        assert_eq!((c.fps, c.idle, c.scenes[0].name), (20, 30, "neo"));
        assert!(c.warnings.iter().any(|w| w.contains("bogus")));
        assert!(c.warnings.iter().any(|w| w.contains("nope")));
        let c = Config::load(Some(&p), &[Override::parse("fps=60").unwrap()], true).unwrap();
        assert_eq!(c.fps, 60);
        let c = Config::load(Some(&p), &[], true).unwrap();
        assert_eq!(c.fps, 5);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn template_parses() {
        let t: toml::Table = template().parse().unwrap();
        assert!(t.is_empty());
    }
}
