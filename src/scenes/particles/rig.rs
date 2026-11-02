//! What the costumes' tests share.

use super::{INFO, Particles};
use crate::color::Color;
use crate::fiesta::Fiesta;
use crate::render::Canvas;
use crate::scene::{Ctx, Opts, Scene};
use crate::theme::Theme;

pub const MODES: [&str; 3] = ["flow", "fireworks", "fountain"];

pub fn opts(mode: &str, more: &str) -> Opts {
    Opts::new(
        INFO.options,
        format!("mode = '{mode}'{more}").parse().unwrap(),
    )
}

pub fn ctx<'a>(theme: &'a Theme, opts: &'a Opts, fiesta: Fiesta) -> Ctx<'a> {
    Ctx {
        theme,
        opts,
        bg: theme.bg,
        seed: 7,
        fiesta,
    }
}

/// A scene of `w`×`h` cells, themed and dressed for the day `ctx` says.
pub fn scene(ctx: &Ctx, w: u16, h: u16) -> (Particles, Canvas) {
    let mut s = Particles::new(ctx);
    s.resize(ctx, w, h);
    s.retheme(ctx);
    (s, Canvas::new(w, h, Color::Reset))
}

/// Runs a scene for `frames` and draws it.
pub fn run(s: &mut Particles, ctx: &Ctx, c: &mut Canvas, frames: usize) {
    for _ in 0..frames {
        s.update(ctx, 1.0 / 30.0);
    }
    c.clear();
    s.draw(ctx, c);
}

/// How many cells of the canvas are drawn in one of `colors`.
pub fn seen(c: &Canvas, colors: &[Color]) -> usize {
    c.cells().iter().filter(|x| colors.contains(&x.fg)).count()
}

/// Runs a scene for `frames`, drawing it every few and telling whether any
/// drawing had something in one of `colors`.
pub fn shows(
    s: &mut Particles,
    ctx: &Ctx,
    c: &mut Canvas,
    frames: usize,
    colors: &[Color],
) -> bool {
    let mut shown = false;
    for f in 0..frames {
        s.update(ctx, 1.0 / 30.0);
        if f % 5 == 4 {
            c.clear();
            s.draw(ctx, c);
            shown |= seen(c, colors) > 0;
        }
    }
    shown
}

/// The scene survives odd sizes, odd options and huge steps under two
/// themes while dressed for `fiesta`.
pub fn survives(fiesta: Fiesta) {
    let themes = [Theme::classic(), Theme::builtin("dracula").unwrap()];
    for theme in &themes {
        for mode in MODES {
            for more in ["", "\ncount = 0\nspeed = 10.0\nscale = 10.0"] {
                let o = opts(mode, more);
                let ctx = ctx(theme, &o, fiesta);
                let mut s = Particles::new(&ctx);
                let sizes = [(80, 24), (1, 1), (2, 3), (7, 2), (300, 90), (3, 40)];
                for (i, &(w, h)) in sizes.iter().enumerate() {
                    s.resize(&ctx, w, h);
                    if i == 0 {
                        s.retheme(&ctx);
                    }
                    let mut c = Canvas::new(w, h, Color::Reset);
                    for f in 0..30 {
                        s.update(&ctx, if f % 13 == 12 { 5.0 } else { 1.0 / 30.0 });
                        c.clear();
                        s.draw(&ctx, &mut c);
                    }
                }
            }
        }
    }
}

/// A holiday the scene does not dress for is an ordinary day.
pub fn is_ordinary(fiesta: Fiesta) {
    let theme = Theme::classic();
    for mode in MODES {
        let o = opts(mode, "");
        let (plain, other) = (ctx(&theme, &o, Fiesta::NONE), ctx(&theme, &o, fiesta));
        let (mut a, mut ca) = scene(&plain, 90, 30);
        let (mut b, mut cb) = scene(&other, 90, 30);
        run(&mut a, &plain, &mut ca, 150);
        run(&mut b, &other, &mut cb, 150);
        assert!(b.costume.is_none(), "{mode}");
        assert!(ca.cells() == cb.cells(), "{mode}: not an ordinary day");
    }
}

/// The holiday comes and goes through `retheme` and leaves no trace: a scene
/// that had it ends up drawing exactly what one that never did does. While
/// it lasted something of it (`marks`) showed.
pub fn comes_and_goes(fiesta: Fiesta, marks: &[Color]) {
    let theme = Theme::classic();
    for mode in MODES {
        let o = opts(mode, "");
        let plain = ctx(&theme, &o, Fiesta::NONE);
        let dressed = ctx(&theme, &o, fiesta);
        let (mut a, mut ca) = scene(&plain, 90, 30);
        let (mut b, mut cb) = scene(&plain, 90, 30);
        run(&mut a, &plain, &mut ca, 30);
        run(&mut b, &plain, &mut cb, 30);
        b.retheme(&dressed);
        assert!(b.costume.is_some(), "{mode}");
        run(&mut a, &plain, &mut ca, 300);
        assert!(shows(&mut b, &dressed, &mut cb, 300, marks), "{mode}");
        b.retheme(&plain);
        assert!(b.costume.is_none(), "{mode}");
        run(&mut a, &plain, &mut ca, 90);
        run(&mut b, &plain, &mut cb, 90);
        assert!(ca.cells() == cb.cells(), "{mode}: the holiday left a trace");
    }
}
