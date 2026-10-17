//! Scene registry. Each scene lives in its own file and exports `INFO`.

use crate::scene::SceneInfo;

pub mod aquarium;
pub mod boids;
pub mod bonsai;
pub mod constellation;
pub mod dog;
pub mod dvd;
pub mod fire;
pub mod fireworks;
pub mod hollywood;
pub mod metaballs;
pub mod neo;
pub mod nyancat;
pub mod orrery;
pub mod particles;
pub mod pipes;
pub mod rain;
pub mod shark;
pub mod waveform;

/// Every scene, in the order `all` plays them.
pub const ALL: &[&SceneInfo] = &[
    &bonsai::INFO,
    &aquarium::INFO,
    &neo::INFO,
    &nyancat::INFO,
    &dog::INFO,
    &shark::INFO,
    &fire::INFO,
    &fireworks::INFO,
    &constellation::INFO,
    &waveform::INFO,
    &pipes::INFO,
    &rain::INFO,
    &dvd::INFO,
    &boids::INFO,
    &orrery::INFO,
    &particles::INFO,
    &metaballs::INFO,
    &hollywood::INFO,
];

/// Finds a scene by name or alias (case-insensitive).
pub fn find(name: &str) -> Option<&'static SceneInfo> {
    let n = name.trim().to_ascii_lowercase();
    ALL.iter()
        .copied()
        .find(|s| s.name == n || s.aliases.contains(&n.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Player;
    use crate::config::{Config, Override};
    use crate::render::Canvas;
    use crate::theme::TermColors;

    /// Every scene must survive odd sizes, resizes and long runs without panicking.
    #[test]
    fn scenes_survive_sizes_and_time() {
        for info in ALL {
            for theme in ["classic", "dracula"] {
                let ov = [
                    Override::new("scene", toml::Value::String(info.name.into())),
                    Override::new("theme", toml::Value::String(theme.into())),
                    Override::new("seed", toml::Value::Integer(7)),
                ];
                let cfg = Config::load(
                    Some(std::path::Path::new("/nonexistent-oxiper")),
                    &ov,
                    false,
                );
                // The explicit missing path is an error by design; fall back to no file.
                let cfg = cfg.or_else(|_| Config::load(None, &ov, false)).unwrap();
                let mut p = Player::new(cfg, TermColors::default(), 80, 24).unwrap();
                for &(w, h) in &[(80u16, 24u16), (1, 1), (2, 3), (7, 2), (300, 90), (40, 12)] {
                    p.resize(w, h);
                    let mut c = Canvas::new(w, h, crate::color::Color::Reset);
                    for _ in 0..240 {
                        p.update(1.0 / 30.0);
                        c.clear();
                        p.draw(&mut c);
                    }
                    // A few huge steps, like after a suspend.
                    for _ in 0..3 {
                        p.update(5.0);
                        c.clear();
                        p.draw(&mut c);
                    }
                }
            }
        }
    }
}
