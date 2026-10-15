# Writing a scene

A scene is one file in `src/scenes/` that exports `pub const INFO: SceneInfo`
and is listed in `src/scenes/mod.rs` (`pub mod` + an entry in `ALL`).

```rust
use crate::color::{Color, rgb};
use crate::render::Canvas;
use crate::scene::{Ctx, Opt, Scene, SceneInfo, Val};

pub const INFO: SceneInfo = SceneInfo {
    name: "sparks",
    aliases: &["spark"],
    about: "sparks fly upward",                       // shown by `oxiper list`
    options: &[
        Opt { key: "count", default: Val::I(40), help: "How many sparks" },
        Opt { key: "gravity", default: Val::F(9.8), help: "Downward pull" },
    ],
    make: |ctx| Box::new(Sparks::new(ctx)),
};
```

## Lifecycle

1. `make(ctx)` builds the scene (no size yet). Read options here with
   `ctx.opts.i("count")`, `.f()`, `.b()`, `.s()`, `.color()`; the declared
   default is returned when the user did not set one. Seed your RNG with
   `ctx.rng()` so `--seed`, `record` and `bench` are reproducible.
2. `resize(ctx, w, h)` is called right after `make` and on every terminal
   resize. `w`/`h` are in cells and can be as small as 1×1.
3. `retheme(ctx)` is called after creation and whenever the theme changes.
   Cache colors and gradients here instead of computing them per frame.
4. Every frame: `update(ctx, dt)` with `dt` in seconds (already multiplied by the
   speed setting), then `draw(ctx, canvas)` on a freshly cleared canvas.
5. Optionally, `next_change()` returns the seconds until the picture changes again
   (`f32::INFINITY` when it is static). The app sleeps until then instead of redrawing
   identical frames; that's how a grown bonsai costs no CPU. The default (0) means
   "every frame".

## Drawing

`Canvas` (cells, clipped, `i32` coordinates):

- `set(x, y, ch, fg)`, `set_attr(x, y, ch, fg, BOLD | DIM)`
- `text(x, y, "str", fg)`, `art(x, y, &["lines"], fg)` (spaces transparent)
- `set_cell(x, y, Cell { .. })`, `set_bg(x, y, color)` — only when a
  background is really part of the picture; both are dropped in strict mode
- `get(x, y)`, `get_mut(x, y)`, `w()`, `h()`, `bg()`

Sub-cell layers (allocate once in `resize`, `clear()` each frame, `blit` onto the canvas):

- `Braille` — 2×4 dots per cell, one color per cell: `set`, `setf`, `line`,
  `line_with`, `pw()`/`ph()` in dots. Great for thin lines, orbits, waves.
- `Pixels` — 1×2 square-ish pixels per cell using half blocks: `set`, `get`,
  `fill_rect`, `pw()`/`ph()`. `Color::Reset` pixels stay transparent. Great
  for fire, plasma and pixel art.

## Colors

- `Color::Reset` is "the terminal's default" — that is what keeps translucent
  terminals see-through. Never paint a background just to clear it.
- `ctx.pick(i, native)` → theme accent `i`, or `native` under `classic`.
- `ctx.ramp(&[stops])` → a 256-step `Gradient` (`.at(t)`), your stops under
  `classic`, the theme ramp otherwise.
- `ctx.fade(color, k)` blends toward the terminal background (`k = 1` vanishes).
  Use it for trails instead of blending toward black.
- `Color::lerp`, `Color::scale`, `color::mix_oklab` for the rest.

Under `classic` a scene should look like the program it pays homage to.
Under every other theme it should look good using only theme colors.

## Rules

- Never panic: any size from 1×1 to 500×150, any `dt` (the stress test passes
  `dt = 5.0`; clamp or sub-step internally).
- No per-frame heap allocation in hot paths; reuse buffers.
- Budget: update + draw well under 1 ms at 200×60 (`oxiper bench`).
- Fill the screen sensibly at any aspect ratio; scale counts with the area.

## Checking your work

```sh
cargo test                                   # includes the size/time stress test
oxiper snap -s sparks --size 100x30 --at 8   # one frame as text (--ansi for color)
oxiper bench -s sparks                       # µs per frame, bytes per frame
oxiper record /tmp/s.cast -s sparks --size 100x30 --seconds 6
agg --font-family "DejaVu Sans Mono,DejaVu Sans,Symbola" /tmp/s.cast /tmp/s.gif
```
