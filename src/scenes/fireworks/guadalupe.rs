//! The fireworks on the Virgen de Guadalupe, December 12: the Basílica at
//! midnight, with the mañanitas' cohetes over it, roses and stars of her
//! mantle among the shells, and a castillo whose top lights up with her rays.
//! Her image is only ever what it is for the people who raise it: the lit
//! picture on the castillo, still, never a shell.

use super::*;

/// Seeds the Virgen's night's own random numbers.
const GUADALUPE_SEED: u64 = 0x6775_6164_616c_7570;

// The Virgen de Guadalupe under `classic`: the rosas de Castilla in two
// pinks, her mantle verde azulado with its stars of gold, her rose gown and
// the leaves of the roses, brightened to burn in the night sky.
pub(super) const GL_ROSA: V3 = c8(255, 56, 132);
pub(super) const GL_ROSA_CLARO: V3 = c8(255, 132, 176);
pub(super) const GL_ORO: V3 = c8(255, 200, 70);
pub(super) const GL_MANTO: V3 = c8(32, 196, 176);
pub(super) const GL_VESTIDO: V3 = c8(255, 128, 128);
pub(super) const GL_HOJA: V3 = c8(70, 196, 90);
/// The cantera of the Basílica, and its roofs gone copper-green, as the
/// floodlights show them.
pub(super) const CANTERA: [V3; 3] = [c8(191, 169, 138), c8(172, 150, 118), c8(204, 184, 152)];
pub(super) const COBRE: V3 = c8(79, 143, 122);
/// The glass of a veladora: rojo, blanco and verde.
const VIDRIO: [V3; 3] = [c8(198, 40, 40), c8(244, 241, 234), c8(46, 154, 90)];

/// A rosa de Castilla: five leaves under rings of petals in two pinks round
/// a golden heart.
pub(super) const ROSA_CASTILLA: &[Stroke] = &[
    Stroke::Leaf(0.0, 0.0, 1.0, 0.16, 5, 0.1, 4),
    Stroke::Bloom(0.0, 0.0, 0.86, 0.22, 9.0, 0),
    Stroke::Bloom(0.0, 0.0, 0.62, 0.26, 7.0, 1),
    Stroke::Bloom(0.0, 0.0, 0.4, 0.3, 5.0, 0),
    Stroke::Bloom(0.0, 0.0, 0.2, 0.3, 4.0, 6),
    Stroke::Arc(0.0, 0.0, 0.07, 0.0, 1.0, 2),
];

/// A star of her mantle: eight points in gold with a lighter one at its
/// heart.
pub(super) const LUCERO: &[Stroke] = &[
    Stroke::Star(0.0, 0.0, 1.0, 0.42, 8, 2, 0),
    Stroke::Star(0.0, 0.0, 0.5, 0.22, 8, 5, 0),
];

/// How many steps the picture on the top of a castillo stays lit.
pub(super) const RAYOS_STEPS: i32 = 270;

/// How tall the picture on the top of a castillo is, in pixels.
pub(super) fn rayos_h(size: f32) -> i32 {
    ((size * 0.55).round() as i32).clamp(9, 50)
}

/// How lit the picture on a castillo is, 0 to 1: dark while the wheels burn
/// from the bottom up, lighting as the last one does, and lit for a good
/// while after, dimming at the end.
pub(super) fn rayos_glow(a: &Actor) -> f32 {
    let tiers = castillo_tiers(a.size);
    let state = a.state as usize;
    if state < tiers {
        0.0
    } else if state == tiers {
        0.6 * (WHEEL_STEPS - a.left) as f32 / WHEEL_STEPS as f32
    } else if state == tiers + 1 {
        0.6
    } else {
        // Brighter for a moment, then steady, and dimming at the end.
        let rise = 0.6 + 0.4 * (RAYOS_STEPS - a.left) as f32 / 60.0;
        rise.min(a.left as f32 / 40.0).clamp(0.0, 1.0)
    }
}

impl Fireworks {
    /// Lays out the Basílica de Guadalupe that stands in for the city on
    /// December 12: the new Basílica, round, under its great roof gone
    /// copper-green, and the old one beside it with its towers and dome,
    /// floodlit cantera with their windows lit; the houses of the barrio
    /// either side, a crowd of pilgrims in front with their banderitas, rows
    /// of veladoras and rosas de Castilla at the foot of the walls, lights
    /// along the eaves, and a castillo to be lit; the mañanitas' cohetes go
    /// up from the atrio. The layout depends only on the seed and the size.
    pub(super) fn build_basilica(&mut self) {
        let (w, ph) = (self.w, self.ph);
        if !self.fiesta.guadalupe() || !self.show_city || w == 0 || ph < 12 {
            return;
        }
        self.town.resize(w * ph, 0);
        self.town_col.resize(w * ph, [0.0; 3]);
        let rng = &mut fastrand::Rng::with_seed(self.seed ^ GUADALUPE_SEED);
        let lit = 0.9;
        let k = (ph / 50).clamp(1, 3);
        let ground = (if ph >= 40 { 2 } else { 1 }) * k;
        let gy = ph - ground;
        self.lay_ground(rng, gy);
        let mut tops = vec![gy as i32; w];
        let (wi, b) = (w as i32, gy as i32);
        // The Basílicas are 67 units wide at most and 27 tall, a unit being a
        // pixel, or a block of them on a big screen as long as it fits.
        let u = k.min(w / 56).min(ph / 38) as i32;
        if u < 1 {
            self.facades(rng, (0, w), gy, k, lit, &mut tops);
            self.crowd(rng, gy, k, 0.5, 0.12);
            self.sites.push(wi / 2);
            return;
        }
        let castillo = w >= 70 && ph >= 44;
        let avail = wi - 8 * u - if castillo { 18 * u } else { 0 };
        let old = avail >= 48 * u;
        let d = if old {
            (avail - 27 * u).clamp(21 * u, 40 * u)
        } else {
            avail.clamp(14 * u, 40 * u)
        };
        let gw = d + if old { 27 * u } else { 0 };
        let xg = 4 * u + (avail - gw) / 2;
        let xn = if old { xg + 27 * u } else { xg };
        self.facades(rng, (0, (xg - u).max(0) as usize), gy, k, lit, &mut tops);
        self.facades(
            rng,
            ((xg + gw + u).min(wi) as usize, w),
            gy,
            k,
            lit,
            &mut tops,
        );
        if old {
            self.build_antigua(xg, b, u, lit, rng);
        }
        self.build_nueva((xn, b), (d, u), lit, rng);
        self.crowd(rng, gy, k, 0.5, 0.1);
        self.rosal_y_veladoras((xg, xg + gw), b, u, rng);

        // The mañanitas' cohetes go up from the atrio.
        if old {
            self.sites.extend([xg + 12 * u, xg + 2 * u]);
        }
        self.sites.extend([xn + d / 2, xn + d - 3 * u]);
        // The castillo, in front of the houses on the right.
        if castillo {
            self.actors.push(Actor {
                role: Role::Castillo,
                x: (wi - 9 * u) as f32,
                y: gy as f32,
                size: (ph as f32 * 0.34).clamp(16.0, 90.0),
                state: 0,
                left: rng.i32(120..240),
                seed: rng.f32() * TAU,
            });
        }
    }

    /// The new Basílica, round, its great roof a tent of copper-green with
    /// its ribs, a lantern and a cross on top, over a ring of cantera with
    /// tall lit windows and three doors: `d` wide, its left edge at `x0`,
    /// standing on row `b`, in units of `u` pixels.
    fn build_nueva(
        &mut self,
        (x0, b): (i32, i32),
        (d, u): (i32, i32),
        lit: f32,
        rng: &mut fastrand::Rng,
    ) {
        let ft = &self.fest;
        let (wall, roof, window, gold) = (ft.wall[0], ft.roof, ft.window, ft.gold);
        let (white, stone) = (ft.white, ft.stone);
        let wall = scale(wall, 1.15);
        let trim = mix(wall, white, 0.4);
        let glow = scale(window, 1.3);
        let dark = scale(stone, 0.5);
        let (cx, wh) = (x0 + d / 2, 7 * u);
        let top = b - wh;
        // The roof: a tent whose sides curve in, ribbed, lighter near the
        // eaves where the floodlights are.
        let rh = d * 36 / 100;
        for r in 0..rh {
            let t = r as f32 / rh as f32;
            let hw = ((d as f32 / 2.0 + 0.8 * u as f32) * (1.0 - t).powf(1.5)).max(u as f32);
            let shade = 0.7 + 0.5 * (1.0 - t);
            for x in (cx as f32 - hw).round() as i32..(cx as f32 + hw).round() as i32 {
                let rib = (x - cx).rem_euclid(5 * u) == 0;
                self.put(
                    x,
                    top - 1 - r,
                    scale(roof, shade * if rib { 0.78 } else { 1.0 }),
                );
            }
        }
        // The ring of cantera, its cornice and its tall windows.
        self.rect(x0, top, d, wh, wall);
        self.rect(x0 - u, top, d + 2 * u, u, trim);
        for x in (x0 + 2 * u..x0 + d - 2 * u).step_by(4 * u as usize) {
            let on = rng.f32() < lit;
            self.rect(x, top + 2 * u, u, 4 * u, if on { window } else { dark });
        }
        // Three doors, open on the light inside.
        for dx in [-6 * u, 0, 6 * u] {
            if d < 24 * u && dx != 0 {
                continue;
            }
            let x = cx + dx - 3 * u / 2;
            self.rect(x - u / 2, b - 5 * u, 4 * u, 5 * u, trim);
            self.rect(x, b - 4 * u, 3 * u, 4 * u, glow);
            self.disc(
                (x as f32 + 1.5 * u as f32, (b - 4 * u) as f32),
                1.5 * u as f32,
                true,
                glow,
            );
        }
        // The lantern on the apex and its cross.
        let apex = top - rh;
        self.rect(cx - u, apex - 3 * u, 2 * u, 3 * u, glow);
        self.rect(cx, apex - 6 * u, u, 3 * u, mix(gold, white, 0.3));
        self.rect(cx - u, apex - 5 * u, 3 * u, u, mix(gold, white, 0.3));
        // Lights along the eaves, chasing.
        for (i, x) in (x0..x0 + d).step_by(3 * u as usize).enumerate() {
            if rng.f32() < lit {
                self.bulbs.push(Bulb {
                    x,
                    y: top - 1,
                    c: (i % 6) as u8,
                    kind: 1,
                    phase: 0.0,
                });
            }
        }
    }

    /// The old Basílica, baroque: a facade of cantera between two towers
    /// with domed caps, a pediment over it and the great dome rising behind,
    /// the door open, a niche lit over it: 24 units wide, its left edge at
    /// `x0`.
    fn build_antigua(&mut self, x0: i32, b: i32, u: i32, lit: f32, rng: &mut fastrand::Rng) {
        let ft = &self.fest;
        let (wall, roof, window, gold, white) = (ft.wall[1], ft.roof, ft.window, ft.gold, ft.white);
        let wall = scale(wall, 1.15);
        let trim = mix(wall, white, 0.4);
        let shade = scale(wall, 0.8);
        let dark = scale(ft.stone, 0.5);
        let glow = scale(window, 1.3);
        let cx = x0 + 12 * u;
        let cap = mix(roof, gold, 0.1);
        // The dome behind, with its lantern and cross.
        self.rect(cx - 5 * u, b - 14 * u, 10 * u, 3 * u, shade);
        self.disc((cx as f32, (b - 14 * u) as f32), 5.0 * u as f32, true, cap);
        for dx in [-3 * u, 0, 3 * u] {
            self.rect(cx + dx, b - 18 * u, u, 4 * u, scale(cap, 1.25));
        }
        self.rect(cx, b - 21 * u, u, 3 * u, trim);
        self.rect(cx - u, b - 20 * u, 3 * u, u, trim);
        // The facade: body, cornice, pediment.
        self.rect(x0 + 5 * u, b - 11 * u, 14 * u, 11 * u, wall);
        self.gable(cx, (b - 11 * u, 4 * u), 7.0 * u as f32, trim);
        self.gable(cx, (b - 11 * u, 3 * u), 6.0 * u as f32, wall);
        self.rect(x0 + 4 * u, b - 12 * u, 16 * u, u, trim);
        // The door in its frame, the niche over it, a window on either side.
        self.rect(x0 + 9 * u, b - 7 * u, 6 * u, 7 * u, trim);
        self.disc((cx as f32, (b - 7 * u) as f32), 3.0 * u as f32, true, trim);
        self.rect(x0 + 10 * u, b - 6 * u, 4 * u, 6 * u, glow);
        self.disc((cx as f32, (b - 6 * u) as f32), 2.0 * u as f32, true, glow);
        self.rect(cx - u, b - 11 * u, 2 * u, 3 * u, scale(glow, 1.1));
        self.disc(
            (cx as f32, (b - 11 * u) as f32),
            u as f32,
            true,
            scale(glow, 1.1),
        );
        for wx in [x0 + 6 * u, x0 + 17 * u] {
            self.rect(
                wx,
                b - 8 * u,
                u,
                4 * u,
                if rng.f32() < lit { window } else { dark },
            );
        }
        // The towers, each with its belfry and a cap with a cross.
        for tx in [x0, x0 + 19 * u] {
            self.rect(tx, b - 15 * u, 5 * u, 15 * u, wall);
            self.rect(tx, b - 15 * u, u, 15 * u, trim);
            self.rect(tx + 4 * u, b - 15 * u, u, 15 * u, shade);
            self.rect(tx - u, b - 16 * u, 7 * u, u, trim);
            self.rect(tx + 2 * u, b - 14 * u, 2 * u, 4 * u, scale(glow, 0.6));
            self.rect(
                tx + 2 * u,
                b - 7 * u,
                u,
                4 * u,
                if rng.f32() < lit { window } else { dark },
            );
            self.disc(
                ((tx + 5 * u / 2) as f32, (b - 16 * u) as f32),
                2.5 * u as f32,
                true,
                cap,
            );
            self.rect(tx + 2 * u, b - 21 * u, u, 3 * u, trim);
            self.rect(tx + u, b - 20 * u, 3 * u, u, trim);
        }
        // Lights along the cornice.
        for (i, x) in (x0 + 4 * u..x0 + 20 * u)
            .step_by(3 * u as usize)
            .enumerate()
        {
            if rng.f32() < lit {
                self.bulbs.push(Bulb {
                    x,
                    y: b - 13 * u,
                    c: (i % 6) as u8,
                    kind: 1,
                    phase: 0.0,
                });
            }
        }
    }

    /// Veladoras in their glass, in threes, and little beds of rosas de
    /// Castilla at the foot of the Basílicas, between the columns `x`.
    fn rosal_y_veladoras(&mut self, (xa, xb): (i32, i32), b: i32, u: i32, rng: &mut fastrand::Rng) {
        let (w, ph) = (self.w, self.ph);
        let ft = &self.fest;
        let native = self.pal.native;
        let glass = if native {
            VIDRIO
        } else {
            [ft.hues[0], ft.white, ft.hues[3]]
        };
        let (rosa, claro, hoja) = (ft.hues[0], ft.hues[1], ft.hues[5]);
        let mut x = xa + rng.i32(1..5 * u + 1);
        while x < xb - 6 * u {
            if rng.f32() < 0.55 {
                // Three veladoras.
                for (i, g) in glass.iter().enumerate() {
                    let vx = x + i as i32 * 2 * u;
                    self.rect(vx, b - 2 * u, u, 2 * u, scale(*g, 0.55));
                    let (fx, fy) = (vx as usize, (b - 2 * u - 1) as usize);
                    if fx < w && fy < ph {
                        self.town[fy * w + fx] = 2;
                        self.candles.push(Candle {
                            x: fx as i32,
                            y: fy as i32,
                            phase: rng.f32() * TAU,
                            power: 1.0,
                        });
                    }
                }
                x += 8 * u;
            } else {
                // A little bed of roses: rosas in two pinks in their leaves.
                self.rect(x, b - u, 5 * u, u, scale(hoja, 0.5));
                for i in 0..3 {
                    let c = if i == 1 { claro } else { rosa };
                    self.rect(x + u + 2 * u * i, b - 2 * u, u, u, scale(c, 0.8));
                }
                x += 7 * u;
            }
            x += rng.i32(1..6 * u + 1);
        }
    }

    /// The picture on the top of a castillo, in lights: the rays of her
    /// mandorla, gold, shimmering; the verde azulado of her mantle with its
    /// stars and the rose of her gown inside them. It is drawn still and
    /// upright. Dark, a frame of cane and a faint shape, until the castillo
    /// burns to its last tier.
    pub(super) fn draw_rayos(&mut self, a: &Actor) {
        let glow = rayos_glow(a);
        let h = rayos_h(a.size) as f32;
        let (cx, cy) = (a.x, a.y - a.size - 4.0 - h * 0.5);
        let (rx, ry) = (h * 0.4, h * 0.5);
        let t = self.frame as f32;
        let ft = &self.fest;
        let (cane, gold, manto, vestido) = (ft.cane, ft.gold, ft.green, ft.hues[4]);
        let skin = if self.pal.native {
            c8(170, 112, 78)
        } else {
            scale(gold, 0.65)
        };
        // The rays: long and short in turn, from her mantle out to the rim.
        let n = (h as usize).clamp(12, 36) & !1;
        for i in 0..n {
            let (s, c) = (i as f32 / n as f32 * TAU).sin_cos();
            let reach = if i % 2 == 0 { 1.0 } else { 0.78 };
            let shimmer = 0.85 + 0.15 * (t * 0.12 + i as f32 * 0.9).sin();
            let col = to_color(mix(scale(cane, 0.3), scale(gold, shimmer), glow));
            let steps = (ry * (reach - 0.5)).ceil().max(1.0) as i32 + 1;
            for j in 0..=steps {
                let f = 0.5 + (reach - 0.5) * j as f32 / steps as f32;
                self.px.set(
                    (cx + c * rx * f).round() as i32,
                    (cy + s * ry * f).round() as i32,
                    col,
                );
            }
        }
        // Her mantle with its stars, her gown and her face.
        let (mx, my, mrx, mry) = (cx, cy + ry * 0.1, rx * 0.46, ry * 0.68);
        let lighten = |c: V3| to_color(mix(scale(c, 0.25), c, glow));
        let (x0, x1) = ((mx - mrx).floor() as i32, (mx + mrx).ceil() as i32);
        for y in (my - mry).floor() as i32..=(my + mry).ceil() as i32 {
            for x in x0..=x1 {
                let (dx, dy) = ((x as f32 + 0.5 - mx) / mrx, (y as f32 + 0.5 - my) / mry);
                if dx * dx + dy * dy <= 1.0 {
                    let gown = dx.abs() < 0.32 && dy > -0.3;
                    self.px
                        .set(x, y, lighten(if gown { vestido } else { manto }));
                }
            }
        }
        let star = to_color(mix(scale(gold, 0.3), gold, glow));
        for (sx, sy) in [(-0.55, -0.1), (0.55, 0.15), (-0.45, 0.55), (0.45, 0.6)] {
            self.px.set(
                (mx + sx * mrx).round() as i32,
                (my + sy * mry).round() as i32,
                star,
            );
        }
        let head = to_color(mix(scale(skin, 0.3), skin, glow));
        let hr = (ry * 0.12).max(0.9);
        for dy in 0..=(2.0 * hr) as i32 {
            for dx in -(hr as i32)..=(hr as i32) {
                self.px.set(
                    (mx + dx as f32).round() as i32,
                    (my - mry - hr + dy as f32).round() as i32,
                    head,
                );
            }
        }
    }
}
