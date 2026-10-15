//! Color themes. A theme is a ramp (dark → bright, used for gradients) plus a
//! set of distinct accent colors (used for things that need to differ, like
//! pipes or planets). The `classic` theme lets every scene keep the colors of
//! the program it pays homage to.

use crate::color::{Color, Gradient, rgb};

pub struct Theme {
    pub name: String,
    /// When true, scenes use their own native colors.
    pub native: bool,
    pub ramp: Vec<Color>,
    pub colors: Vec<Color>,
    pub fg: Color,
    pub bg: Color,
    pub grad: Gradient,
}

struct Def {
    name: &'static str,
    bg: u32,
    fg: u32,
    ramp: &'static [u32],
    colors: &'static [u32],
}

const DEFS: &[Def] = &[
    Def {
        name: "catppuccin",
        bg: 0x1e1e2e,
        fg: 0xcdd6f4,
        ramp: &[0x313244, 0x89b4fa, 0xcba6f7, 0xf5c2e7, 0xf5e0dc],
        colors: &[
            0xcba6f7, 0x89b4fa, 0x74c7ec, 0xa6e3a1, 0xf9e2af, 0xfab387, 0xf38ba8, 0xf5c2e7,
            0x94e2d5,
        ],
    },
    Def {
        name: "dracula",
        bg: 0x282a36,
        fg: 0xf8f8f2,
        ramp: &[0x44475a, 0x6272a4, 0xbd93f9, 0xff79c6, 0xf8f8f2],
        colors: &[
            0xbd93f9, 0xff79c6, 0x8be9fd, 0x50fa7b, 0xffb86c, 0xf1fa8c, 0xff5555,
        ],
    },
    Def {
        name: "gruvbox",
        bg: 0x282828,
        fg: 0xebdbb2,
        ramp: &[0x3c3836, 0xaf3a03, 0xfe8019, 0xfabd2f, 0xfbf1c7],
        colors: &[
            0xfabd2f, 0xb8bb26, 0xfe8019, 0x8ec07c, 0x83a598, 0xd3869b, 0xfb4934,
        ],
    },
    Def {
        name: "nord",
        bg: 0x2e3440,
        fg: 0xd8dee9,
        ramp: &[0x3b4252, 0x5e81ac, 0x81a1c1, 0x88c0d0, 0xeceff4],
        colors: &[
            0x88c0d0, 0x81a1c1, 0x5e81ac, 0x8fbcbb, 0xa3be8c, 0xebcb8b, 0xd08770, 0xb48ead,
            0xbf616a,
        ],
    },
    Def {
        name: "rosepine",
        bg: 0x191724,
        fg: 0xe0def4,
        ramp: &[0x26233a, 0x31748f, 0x9ccfd8, 0xc4a7e7, 0xebbcba],
        colors: &[0xeb6f92, 0xf6c177, 0xebbcba, 0x31748f, 0x9ccfd8, 0xc4a7e7],
    },
    Def {
        name: "tokyonight",
        bg: 0x1a1b26,
        fg: 0xc0caf5,
        ramp: &[0x24283b, 0x3d59a1, 0x7aa2f7, 0xbb9af7, 0xc0caf5],
        colors: &[
            0x7aa2f7, 0xbb9af7, 0x7dcfff, 0x9ece6a, 0xe0af68, 0xff9e64, 0xf7768e, 0x2ac3de,
        ],
    },
    Def {
        name: "kanagawa",
        bg: 0x1f1f28,
        fg: 0xdcd7ba,
        ramp: &[0x2a2a37, 0x223249, 0x7e9cd8, 0x7fb4ca, 0xdcd7ba],
        colors: &[
            0x7e9cd8, 0x957fb8, 0x7fb4ca, 0x98bb6c, 0xe6c384, 0xffa066, 0xe46876, 0xd27e99,
        ],
    },
    Def {
        name: "everforest",
        bg: 0x2d353b,
        fg: 0xd3c6aa,
        ramp: &[0x343f44, 0x425047, 0x83c092, 0xa7c080, 0xd3c6aa],
        colors: &[
            0xa7c080, 0x83c092, 0x7fbbb3, 0xdbbc7f, 0xe69875, 0xd699b6, 0xe67e80,
        ],
    },
    Def {
        name: "solarized",
        bg: 0x002b36,
        fg: 0x839496,
        ramp: &[0x073642, 0x268bd2, 0x2aa198, 0x859900, 0xb58900],
        colors: &[
            0xb58900, 0xcb4b16, 0xdc322f, 0xd33682, 0x6c71c4, 0x268bd2, 0x2aa198, 0x859900,
        ],
    },
    Def {
        name: "onedark",
        bg: 0x282c34,
        fg: 0xabb2bf,
        ramp: &[0x3e4451, 0x61afef, 0xc678dd, 0xe06c75, 0xe5c07b],
        colors: &[
            0x61afef, 0xc678dd, 0x56b6c2, 0x98c379, 0xe5c07b, 0xd19a66, 0xe06c75,
        ],
    },
    Def {
        name: "monokai",
        bg: 0x272822,
        fg: 0xf8f8f2,
        ramp: &[0x49483e, 0x66d9ef, 0xae81ff, 0xf92672, 0xe6db74],
        colors: &[0xf92672, 0xa6e22e, 0xfd971f, 0xe6db74, 0x66d9ef, 0xae81ff],
    },
    Def {
        name: "synthwave",
        bg: 0x241b2f,
        fg: 0xf4eee4,
        ramp: &[0x2a1a3e, 0x7b2cbf, 0xff7edb, 0xf97e72, 0xfede5d],
        colors: &[
            0xff7edb, 0xf97e72, 0xfede5d, 0x72f1b8, 0x36f9f6, 0xfe4450, 0xb084eb,
        ],
    },
    Def {
        name: "matrix",
        bg: 0x000000,
        fg: 0x00ff41,
        ramp: &[0x001a00, 0x003b00, 0x008f11, 0x00ff41, 0xccffcc],
        colors: &[0x00ff41, 0x39ff14, 0x008f11, 0x7dff9b, 0x00c832],
    },
    Def {
        name: "amber",
        bg: 0x0d0700,
        fg: 0xffb000,
        ramp: &[0x1a0f00, 0x663c00, 0xb36b00, 0xffb000, 0xffd98a],
        colors: &[0xffb000, 0xffcc4d, 0xcc8400, 0xff9500, 0xffe0a0],
    },
    Def {
        name: "ember",
        bg: 0x120400,
        fg: 0xffd27f,
        ramp: &[0x2a0500, 0x7a1200, 0xd43f00, 0xff8c00, 0xffe08a],
        colors: &[0xff4d00, 0xff8c00, 0xffb703, 0xd43f00, 0xffe08a, 0x9d0208],
    },
    Def {
        name: "ocean",
        bg: 0x020c1b,
        fg: 0xcaf0f8,
        ramp: &[0x03045e, 0x0077b6, 0x00b4d8, 0x90e0ef, 0xcaf0f8],
        colors: &[0x00b4d8, 0x48cae4, 0x0096c7, 0x90e0ef, 0x2ec4b6, 0xcaf0f8],
    },
    Def {
        name: "forest",
        bg: 0x0b1a10,
        fg: 0xd8f3dc,
        ramp: &[0x0b2014, 0x1b4332, 0x2d6a4f, 0x52b788, 0xb7e4c7],
        colors: &[0x52b788, 0x74c69d, 0x95d5b2, 0x40916c, 0xb5c99a, 0xe9c46a],
    },
    Def {
        name: "sakura",
        bg: 0x1f1018,
        fg: 0xffe4ee,
        ramp: &[0x3d1c2e, 0x9d4e78, 0xe07aa4, 0xf7b2c9, 0xffe4ee],
        colors: &[0xf7b2c9, 0xe07aa4, 0xffc8dd, 0xcdb4db, 0xbde0fe, 0xffafcc],
    },
    Def {
        name: "vaporwave",
        bg: 0x1a1033,
        fg: 0xf0e6ff,
        ramp: &[0x2d1b69, 0x5f4bb6, 0xff71ce, 0x01cdfe, 0x05ffa1],
        colors: &[0xff71ce, 0x01cdfe, 0x05ffa1, 0xb967ff, 0xfffb96],
    },
    Def {
        name: "mono",
        bg: 0x101010,
        fg: 0xe0e0e0,
        ramp: &[0x2a2a2a, 0x555555, 0x888888, 0xbbbbbb, 0xf0f0f0],
        colors: &[0xf0f0f0, 0xc8c8c8, 0xa0a0a0, 0x787878, 0xdcdcdc],
    },
];

/// Themes that need no outside information, plus the dynamic ones.
pub fn names() -> Vec<&'static str> {
    let mut v = vec!["classic", "terminal", "wal"];
    v.extend(DEFS.iter().map(|d| d.name));
    v
}

impl Theme {
    fn build(
        name: &str,
        native: bool,
        ramp: Vec<Color>,
        colors: Vec<Color>,
        fg: Color,
        bg: Color,
    ) -> Theme {
        let grad = Gradient::new(&ramp);
        Theme {
            name: name.to_string(),
            native,
            ramp,
            colors,
            fg,
            bg,
            grad,
        }
    }

    pub fn classic() -> Theme {
        // The ramp only matters for HUD text and scenes without native colors.
        Theme::build(
            "classic",
            true,
            vec![rgb(0x1d3557), rgb(0x457b9d), rgb(0xa8dadc), rgb(0xf1faee)],
            vec![
                rgb(0xe63946),
                rgb(0xf4a261),
                rgb(0xe9c46a),
                rgb(0x2a9d8f),
                rgb(0x457b9d),
                rgb(0xa8dadc),
                rgb(0xb5838d),
            ],
            Color::Reset,
            Color::Reset,
        )
    }

    /// Follows the terminal's own color scheme. With the RGB values the
    /// terminal reported, gradients stay smooth; without them we fall back to
    /// palette indices, which the terminal resolves itself.
    pub fn terminal(t: &TermColors) -> Theme {
        let p = |i: usize| t.palette.as_ref().map_or(Color::Ansi(i as u8), |p| p[i]);
        let colors = vec![
            p(4),
            p(5),
            p(6),
            p(2),
            p(3),
            p(1),
            p(12),
            p(13),
            p(14),
            p(10),
            p(11),
            p(9),
        ];
        let ramp = vec![p(4), p(12), p(6), p(14), p(15)];
        let fg = t.fg.unwrap_or(Color::Reset);
        Theme::build(
            "terminal",
            false,
            ramp,
            colors,
            fg,
            t.bg.unwrap_or(Color::Reset),
        )
    }

    /// Reads pywal's generated scheme from ~/.cache/wal/colors.json.
    pub fn wal() -> Result<Theme, String> {
        let home = std::env::var_os("HOME").ok_or("HOME is not set")?;
        let path = std::path::Path::new(&home).join(".cache/wal/colors.json");
        let text =
            std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let get = |key: &str| -> Option<Color> {
            let at = text.find(&format!("\"{key}\""))?;
            let rest = &text[at + key.len() + 2..];
            let start = rest.find('"')? + 1;
            let end = start + rest[start..].find('"')?;
            Color::parse(&rest[start..end])
        };
        let c: Vec<Color> = (0..16)
            .map(|i| get(&format!("color{i}")).unwrap_or(Color::Ansi(i as u8)))
            .collect();
        let mut ramp: Vec<Color> = c[1..7].to_vec();
        ramp.sort_by(|a, b| a.luma().total_cmp(&b.luma()));
        ramp.push(c[15]);
        let colors = c[1..7].iter().chain(&c[9..15]).copied().collect();
        Ok(Theme::build(
            "wal",
            false,
            ramp,
            colors,
            get("foreground").unwrap_or(c[15]),
            get("background").unwrap_or(c[0]),
        ))
    }

    pub fn builtin(name: &str) -> Option<Theme> {
        if name == "classic" {
            return Some(Theme::classic());
        }
        let d = DEFS.iter().find(|d| d.name == name)?;
        Some(Theme::build(
            d.name,
            false,
            d.ramp.iter().map(|&h| rgb(h)).collect(),
            d.colors.iter().map(|&h| rgb(h)).collect(),
            rgb(d.fg),
            rgb(d.bg),
        ))
    }

    /// A user theme from a `[themes.<name>]` table. It can `inherit` another
    /// theme and override any of `ramp`, `colors`, `fg`, `bg`.
    pub fn from_table(name: &str, t: &toml::Table, base: Option<Theme>) -> Result<Theme, String> {
        let colors_of = |key: &str| -> Result<Option<Vec<Color>>, String> {
            match t.get(key) {
                None => Ok(None),
                Some(toml::Value::Array(a)) => a
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .and_then(Color::parse)
                            .ok_or(format!("themes.{name}.{key}: bad color {v}"))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(Some),
                Some(v) => Err(format!(
                    "themes.{name}.{key}: expected a list of colors, got {v}"
                )),
            }
        };
        let one = |key: &str| -> Result<Option<Color>, String> {
            match t.get(key) {
                None => Ok(None),
                Some(v) => v
                    .as_str()
                    .and_then(Color::parse)
                    .map(Some)
                    .ok_or(format!("themes.{name}.{key}: bad color {v}")),
            }
        };
        let base = base.unwrap_or_else(|| Theme::builtin("mono").unwrap());
        let ramp = colors_of("ramp")?.or(colors_of("palette")?);
        let colors = colors_of("colors")?;
        let (ramp, colors) = match (ramp, colors) {
            (Some(r), Some(c)) => (r, c),
            (Some(r), None) => (r.clone(), r),
            (None, Some(c)) => (c.clone(), c),
            (None, None) => (base.ramp.clone(), base.colors.clone()),
        };
        if ramp.is_empty() || colors.is_empty() {
            return Err(format!("themes.{name}: needs at least one color"));
        }
        let native = t.get("native").and_then(|v| v.as_bool()).unwrap_or(false);
        Ok(Theme::build(
            name,
            native,
            ramp,
            colors,
            one("fg")?.unwrap_or(base.fg),
            one("bg")?.unwrap_or(base.bg),
        ))
    }

    /// The i-th accent color, or `native` under the classic theme.
    #[inline]
    pub fn pick(&self, i: usize, native: Color) -> Color {
        if self.native {
            native
        } else {
            self.colors[i % self.colors.len()]
        }
    }

    /// A gradient for a scene: its own stops under `classic`, the theme ramp otherwise.
    pub fn ramp_or(&self, native: &[Color]) -> Gradient {
        if self.native {
            Gradient::new(native)
        } else {
            self.grad.clone()
        }
    }

    /// Swatch line for `oxiper list themes`.
    pub fn swatch(&self) -> String {
        let mut s = String::new();
        for c in self
            .ramp
            .iter()
            .chain(std::iter::once(&Color::Reset))
            .chain(self.colors.iter())
        {
            match *c {
                Color::Rgb(r, g, b) => s.push_str(&format!("\x1b[38;2;{r};{g};{b}m██")),
                Color::Ansi(n) => s.push_str(&format!("\x1b[38;5;{n}m██")),
                Color::Reset => s.push_str("\x1b[0m "),
            }
        }
        s.push_str("\x1b[0m");
        s
    }
}

/// What the terminal told us about its colors (OSC 10/11/4), when it answered.
#[derive(Default, Clone)]
pub struct TermColors {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub palette: Option<[Color; 16]>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_builtin_resolves() {
        for n in names() {
            if n == "terminal" || n == "wal" {
                continue;
            }
            let t = Theme::builtin(n).unwrap_or_else(|| panic!("{n}"));
            assert!(!t.colors.is_empty() && !t.ramp.is_empty());
        }
    }

    #[test]
    fn custom_theme_inherits() {
        let t: toml::Table = "inherit = 'nord'\nfg = '#ff0000'".parse().unwrap();
        let th = Theme::from_table("mine", &t, Theme::builtin("nord")).unwrap();
        assert_eq!(th.fg, Color::Rgb(255, 0, 0));
        assert_eq!(th.ramp, Theme::builtin("nord").unwrap().ramp);
    }
}
