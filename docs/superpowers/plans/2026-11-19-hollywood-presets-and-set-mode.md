# hollywood: presets and set mode — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** hollywood can talk about a video's topic in Spanish from a small preset file, and can hold still as a backdrop on a film set (set mode), on a screen on its side or lying down.

**Architecture:** A preset is loaded into the `Vocab` + `Dress` structures the holidays already use, plus a small `Preset` for what those have no field for; the widgets learn about it through `Garb`, a `Copy` value that stands where the `Fiesta` stands today. Set mode is half the app's (calendar, playlist, eggs, seed, keys) and half hollywood's (a layout that is a pure function of seed and size, a calm log, calm widgets, pace and dimming). Ordinary days and holidays do not move: a fingerprint recorded before any change guards them.

**Tech Stack:** Rust 2024, MSRV 1.88; `clap`, `clap_complete`, `fastrand`, `libc`, `toml`. No new dependency.

**Spec:** `docs/superpowers/specs/2026-11-18-hollywood-presets-and-set-mode-design.md` (amended with K13: on a tall screen every band holds exactly one pane). The spec is the authority: a task's implementer reads the sections its task names, and where this plan and the spec disagree the spec wins and the task's report says so.

## Global Constraints

Every task's requirements include this section.

**Where and how**

- Work directly on `main` in the repository root. No worktrees, no branches, no push (the coordinator pushes after review).
- One task, one local commit, in the repo's style: `area: what changed` in lower case, a short body when it helps (`git log --oneline -30` shows it). Each task names its message.
- The repo is public. No personal details (names, e-mail addresses, home paths, what any video is about) in code, tests, docs or commit messages. Example text uses what the spec's example preset uses.
- Code, comments and docs are English; preset keys and blanks are Spanish and unaccented (`titulos`), as the spec has them.
- A run by hand passes `--config /dev/null`, so nobody's own config file is read: `cargo run -q -- --config /dev/null snap -s hollywood --size 67x60 --at 20`.

**The gates** (run all of them before the commit; every one must pass)

1. `cargo fmt --check`
2. `cargo clippy --all-targets --locked -- -D warnings`
3. `cargo test --locked` (about 2.5 minutes; 986 tests before Task 1, and the count only grows)
4. `cargo build --release --locked && ./target/release/oxiper bench --size 200x60 --frames 300` (it must finish; release is `panic = "abort"`)
5. `cargo +1.88 check --locked` when the 1.88 toolchain is installed (`rustup toolchain list`). When it is not, say so in the task's report and use nothing stabilized after Rust 1.88.

Known state before Task 1: gate 2 fails with nine `needless_range_loop` errors from a clippy newer than the code (six in scenes, three in tests). Task 1 is held to "no error beyond those nine"; Task 2 clears them; from Task 3 on gate 2 is strict.

**What must not move** (spec anti-goal 5, C20)

- No existing `#[test]` function, test helper or test constant is changed or removed. New tests are added beside them or in new test modules. The existing tests compile and pass as they stand.
- The constants recorded in Task 1 (`fingerprint.rs`, and the seeds in `app.rs`) are never edited afterwards. If one stops matching, the change is wrong.
- On a path that is reached with no preset and outside set mode, no random number is added, removed or reordered. Anything a preset or set mode needs at random comes from inside a branch only they reach, or from a generator of their own.
- These keep their shape, because tests and the holiday modules write them out: `Ctx`, `Env`, `Vocab`, `Dress` and `Packet` gain no field; `Opts::new(specs, table)`, `Pal::new(ctx)`, `Feed::new(kind, seed, level, mx)`, `make_widget(kind, seed, level, mx)` and every widget's `new(seed, mx)` / `new(seed, level, mx)` keep accepting a `Fiesta` where they take one today (`impl Into<Garb>`); `Hollywood`'s `fiesta` field stays a `Fiesta`.
- Not touched at all: the holiday modules (`navidad.rs`, `patrias.rs`, `revolucion.rs`, `muertos.rs`, `posada.rs`, `grito.rs`, `archivo.rs`, `mictlan.rs`, `rostro.rs`, `inocentes.rs`), the eggs other than `src/eggs/mod.rs`, and `widgets::glyph` (the holidays draw arbitrary text with it; new glyphs live in `letrero.rs`).

**The scene rules** (`docs/SCENES.md`)

- Never panic: any size from 1x1, any `dt`, any preset file. No `unwrap`/`expect`/indexing on anything that came from a file or from the terminal's size.
- No per-frame heap allocation in hot paths; update + draw well under 1 ms at 200x60.

**Code that is not wired yet**

- oxiper is a binary crate, so code that no non-test path reaches is `dead_code` and fails gate 2, a field that only tests read included. A module added before all of it is reached starts with `#![allow(dead_code)]` and a comment naming the task expected to use the rest. Every later task that touches such a module tries gate 2 without the line and removes it as soon as that passes; after Task 22 none is left. Nothing else is silenced, with two exceptions the repo already has a form for: a hook that exists for tests is `#[cfg(test)]`, and a method that is scene API before a scene uses it carries `#[allow(dead_code)] // scene API (docs/SCENES.md)`, as `Opts::color` does.

**Tests**

- Process-wide state (the preset cache, the string interner) is shared by tests that run in parallel. A test that needs a file writes its own under `std::env::temp_dir()`, with the process id and a per-test name in the file name, and never asserts on a global count (the interner's size); it asserts on what its own calls did (`preset::leaked()`, `cache::parses(path)`, `std::ptr::eq`).
- No `std::env::set_var` (unsafe in edition 2024 and racy): functions that need the config directory take it as a parameter.
- A file's stamp is its mtime and length. A test that rewrites a file changes its length, or sets the mtime with `File::set_modified`.
- A new test that would run longer than about five seconds does less or uses `std::thread::scope`, as `scenes_survive_every_fiesta` does.
- hollywood's scene-level tests for this work go in new test-only files (`fingerprint.rs`, `preset_tests.rs`, `set_tests.rs`), each declared in `mod.rs` as `#[cfg(test)] mod name;`. A child module can read `Hollywood`'s private fields.

**Wording**

- HUD messages are one line, 60 characters or fewer, the specific part first (`preset: line 12: expected a string`). Fixed by the spec and used as written: `set mode on · seed 4821`, `set mode off`, `set mode · seed 4821`, `hollywood · seed 4821`, `eggs off`, `eggs on`, `preset reloaded`.

## File Structure

New files, all under `src/scenes/hollywood/` unless a path says otherwise:

| file | responsibility | task |
|---|---|---|
| `fingerprint.rs` (test only) | the fingerprint of ordinary days and holidays | 1 |
| `garb.rs` | `Garb`: the holiday, the preset, calm, keep; the vocabulary's share | 3, 10, 19, 22 |
| `letrero.rs` | the fitted banner: glyphs, sizes, wrapping, drawing | 4 |
| `gate.rs` | the last gate: every rule the engine relies on, over one `Vocab` | 5 |
| `neutro.rs` | `BASE`, the neutral Spanish vocabulary; call signs; transfer verbs | 6 |
| `gramatica.rs` | pure text rules: nouns, adjectives, verbs, slugs, blanks, banner lines | 7 |
| `preset.rs` | `Preset`, `Banner`, `Round`; TOML to `Preset`; the interner | 8 |
| `cache.rs` | where a preset file lives; the last good preset per path | 9 |
| `preset_tests.rs` (test only) | the scene and the widgets under a preset | 10, 11, 13 |
| `foro.rs` | the set layout, a pure function of seed and size | 17 |
| `set_tests.rs` (test only) | the scene in set mode | 18 to 22 |
| `docs/presets/hollywood/software-libre.toml` | the example preset (spec 7.2, verbatim) | 8 |
| `docs/PRESETS.md` | the reference and authoring guide | 23 |

Changed files: `src/scene.rs`, `src/config.rs`, `src/app.rs`, `src/main.rs`, `src/headless.rs`, `src/eggs/mod.rs`, and in hollywood `mod.rs`, `lexico.rs`, `feed.rs`, `widgets.rs`, `map.rs`; `README.md`, `docs/SCENES.md`.

Widget kinds, as `make_widget` numbers them: feeds 0 to 8 (8 is the mission log), 9 htop, 10 bandwidth, 11 rain, 12 world map, 13 radar, 14 progress, 15 satellites, 16 market, 17 access banner, 18 keygen. Only 12, 13 and 15 use `sin`/`cos`.

## Calls made in this plan

Where the spec left a choice or was silent. Each is used by the tasks below as written.

1. **Clippy first (Task 2).** `main` fails CI's clippy step today, before this work. It is cleared right after the fingerprint so every later gate means something. The three hits inside existing tests are silenced with a crate-level `cfg_attr(test, ...)`, because existing tests are not edited.
2. **The fingerprint hashes drawn characters**, never colors, of every pane whose kind is below 19 and is not 12, 13 or 15, with FNV-1a 64 written out in the test (std's hasher is not promised stable).
3. **New glyphs live in `letrero.rs`**, not in `widgets::glyph`: `grito.rs` and `inocentes.rs` draw their own strings through that function, and a character that is blank there today must stay blank.
4. **One round of banners.** `letrero_verde` and `letrero_rojo` go into one shuffled round, each pair with its color. With neither key the round is the base's two pairs.
5. **A banner without a caption has none**; the base's bottom caption is never put under the author's words. The top caption is `tema`, or the base's (K11).
6. **Banner lines split on `" / "`** (space, slash, space), at most three parts, so a caption may contain a slash.
7. **Limits the spec does not give:** `cosas` and `adjetivos` 40 characters, `verbos` 60 (room for three forms), a banner caption 40.
8. **Titles when `titulos` run out.** Outside set mode the round starts over. In set mode the i-th pane takes the i-th shuffled title and the panes beyond the list get a generated one.
9. **`frases` in the set's log: every third sentence**, not one in three by chance.
10. **`set_eggs` changed by a reload outside set mode** leaves the flag alone; it is read when set mode next goes on.
11. **The arrows in set mode** show `<scene> · seed <n>`, the form the spec gives for `r`.
12. **Layout arithmetic** the spec leaves to the builder is pinned in Task 17 (round half up; 40% rounds down; the first of two halves gets the odd row or column; the banner yardstick stops at 46 rows) and was checked against both tables of spec 10.3.
13. **Set mode's flat layout** is a new `Node::Seats` variant; the tree code for ordinary days is not rearranged.
14. **The seed number is a `u64`** (`seed_no`), since a `seed` from the config may be any; a drawn one is 1 to 9999.
15. **The fitted banner is built before the gate and the loader** (the spec's order of work has it after): both ask it which characters the banner can draw.
16. **`tema` and a banner's caption are shown as written**, not in capitals; only the ticker and the big letters are upper-cased.
17. **A warning counts entries from 1** (`titulares[4]` is the fourth headline; blank and comment lines are not counted).
18. **`preset_check` is called by `main` before the terminal is taken**, as well as by `Player::new`, so a broken preset never flashes the alternate screen.

## Review Focus

What the spec implies and no test in it exercises, most likely first. Each has a test in the task that owns the code.

1. **A file saved by an editor that adds a BOM, CRLF line endings or decomposed accents** (`o` + U+0301) loads to the same `Preset` as the clean file, with no warning. (Task 8)
2. **A slash inside a banner** (`GNU/LINUX / LIBRE`, or a caption `GNU/Linux`): the big letters are refused with a warning that names `/`; the caption is kept whole. Nothing is split in the wrong place. (Tasks 7 and 8)
3. **A TOML error is several lines long**: what reaches stderr, `config check` and the HUD is one line, with the line number, and the HUD form is 60 characters or fewer. (Tasks 8 and 13)
4. **`set_pace` or `set_dim` out of range** (`0`, `-1`, `99`, `nan`): clamped to 0.1 to 1 and 0 to 0.9; the scene keeps moving and nothing divides by zero. (Task 18)
5. **`preset` names something that is not a regular file** (a directory, `/dev/zero`): an error at once, never a hang or an unbounded read. (Task 9)

---

### Task 1: The fingerprint, and the seeds of `r`

Spec 11.1. Tests only; no other line of the repo changes.

**Files:**
- Create: `src/scenes/hollywood/fingerprint.rs`
- Modify: `src/scenes/hollywood/mod.rs` (one line: `#[cfg(test)] mod fingerprint;` after `mod widgets;`)
- Modify: `src/app.rs` (one new test appended to `mod tests`)

**Interfaces:**
- Consumes: `Hollywood::new`, `resize`, `retheme`, `tick`, `draw`; the private fields `root`, `tape`; `Node::each`; `Pane { rect, kind, title }`.
- Produces: `fingerprint::of(fiesta: Fiesta, w: u16, h: u16) -> u64` (test only), which later tasks never call but must keep green.

- [ ] **Step 1: Write `fingerprint::of`**

  Classic theme, default options, `bg: Color::Reset`, `seed: 2026`, the given `fiesta`. Build as `Player::make` does (`new`, `resize`, `retheme`). Then 3600 times `tick(ctx, 1.0 / 30.0, 1.0 / 30.0)`; after every 30th tick draw into a fresh `Canvas::new(w, h, Color::Reset)` and feed the hash, in this order: for each pane in `each` order its `rect` (`x`, `y`, `w`, `h` as `i32` little-endian), its `kind` as `u64` little-endian and its `title` bytes; then, for each pane with `kind < 19` and `kind` not 12, 13 or 15, the characters of its cells row by row as UTF-8; then the first `w` characters of `tape` as UTF-8. The hash is FNV-1a 64 (`0xcbf29ce484222325`, prime `0x100000001b3`), written in the file.

- [ ] **Step 2: Write the tests with placeholders, run, record**

  ```rust
  #[test]
  fn ordinary_days_and_holidays_do_not_move() {
      let days = [Fiesta::NONE, Fiesta::height(Kind::Muertos), Fiesta::height(Kind::Navidad),
                  Fiesta::height(Kind::Patrias), Fiesta::height(Kind::Revolucion)];
      const WIDE: [u64; 5] = [/* recorded */];
      const TALL: [u64; 5] = [/* recorded */];
      for (i, f) in days.into_iter().enumerate() {
          assert_eq!(of(f, 140, 40), WIDE[i], "140x40 {f:?}");
          assert_eq!(of(f, 67, 60), TALL[i], "67x60 {f:?}");
      }
  }
  #[test]
  fn the_fingerprint_is_its_own_twice() { assert_eq!(of(Fiesta::NONE, 140, 40), of(Fiesta::NONE, 140, 40)); }
  #[test]
  fn the_fingerprint_sees_a_holiday() { assert_ne!(of(Fiesta::NONE, 140, 40), of(Fiesta::height(Kind::Navidad), 140, 40)); }
  ```

  Run `cargo test --locked fingerprint` with zeros in the arrays, copy the ten values from the failure messages into the arrays, run again: PASS. `git status` must show no change outside the three files of this task while the values are recorded.

- [ ] **Step 3: The seeds of `r` and the arrows, in `src/app.rs`**

  ```rust
  /// Outside set mode `r` and the arrow keys draw the seeds they always drew.
  #[test]
  fn reseeding_draws_the_seeds_it_always_drew() {
      for (shuffle, want) in [(false, [/* 5 recorded */]), (true, [/* 5 recorded */])] {
          let mut p = player(&format!("scene = 'neo,fire,rain'\nseed = 7\nshuffle = {shuffle}\nfiestas = 'none'\n")).ok().unwrap();
          let mut got = vec![p.seed];
          p.reseed(); got.push(p.seed);
          p.step_scene(1); got.push(p.seed);
          p.step_scene(-1); got.push(p.seed);
          p.reseed(); got.push(p.seed);
          assert_eq!(got, want, "shuffle {shuffle}");
      }
  }
  ```

  Record the ten values the same way.

- [ ] **Step 4: Gates and commit**

  Gate 2 may show the nine known errors and no other. Commit `fingerprint.rs`, `mod.rs`, `app.rs`: `hollywood: a fingerprint of ordinary days and holidays`.

---

### Task 2: The loops clippy no longer accepts

Not in the spec; see "Calls made in this plan", 1.

**Files:**
- Modify: `src/scenes/constellation/holiday.rs:432`, `src/scenes/hollywood/widgets.rs:193`, `src/scenes/shark/life.rs:430`, `src/scenes/shark/muertos.rs:847`, `src/scenes/shark/water.rs:176`, `src/scenes/waveform/muertos.rs:220`
- Modify: `src/main.rs` (one crate attribute)

**Interfaces:** none.

- [ ] **Step 1: See the nine**

  Run gate 2. Expected: nine `needless_range_loop` errors; six in the files above, three in existing tests (`src/scenes/neo.rs:3397`, `:3766`, `src/scenes/fireworks.rs:6204`).

- [ ] **Step 2: The six in scene code**

  Rewrite each loop the way clippy's own suggestion does (`iter().enumerate()`, `iter_mut()`, `take(n)`), keeping the order of every read, write and random draw. Where the index form is the readable one (two arrays walked by the same index), put `#[allow(clippy::needless_range_loop)]` on that function instead, with a comment saying why.

- [ ] **Step 3: The three in tests**

  Existing tests are not edited. Add as the first lines of `src/main.rs`, before `mod app;` (an inner attribute comes before every item): a comment saying that tests index arrays to name the slot they assert on, and `#![cfg_attr(test, allow(clippy::needless_range_loop))]`.

- [ ] **Step 4: Gates and commit**

  Gate 2 is clean. Gate 3: the 990 tests of Task 1 pass, the fingerprint among them (it covers the `htop` loop). `clippy: the loops that index`.

**Checkpoint for the coordinator:** push after this task and let CI finish before Task 3 starts. CI stops at clippy until this commit, so this is the first run of the fingerprint on macOS, and the only proof that it hashes nothing that depends on the platform. If macOS disagrees with a recorded value, stop: what differs has to be found and left out of the hash, and the values recorded again, before any scene code changes.

---

### Task 3: A garb where the fiesta stood

Spec 6.2, 8.3 ("One way to write it"), 13 step 2. No behavior changes.

**Files:**
- Create: `src/scenes/hollywood/garb.rs`
- Modify: `src/scenes/hollywood/mod.rs` (`mod garb;`, `make_widget`, the calls that pass `self.fiesta` to widgets and `retitle`), `lexico.rs` (`of`), `feed.rs`, `widgets.rs`

**Interfaces:**
- Produces, in `garb.rs`:
  ```rust
  /// What a widget wears: today the day's holiday and nothing else.
  #[derive(Clone, Copy)]
  pub struct Garb { fiesta: Fiesta }
  impl Garb {
      pub fn plain(fiesta: Fiesta) -> Garb;
      /// Does the holiday's vocabulary take this one? `rng.f32() < k * lexico::share(self.level)`.
      pub fn takes(self, rng: &mut Rng, k: f32) -> bool;
  }
  impl From<Fiesta> for Garb;
  impl std::ops::Deref for Garb { type Target = Fiesta; }
  ```
- Produces: `lexico::of(g: impl Into<Garb>) -> Option<&'static Vocab>`; `widgets::dress(mx: Garb)`; `make_widget(kind, seed, level, mx: impl Into<Garb>)`; `Feed::new(kind, seed, level, mx: impl Into<Garb>)`; every ordinary widget's constructor takes `impl Into<Garb>` and stores a `Garb`; the holiday widgets (kinds 19 and up) are still handed a `Fiesta` (`*garb`).

- [ ] **Step 1: A test for `takes`** in `garb.rs`:

  ```rust
  #[test]
  fn takes_draws_what_the_holidays_drew() {
      for (level, k) in [(0.25f32, 1.0f32), (1.0, 1.0), (0.6, 0.5), (1.0, 0.6)] {
          let f = Fiesta { level, ..Fiesta::height(Kind::Navidad) };
          let (mut a, mut b) = (Rng::with_seed(5), Rng::with_seed(5));
          for _ in 0..1000 {
              assert_eq!(Garb::plain(f).takes(&mut a, k), b.f32() < k * lexico::share(level));
          }
          assert_eq!(a.u64(..), b.u64(..)); // one draw each time, no more
      }
  }
  ```

  Run `cargo test --locked garb`: it does not compile. That is the failure.

- [ ] **Step 2: `Garb`, and the type of `mx`**

  In `feed.rs` and `widgets.rs` the field and parameter type `Fiesta` becomes `Garb` wherever it is the widget's holiday. Each `rng.f32() < lexico::share(mx.level)` becomes `mx.takes(rng, 1.0)`, `0.5 * lexico::share(..)` becomes `mx.takes(rng, 0.5)`, `0.6 * ...` becomes `mx.takes(rng, 0.6)`, in the same place in each `&&` chain, so the draw happens exactly when it did. The `muertos::share` tests stay as they are. `k * share` with `k = 1.0` is the same `f32` as `share`.

- [ ] **Step 3: Gates and commit**

  All tests pass unedited, the fingerprint among them. `hollywood: a garb where the fiesta stood`.

---

### Task 4: The fitted banner

Spec 7.5 (C28, C29), 11.2 (the glyphs), 11.3 ("The fitted banner"), 13 step 6. Not wired yet. It comes before the gate and the loader because both ask it which characters the banner can draw.

**Files:**
- Create: `src/scenes/hollywood/letrero.rs` (starts with `#![allow(dead_code)] // wired in Task 11`)
- Modify: `src/scenes/hollywood/mod.rs` (`mod letrero;`)

**Interfaces:**
- Consumes: `widgets::glyph` (read, not changed), `Rect`, `put`, `put_ch`.
- Produces:
  ```rust
  /// The banner's font: `widgets::glyph`, and `Ñ ? ¿ - . +`.
  pub fn glyph(ch: char) -> [u8; 5];
  pub fn has_glyph(ch: char) -> bool;
  #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
  pub enum Size { Pocket, Compact, Narrow, X2, X3, X4, X5, X6 } // ordered small to big
  impl Size {
      pub const BIG_FIRST: [Size; 8];
      /// What one font pixel takes, in half columns and half rows.
      pub fn pixel(self) -> (i32, i32);
      /// Columns a letter takes with its gap: 4 pixels wide.
      pub fn letter(self) -> i32;
  }
  #[derive(Clone, PartialEq, Eq, Debug)]
  pub struct Fit { pub size: Size, /// two to four lines of big letters
                   pub lines: Vec<String>, /// the box, frame included
                   pub w: i32, pub h: i32 }
  /// `w` x `h` is the inside of the pane. None: below pocket (the caller draws one line of text, or nothing).
  pub fn fit(top: &str, bottom: &str, w: i32, h: i32) -> Option<Fit>;
  pub struct Look { pub letters: Color, pub frame: Color, pub caption: Color, /// the box's background; None paints none
                    pub ink: Option<Color> }
  /// The box centered in `r`: background, frame, the two captions when they fit in it, the letters.
  pub fn draw(c: &mut Canvas, r: Rect, fit: &Fit, captions: (&str, &str), look: &Look);
  ```

- [ ] **Step 1: Tests** in `letrero.rs`

  - `every_glyph_is_its_own`: A to Z, `Ñ`, 0 to 9 and `! ¡ ? ¿ : - . +` all have a glyph and no two are equal; `glyph('Ñ') == [0b111, 0b000, 0b110, 0b101, 0b101]` and differs from `glyph('N')`; for every character `widgets::glyph` draws, `glyph` returns the same rows.
  - `the_table_of_cases` (spec 7.5), as (size, lines of letters, box):

    | inside of the pane | `SOFTWARE` / `LIBRE` | `GNU` / `NO ES UNIX` | `CUATRO` / `LIBERTADES` |
    |---|---|---|---|
    | 66x19 | X2, 2, 66x13 | X2, 3 (`GNU`, `NO ES`, `UNIX`), 42x19 | Narrow, 2, 42x13 |
    | 119x28 | X3, 2, 98x19 | X3, 3, 62x28 | X2, 2, 82x13 |
    | 131x44 | X4, 2, 130x24 | X4, 3, 82x36 | X3, 2, 122x19 |
    | 73x21 | X2, 2, 66x13 | X2, 3, 42x19 | Narrow, 2, 42x13 |
    | 49x15 | Narrow, 2, 34x13 | Narrow, 2, 42x13 | Narrow, 2, 42x13 |
    | 33x9 | Pocket, 2, 18x8 | Pocket, 2, 22x8 | Pocket, 2, 22x8 |

  - `the_biggest_that_fits`: for the four pairs of the example preset and every inside from 1x1 to 150x60: when there is a fit, the box is inside the pane; the lines, joined, are the words of `top` then of `bottom`, none broken, and no line has words of both; there are 2 to 4 lines; and no bigger size passes the same checks. When there is none: `h < 8`, or the longest word does not fit at two columns a letter.
  - `below_pocket_there_is_no_fit`: `fit("SOFTWARE", "LIBRE", 33, 7)` and `(17, 9)` are `None`; `(18, 8)` is `Pocket`.
  - `the_letters_are_the_fonts_pixels`: `AÑO` over `AÑO` drawn at each of the eight sizes: every quarter cell that a set font pixel covers is lit, and every other quarter cell inside the frame is not.
  - `nothing_outside_the_pane`: every example pair, every inside from 1x1 to 80x40, drawn into a `Rect` set in from the canvas's edges: no cell outside it changes.
  - `letters_are_one_color`: with `canvas.strict` on and off, every cell of the letters has `fg == look.letters`; with it on, no cell has a background.
  - `captions_fit_or_are_left_out`: a caption shorter than the box by 2 or more is on its frame row, centered by characters; a longer one is not drawn.

- [ ] **Step 2: Implement**

  The pixel of each size, in (half columns, half rows): X6 (12, 6), X5 (10, 5), X4 (8, 4), X3 (6, 3), X2 (4, 2), Narrow (2, 2), Compact (2, 1), Pocket (1, 1); `letter()` is twice the first number. `fit` tries `BIG_FIRST` in order. At a size, a line holds `(w - 2) / letter()` letters; each of the two lines is wrapped greedily between words to that many; the size fails if a word is longer, if there are more than four lines in all, or if the box is higher than `h`. The box is `2 + letter() x (letters of the longest line)` columns by `2 + ceil((6N - 1) x half rows / 2)` rows for `N` lines. Lines and box are centered by characters, never by bytes.

  One routine draws every size: rasterize into quarter cells and choose each cell's character from its four quarters (`█ ▀ ▄ ▌ ▐ ▘ ▝ ▖ ▗ ▛ ▜ ▙ ▟ ▚ ▞`). Letters set a character and a foreground only; the box's background goes through `Canvas::set_bg`, so strict transparency drops it and never blends. New glyphs: `?` `[0b110, 0b001, 0b010, 0b000, 0b010]`, `¿` `[0b010, 0b000, 0b010, 0b100, 0b011]`, `-` `[0, 0, 0b111, 0, 0]`, `.` `[0, 0, 0, 0, 0b010]`, `+` `[0, 0b010, 0b111, 0b010, 0]`.

- [ ] **Step 3: Gates and commit.** `hollywood: a banner fitted to its pane, in eight sizes`.

---

### Task 5: The last gate

Spec 8.4 ("The last gate"), 5 ("`Dress` has hidden shape rules"), K12, 13 step 3. Not wired yet.

**Files:**
- Create: `src/scenes/hollywood/gate.rs` (starts with `#![allow(dead_code)] // wired in Task 10`)
- Modify: `src/scenes/hollywood/mod.rs` (`mod gate;`)

**Interfaces:**
- Consumes: `letrero::has_glyph`.
- Produces: `gate::check(v: &Vocab) -> Result<(), String>`; the error names the field and the entry (`dress.key_title: 15 bytes, at most 14`).

- [ ] **Step 1: Tests** in `gate.rs`. A helper `broken(f: impl FnOnce(&mut Vocab)) -> String` copies `navidad::VOCAB` field by field into a local `Vocab`, applies `f`, and returns the error. (`VOCAB` is a `static` and `Vocab` derives nothing; every field is `Copy`, so either write the copy with struct-update syntax, `Vocab { dress: Dress { ..V.dress }, ..V }`, or add `#[derive(Clone, Copy)]` to `Vocab`, `Dress` and `Packet`. A derive is not a field.)

  - `the_holidays_pass`: `check` is `Ok` for each of `lexico::ALL`.
  - `every_rule_fires`: one case per rule below; each error contains the field's name.

- [ ] **Step 2: `check`**

  The floor of rules. Read every use of every `Vocab` and `Dress` field in `lexico.rs`, `feed.rs` and `widgets.rs` and add any other that can panic or draw outside its pane:

  | rule | why |
  |---|---|
  | none of these is empty: `people`, `users`, `things`, `places`, `files`, `dirs`, `daemons`, `hosts`, `errnos`, `locals`, `errs`, `holy`, `passwords`, `headlines`, `nouns`, `adjs`, `ofs`, `verbs`, `states`, `errors`, `units`, `canned_es`, `canned_en`, `mundane`, `packets`, `dress.rain_words` | `pick` indexes `rng.usize(..len)` |
  | no entry of those is an empty string; no `feeds` entry is | titles and names |
  | every song has at least one line | `lines[..len - 1]` |
  | `mundane_es <= mundane.len()` | K12 |
  | every packet has `len.0 <= len.1` | `rng.u16(a..=b)` |
  | `dress.key_title.len() <= 14` (bytes) | `17 - 3 - len` in `Keygen` |
  | every `dress.rain_words` entry is ASCII and 3 bytes or more | read byte by byte |
  | `dress.sat_names.len() >= 5`; `dress.symbols.len() >= 8` with one bullish | the trackers |
  | every `dress.users` entry is non-empty | `htop` |
  | `dress.granted[0]`, `[1]`, `dress.denied[0]`, `[1]` are non-empty and every character is a space or has a glyph (`letrero::has_glyph`) | the banner |
  | `passwords`, `holy`, `locals`, `errs`, `files`, `dirs`, `daemons`, `hosts` are ASCII | identifiers |

- [ ] **Step 3: Gates and commit.** `hollywood: the last gate, over the three holidays`.

---

### Task 6: The neutral base, and "del" where a thing is filled in

Spec 8.1, 7.4 (C30), 13 step 4.

**Files:**
- Create: `src/scenes/hollywood/neutro.rs` (starts with `#![allow(dead_code)] // wired in Task 10`)
- Modify: `src/scenes/hollywood/mod.rs` (`mod neutro;`), `lexico.rs` (`NOUNS`, `ADJS`, `VERBS`, `STATES`, `ERRORS`, `UNITS` become `pub(super)`; `Vocab::expand`)

**Interfaces:**
- Consumes: `gate::check`.
- Produces: `neutro::BASE: Vocab` (a `pub static`); `neutro::CALLS: &[&str]` (12 or more upper-case ASCII call signs of 9 characters or fewer: `NODO`, `ECO`, `VECTOR`, ...; no people); `neutro::TRANSFERS: &[&str]` = `subiendo`, `descargando`, `respaldando`, `transfiriendo`, `restaurando`, `sincronizando`.
- Produces: in `Vocab::expand`, when `{o}` is filled with a thing that starts with `el ` and what is already in `out` ends with `de ` or `a ` as a whole word (at the start of `out`, or after a space; any case of the `d` or `a`), the two are contracted: `de el kernel` to `del kernel`, `a el` to `al`, `De el` to `Del`.

- [ ] **Step 1: Tests**

  In `lexico.rs`, appended to `mod tests`:
  - `things_contract_after_de_and_a`: a local `Vocab` copied from `neutro::BASE` with `things: &["el kernel", "la licencia", "las cuatro libertades"]`; over 10,000 expansions each of `Auditoría de {o}`, `Acceso a {o}`, `De {o} depende todo`, `para {o}`: never `de el` or `a el` in any case; the outputs include `Auditoría del kernel`, `Acceso al kernel`, `Auditoría de la licencia`, `Del kernel depende todo`, `para el kernel`.
  - `the_holidays_things_have_no_article`: no entry of `things` in `lexico::ALL` starts with `el `.

  In `neutro.rs`:
  - `the_base_passes_the_gate`: `gate::check(&BASE)` is `Ok`.
  - `the_base_has_its_floor`: `canned_es.len() >= 40`, `headlines.len() >= 24`, `people.len() >= 16`, `errnos.len() >= 24`, `CALLS.len() >= 12`, and 12 or more in each of `users`, `things`, `places`, `files`, `dirs`, `daemons`, `hosts`, `locals`, `errs`, `holy`, `passwords`.
  - `the_base_is_all_spanish`: `mundane_es == mundane.len()`; every line of `canned_en` is also in `canned_es`; `(BASE.days)(f)` is empty for `Fiesta::NONE` and for a holiday; `songs` is empty.
  - `the_base_writes_clean_text`: the checks of `plenty_of_clean_text_in_both_languages` and `titles_labels_idents_passwords_and_packets_are_clean` over `BASE` (10,000 sentences at level 7 with no `{`, `}`, `*` or double space; titles lower case; identifiers and passwords ASCII; headlines upper case), without the English count.
  - `the_base_things_carry_their_article`: every `things` entry starts with `el `, `la `, `los ` or `las `.
  - `call_signs_fit_the_radar`: each of `CALLS` is ASCII upper case, 1 to 9 characters.

- [ ] **Step 2: Write `BASE`**

  Every field of `Vocab` and `Dress`, in Spanish, about nothing in particular (no holiday, no country, no real person; its `people` are roles or invented names). `nouns`, `adjs`, `verbs`, `states`, `errors`, `units` may be the plain tables. Canned lines and headlines lean on blanks (`Aviso de {s}: reinicio programado en {n} minutos`). Errno messages, packet `info`, labels, `deny`/`admit`, `phase`, `dna_head`, `dna_kin`, `tree_end` are Spanish. `dress.granted` = `["ACCESO", "CONCEDIDO", ..]`, `dress.denied` = `["ACCESO", "DENEGADO", ..]`. `domain` ends in `.example`. `mark` is `'\u{25c6}'`.

- [ ] **Step 3: The contraction in `expand`, then gates and commit.** `hollywood: a neutral Spanish base, and "del" where a thing is filled in`.

---

### Task 7: The grammar a preset is written in

Spec 7.4 (nouns, adjectives, verbs, blanks), 7.5 ("What can be written"), 8.2 (slugs), 8.4 (the per-entry warnings), 11.2.

**Files:**
- Create: `src/scenes/hollywood/gramatica.rs` (starts with `#![allow(dead_code)] // wired in Task 10`)
- Modify: `src/scenes/hollywood/mod.rs` (`mod gramatica;`)

**Interfaces:**
- Consumes: `letrero::has_glyph` (in `letters`).
- Produces (all pure, none panics on any `&str`; an `Err` is the text of a warning without the key):
  ```rust
  /// Composes the Spanish combining marks, collapses runs of spaces, trims.
  /// Err for a character that does not take one cell (control, wide, a stray combining mark).
  pub fn tidy(s: &str) -> Result<String, String>;
  pub struct Noun { pub word: String, pub with_article: String, pub fem: bool, pub plural: bool }
  pub fn noun(entry: &str) -> Result<Noun, String>;
  /// The engine's form: `privativ*`, `libre`, `con copyleft`, `protegid* por copyleft`.
  pub fn adjective(entry: &str) -> Result<String, String>;
  /// (infinitive, gerund, participle without its final `o`), the engine's form.
  pub fn verb(entry: &str) -> Result<(String, String, String), String>;
  /// Lower case, accents stripped, spaces to `_`, ASCII only. None for a word with `ñ` or with nothing left.
  pub fn slug(s: &str) -> Option<String>;
  /// "Richard Stallman" -> "stallman": the slug of the last word.
  pub fn login(person: &str) -> Option<String>;
  /// `{persona}` -> `{s}` and the rest of the table in spec 7.4. Err for an unknown blank or a stray brace.
  pub fn blanks(line: &str) -> Result<String, String>;
  /// Big letters: upper case, accents stripped, `Ñ` kept. Err names a character outside
  /// A-Z, Ñ, 0-9, space and `! ¡ ? ¿ : - . +`, a line over 12 characters, a word over 12.
  pub fn letters(line: &str) -> Result<String, String>;
  /// "TOP / BOTTOM" or "TOP / BOTTOM / caption": split on " / ", at most three parts.
  pub fn banner(line: &str) -> Result<(String, String, Option<String>), String>;
  ```

- [ ] **Step 1: Tests** (table-driven, in `gramatica.rs`)

  - `nouns_carry_their_article`: `el kernel` -> (`kernel`, masc, singular, `el kernel`); `la licencia` -> fem; `los parches` -> masc plural; `las cuatro libertades` -> fem plural, word `cuatro libertades`; `el área (f)` -> fem, `with_article` `el área`; `kernel`, `un kernel`, `el`, `` are `Err`.
  - `adjectives_by_their_ending`: `privativo` -> `privativ*`; `libre` -> `libre`; `con copyleft` and `sin puertas traseras` unchanged; `protegid* por copyleft` unchanged; `español` unchanged.
  - `verbs_from_the_infinitive`, as (gerund, participle + `o`):

    | infinitive | gerund | participle |
    |---|---|---|
    | liberar | liberando | liberado |
    | compartir | compartiendo | compartido |
    | distribuir | distribuyendo | distribuido |
    | leer | leyendo | leído |
    | extraer | extrayendo | extraído |
    | distinguir | distinguiendo | distinguido |
    | escribir, describir | escribiendo, describiendo | escrito, descrito |
    | abrir | abriendo | abierto |
    | hacer, deshacer | haciendo, deshaciendo | hecho, deshecho |
    | componer | componiendo | compuesto |
    | resolver | resolviendo | resuelto |
    | devolver | devolviendo | devuelto |
    | ver | viendo | visto |
    | romper | rompiendo | roto |
    | decir | diciendo | dicho |
    | convertir | convirtiendo | convertido |
    | pedir | pidiendo | pedido |
    | seguir | siguiendo | seguido |
    | elegir | eligiendo | elegido |
    | corregir | corrigiendo | corregido |
    | servir | sirviendo | servido |
    | dormir | durmiendo | dormido |

    `proveer | proveyendo | provisto` gives exactly those three; `liberarse`, `libre`, `ar`, `` are `Err`.
  - `slugs_are_ascii`: `Código fuente` -> `codigo_fuente`; `GNU` -> `gnu`; `diseño` -> `None`; `¡!` -> `None`. `login("Richard Stallman")` is `stallman`; `login("Ñu")` is `None`.
  - `blanks_become_the_engines`: `{persona} revisa {cosa} de {nombre}: {n} {d} {pct} {hex} {usuario}` -> `{s} revisa {o} de {t}: {n} {l} {p} {x} {u}`; `{cosa`, `}`, `{nada}`, `{}` are `Err`.
  - `letters_for_the_banner`: `Código` -> `CODIGO`; `año` -> `AÑO`; `¿Qué es` -> `¿QUE ES`; `C++`, `WEB 3.0`, `GPL-3` pass; `GNU/LINUX` is `Err` and the message contains `/`; `ABCDEFGHIJKLM` (13) and `CONSTITUCIONAL X` (a 14-letter word) are `Err`; a line of exactly 12 passes.
  - `banner_lines_split_on_spaced_slashes`: `SOFTWARE / LIBRE` -> (`SOFTWARE`, `LIBRE`, None); `GNU / NO ES UNIX / desde 1983` -> caption `desde 1983`; `A / B / GNU/Linux / x` -> caption `GNU/Linux / x`; `SOLO` and ` / LIBRE` are `Err`.
  - `tidy_composes_and_refuses`: `"co\u{301}digo"` -> `código`; `"n\u{303}"` -> `ñ`; `"  a   b "` -> `a b`; a string with `\u{0}`, with `界`, with an emoji, with a lone `\u{301}` are `Err`.
  - `nothing_panics`: each function over `""`, one character, 10,000 characters, emoji, right-to-left text, `{{{{`, `*` alone.

- [ ] **Step 2: Implement.** The irregular table matches by ending so compounds are covered (`-scribir`, `-poner`, `-hacer`, `-volver`, `-solver`, `-brir`, `-decir`, `ver`, `romper`, and the stem-changers `-vertir`, `pedir`, `seguir`, `servir`, `elegir`, `corregir`, `medir`, `repetir`, `dormir`, `morir`); `-ar` is always regular; a vowel stem takes `-yendo` and an accented `-ído`; `-guir` keeps `-guiendo`.

- [ ] **Step 3: Gates and commit.** `hollywood: the grammar a preset is written in`.

---

### Task 8: From a file to a `Preset`

Spec 7.1 to 7.6, 8.2, 8.4 ("Reading", "Becoming `'static`"), 11.2, C22.

**Files:**
- Create: `src/scenes/hollywood/preset.rs` (starts with `#![allow(dead_code)] // wired in Tasks 10, 11 and 19`)
- Create: `docs/presets/hollywood/software-libre.toml` (the example of spec 7.2, character for character)
- Modify: `src/scenes/hollywood/mod.rs` (`mod preset;`)

**Interfaces:**
- Consumes: `gramatica::*`, `neutro::{BASE, CALLS, TRANSFERS}`, `gate::check`.
- Produces:
  ```rust
  pub struct Banner { pub green: bool, pub top: &'static str, pub bottom: &'static str, pub caption: &'static str }
  pub struct Preset {
      pub vocab: &'static Vocab,
      /// `titulos`, blanks in the engine's keys. Empty: titles are generated.
      pub titles: &'static [&'static str],
      /// `titulares` was given: the ticker shows `vocab.headlines` and nothing else.
      pub verbatim_tape: bool,
      /// Never empty: the file's pairs, or the base's two.
      pub banners: &'static [Banner],
      /// `frases` alone, for the set's log. May be empty.
      pub phrases: &'static [&'static str],
      /// Eight or more: upper-case slugs of `nombres` of 9 characters or fewer, then `neutro::CALLS`.
      pub contacts: &'static [&'static str],
      pub transfers: &'static [&'static str],
  }
  pub struct Loaded {
      pub preset: &'static Preset,
      pub warnings: Vec<String>,
      /// Known keys present in the file (0: an empty preset).
      pub keys: usize,
  }
  /// Total: never panics. Err is one line.
  pub fn parse(bytes: &[u8]) -> Result<Loaded, String>;
  /// Strings that this thread's parses have added to the interner (a thread-local count:
  /// each test runs on its own thread, so nobody else's parse is in it).
  #[cfg(test)] pub fn leaked() -> usize;
  /// Every index once, in a shuffled order, before any repeats; a new round never starts with the last one's last.
  pub struct Round { /* order, at */ }
  impl Round { pub fn new(len: usize) -> Round; pub fn next(&mut self, rng: &mut Rng) -> usize; } // 0 when len is 0
  ```

- [ ] **Step 1: Tests** in `preset.rs` (`EXAMPLE` is `include_str!("../../../docs/presets/hollywood/software-libre.toml")`)

  - `the_example_loads_clean`: no warnings; `keys == 11`; `gate::check(preset.vocab)` is `Ok`; `verbatim_tape`; 7 titles; 4 banners, three green and one red, the first `SOFTWARE`/`LIBRE`; `phrases.len() == 4`.
  - `an_empty_file_is_the_base`: `parse(b"")` has `keys == 0`, no warnings; `vocab.headlines`, `people`, `canned_es` equal `BASE`'s; `titles` empty; `!verbatim_tape`; banners are the base's two (`ACCESO`/`CONCEDIDO` green, `ACCESO`/`DENEGADO` red).
  - `blocks_and_arrays_are_the_same`: each list key written as a `'''` block (with a comment line and a blank line inside) and as an array gives equal lists.
  - `what_each_key_feeds` (spec 8.2), on the example: `people` are the two `personas` and no base person; `places` are the `nombres`; `things` are the ten `cosas` with their articles; `nouns` has `("licencia", true)` and `("kernel", false)` and no plural; `verbs` has `("distribuir", "distribuyendo", "distribuid")`; `adjs` has `privativ*`; `canned_es` contains the four `frases` (blanks rewritten) and the base's lines; `ofs` has `del kernel`, `de la licencia`, `de las cuatro libertades`, `de GNU`, `de Richard Stallman`; `users` has `stallman` and `torvalds`; `dress.users` has six non-empty entries; `dress.rain_words` has `gnu`, `linux`, `emacs`; `domain` ends with `.example`.
  - `tema_is_the_caption_or_the_bases`: with `tema = "Software libre"` `dress.granted[2]` and `dress.denied[2]` contain `Software libre`, as written; without it `dress.granted[2] == BASE.dress.granted[2]`; nothing in the `Preset` contains the file's name (`parse` never sees one).
  - `contacts_are_never_people`: on the example `contacts.len() >= 8`, all upper-case ASCII of 9 or fewer, `GNU` and `EMACS` among them; none equals a slug of a word of `personas`.
  - `every_warning_names_its_entry`: one case each, the warning starting with the key and, where there is an entry, its number counted from 1 (`titulares[4]:`): an unknown key (`titulo = "x"`), a key of the wrong type (`personas = 3`), a table (`[seccion.uno]`), an entry over its limit (101 characters in `titulares`, 41 in `titulos`, 29 in `personas`, 21 in `nombres`, 121 in `frases`, 25 in `tema`, 41 in `cosas`), the 401st entry, a wide character, an unknown blank, a stray `{`, a `*` in `titulares`, a noun without its article, a verb that is not an infinitive, a banner with a `/` in its letters (the warning contains `/`), a banner line of 13, a banner of one line. In each case the other entries of the list are still there.
  - `every_error_is_one_line`: invalid UTF-8, 256 KiB + 1 byte, `titulares = ` (not TOML). The TOML error contains `line 1` (or the right line for an error on line 3) and no `\n`.
  - `bom_crlf_and_decomposed_text_load_the_same` (Review Focus 1): the example with a leading `\u{feff}`, with every `\n` as `\r\n`, and with `ó` written as `o\u{301}`: each parses with no warning and the same `vocab.headlines`, `titles` and banners as the clean text.
  - `a_caption_may_have_a_slash` (Review Focus 2): `letrero_verde = "SOFTWARE / LIBRE / GNU/Linux"` gives caption `GNU/Linux` and no warning; `letrero_verde = "GNU/LINUX / LIBRE"` gives one warning and the base's banners.
  - `hostile_input_never_panics`: empty strings, one character, 10,000 characters, emoji, wide and combining characters, right-to-left text, NUL, braces, only plurals in `cosas`, only `ñ` words in `nombres`, 5,000 entries, a 1 MiB file, 2,000 arrays of random bytes from a seeded `Rng`. Each is `Err`, or `Ok` with a `Vocab` that passes `gate::check`.
  - `the_same_text_leaks_once`: a text unique to this test, parsed twice: `leaked()` grows with the first parse and not with the second.
  - `the_preset_writes_clean_text`: the checks of Task 6's `the_base_writes_clean_text` over the example's `Vocab`, and never `de el` or `a el` in 10,000 sentences and 1,000 expansions of each headline, title and phrase.
  - `a_round_goes_through_everything`: for `len` 1, 2, 7: every index once per `len` draws, and no index twice in a row across 50 rounds when `len > 1`; `Round::new(0).next(..) == 0`.

- [ ] **Step 2: `parse`**

  Order: size, UTF-8, strip a BOM, TOML (flatten its error to one line: `line N: message`), then key by key with `gramatica`, then build the `Vocab` by spec 8.2 over a copy of `BASE` (replace for names and readable text, the engine blends grammar, `frases` are added to the base's canned lines), then `gate::check`, whose failure is an `Err`. Entries are `tidy`'d before anything else. Strings are interned (a `Mutex<HashSet<&'static str>>` in a `static`, `String::leak`); the slices, the `Vocab` and the `Preset` are leaked (`Box::leak`). The patterns by which slugs become file names, daemons, hosts, satellites, symbols and process names are yours (spec 12); the minimums are the gate's and the table of 8.2 (5 satellites, 8 symbols with one bullish, 8 contacts, 6 users), topped up from `BASE`.

- [ ] **Step 3: Gates and commit.** `hollywood: a preset, from its file to a vocabulary`, with the example preset in the same commit.

---

### Task 9: The last good preset

Spec 8.4 ("Where files live", "The last good preset", K5, K10), 11.4.

**Files:**
- Create: `src/scenes/hollywood/cache.rs` (starts with `#![allow(dead_code)] // wired in Task 10`)
- Modify: `src/scenes/hollywood/mod.rs` (`mod cache;`)

**Interfaces:**
- Consumes: `preset::parse`, `preset::Preset`.
- Produces:
  ```rust
  pub enum Answer {
      /// The file as it is on disk.
      Good { preset: &'static Preset, warnings: Vec<String> },
      /// The file is bad now: the last good preset from this path, and why.
      Kept { preset: &'static Preset, why: String },
      /// This path has never loaded.
      Bad(String),
  }
  impl Answer { pub fn preset(&self) -> Option<&'static Preset>; }
  /// A bare name is `<config_dir>/presets/hollywood/<name>.toml`; a value with a `/`
  /// or ending in `.toml` is a path (a leading `~/` is the home directory).
  pub fn locate(value: &str, config_dir: Option<&Path>) -> PathBuf;
  /// The `preset` option of a `[scenes.hollywood]` table. Ok(None): not set, or "".
  /// Err: set and not a string; the message says to quote the name.
  pub fn wanted(table: &toml::Table, config_dir: Option<&Path>) -> Result<Option<PathBuf>, String>;
  pub fn load(path: &Path) -> Answer;
  /// The HUD's form of a reason: `preset: <why>`, one line, 60 characters at most.
  pub fn brief(why: &str) -> String;
  #[cfg(test)] pub fn parses(path: &Path) -> u32;            // how often this path was parsed
  #[cfg(test)] pub fn temp(name: &str, text: &str) -> PathBuf; // a file of this test's own
  ```
  `why` and `Bad`'s text are the reason alone, one line, without the path (`line 12: expected a string`, `the file is empty`, `not a file`); callers that print to stderr put the path in front.

- [ ] **Step 1: Tests** in `cache.rs`

  - `a_name_is_a_file_under_the_config_dir`: `locate("software-libre", Some(Path::new("/c/oxiper")))` is `/c/oxiper/presets/hollywood/software-libre.toml`; `locate("borrador.toml", ..)` is `borrador.toml`; `locate("a/b", ..)` is `a/b`; `locate("~/p.toml", ..)` starts with `$HOME` when `HOME` is set (read, never set, in the test).
  - `the_option_must_be_a_string`: an empty table and `preset = ""` give `Ok(None)`; `preset = "x"` gives the located path; `preset = 2026` and `preset = true` give `Err` containing `quote`.
  - `an_unchanged_file_is_not_read_again`: 1,000 `load`s of one good file: `std::ptr::eq` on every `preset`, `parses(path) == 1`.
  - `a_bad_save_keeps_the_last_good`: good, then invalid TOML: `Kept` with the same `preset` and a `why` containing `line`; `load` again without a save: `Kept`, and `parses` did not grow; then a good save: `Good` with a different `preset`.
  - `an_emptied_file_keeps_the_last_good`: good with keys, then zero bytes: `Kept`, `why == "the file is empty"`.
  - `a_deleted_file_keeps_the_last_good`: `Kept`, no panic.
  - `nothing_good_before_is_an_error`: a path that never loaded and is not TOML, or is missing: `Bad`. A path that never loaded and is empty: `Good`, the base.
  - `paths_do_not_share`: two files give two presets; one path asked twice gives one.
  - `not_a_regular_file_is_an_error_at_once` (Review Focus 5): `load(&std::env::temp_dir())` is `Bad`; under `#[cfg(unix)]`, `load(Path::new("/dev/zero"))` is `Bad`.
  - `brief_fits_the_hud`: for a 200-character reason and for a reason with a `\n`: starts with `preset: `, has no `\n`, `chars().count() <= 60`.

- [ ] **Step 2: Implement**

  A `static` `Mutex<HashMap<PathBuf, Entry>>`; an entry holds the stamp last seen (`Option<(SystemTime, u64)>`, `None` for a file that is not there), the last good `Loaded`, and the reason of the last failure. `load`: `metadata`; not a regular file is a failure; stamp unchanged answers from the entry; changed reads at most 256 KiB + 1 bytes (`Read::take`) and parses. A parse with `keys == 0` after a good one with keys is the failure `the file is empty`. A poisoned mutex is recovered (`into_inner`), never unwrapped.

- [ ] **Step 3: Gates and commit.** `hollywood: the last good preset, by path`.

---

### Task 10: The engine with a preset

Spec 8.3 (all eight points but the fourth, and "How the share becomes 1"), 8.2 (contact names), 8.4 (`Hollywood::new` asks the cache), 11.3. The readable surfaces are Task 11; start-up checks are Task 12.

**Files:**
- Modify: `src/scenes/hollywood/mod.rs` (the `preset` option; `Hollywood`; `Pal`), `garb.rs`, `lexico.rs`, `feed.rs`, `widgets.rs`; the `allow(dead_code)` lines this task makes unnecessary go
- Create: `src/scenes/hollywood/preset_tests.rs`

**Interfaces:**
- Consumes: `cache::{wanted, load, Answer}`, `preset::Preset`, `neutro`, `Garb`.
- Produces:
  ```rust
  // INFO.options, after `babble`:
  Opt { key: "preset", default: Val::S(""), help: "Preset with the topic the screen talks about: a name under presets/hollywood/ in the config folder, or a path (docs/PRESETS.md)" }
  // garb.rs
  pub struct Garb { fiesta: Fiesta, pub preset: Option<&'static Preset>, leaky: bool }
  impl Garb {
      /// Under a preset the holiday is none, at its height: derefs to
      /// `Fiesta { kind: Kind::None, level: 1.0, month: 0, day: 0 }`.
      pub fn preset(p: &'static Preset) -> Garb;
      /// Is every word the preset's? (No draw is made for the share.)
      pub fn whole(self) -> bool;                       // preset.is_some() && !leaky
      #[cfg(test)] pub fn leaky(p: &'static Preset) -> Garb; // the preset's Vocab, a holiday's share
  }
  // `takes` answers true without drawing when `whole()`.
  // lexico.rs: `of` answers `g.preset.map(|p| p.vocab)` first.
  impl Vocab { pub fn progress_label_es(&self, rng: &mut Rng, level: u8, transfers: &[&'static str], out: &mut String); }
  // mod.rs
  impl Pal { fn dressed(ctx: &Ctx, plain: bool) -> Pal; } // plain: the holiday counts as none, and oro is amber
  // Pal::new(ctx) is dressed(ctx, false).
  struct Hollywood { /* + */ preset: Option<&'static Preset>, /* .. */ }
  impl Hollywood { fn garb(&self) -> Garb; }           // what every widget made from now on wears
  ```
  `Hollywood::new` asks `cache::wanted` (config dir: the parent of `config::default_path()`) and `cache::load`, and takes whatever preset the answer has; with none it starts without a preset and never panics. With a preset, `self.fiesta` is `Fiesta::NONE` (no garland, no signature widget, no prank) and `retheme` does not dress for the calendar.

- [ ] **Step 1: Tests** in `preset_tests.rs`

  Helpers: `example() -> &'static Preset` (parses the example), `bare() -> &'static Preset` (parses `""`), `scene(opts: &str, fiesta: Fiesta, w: u16, h: u16)`, `outside(c: &Canvas, r: Rect) -> usize` (cells drawn outside `r`).

  - `no_english_prose_under_a_preset` (spec 11.3, three parts). `fn leaks(garb: Garb) -> (BTreeSet<String>, Vec<String>)`: each of the nineteen kinds from `make_widget(kind, seed, 7, garb)` in a 120x40 and a 60x20 pane, three simulated minutes each at `dt = 0.1`, every drawn row collected; the boilerplate written out in the test is removed (`Kernel panic - not syncing`, `Call Trace:`, `Tainted:`, `[ OK ]`, `[permission denied]`, the level tags, the fixed words of the packet lines); the rest is split into lower-case words. It returns the words that are on the stop list (`the`, `of`, `and`, `with`, `from`, `into`, `your`, `our`, `their`, `this`, `that`, `these`, `each`, `every`, `please`, `unable`, `failed`, `cannot`, `not`, `such`, `uploading`, `downloading`, `syncing`) and the rows they came from.
    1. `leaks(Garb::preset(example()))` and `leaks(Garb::preset(bare()))` are empty.
    2. `leaks(Garb::leaky(example()))` is not: it has one of `uploading`/`downloading`/`syncing`, one of `the`/`of`/`with`/`and`, and a row that contains a message of `feed::ERRNOS`.
  - `the_words_come_from_the_preset`: over 10,000 draws each with `Garb::preset(example())`: no errno message is one of `feed::ERRNOS`, no daemon one of `babble::DAEMONS`, no login one of `feed::USERS` (make those `pub(super)`); and `feed::say`, `feed::title`, `feed::label` leave the same text and the same generator state as `v.sentence`, `v.title` and `v.progress_label_es` called directly on a clone of the generator. `say` returns `true` (so logs write `desde`, never `from`).
  - `people_never_stand_beside_a_number`: the radar at 80x24 and the aligner (`FeedKind::Dna`) at 100x30 under the example, 60 simulated seconds: no drawn row contains `Stallman` or `Torvalds`; every radar list row starts with `<contact>-<3 digits>` for a `contact` of `example().contacts`; the aligner shows at least one contact.
  - `a_preset_radar_is_at_its_height`: `Radar::new(1, Garb::preset(example())).blips_len() == 11`.
  - `dressed_widgets_stay_in_their_panes`: every kind below 19 under the example, at the sizes of `every_widget_stays_in_its_pane_on_and_off_the_holiday` and at 34x29, 60x54, 67x59 and 120x105, forty steps: `outside == 0`, no panic.
  - `a_preset_wins_over_the_holiday`: with `ctx.fiesta` at the height of Navidad and of Muertos and the example as the option, 60 seconds of ticks at 140x40: `s.fiesta == Fiesta::NONE`, every kind below 19, `s.fool.is_none()`, `s.env.pal.main == rgb(0x33ee66)` under classic, `s.env.pal.oro == s.env.pal.amber`, and no `●` or `▼` on any title row.
  - `the_map_runs_between_the_worlds_cities`: the map's status row names two entries of `map::CITIES`.
  - `names_fall_in_the_rain`: the rain at 80x30 for 60 seconds: some column, read downward, contains one of `example().vocab.dress.rain_words`.
  - `babble_zero_shows_the_bases_plain_lines`: a log feed at level 0: every message starts with one of `BASE.mundane`.
  - `a_preset_that_is_not_there_is_no_preset`: the option names a missing file: the scene builds, `s.preset.is_none()`.

- [ ] **Step 2: Implement** by the table of spec 8.3:

  | where | under `whole()` |
  |---|---|
  | `feed::say`, `feed::title`, `Hollywood::retitle`, `Hollywood::feed_tape` | `v.sentence` / `v.title` directly, never `mixed` or `mixed_title` |
  | every `takes(rng, 1.0)` | true, no draw |
  | daemon names (`takes(rng, 0.5)`), errno lines (`takes(rng, 0.6)`) | true, no draw |
  | `Vocab::progress_label` (in `feed::label` and `Progress`) | `progress_label_es` with `preset.transfers` |
  | the radar's and the aligner's names | `preset.contacts`; the radar lists `<name>-<id>` |
  | `WorldMap`'s routes | `map::CITIES`, as on an ordinary day |

  Everything else the dressed widgets do for a holiday they do for a preset (spec 8.3, last paragraph).

- [ ] **Step 3: Look at it**

  `cargo run -q -- --config /dev/null snap -s hollywood --size 140x40 --at 20 -o hollywood.preset=docs/presets/hollywood/software-libre.toml`: Spanish text, the preset's names in the logs, no English sentence.

- [ ] **Step 4: Gates and commit.** `hollywood: a preset dresses the screen`.

---

### Task 11: What the camera reads

Spec 7.3, 7.5 ("Where this applies"), 8.3 point 4, 11.3 ("The scene with a preset").

**Files:**
- Modify: `src/scenes/hollywood/mod.rs` (`retitle`, `feed_tape`, the ticker's mark), `widgets.rs` (`Access`), `letrero.rs` (its `allow(dead_code)` goes), `preset_tests.rs`

**Interfaces:**
- Consumes: `preset::{Preset, Banner, Round}`, `letrero::{fit, draw, Fit, Look}`.
- Produces: `Access` under a preset takes its banners from `preset.banners` through a `Round` and draws them with `letrero`; the `Fit` is computed when a banner comes up or the pane is resized, never per frame. `#[cfg(test)] Access::showing(&self) -> Option<usize>` (the index of the banner that is up).

- [ ] **Step 1: Tests** in `preset_tests.rs`

  - `the_ticker_is_the_files_headlines`: a preset with five `titulares` without blanks, five minutes at 140x40: split at `"  ◆  "` (the base's mark between two spaces each side), the items are exactly those five in capitals, and every run of five consecutive items from the start is a permutation of them. With the example, and with `babble = 0` as well: no item contains `MISIÓN` or `MISSION`.
  - `without_titulares_the_ticker_is_the_engines_mix`: a preset with only `nombres`: items start with `MISIÓN ◆ ` or are base headlines; none contains `MISSION`.
  - `titles_come_first`: with `titulos`, every pane's title is `<one of them> · <widget.name()>`, with `babble = 0` too; over ten minutes every one of the seven shows. On the title row the cells of the title's first word have `fg == pal.bright`.
  - `without_titulos_titles_are_generated`: `<widget.name()> · <lower-case words>`, never from `babble::title` (same generator state as `v.title`).
  - `the_banner_shows_the_files_pairs`: `Access` under the example, 200 banners: `showing()` goes through all four before any repeats; the box's frame is `pal.green` for the three green pairs and `pal.red` for the red one. Under a preset with only `letrero_verde`: never `pal.red`.
  - `the_banner_is_fitted`: inner 66x19 with `SOFTWARE / LIBRE`: 66 columns by 13 rows of cells have `bg == pal.ink`; inner 33x9: 18 by 8; inner 33x7: one row of text `SOFTWARE LIBRE` in a box 3 rows high, as today's small form; inner 9x9 or 33x2: no banner.
  - `the_captions_are_tema_and_the_authors`: with `tema` and `SOFTWARE / LIBRE / desde 1983` at 66x19: the top frame row contains `Software libre`, the bottom `desde 1983`; a pair without a caption has a plain bottom frame.

- [ ] **Step 2: Implement.** The ticker under `verbatim_tape`: `v.expand(rng, v.headlines[round.next(rng)], ..)`, upper-cased, separated by `"  <mark>  "`. Without it: the holidays' branch at level 1 with `v.sentence` (Task 10). `retitle` with titles: `expand` of the next title of a scene-level `Round`, then ` · `, then the process name (the draw code already paints what comes before the first `·` bright). `Access` with no preset and outside set mode runs the code it has today, untouched: the fingerprint and the existing banner tests are that check.

- [ ] **Step 3: Look at it** at `--size 140x40` and `--size 67x60` with the example preset; then gates and commit. `hollywood: the ticker, the titles and the banner say what the preset says`.

---

### Task 12: A preset is checked at the door

Spec 8.5, 8.6, K9, K10, 11.4 (the two "fatal" items), 11.6 (the last two items).

**Files:**
- Modify: `src/scenes/hollywood/mod.rs` (`preset_check`), `src/app.rs`, `src/main.rs`

**Interfaces:**
- Consumes: `cache::{wanted, load, Answer}`.
- Produces:
  ```rust
  // src/scenes/hollywood/mod.rs
  /// Ok: the preset's warnings, each prefixed with its name (`software-libre: titulares[4]: ...`); none when no preset is set.
  /// Err: `<path>: <why>` for a preset that cannot be used and has never loaded, or an option that is not a string.
  pub fn preset_check(table: &toml::Table) -> Result<Vec<String>, String>;
  // src/app.rs
  pub fn preset_problems(cfg: &Config) -> Result<Vec<String>, String>; // preset_check(&cfg.scene_table("hollywood"))
  pub struct Reloaded { pub bg_changed: bool, pub mode: ColorMode }
  /// The once-a-second look at config.toml. None: unchanged. A refusal records the file's mtime.
  fn poll_config(player: &mut Player, ro: &RunOpts) -> Option<Result<Reloaded, String>>;
  ```

- [ ] **Step 1: Tests** in `src/app.rs` (files under `temp_dir()`; a preset is named by its absolute path in `[scenes.hollywood]`)

  - `a_broken_preset_stops_the_start`: a never-loaded file that is not TOML: `player(..)` is `Err` containing the path and `line`; a missing file: `Err`; `preset = 2026`: `Err` containing `quote`; the same with `scene = 'neo'` (hollywood is not in the playlist).
  - `an_empty_preset_that_never_loaded_is_the_base`: `Ok`.
  - `a_reload_naming_a_broken_preset_is_refused`: `reload` is `Err` and `p.cfg.fps` is still the old one.
  - `config_check_sees_presets`: a preset with one bad entry: `preset_problems` is `Ok` with one line that contains the file's stem and `titulares[`; a broken one: `Err`.
  - `a_refused_reload_is_tried_once`: a `Player` from `Config::load(Some(path), &[], false)`; the file rewritten with `theme = 'nope'`: `poll_config` is `Some(Err(_))`, and again at once is `None`; rewritten well: `Some(Ok(_))`.

- [ ] **Step 2: Implement.** `Player::new` and `Player::reload` return the `Err` beside the `scene_theme_problems` check. `poll_config` is the body of the once-a-second block of `app::run`, moved; it records `player.cfg.mtime` when `Player::reload` refuses, as the `Config::load` arm already does. In `main.rs`: `config check` adds the error or the warnings to its problems; before `app::run`, `preset_problems(&cfg)?` (so nothing has taken the terminal), and its warnings are printed with the config's before the existing 1.2 s pause; `snap` and `record` print them to stderr.

- [ ] **Step 3: By hand**

  - `cargo run -q -- --config /dev/null config check -o hollywood.preset=docs/presets/hollywood/software-libre.toml` ends in `ok`, exit 0.
  - `... -o hollywood.preset=2026` prints one line that says to quote the name, exit 1.
  - `... -o hollywood.preset=no-such-file.toml` prints the path and the reason, exit 1; `snap` with it prints the same and draws nothing.

- [ ] **Step 4: Gates and commit.** `app: a preset is checked at the door, and a refused reload is not retried`.

---

### Task 13: Live edits

Spec 8.7, 8.6 ("while running"), 11.4.

**Files:**
- Modify: `src/scene.rs` (`Scene::refresh`), `src/app.rs` (`Player::refresh`, the poll, the HUD), `src/scenes/hollywood/mod.rs`, `preset_tests.rs`

**Interfaces:**
- Produces:
  ```rust
  // trait Scene, with the doc comment of spec 8.7:
  fn refresh(&mut self, _ctx: &Ctx) -> Option<Result<String, String>> { None }
  // Player
  pub fn refresh(&mut self) -> Option<Result<String, String>>;
  // Hollywood
  struct Hollywood { /* + */ preset_path: Option<PathBuf>, /// the failure already shown on the HUD
                     reported: Option<String>, /* .. */ }
  ```
  `app::run` calls `player.refresh()` in the once-a-second block, paused or not: `Ok(note)` on the HUD for 2 s, `Err(why)` for 5 s, and the frame is redrawn. hollywood: a new good preset re-dresses the panes where they stand (the path `Hollywood::dress` takes: same rectangles, same kinds, fresh widgets), the ticker starts over, and the note is `preset reloaded` or `preset reloaded · 2 warnings`. A bad file: the reason once (`cache::brief`), then `None` until the reason changes. A scene built over a file that is bad now reports it at its first `refresh`.

- [ ] **Step 1: Tests** in `preset_tests.rs`

  - `an_edit_shows_after_refresh`: rectangles and kinds are equal before and after; the tape holds the new headline; the result is `Some(Ok(s))` with `s == "preset reloaded"`. With one bad entry in the new file: `preset reloaded · 1 warning`.
  - `a_broken_file_is_said_once`: `Some(Err(m))` with `m` starting `preset: `, containing `line`, without `\n`, 60 characters or fewer (Review Focus 3); the old headlines are still on the tape after ten more seconds; the next `refresh` is `None`; a fixed file gives `Some(Ok(_))`.
  - `a_deleted_file_keeps_the_words`: `Some(Err(_))`, then `None`, no panic.
  - `a_bad_file_does_not_reach_the_screen_by_r`: a good load, the file broken, a new `Hollywood` from the same options: its `preset` is the good one (`ptr::eq`), its first `refresh` is `Some(Err(_))` and its second `None`. The same for a file truncated to nothing and for a deleted one.
  - `rebuilds_do_not_read`: 1,000 scenes over one unchanged file: `cache::parses(path) == 1`, one `preset` pointer.
  - `refresh_without_a_preset_is_none`: a scene with no `preset` option answers `None` every time.

  In `src/app.rs`: `refresh_reaches_the_scene`: a `Player` on hollywood with a preset file, the file edited: `p.refresh()` is `Some(Ok(_))`; a `Player` on `neo`: `None`.

- [ ] **Step 2: Implement, gates, commit.** `hollywood: a preset can be edited while it runs`.

---

### Task 14: Set mode in the app

Spec 6.3, 9.1, 9.2, 11.6. The eggs and the seed are Tasks 15 and 16.

**Files:**
- Modify: `src/scene.rs`, `src/config.rs`, `src/app.rs`, `src/main.rs` (`KEYS`), `src/headless.rs`

**Interfaces:**
- Produces:
  ```rust
  // scene.rs
  impl Opts { pub fn set_mode(&self) -> bool; pub fn with_set_mode(self, on: bool) -> Opts; } // false from Opts::new
  // config.rs: GLOBAL, after `fiestas`
  Opt { key: "set_mode", default: Val::B(false), help: "On a film set: hollywood holds still and nothing follows the calendar or the playlist" }
  pub struct Config { /* + */ pub set_mode: bool }
  // app.rs
  /// The holiday on the set: only a forced one.
  pub fn set_day(s: Setting) -> Fiesta;          // Fixed(f) => f; Auto, Off => Fiesta::NONE
  impl Player {
      /// Flips set mode and rebuilds the scene; the HUD's note.
      pub fn toggle_set(&mut self) -> String;     // "set mode on" / "set mode off"
      /// Does the playlist move on by itself?
      pub fn cycles(&self) -> bool;               // !set && duration() > 0.0 && cfg.scenes.len() > 1
  }
  ```
  The running state is a private `set: bool` of the `Player`, `cfg.set_mode` at start. Every `Opts` the `Player` builds carries the flag (`Opts::set_mode` is scene API before hollywood reads it in Task 18: see "Code that is not wired yet"). Wherever the `Player` resolves the calendar (`new`, `watch_calendar`, `reload`, and `toggle_set` before it rebuilds, with the overlay updated) it goes through one private `day()` that is `set_day(cfg.fiestas)` in set mode and `cfg.fiestas.resolve()` otherwise. `app::run` and `headless::record` both ask `cycles()`. The `s` key calls `toggle_set` and shows its note. The help overlay gains `("s", "set mode")`, and `KEYS` in `main.rs` a fourth line: `  s    set mode`.

- [ ] **Step 1: Tests**

  - `scene.rs`, `set_mode_rides_on_the_options`: `!Opts::new(SPECS, Table::new()).set_mode()`; `with_set_mode(true).set_mode()`; a clone keeps it.
  - `config.rs`, `set_mode_is_a_setting`: default `false`; `set_mode = true`; `-o set_mode=true` through `Override::parse`; in `[saver]` with `saver = true`; `set_mode = 3` leaves it `false` with a warning; `template()` documents `set_mode`.
  - `app.rs`:
    - `only_a_forced_holiday_reaches_the_set`: `set_day(Setting::Fixed(f)) == f` for `Fiesta::height(Kind::Navidad)`; `Auto` and `Off` give `Fiesta::NONE`.
    - `the_calendar_stays_out_of_set_mode`: `set_mode = true` with `fiestas = 'navidad'`: `p.fiesta.is(Kind::Navidad)`; with `'none'` and with `'auto'`: `Fiesta::NONE`, still after `watch_calendar(61.0)`.
    - `the_playlist_stays_on_the_set`: `scene = 'neo,fire'`, `duration = 1`: `cycles()` is true, and false after `toggle_set()`; with one scene false in both.
    - `s_rebuilds_and_keeps_what_it_can`: `seed = 7`: after `toggle_set()` `p.set` and `p.opts.set_mode()` are true and `p.seed` is what it was; after `step_theme(1)` and two toggles the theme's name is the stepped one.
    - `the_last_word_wins`: the config says `false`, `s` turns it on; a reload that changes only `fps` leaves it on; a reload with `set_mode = true` leaves it on; the next reload with `set_mode = false` turns it off.
    - `every_scene_survives_set_mode`: `scenes_survive_sizes_and_time`'s loop with `set_mode = true`, classic only, one thread per scene.

- [ ] **Step 2: Implement, gates, commit.** `app: set mode: the calendar and the playlist hold still, and the s key`.

---

### Task 15: Easter eggs on the set

Spec 9.3, 11.6 ("Eggs", "Eggs on a tall screen").

**Files:**
- Modify: `src/eggs/mod.rs` (`Eggs::stop`; two tests; the module docs gain `set_eggs` and the `e` key), `src/config.rs`, `src/app.rs`, `src/main.rs` (`KEYS`)

**Interfaces:**
- Produces:
  ```rust
  // eggs/mod.rs
  impl Eggs { /// Drops the egg that is playing.
              pub fn stop(&mut self); }
  // config.rs: GLOBAL, after `set_mode`
  Opt { key: "set_eggs", default: Val::B(true), help: "In set mode, may the easter eggs play? (the e key switches them off and on while running)" }
  pub struct Config { /* + */ pub set_eggs: bool }
  // app.rs
  impl Player {
      /// Flips the eggs' switch; turning it off cuts the egg that is playing. The new state.
      pub fn toggle_eggs(&mut self) -> bool;
      /// Plays an egg now (the hidden `E` key). False: the eggs are off.
      pub fn start_egg(&mut self) -> bool;
  }
  ```
  One flag in the `Player`: on at start outside set mode; `cfg.set_eggs` whenever set mode goes on (at start-up too); on whenever it goes off; flipped by `e`; set again by a reload only when `set_eggs` changed in it and set mode is on. While it is off nothing is rolled, a forced `egg_rate` plays nothing (in `new` and in `update`), and `E` plays nothing. `e` shows `eggs off` or `eggs on`; `E` with the eggs off shows `eggs off`. The help overlay gains `("e", "eggs on/off")` and `KEYS` `e    eggs on/off`.

- [ ] **Step 1: Tests**

  - `eggs/mod.rs`:
    - `stop_drops_the_egg`: `start` with `sl`, `active()`, `stop()`, `!active()`; `stop()` again is harmless.
    - `eggs_draw_on_tall_screens`: for every egg of `ALL` at 60x55, 67x60, 120x106 and 130x110, started through `Eggs::start(&[info], its_day(info))` and run at 30 frames a second: some frame has a cell that is not blank, it is over within 31 seconds, nothing panics. (`eggs_survive_odd_sizes_and_huge_steps` is not edited.)
  - `app.rs` (`egg_rate = 1000000.0` is the forced rate):
    - `eggs_play_on_the_set`: `set_mode = true`, forced rate: `p.eggs.active()` after `new`.
    - `set_eggs_off_silences_every_route`: with `set_eggs = false` too: not active after `new`, after 100 updates, after `roll_egg(3600.0, true)`; `start_egg()` is `false`.
    - `e_cuts_the_egg`: active, `toggle_eggs()` is `false` and the egg is gone in that call; `toggle_eggs()` is `true`; the same outside set mode.
    - `the_switch_follows_set_mode`: `set_eggs = false`, started outside set mode: an egg can start; `toggle_set()`: none can; `toggle_set()`: one can.
    - `a_reload_leaves_the_keys_choice`: in set mode `e` turns them off; a reload that changes `fps` leaves them off; a reload that changes `set_eggs` sets the flag to the new value.
    - `only_everyday_eggs_on_an_unforced_set`: `set_mode = true`, `fiestas = 'auto'`: `p.fiesta == Fiesta::NONE` whatever the date (the eggs take their day from the `Player`; that a holiday's egg never starts on such a day is `eggs`' own existing test).

- [ ] **Step 2: Implement, gates, commit.** `app: easter eggs on the set, with a setting and a key to silence them`.

---

### Task 16: A seed that can be written down

Spec 9.4 (K3, C33), 11.6 ("Seeds").

**Files:**
- Modify: `src/app.rs`

**Interfaces:**
- Produces:
  ```rust
  impl Player {
      /// The number that, as `seed =`, leads to the seed in use; None when there is none.
      pub fn seed_no(&self) -> Option<u64>;
  }
  ```
  `seed_no` is `cfg.seed` after `new`. In set mode a new seed is drawn the other way round: `n` from 1 to 9999 from the player's generator, then exactly what `Player::new` does with `seed = n`: a generator started from `n`; if `cfg.shuffle`, a shuffle of a throwaway `Vec` as long as the playlist (the order in use is not touched); then the scene's seed; that generator becomes the player's. This happens on `reseed` and `step_scene` in set mode, in `new` in set mode when `cfg.seed` is `None` (`n` from `fastrand::Rng::new()`), and in `toggle_set` when set mode goes on and `seed_no` is `None`. Outside set mode `reseed` and `step_scene` do what they do today, number for number, and clear `seed_no`.

  HUD, four seconds: after `r` and after the scene arrows in set mode `<scene> · seed <n>`; `toggle_set` answers `set mode on · seed <n>`; at start-up in set mode, when `hud` is on and not in `--saver`, `set mode · seed <n>`. Outside set mode `r` still says `reseeded`.

- [ ] **Step 1: Tests** in `src/app.rs`

  - `a_written_seed_brings_the_scene_back`: for `shuffle` false and true, `scene = 'hollywood,neo,fire'`, `set_mode = true`: after `reseed()`, `n = p.seed_no().unwrap()` is in `1..=9999`, and a `Player` started with the same config plus `seed = n` has `q.seed == p.seed`. The same for `n` right after `new` without a `seed`, and for `n` after `toggle_set()` on a player that was reseeded outside set mode. (That hollywood then has the same panes is Task 18's `the_layout_is_the_plans`: the panes are a function of that seed.)
  - `s_off_and_on_keeps_the_number`: `seed_no()` and `seed` are equal before and after two toggles.
  - `a_seed_from_the_config_is_its_own_number`: `seed = 4821`, `set_mode = true`: `seed_no() == Some(4821)`; `toggle_set()` twice changes neither.
  - `the_arrows_draw_a_number_too`: in set mode `step_scene(1)` leaves `seed_no()` in `1..=9999`.
  - `outside_set_mode_there_is_no_number`: after `reseed()`, `seed_no()` is `None`; `toggle_set()` then answers `set mode on · seed <n>` with `n == seed_no().unwrap()`.
  - Task 1's `reseeding_draws_the_seeds_it_always_drew` passes unedited.

- [ ] **Step 2: Implement, gates, commit.** `app: in set mode a seed is a number you can write down`.

---

### Task 17: The set layout

Spec 10.3 (with K13), 11.5 ("The layout, as a pure function"). Not wired yet.

**Files:**
- Create: `src/scenes/hollywood/foro.rs` (starts with `#![allow(dead_code)] // wired in Task 18`)
- Modify: `src/scenes/hollywood/mod.rs` (`mod foro;`)

**Interfaces:**
- Consumes: `Rect`; `letrero::fit` (in one test).
- Produces:
  ```rust
  pub const BANNER: usize = 17;                    // the access pane
  pub const LOG: usize = 8;                        // the mission log
  pub const POOL: [usize; 5] = [13, 12, 11, 10, 14]; // radar, map, rain, bandwidth, progress
  #[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum Role { Banner, Log, Shape }
  #[derive(Clone, Copy, PartialEq, Eq, Debug)] pub struct Seat { pub rect: Rect, pub role: Role, pub kind: usize, pub seed: u64 }
  #[derive(Clone, PartialEq, Eq, Debug)]
  pub struct Plan {
      /// In reading order: from the top, then from the left.
      pub seats: Vec<Seat>,
      /// Seeds the dealing of the titles.
      pub title_seed: u64,
      /// A spare draw: which signature widget a forced holiday gets.
      pub lot: u64,
  }
  /// `c` x `r` is the area the panes tile (the terminal less the ticker's row).
  pub fn plan(seed: u64, c: i32, r: i32) -> Plan;
  pub fn suits(kind: usize, w: i32, h: i32) -> bool;
  /// Rows of the banner band a tall screen of `c` columns asks for.
  pub fn banner_band(c: i32) -> i32;
  ```

- [ ] **Step 1: Tests** in `foro.rs`

  `SIZES` (`#[cfg(test)] pub(super) const SIZES: [(u16, u16); 14]`, which Task 18's tests use too) are the terminals of spec 11.5: tall 44x31, 60x55, 67x60, 90x80, 120x106, 130x110; wide 80x24, 100x30, 120x33, 213x60, 240x35, 240x65; in between 106x60 and 100x50. The area of a terminal `w` x `h` is `(w, h - 1)`. Each test runs over `SIZES` and seeds 0 to 99 unless it says otherwise.

  - `the_panes_tile_the_area`: the rectangles are inside `c` x `r`, do not overlap, and their areas sum to `c * r`.
  - `one_banner_one_log`: exactly one `Role::Banner` (kind 17); exactly one `Role::Log` (kind 8) when `r >= 21`.
  - `no_kind_twice`: all kinds differ; every `Shape` kind is in `POOL`.
  - `a_tall_screen_is_bands` (K13): when `2 * r > c`, every seat has `x == 0` and `w == c`; the banner is the first or second seat; there are never more than 4 seats, and exactly 4 when `r >= 41`.
  - `a_wide_screen_is_two_columns`: the log is right under the banner with its `x` and `w`; 3 or 4 seats.
  - `panes_are_big`: over every area from 1x1 to 80x40 as well: no seat but the banner's is under 30 columns or 7 rows.
  - `the_banner_has_room`: when `r >= 14` the banner's `rect.h - 1 >= 13`; when also `c >= 60`, `letrero::fit("ABCDEFGH", "ABCDEFGH", rect.w - 1, rect.h - 1)` is two lines at `Narrow` or bigger.
  - `widgets_go_where_their_shape_fits`: walking the `Shape` seats in order with the kinds not yet taken: the seat's kind suits it, or none of those left did.
  - `suits_by_the_table`:

    | kind | suits | does not |
    |---|---|---|
    | radar | 67x16, 60x13, 38x15 | 60x12 (too flat), 30x30 (too tall) |
    | map | 46x16, 67x16 | 38x16 |
    | rain | 1x100, 200x9 | |
    | bandwidth | 45x34 | 30x30 |
    | progress | 60x14, 40x20 | 40x21 (too tall), 38x15 (under 40 columns) |

  - `the_banner_band_by_width`: `banner_band` is 20 at 60, 67 and 98; 29 at 99 and 130; 37 at 131 and 162; 46 at 163 and 300.
  - `the_tables_of_examples`: for each row some seed below 2,000 gives exactly these panes (as kind, columns, rows):

    | terminal | panes |
    |---|---|
    | 60x55 | banner 60x20, radar 60x13, progress 60x14, log 60x7 |
    | 67x60 | banner 67x20, radar 67x16, map 67x16, log 67x7 |
    | 120x106 | banner 120x29, radar 120x31, map 120x32, log 120x13 |
    | 100x30 | banner 62x20, log 62x9, radar 38x15, rain 38x14 |
    | 120x33 | banner 74x22, log 74x10, radar 46x16, map 46x16 |
    | 213x60 | banner 132x45, log 132x14, radar 81x30, map 81x29 |
    | 240x35 | banner 149x24, log 149x10, rain 46x34, bandwidth 45x34 |

  - `the_minimums_at_their_boundaries` (areas, any seed): tall, 36 columns: 20 rows is the banner alone, 21 adds the log, 29 has no shape pane, 30 has one; tall, 60 columns: 40 rows has one shape pane, 41 has two. Wide, 30 rows: 73 columns has no shape pane, 74 has two; wide, 120 columns: 20 rows has no log, 21 has one.
  - `the_same_seed_the_same_panes`: `plan(s, c, r) == plan(s, c, r)`; seats are sorted by `(y, x)`; the seats' seeds differ from each other.
  - `it_returns_at_every_size`: every area from 1x1 to 80x40, and 1x1, 2x3, 7x2, 13x5, 300x89: it returns, and from 3x3 there is a banner seat. `plan(1, 0, 0)` has no seats.

- [ ] **Step 2: Implement.** The spec fixes the rules; the arithmetic it leaves open is fixed here, and was checked against the tests above:

  ```text
  round(x) = floor(x + 0.5);   tall = 2r > c
  TALL   only the banner when c < 30 or r < 21. Otherwise:
         log  = clamp(round(r / 8), 7, 14)
         ban  = max(min(banner_band(c), floor(0.4 * r)), 14)
         rest = r - ban - log
         shape bands: two when rest >= 18, of ceil(rest / 2) and floor(rest / 2) rows;
                      one when rest >= 9; none below, and then ban = r - log
         from the top: the log and the shape bands in a shuffled order, the banner put first or second
  WIDE   side = max(round(0.38 * c), 30);  main = c - side;  no side column when main < 44 or r < 7
         log  = clamp(round(0.3 * r), 7, 14), none when r < 21; the banner above it has the rest
         side column: two panes stacked (ceil(r / 2) over floor(r / 2)) when 2r >= side and r >= 24;
                      two side by side (ceil(side / 2), then floor(side / 2)) when 2r < side and side >= 60; else one
         the seed picks which side the main column is on
  banner_band(c) = 46 from 163 columns, 37 from 131, 29 from 99, else 20
  suits: shape = w / (2h); radar 0.6..=2.4; map >= 1.2; rain any; bandwidth >= 0.6; progress >= 1.0 and w >= 40
  kinds: POOL is shuffled; each shape seat, in reading order, takes the first kind left that suits it, else the first left
  seeds: one u64 per seat in reading order, then title_seed, then lot
  ```

  Every draw comes from one `Rng` seeded from `seed` XOR a constant of this module, so it is not the scene's own stream. Nothing else goes in: not the preset, not the clock.

- [ ] **Step 3: Gates and commit.** `hollywood: the set layout, a function of the seed and the size`.

---

### Task 18: Set mode in hollywood

Spec 10.1, 10.3 ("The rest of the function"), 10.4, 10.5 (the border), 10.6, 10.7, 11.5 ("The scene in set mode", "Stress").

**Files:**
- Modify: `src/scenes/hollywood/mod.rs`; `foro.rs` (its `allow(dead_code)` goes)
- Create: `src/scenes/hollywood/set_tests.rs`

**Interfaces:**
- Consumes: `ctx.opts.set_mode()`, `foro::plan`, `Garb`, `Preset::titles`.
- Produces:
  ```rust
  // INFO.options, after `preset`:
  Opt { key: "set_pace", default: Val::F(0.5), help: "In set mode, the pace of the widgets and the ticker, on top of speed (0.1 to 1)" }
  Opt { key: "set_dim", default: Val::F(0.25), help: "In set mode, how far the colors fade toward the background (0 = as usual, 0.9 = barely there)" }
  enum Node { Leaf(..), Split { .. }, /// Set mode: the plan's panes, each with its rectangle.
              Seats(Vec<Box<Pane>>) }
  impl Pal { fn dressed(ctx: &Ctx, plain: bool, dim: f32) -> Pal; } // Pal::new(ctx) is dressed(ctx, false, 0.0)
  struct Hollywood { /* + */ set: bool, pace: f32, /* .. */ }
  ```
  In set mode: `resize` seats the screen from `foro::plan(self.seed, area.w, area.h)` with fresh widgets from the seats' seeds; so does a holiday that changes and a preset that is reloaded (same seed, same size: the same panes). Titles are dealt once, by position, from a generator seeded with `plan.title_seed`: the i-th pane takes the i-th of the shuffled `titulos`, the panes beyond the list a generated title. `tick` runs neither the layout timer nor the title timers, and `ensure_signature` is not called. With a forced holiday and no preset the first `Shape` seat holds a signature widget: going round `signature(fiesta)` from index `lot % len`, the first kind that no other seat holds (Día de Muertos counts the radar among its own). `update` multiplies `dt` by `speed` and by `pace` (`set_pace`, clamped to 0.1 to 1; not finite: 0.5). `dim` (`set_dim`, clamped to 0 to 0.9; not finite: 0.25) fades every tone, `ink`, the ramp (sample it, fade, `Gradient::new`; `color.rs` is not touched), `picado`, `papel`, `flor`, `vela` and `oro` with the fade the palette has. A pane's border is the settled color from its first frame. `panes` and `reshuffle` are not read.

- [ ] **Step 1: Tests** in `set_tests.rs` (helper: `set_scene(opts: &str, fiesta: Fiesta, w: u16, h: u16)` builds with `Opts::new(..).with_set_mode(true)`)

  - `the_layout_is_the_plans`: at every size of `foro`'s `SIZES`, rectangles and kinds equal the seats of `foro::plan(seed, w, h - 1)`.
  - `nothing_moves_in_ten_minutes`: at 67x60 and 120x33, 600 times `tick(ctx, 0.5, 1.0)`: rectangles, kinds and titles are what they were; with `panes = 12` and `reshuffle = 1` too.
  - `a_preset_cannot_move_a_pane`: the same seed and size with no preset, with the example and with another preset: equal rectangles and kinds.
  - `a_resize_and_back`: 67x60, then 120x33 (the plan for that size), then 67x60 again: the first layout.
  - `titles_are_dealt_by_position`: with the example, the four panes at 67x60 start with four different `titulos`, the same four in a second scene from the same seed; with a preset of two `titulos` the first two panes have them and the others read `<name> · <words>`.
  - `a_forced_holiday_gets_its_signature`: for Muertos, Navidad, Patrias and Revolución at their height: kinds 17 and 8 are there, the first shape pane's kind is in `signature(fiesta)`, and no kind is on screen twice (seeds 0 to 49). With a preset as well: every shape kind is in `foro::POOL`.
  - `no_border_is_bright_at_birth`: on the first frame every `┌` has `fg == pal.fade(pal.main, 0.45)`.
  - `set_dim_fades_every_tone`: with `ctx.bg = Color::Rgb(16, 16, 24)`: `set_dim = 0` gives the seven tones of `Pal::new(ctx)`; `0.25` gives a lower `luma()` for each.
  - `set_pace_halves_the_ticker`: in ten seconds of `update(ctx, 1.0 / 30.0)` the tape loses 70 characters, give or take one, at the default and 140 with `set_pace = 1`.
  - `pace_and_dim_out_of_range_are_clamped` (Review Focus 4): `set_pace` of `0`, `-1`, `99`, `nan` and `set_dim` of `-1`, `5`, `nan`: `0.1 <= s.pace && s.pace <= 1.0`, the tape still moves in ten seconds, a frame draws, nothing panics.
  - `set_mode_survives_stress`: 1x1, 2x3, 7x2, 13x5, 300x90 and `SIZES`, 240 frames and three steps of 5 seconds each, with and without the example, with and without each forced holiday.

- [ ] **Step 2: Implement.** `Node::Seats` gets an arm in `layout` (rectangles are the plan's; only tell a widget whose content size changed), `count`, `each`, `each_mut` and `into_panes`; `splittable`, `split_at`, `mergeable` and `merge_nth` treat it as nothing to do. The arms for ordinary days are not rearranged.

- [ ] **Step 3: Look at it**

  `cargo run -q -- --config /dev/null snap -s hollywood -o set_mode=true --seed 7 --size 67x60 --at 20`, then `--size 120x33`, then both with `-o hollywood.preset=docs/presets/hollywood/software-libre.toml`: four panes as wide as the screen on the tall one; the banner over the log beside two panes on the wide one; the same panes with and without the preset.

- [ ] **Step 4: Gates and commit.** `hollywood: set mode: a layout that holds still, slower and dimmer`.

---

### Task 19: The calm log

Spec 10.2 (C27), 10.5 (the `Feed` row), 11.5 ("The log and the logins", "With a preset of four `frases`").

**Files:**
- Modify: `src/scenes/hollywood/garb.rs`, `mod.rs` (`garb()`), `feed.rs`, `set_tests.rs`

**Interfaces:**
- Produces: `Garb { /* + */ pub calm: bool }` and `Garb::with_calm(self, on: bool) -> Garb`; `Hollywood::garb()` sets it in set mode. `#[cfg(test)] Feed::born(&self) -> u32` (lines brought in by `update` so far).
- A calm `Feed` (any kind; the set only seats the mission log and the logins under the banner) brings one line each scene second (`acc += dt`, no jitter) and never more than one in an `update`, whatever `dt`. A calm mission log under a preset with `phrases`: every third sentence (a line that is not a phase line) is `expand` of the next phrase of a `Round`; the other two are the engine's as in any log.

- [ ] **Step 1: Tests** in `set_tests.rs`

  - `the_calm_log_brings_one_line_at_a_time`: the mission log and the logins (`FeedKind::Auth`) with `Garb::plain(Fiesta::NONE).with_calm(true)`: 1,800 updates of `1.0 / 60.0` (a minute on the wall at 30 frames a second and the default pace): `born()` never grows by more than 1 in an update and ends between 20 and 40; one update of `5.0` adds exactly 1.
  - `one_sentence_in_three_is_a_frase`: a preset with four `frases` without blanks, a calm mission log 200 columns wide, until 300 sentences have come by: every third is a phrase; the phrases, taken in fours, are permutations of the four; between a quarter and four tenths of the sentences are phrases.
  - `a_feed_that_is_not_calm_is_as_it_was`: with `with_calm(false)` the lines of ten simulated seconds equal those of a `Feed` built from the bare `Fiesta` (and the fingerprint passes).

- [ ] **Step 2: Implement, look at a tall snap, gates, commit.** `hollywood: on the set the log comes a line at a time`.

---

### Task 20: The calm banner

Spec 10.5 (the two `Access` rows, "The banner's cross-fade"), 7.5 ("Where this applies": in set mode with or without a preset), 11.5 ("The banner").

**Files:**
- Modify: `src/scenes/hollywood/widgets.rs` (`Access`), `set_tests.rs`

**Interfaces:**
- Consumes: `Garb::calm`, `letrero::{fit, draw, Look}`, `Pal::fade`.
- Produces: `#[cfg(test)] Access::strength(&self) -> Option<f32>`: `None` with no banner; 0 to 1 while one comes in or leaves; 1 while it stays.
- A calm `Access`, on the scene's clock: the first banner starts within 2 s; a banner comes in over 0.5 s, stays 6 to 10 s, leaves over 0.5 s; 2 to 4 s pass between banners; it never blinks. It is fitted (`letrero`) with or without a preset; without one the words are the ones `Access` uses today (the holiday's `dress`, or `ACCESS` / `GRANTED` and `DENIED`). Coming in, with `k` from 0 to 1 (leaving is the same backwards): while `k < 0.5` only the cells already drawn inside the box's rectangle change, their `fg` faded by `pal.fade(fg, 2k)`; from 0.5, with `s = 2k - 1`, the box is drawn with `Look { ink: Some(pal.fade(pal.ink, 1 - s)), letters: pal.fade(col, 1 - s), frame: pal.fade(col, 1 - s), caption: pal.fade(pal.bright, 1 - s) }`.

- [ ] **Step 1: Tests** in `set_tests.rs` (the palette from a `Ctx` with `bg: Color::Rgb(16, 16, 24)`; a pane of 66x19; frames of `1.0 / 60.0`)

  - `the_calm_banner_keeps_its_times`: over 120 scene seconds: the first banner starts before 2 s; from a banner's first frame to its last `strength()` is `Some` on every frame; it is 1 for 6 to 10 s; each ramp lasts 0.5 s, give or take a frame; the gaps are 2 to 4 s.
  - `the_calm_banner_eases`: from one frame to the next, for every cell of the box's rectangle, each channel of `bg` moves by no more than a fifth of its distance between the pane's background and `pal.ink`, plus 1 for rounding, and each channel of the `fg` of the letters and of the frame by no more than a fifth of its distance between the background and its final color, plus 1. In particular the first frame with a box has it at less than a fifth.
  - `the_log_goes_first`: while `strength() < Some(0.5)` no cell's `bg` differs from the frame before the banner, and the text inside the rectangle is fainter each frame.
  - `the_set_banner_is_fitted_without_a_preset`: at full strength in 66x19 the box is 13 rows high, and 58 columns wide for `GRANTED` or 50 for `DENIED` (x2 letters); with `Fiesta::height(Kind::Navidad)`: the words of its `dress`.
  - `a_palette_without_rgb_shows_whole_banners`: `Theme::terminal(&TermColors::default())` and `bg: Color::Reset`, 60 scene seconds at 66x19 and at 20x5: no panic, nothing outside the pane, and a banner reaches strength 1.
  - The existing banner tests and the fingerprint pass: outside set mode `Access` blinks as it did.

- [ ] **Step 2: Implement, look at it (`snap --at` 3, 4, 8 with `-o set_mode=true`), gates, commit.** `hollywood: on the set the banner fades in and out`.

---

### Task 21: The calm map, radar, bars and graph

Spec 10.5 (the rows of `WorldMap`, `Radar`, `Progress`, `Bandwidth`; "What is left alone"), 11.5 ("Calm").

**Files:**
- Modify: `src/scenes/hollywood/widgets.rs`, `set_tests.rs`

**Interfaces:**
- Consumes: `Garb::calm`. Every branch below is taken only when `mx.calm`; the code beside it is not changed.
- Produces: `#[cfg(test)] WorldMap::arcs_len(&self) -> usize`.

  | widget | calm (scene seconds) |
  |---|---|
  | `WorldMap` | an arc that arrives keeps its color and its marker is one glyph; both fade to the background over 1.5 s; the arc is dropped when the fade is over |
  | `Radar` | the center is one glyph; a contact's list line is rewritten when the sweep passes it (the widget remembers the text) |
  | `Progress` | a bar that finishes keeps its colors and gets its tag; after 3 s its two rows fade out over 0.5 s; the bar that follows starts at 0; with 27 rows or more a bar every third row, up to twelve |
  | `Bandwidth` | the figures are rewritten once a second; the graph moves as before |

- [ ] **Step 1: Tests** in `set_tests.rs` (widgets from `Garb::plain(..).with_calm(true)` and from a calm preset garb; frames of `1.0 / 60.0`)

  - `an_arrival_does_not_flash`: the cells of an arc have the same `fg` on the frame before and the frame after it arrives, and none is `pal.red`; its marker's character is the same on every frame; `arcs_len()` drops only 1.5 s after the arrival, and by then the arc's cells have reached the background.
  - `the_radars_center_is_one_glyph`: under a calm preset the center cell holds one character through two full sweeps.
  - `a_contacts_line_waits_for_the_sweep`: an undressed calm radar in an 80x24 pane over 30 s: a list row changes its text only on a frame in which that row is drawn in `pal.red` (it was just swept), and each row changes at most 9 times.
  - `a_finished_bar_keeps_its_colors`: on the frame a bar reaches 100% the cells of its bar row have the `fg` they had the frame before, and `COMPLETE` (`COMPLETO` when dressed) appears; 3 s later its rows fade for 0.5 s; the next bar's row reads `  0%`.
  - `tall_panes_spread_their_bars`: 30 rows: bars on rows 0, 3, 6, ... (ten of them); 40 rows: twelve; 26 rows: every second row, nine at most, as outside set mode.
  - `the_figures_tick_once_a_second`: over 20 s the text of the figures changes on at most 21 frames, while the graph's cells change on more than 100.
  - The fingerprint passes.

- [ ] **Step 2: Implement, gates, commit.** `hollywood: on the set nothing flashes: the map, the radar, the bars, the graph`.

---

### Task 22: Maps that keep their shape

Spec 10.8 (C32), 13 step 12. The one deliberate change to an ordinary day, on a tall terminal only.

**Files:**
- Modify: `src/scenes/hollywood/map.rs`, `garb.rs`, `mod.rs` (`garb()`), `widgets.rs` (`WorldMap`, `Satellites`), `set_tests.rs`

**Interfaces:**
- Produces:
  ```rust
  // map.rs
  /// The largest rectangle inside `w` x `h` that is between three and six times as many
  /// columns as rows, centered: (x, y, w, h). Never bigger than the area; at least 1x1 when the area is.
  pub fn keep(w: i32, h: i32) -> (i32, i32, i32, i32);
  // garb.rs
  pub struct Garb { /* + */ pub keep: bool }
  impl Garb { pub fn with_keep(self, on: bool) -> Garb; }
  ```
  `Hollywood::garb()` sets `keep` when a preset is on, or set mode is, or the terminal is a screen on its side (`3 * rows >= 2 * columns`, the terminal's own size). A widget keeps what it was told when it was made. `WorldMap` and `Satellites` draw their map inside `map::keep` of the area the map has today when `mx.keep`, and exactly as today when not.

- [ ] **Step 1: Tests**

  - `map.rs`, `keep_is_the_shape_of_the_world`: `keep(34, 28) == (0, 8, 34, 11)`; `keep(140, 10) == (40, 0, 60, 10)`; `keep(120, 30) == (0, 0, 120, 30)`; for every `w` and `h` from 0 to 60 the result is inside the area, and `3 * rh <= rw && rw <= 6 * rh` whenever `w >= 3` and `h >= 1`.
  - `set_tests.rs`:
    - `a_screen_on_its_side_keeps_shapes`: after `resize`, `s.garb().keep` is true at 67x60 and 120x106 and false at 80x24, 213x60 and 106x60 on an ordinary day; true at 80x24 with a preset, and in set mode.
    - `a_kept_map_is_not_stretched`: `WorldMap` with `with_keep(true)` in an area of 34x29: land is drawn on no more than 12 rows, and those rows are consecutive; without `keep`, on more than 20.
    - `kept_maps_stay_in_their_panes`: `WorldMap` and `Satellites` with `keep`, at 1x1, 2x3, 7x2, 34x29, 66x8, 140x11 and 120x40, forty steps: nothing outside the pane, no panic.
  - The fingerprint passes at 67x60: the maps are not in it, and nothing else moved.

- [ ] **Step 2: Implement, look at it** (`snap -s hollywood --size 67x60` on an ordinary day until a map is on screen: the world is the shape of the world), **gates, commit.** `hollywood: on a tall screen the maps keep the shape of the world`.

---

### Task 23: Docs

Spec 14.

**Files:**
- Create: `docs/PRESETS.md`
- Modify: `README.md`, `docs/SCENES.md`, `src/scenes/hollywood/mod.rs` (module docs), `src/scenes/hollywood/preset.rs` (one test)

**Interfaces:** none.

- [ ] **Step 1: A test that keeps the guides honest**, in `preset.rs`:

  `the_guides_examples_load_clean`: every fenced `toml` block of `docs/PRESETS.md` (through `include_str!`) that has no `[` table header parses with no warning, and so does the preset example of the README's "On camera" section. Run it: it fails, the file is not there.

- [ ] **Step 2: Write**

  - `README.md`: a section "On camera" after "Fiestas", with everything spec 14 lists for it (set mode and how to switch it; a preset in ten lines and a pointer to the guide; the eggs on the set and the two ways to silence them; the six practical notes for the set machine). `s` and `e` in the key table. `set_mode` and `set_eggs` in "Make it yours". hollywood's option table replaced by what `cargo run -q -- --config /dev/null docs` prints for hollywood.
  - `docs/PRESETS.md`: the reference and the authoring guide of spec 14, in English with Spanish examples, written so that an assistant given a script produces a correct preset in one pass. The limits and rules are the ones the loader enforces (Tasks 7 and 8 and "Calls made in this plan", 6 and 7); the two checking commands of spec 8.5 with a tall and a wide `--size`.
  - `docs/SCENES.md`: `ctx.opts.set_mode()` and what a scene owes the set; `refresh` in the lifecycle; under "Holidays", that hollywood's maps keep their shape on a tall terminal.
  - `src/scenes/hollywood/mod.rs`: a paragraph each for presets and set mode in the module docs, in the voice of the holiday paragraphs; the "Structure" paragraph names the new files.

- [ ] **Step 3: Check**

  `cargo run -q -- --help` lists `s` and `e`; `cargo run -q -- config template | grep -n "set_"` shows `set_mode` and `set_eggs` with their help; the three `snap` commands the README gives run as written.

- [ ] **Step 4: Gates and commit.** `docs: on camera: presets and set mode`.

---

## After the last task: by hand

Spec 11.7 lists what no test can say (through the camera: how slow, how dim, where the banner band sits; ten minutes of footage with the eggs off; a layout kept by its number; a preset saved and broken mid-run; `oxiper bench -s hollywood` with a preset and with `set_mode=true` at 213x60 and 120x106). It belongs to whoever is on the set, with the three constants the spec names for it: `set_pace`, `set_dim`, and the banner band's place.
