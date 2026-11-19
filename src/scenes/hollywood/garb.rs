//! What the widgets wear. Every ordinary widget is told what to look like by
//! a `Garb`, which today is the day's holiday and nothing else: it derefs to
//! that `Fiesta`, so `mx.is(..)`, `mx.level` and `mx.kind` read as they did
//! when the widgets held the `Fiesta` itself.

use super::lexico;
use crate::fiesta::Fiesta;
use fastrand::Rng;

/// What a widget wears: today the day's holiday and nothing else.
#[derive(Clone, Copy)]
pub struct Garb {
    fiesta: Fiesta,
}

impl Garb {
    /// The day's holiday as it is (`Fiesta::NONE` on an ordinary day).
    pub fn plain(fiesta: Fiesta) -> Garb {
        Garb { fiesta }
    }

    /// Does the holiday's vocabulary take this one? `rng.f32() < k *
    /// lexico::share(self.level)`, the very draw the widgets made before.
    pub fn takes(self, rng: &mut Rng, k: f32) -> bool {
        rng.f32() < k * lexico::share(self.fiesta.level)
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
}
