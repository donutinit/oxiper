//! Colors, gradients and terminal color-depth quantization.

/// A terminal color. `Reset` is the terminal's own default, which is what keeps
/// a transparent terminal background transparent.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Color {
    #[default]
    Reset,
    /// One of the terminal's 256 palette entries (0-15 follow the user's color scheme).
    Ansi(u8),
    Rgb(u8, u8, u8),
}

pub const fn rgb(hex: u32) -> Color {
    Color::Rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

impl Color {
    /// Parses `#rrggbb`, `#rgb`, `rrggbb`, `ansi:N` / `N` (0-255) or `default`.
    pub fn parse(s: &str) -> Option<Color> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("default")
            || s.eq_ignore_ascii_case("reset")
            || s.eq_ignore_ascii_case("none")
        {
            return Some(Color::Reset);
        }
        if let Some(n) = s.strip_prefix("ansi:").or(s.strip_prefix("ansi")) {
            return n.trim().parse().ok().map(Color::Ansi);
        }
        let h = s.strip_prefix('#').unwrap_or(s);
        if !h.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        match h.len() {
            6 => u32::from_str_radix(h, 16).ok().map(rgb),
            3 => {
                let v = u32::from_str_radix(h, 16).ok()?;
                let (r, g, b) = ((v >> 8) & 15, (v >> 4) & 15, v & 15);
                Some(Color::Rgb((r * 17) as u8, (g * 17) as u8, (b * 17) as u8))
            }
            1..=3 if s.len() == h.len() => h.parse().ok().map(Color::Ansi),
            _ => None,
        }
    }

    pub fn is_rgb(self) -> bool {
        matches!(self, Color::Rgb(..))
    }

    /// Linear interpolation in sRGB. Non-RGB colors cannot be mixed, so they step at the midpoint.
    #[inline]
    pub fn lerp(self, other: Color, t: f32) -> Color {
        match (self, other) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
                let t = t.clamp(0.0, 1.0);
                let m = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t + 0.5) as u8;
                Color::Rgb(m(r1, r2), m(g1, g2), m(b1, b2))
            }
            _ if t < 0.5 => self,
            _ => other,
        }
    }

    /// Multiplies brightness. Values above 1 push toward white.
    #[inline]
    pub fn scale(self, k: f32) -> Color {
        match self {
            Color::Rgb(r, g, b) if k <= 1.0 => {
                let k = k.max(0.0);
                Color::Rgb(
                    (r as f32 * k) as u8,
                    (g as f32 * k) as u8,
                    (b as f32 * k) as u8,
                )
            }
            Color::Rgb(..) => self.lerp(Color::Rgb(255, 255, 255), (k - 1.0).min(1.0)),
            c => c,
        }
    }

    /// Perceived brightness in 0..1 (Reset and palette colors report 0.5).
    pub fn luma(self) -> f32 {
        match self {
            Color::Rgb(r, g, b) => {
                (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) / 255.0
            }
            Color::Ansi(n) if n == 0 || n == 8 || n == 16 => 0.1,
            Color::Ansi(n) if n == 7 || n == 15 || n == 231 => 0.9,
            _ => 0.5,
        }
    }
}

fn srgb_to_linear(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(c: f32) -> u8 {
    let c = c.clamp(0.0, 1.0);
    let s = if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (s * 255.0 + 0.5) as u8
}

#[allow(clippy::excessive_precision)]
fn to_oklab(r: u8, g: u8, b: u8) -> [f32; 3] {
    let (r, g, b) = (srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b));
    let l = (0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b).cbrt();
    let m = (0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b).cbrt();
    let s = (0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b).cbrt();
    [
        0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
        1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
        0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
    ]
}

#[allow(clippy::excessive_precision)]
fn from_oklab([l, a, b]: [f32; 3]) -> Color {
    let l_ = (l + 0.396_337_78 * a + 0.215_803_76 * b).powi(3);
    let m_ = (l - 0.105_561_346 * a - 0.063_854_17 * b).powi(3);
    let s_ = (l - 0.089_484_18 * a - 1.291_485_5 * b).powi(3);
    Color::Rgb(
        linear_to_srgb(4.076_741_7 * l_ - 3.307_711_6 * m_ + 0.230_969_94 * s_),
        linear_to_srgb(-1.268_438 * l_ + 2.609_757_4 * m_ - 0.341_319_38 * s_),
        linear_to_srgb(-0.0041960863 * l_ - 0.703_418_6 * m_ + 1.707_614_7 * s_),
    )
}

/// Perceptual (OKLab) mix of two colors. Use for precomputed tables, `lerp` for per-frame work.
pub fn mix_oklab(a: Color, b: Color, t: f32) -> Color {
    match (a, b) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let (p, q) = (to_oklab(r1, g1, b1), to_oklab(r2, g2, b2));
            let t = t.clamp(0.0, 1.0);
            from_oklab([
                p[0] + (q[0] - p[0]) * t,
                p[1] + (q[1] - p[1]) * t,
                p[2] + (q[2] - p[2]) * t,
            ])
        }
        _ => a.lerp(b, t),
    }
}

/// A 256-entry lookup table built from color stops with OKLab interpolation,
/// so sampling a smooth ramp costs one array index per cell.
#[derive(Clone)]
pub struct Gradient {
    lut: Box<[Color; 256]>,
}

impl Gradient {
    pub fn new(stops: &[Color]) -> Gradient {
        let mut lut = Box::new([Color::Reset; 256]);
        match stops.len() {
            0 => {}
            1 => lut.fill(stops[0]),
            n => {
                for (i, slot) in lut.iter_mut().enumerate() {
                    let pos = i as f32 / 255.0 * (n - 1) as f32;
                    let k = (pos as usize).min(n - 2);
                    *slot = mix_oklab(stops[k], stops[k + 1], pos - k as f32);
                }
            }
        }
        Gradient { lut }
    }

    /// Samples the ramp at `t` in 0..1 (clamped).
    #[inline]
    pub fn at(&self, t: f32) -> Color {
        self.lut[(t.clamp(0.0, 1.0) * 255.0) as usize]
    }
}

/// How many colors the terminal can show.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorMode {
    TrueColor,
    Ansi256,
    Ansi16,
    Mono,
}

impl ColorMode {
    pub fn parse(s: &str) -> Option<Option<ColorMode>> {
        Some(match s.to_ascii_lowercase().as_str() {
            "auto" => None,
            "truecolor" | "24bit" | "rgb" => Some(ColorMode::TrueColor),
            "256" | "ansi256" => Some(ColorMode::Ansi256),
            "16" | "ansi" | "ansi16" | "8" => Some(ColorMode::Ansi16),
            "mono" | "none" | "monochrome" => Some(ColorMode::Mono),
            _ => return None,
        })
    }

    /// Guesses from the environment, honoring NO_COLOR.
    pub fn detect() -> ColorMode {
        let var = |k: &str| std::env::var(k).unwrap_or_default().to_ascii_lowercase();
        if std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty()) {
            return ColorMode::Mono;
        }
        let ct = var("COLORTERM");
        let term = var("TERM");
        if ct.contains("truecolor") || ct.contains("24bit") {
            return ColorMode::TrueColor;
        }
        if [
            "kitty",
            "alacritty",
            "wezterm",
            "foot",
            "ghostty",
            "contour",
            "rio",
        ]
        .iter()
        .any(|t| term.contains(t))
        {
            return ColorMode::TrueColor;
        }
        match std::env::var("TERM_PROGRAM").unwrap_or_default().as_str() {
            "iTerm.app" | "WezTerm" | "vscode" | "ghostty" | "Hyper" => {
                return ColorMode::TrueColor;
            }
            "Apple_Terminal" => return ColorMode::Ansi256,
            _ => {}
        }
        if term.contains("256") {
            ColorMode::Ansi256
        } else if term == "dumb" {
            ColorMode::Mono
        } else {
            ColorMode::Ansi16
        }
    }
}

const CUBE: [u8; 6] = [0, 95, 135, 175, 215, 255];

fn cube_idx(v: u8) -> usize {
    match v {
        0..=47 => 0,
        48..=114 => 1,
        _ => ((v as usize - 35) / 40).min(5),
    }
}

fn dist(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
    let d = |x: u8, y: u8| (x as i32 - y as i32).pow(2) as u32;
    // Rough perceptual weighting.
    2 * d(a.0, b.0) + 4 * d(a.1, b.1) + 3 * d(a.2, b.2)
}

/// Nearest xterm-256 index for an RGB color.
pub fn to_256(r: u8, g: u8, b: u8) -> u8 {
    let (ri, gi, bi) = (cube_idx(r), cube_idx(g), cube_idx(b));
    let cube = (CUBE[ri], CUBE[gi], CUBE[bi]);
    let avg = (r as u32 + g as u32 + b as u32) / 3;
    let gi_ = if avg > 238 {
        23
    } else {
        (avg.saturating_sub(3) / 10).min(23)
    } as u8;
    let gv = 8 + 10 * gi_;
    let c_idx = 16 + 36 * ri as u8 + 6 * gi as u8 + bi as u8;
    if dist((r, g, b), (gv, gv, gv)) < dist((r, g, b), cube) {
        232 + gi_
    } else {
        c_idx
    }
}

const BASIC16: [(u8, u8, u8); 16] = [
    (0, 0, 0),
    (205, 49, 49),
    (13, 188, 121),
    (229, 229, 16),
    (36, 114, 200),
    (188, 63, 188),
    (17, 168, 205),
    (229, 229, 229),
    (102, 102, 102),
    (241, 76, 76),
    (35, 209, 139),
    (245, 245, 67),
    (59, 142, 234),
    (214, 112, 214),
    (41, 184, 219),
    (255, 255, 255),
];

/// Nearest of the 16 basic colors. Saturated colors are matched by hue first,
/// because the user's scheme may render "red" very differently from xterm's.
pub fn to_16(r: u8, g: u8, b: u8) -> u8 {
    let mut best = (u32::MAX, 0u8);
    for (i, &c) in BASIC16.iter().enumerate() {
        let d = dist((r, g, b), c);
        if d < best.0 {
            best = (d, i as u8);
        }
    }
    best.1
}

pub fn ansi_rgb(n: u8) -> (u8, u8, u8) {
    match n {
        0..=15 => BASIC16[n as usize],
        16..=231 => {
            let n = n - 16;
            (
                CUBE[(n / 36) as usize],
                CUBE[(n / 6 % 6) as usize],
                CUBE[(n % 6) as usize],
            )
        }
        _ => {
            let v = 8 + 10 * (n - 232);
            (v, v, v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_colors() {
        assert_eq!(Color::parse("#ff8000"), Some(Color::Rgb(255, 128, 0)));
        assert_eq!(Color::parse("f80"), Some(Color::Rgb(255, 136, 0)));
        assert_eq!(Color::parse("ansi:4"), Some(Color::Ansi(4)));
        assert_eq!(Color::parse("12"), Some(Color::Ansi(12)));
        assert_eq!(Color::parse("default"), Some(Color::Reset));
        assert_eq!(Color::parse("#zzz"), None);
    }

    #[test]
    fn quantizes() {
        assert_eq!(to_256(0, 0, 0), 16);
        assert_eq!(to_256(255, 255, 255), 231);
        assert_eq!(to_256(255, 0, 0), 196);
        assert_eq!(to_256(128, 128, 128), 244);
        assert_eq!(to_16(250, 10, 10), 1);
        assert_eq!(to_16(255, 90, 90), 9);
    }

    #[test]
    fn gradient_endpoints() {
        let g = Gradient::new(&[rgb(0x000000), rgb(0xffffff)]);
        assert_eq!(g.at(0.0), Color::Rgb(0, 0, 0));
        assert_eq!(g.at(1.0), Color::Rgb(255, 255, 255));
        assert_eq!(g.at(7.0), Color::Rgb(255, 255, 255));
    }
}
