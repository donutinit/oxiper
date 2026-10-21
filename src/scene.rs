//! The interface every scene implements, and its typed options.

use crate::color::{Color, Gradient};
use crate::fiesta::Fiesta;
use crate::render::Canvas;
use crate::theme::Theme;

/// Everything a scene may read while updating or drawing.
pub struct Ctx<'a> {
    pub theme: &'a Theme,
    pub opts: &'a Opts,
    /// The color "nothing" looks like: the terminal background as reported by
    /// the terminal, the configured hint, or the theme background. Fade trails
    /// toward this instead of black so they also look right on light or
    /// transparent terminals.
    pub bg: Color,
    /// Seed for the scene's RNG, so `record` and `bench` are reproducible.
    pub seed: u64,
    /// The Mexican holiday on today, if any (see `crate::fiesta`). Scenes
    /// dress up for it; `retheme` runs again whenever it changes.
    pub fiesta: Fiesta,
}

impl Ctx<'_> {
    /// Blends `c` toward the background: `k = 0` keeps it, `k = 1` makes it vanish.
    #[inline]
    pub fn fade(&self, c: Color, k: f32) -> Color {
        match self.bg {
            Color::Rgb(..) => c.lerp(self.bg, k),
            // Without a known background, darkening is the closest thing to fading.
            _ => c.scale(1.0 - k),
        }
    }

    /// Accent color `i` of the theme, or `native` under `classic`.
    #[inline]
    pub fn pick(&self, i: usize, native: Color) -> Color {
        self.theme.pick(i, native)
    }

    /// A gradient made of the scene's own stops under `classic`, the theme ramp otherwise.
    pub fn ramp(&self, native: &[Color]) -> Gradient {
        self.theme.ramp_or(native)
    }

    pub fn rng(&self) -> fastrand::Rng {
        fastrand::Rng::with_seed(self.seed)
    }
}

pub trait Scene {
    /// Called once right after creation and again whenever the terminal is resized.
    fn resize(&mut self, ctx: &Ctx, w: u16, h: u16);
    /// Called after creation and whenever the theme changes; cache colors here.
    fn retheme(&mut self, _ctx: &Ctx) {}
    /// Advances the simulation by `dt` seconds of scene time (already scaled
    /// by the speed setting).
    fn update(&mut self, ctx: &Ctx, dt: f32);
    /// Advances the scene: `dt` seconds of scene time, scaled by the speed
    /// setting, set how far things move and animate, while `real` seconds
    /// pass on the wall clock. Clocks (how long something lasts: a biome's
    /// stay, a finished tree's wait, a day) follow `real`, so slowing a scene
    /// down never stretches them. The default is `update(ctx, dt)`, for
    /// scenes without clocks.
    fn tick(&mut self, ctx: &Ctx, dt: f32, real: f32) {
        let _ = real;
        self.update(ctx, dt);
    }
    /// Draws the current state. The canvas has already been cleared.
    fn draw(&mut self, ctx: &Ctx, c: &mut Canvas);
    /// Seconds (of scene time) until the picture next changes. Scenes that only
    /// change now and then (a grown tree, frame-based animations) report it so
    /// the app can sleep instead of redrawing an identical frame. `INFINITY`
    /// means "static until something external happens"; 0 means "every frame".
    fn next_change(&self) -> f32 {
        0.0
    }
}

pub struct SceneInfo {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub about: &'static str,
    pub options: &'static [Opt],
    pub make: fn(&Ctx) -> Box<dyn Scene>,
}

/// A documented option with its default. The default's type is the option's type.
pub struct Opt {
    pub key: &'static str,
    pub default: Val,
    pub help: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub enum Val {
    F(f64),
    I(i64),
    B(bool),
    S(&'static str),
}

impl Val {
    pub fn to_toml(self) -> toml::Value {
        match self {
            Val::F(v) => toml::Value::Float(v),
            Val::I(v) => toml::Value::Integer(v),
            Val::B(v) => toml::Value::Boolean(v),
            Val::S(v) => toml::Value::String(v.into()),
        }
    }

    pub fn type_name(self) -> &'static str {
        match self {
            Val::F(_) => "number",
            Val::I(_) => "integer",
            Val::B(_) => "bool",
            Val::S(_) => "string",
        }
    }

    /// Checks that a user value fits this option's type.
    pub fn accepts(self, v: &toml::Value) -> bool {
        matches!(
            (self, v),
            (Val::F(_), toml::Value::Float(_) | toml::Value::Integer(_))
                | (Val::I(_), toml::Value::Integer(_))
                | (Val::B(_), toml::Value::Boolean(_))
                | (Val::S(_), toml::Value::String(_))
        )
    }
}

/// A scene's options: the user's `[scenes.<name>]` table on top of the defaults.
#[derive(Clone)]
pub struct Opts {
    specs: &'static [Opt],
    table: toml::Table,
}

impl Opts {
    pub fn new(specs: &'static [Opt], table: toml::Table) -> Opts {
        Opts { specs, table }
    }

    fn spec(&self, key: &str) -> Option<Val> {
        self.specs.iter().find(|o| o.key == key).map(|o| o.default)
    }

    pub fn f(&self, key: &str) -> f32 {
        self.f64(key) as f32
    }

    pub fn f64(&self, key: &str) -> f64 {
        match self.table.get(key) {
            Some(toml::Value::Float(v)) => *v,
            Some(toml::Value::Integer(v)) => *v as f64,
            _ => match self.spec(key) {
                Some(Val::F(v)) => v,
                Some(Val::I(v)) => v as f64,
                _ => 0.0,
            },
        }
    }

    pub fn i(&self, key: &str) -> i64 {
        match self.table.get(key) {
            Some(toml::Value::Integer(v)) => *v,
            Some(toml::Value::Float(v)) => *v as i64,
            _ => match self.spec(key) {
                Some(Val::I(v)) => v,
                Some(Val::F(v)) => v as i64,
                _ => 0,
            },
        }
    }

    pub fn b(&self, key: &str) -> bool {
        match self.table.get(key) {
            Some(toml::Value::Boolean(v)) => *v,
            _ => matches!(self.spec(key), Some(Val::B(true))),
        }
    }

    pub fn s(&self, key: &str) -> &str {
        match self.table.get(key) {
            Some(toml::Value::String(v)) => v,
            _ => match self.spec(key) {
                Some(Val::S(v)) => v,
                _ => "",
            },
        }
    }

    /// A color option: a color string, or `""`/`"auto"` for "let the theme decide".
    #[allow(dead_code)] // scene API (docs/SCENES.md)
    pub fn color(&self, key: &str) -> Option<Color> {
        match self.s(key) {
            "" | "auto" | "theme" => None,
            s => Color::parse(s),
        }
    }

    pub fn table(&self) -> &toml::Table {
        &self.table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPECS: &[Opt] = &[
        Opt {
            key: "speed",
            default: Val::F(1.5),
            help: "",
        },
        Opt {
            key: "count",
            default: Val::I(3),
            help: "",
        },
        Opt {
            key: "on",
            default: Val::B(true),
            help: "",
        },
        Opt {
            key: "name",
            default: Val::S("x"),
            help: "",
        },
    ];

    #[test]
    fn defaults_and_overrides() {
        let o = Opts::new(SPECS, toml::Table::new());
        assert_eq!(
            (o.f("speed"), o.i("count"), o.b("on"), o.s("name")),
            (1.5, 3, true, "x")
        );
        let o = Opts::new(
            SPECS,
            "speed = 2\ncount = 9\non = false\nname = 'y'"
                .parse()
                .unwrap(),
        );
        assert_eq!(
            (o.f("speed"), o.i("count"), o.b("on"), o.s("name")),
            (2.0, 9, false, "y")
        );
    }
}
