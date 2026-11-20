//! What the widgets wear. Every ordinary widget is told what to look like by
//! a `Garb`: the day's holiday, or a preset (spec 8.3). It derefs to a
//! `Fiesta`, so `mx.is(..)`, `mx.level` and `mx.kind` read as they did when
//! the widgets held the `Fiesta` itself. A preset is a holiday with no date:
//! no `Kind`, always at its height, and every word its own.

use super::lexico;
use super::preset::Preset;
use crate::fiesta::{Fiesta, Kind};
use fastrand::Rng;

/// What a widget wears: the day's holiday, or a preset.
#[derive(Clone, Copy)]
pub struct Garb {
    fiesta: Fiesta,
    /// The preset whose vocabulary every widget speaks (`lexico::of`).
    pub preset: Option<&'static Preset>,
    /// Test only: the preset's words, but a holiday's share of them.
    leaky: bool,
    /// On a film set (spec 10.5): what changes, eases, and the text feeds
    /// bring a line at a time.
    pub calm: bool,
    /// The maps keep the world's proportions (spec 10.8) instead of
    /// stretching it to their pane.
    pub keep: bool,
}

/// What a preset derefs to: no holiday, at its height.
const PRESET: Fiesta = Fiesta {
    kind: Kind::None,
    level: 1.0,
    month: 0,
    day: 0,
};

impl Garb {
    /// The day's holiday as it is (`Fiesta::NONE` on an ordinary day).
    pub fn plain(fiesta: Fiesta) -> Garb {
        Garb {
            fiesta,
            preset: None,
            leaky: false,
            calm: false,
            keep: false,
        }
    }

    /// Under a preset the holiday is none, at its height: derefs to
    /// `Fiesta { kind: Kind::None, level: 1.0, month: 0, day: 0 }`.
    pub fn preset(p: &'static Preset) -> Garb {
        Garb {
            fiesta: PRESET,
            preset: Some(p),
            leaky: false,
            calm: false,
            keep: false,
        }
    }

    /// The preset's vocabulary with a holiday's share of it at its height,
    /// which lets English through: what the check of spec 11.3 must catch.
    #[cfg(test)]
    pub fn leaky(p: &'static Preset) -> Garb {
        Garb {
            leaky: true,
            ..Garb::preset(p)
        }
    }

    /// The same clothes, calm on a film set or not.
    pub fn with_calm(self, on: bool) -> Garb {
        Garb { calm: on, ..self }
    }

    /// The same clothes, the maps keeping their shape or not.
    pub fn with_keep(self, on: bool) -> Garb {
        Garb { keep: on, ..self }
    }

    /// Is every word the preset's? (No draw is made for the share.)
    pub fn whole(self) -> bool {
        self.preset.is_some() && !self.leaky
    }

    /// Does the vocabulary take this one? Under a preset, always, without a
    /// draw; else `rng.f32() < k * lexico::share(self.level)`, the very draw
    /// the widgets made before.
    pub fn takes(self, rng: &mut Rng, k: f32) -> bool {
        self.whole() || rng.f32() < k * lexico::share(self.fiesta.level)
    }
}

impl From<Fiesta> for Garb {
    fn from(fiesta: Fiesta) -> Garb {
        Garb::plain(fiesta)
    }
}

impl std::ops::Deref for Garb {
    type Target = Fiesta;

    fn deref(&self) -> &Fiesta {
        &self.fiesta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiesta::Kind;

    #[test]
    fn takes_draws_what_the_holidays_drew() {
        for (level, k) in [(0.25f32, 1.0f32), (1.0, 1.0), (0.6, 0.5), (1.0, 0.6)] {
            let f = Fiesta {
                level,
                ..Fiesta::height(Kind::Navidad)
            };
            let (mut a, mut b) = (Rng::with_seed(5), Rng::with_seed(5));
            for _ in 0..1000 {
                assert_eq!(
                    Garb::plain(f).takes(&mut a, k),
                    b.f32() < k * lexico::share(level)
                );
            }
            assert_eq!(a.u64(..), b.u64(..)); // one draw each time, no more
        }
    }

    #[test]
    fn a_preset_takes_every_word_without_a_draw() {
        let p = super::super::preset_tests::bare();
        let g = Garb::preset(p);
        assert_eq!(*g, PRESET);
        assert!(g.whole() && !Garb::leaky(p).whole() && !Garb::plain(Fiesta::NONE).whole());
        let (mut a, b) = (Rng::with_seed(5), Rng::with_seed(5));
        for k in [1.0, 0.6, 0.5] {
            assert!(g.takes(&mut a, k));
        }
        assert_eq!(a.u64(..), b.clone().u64(..));
        // The leaky one draws as a holiday at its height does.
        let (mut a, mut b) = (Rng::with_seed(5), Rng::with_seed(5));
        for _ in 0..1000 {
            assert_eq!(
                Garb::leaky(p).takes(&mut a, 0.6),
                b.f32() < 0.6 * lexico::share(1.0)
            );
        }
    }
}
