# hollywood: presets and set mode

Design, 2026-11-18; revised 2026-11-19 with the user's answers and with what a second read
against the code turned up, and amended the same day with K13 (no divided band on a tall
screen). Written against `main` at `f01e564`. Nothing here is built yet.

Three kinds of decision appear below, and they are told apart on purpose:

- **U** marks what the user decided. These are requirements.
- **C** marks a call made while writing this design. Each is numbered (C1, C2, ...) where it is
  made.
- **K** marks a call made by the coordinator of the work, on the questions the user left open
  and on what the builder's read of the code found.

All of them are listed in [the decision log](#15-decision-log), which also says what the
alternative would be.

The user has read a summary of this design, answered its questions
([16](#16-questions-answered)) and asked for everything to be built without another review. So
nothing below waits for an answer: where a question was still open the call is made and
logged.

Names of types, modules and functions that do not exist yet (`Garb`, `Preset`, `neutro.rs`,
`refresh`) are working names. The builder may rename them.

Sizes are written columns x rows, in terminal cells. A cell is about twice as tall as it is
wide, so a terminal of 67x60 cells is a tall screen (physically 67 wide by 120 high), and
"physically square" means twice as many columns as rows.

## 1. Intent

The `hollywood` scene is going to be a backdrop on a physical screen in a video set. A computer
next to the screen runs oxiper live in a fullscreen terminal, and a camera films that screen,
small and probably out of focus, behind or beside the presenter. The screen is most often a
monitor turned on its side, sometimes an ordinary landscape one. The videos are about
technology and are in Spanish.

Two things are missing for that job.

**The screen talks about nothing in particular.** Its technobabble is English nonsense about
mainframes. For a video about free software it should say GNU, Linux and "las cuatro
libertades", and the few places a camera can read (the ticker along the bottom, the pane
titles, the big access banner) should say what the author chose. A **preset** is a file with
the video's topic that the technobabble weaves in.

**The screen is built to be looked at.** Every 10 to 25 seconds a pane splits, merges or the
whole layout reshuffles; titles change; banners pop up and blink; on a holiday, papel picado
hangs over everything. Behind a presenter all of that pulls the eye, or lands in the middle of
a take. **Set mode** keeps the screen alive and stops it from drawing attention. (The train or
the duck that crosses the screen about once an hour stays: the user wants the easter eggs on
the set, with a switch to silence them.)

The two are independent: either works without the other.

**The screen is built for a wide terminal.** Nothing in hollywood knows that a screen can be
taller than it is wide. On a monitor turned on its side it still runs, but the world map is
stretched to three times its height and the big letters of the banner have no way to use the
room above and below them. Set mode is laid out for the tall screen first, and works on both.

## 2. Goals

1. One preset per video: a small file, quick to write, that makes hollywood talk about the
   video's topic in Spanish. (U)
2. Writing a preset needs no grammar knowledge beyond writing Spanish: plain lists, no gender
   tables, no verb conjugations by hand. (U)
3. The preset drives the readable surfaces: the ticker, the pane titles and the access banner,
   not only the scrolling logs. (U)
4. A preset goes through the same engine as the holidays Navidad, Fiestas Patrias and
   Revolución (`Vocab` + `Dress` in `src/scenes/hollywood/lexico.rs`): its names mix into the
   sentences and its words re-dress the widgets. Whatever the file leaves out comes from a
   neutral Spanish base. (U)
5. While a preset is on, hollywood ignores the calendar's holiday. (U)
6. Presets can be edited while oxiper runs, and a mistake in one never takes the screen down.
7. Set mode: frozen layout, fixed pane titles, few and big panes (3 or 4, on a tall screen as
   on a wide one), only the big visual widgets and one slow log, slower, no blinking or
   flashes, a bit dimmer,
   no calendar holiday unless one is forced with `fiestas=`. Switched by a setting (config,
   `-o`, live reload) and by the `s` key. Works with or without a preset. (U)
8. A preset will usually be drafted by an assistant from the video's script and tweaked by
   hand, so the format has to be easy to generate correctly and easy to check mechanically.
9. The set layout is designed for a portrait screen first and works on a landscape one: about
   60 to 130 columns by 55 to 110 rows on its side, about 100 to 240 by 30 to 65 lying down,
   and something sensible outside those. (U)
10. In set mode the banner pane is always on screen, and so is one log pane, slowed down, where
    the preset's `frases` can be read. (U)
11. Easter eggs still play in set mode, and they are easy to switch off. (U)

## 3. Non-goals

Decided by the user as out of scope for now:

- Video export. The screen is filmed live.
- A darker zone around the presenter's head.
- Presets for other scenes.
- Colors in presets. Custom themes already do colors.
- Font size. That is the terminal's; the docs will say to use a big font on the set machine.
- Sections within a video. The format must not rule them out (see
  [7.6](#76-room-for-sections)), but nothing is built for them.

Left out by this design (C):

- A library of built-in presets, a preset editor, or a `preset` subcommand. `config check` and
  `snap` already cover checking and previewing ([8.5](#85-checking-a-preset-without-running-it)).
- Presets in other languages. The base and the grammar are Spanish.
- A set layout for scenes other than hollywood. Set mode's app-level half applies to every
  scene; only hollywood changes how it draws.
- Calm versions of the holidays' own widgets and garlands
  ([10.6](#106-set-mode-with-a-forced-holiday)).
- A new layout for ordinary hollywood on a tall terminal. Outside set mode the splits, the
  titles and the ticker stay as they are; one thing is fixed there, the stretched maps
  ([10.8](#108-a-tall-terminal-outside-set-mode)).
- Calm or dimmed easter eggs. An egg in set mode is the egg as it is.

## 4. Anti-goals

Each of these makes the work a failure even if every feature technically works.

1. **Set mode that still surprises mid-take.** Anything that appears, vanishes, moves or
   changes on its own initiative in a way a viewer would notice: a layout change, a new title,
   a holiday starting at midnight, the playlist moving to another scene, a banner blinking.
   The easter eggs are the one exception, because the user asked for them (U): an egg in set
   mode is a surprise on purpose, and an egg that cannot be switched off for a take is this
   same failure.
2. **A format that needs grammar tables.** If writing `("licencia", true)` or
   `("compilar", "compilando", "compilad")` is ever required, the format has failed.
3. **A broken preset that stops the show.** No preset file, however malformed, may panic
   oxiper. Release builds are `panic = "abort"`: a panic restores the terminal and the program
   is gone, leaving a shell prompt on the backdrop. A preset that breaks while oxiper runs must
   leave the screen exactly as it was.
4. **A preset that is silently not there.** Starting a shoot with `preset = "x"` and getting the
   generic screen because the file had a typo, noticed only while editing the video.
5. **Any change to ordinary days or holidays when neither feature is on.** Same seed, same
   output, down to the same random numbers. This is already the repo's rule for holidays
   (`docs/SCENES.md`, "Holidays") and it extends to these two features.
6. **A second technobabble engine.** The user rejected a layer that injects lines only into the
   ticker, banner and titles, and rejected rewriting the holidays as data files
   ([6](#6-approaches)).
7. **The camera reads something the author did not write.** (C) Generated sentences are
   nonsense by design, and they attach random adjectives to whatever nouns they are given. On
   the surfaces a viewer can read, a preset that provides text gets exactly that text
   ([7.3](#73-what-the-camera-reads-is-what-the-file-says)).
8. **Editing a preset moves the panes.** In set mode the author picks a layout and then tunes
   the words; the two must not interact.
9. **A set layout that only works lying down.** (U) On a 67x60 terminal the banner must still
   be in big letters, the radar still round and the map still the shape of the world. Small
   letters in a tall, mostly empty pane are a failure even though nothing is cut off.
10. **The generic screen on camera after a bad save.** A preset that breaks while oxiper runs
    must not be dropped by the next `r`, `s`, arrow key or option change either
    ([8.4](#84-loading-and-validation)).

## 5. Constraints

From the code as it is:

- **`&'static str` everywhere.** `Vocab` and `Dress` hold `&'static str` and fixed-size arrays;
  `Widget::name()` returns `&'static str`; widgets keep `&'static str` in their own state (a
  radar blip's name, a rain drop's word, an `htop` user). Text loaded at run time has to become
  `'static`.
- **Empty lists panic.** `lexico::pick` and `babble::pick` index with `rng.usize(..len)`, which
  panics on an empty slice. The engine picks from most `Vocab` lists without looking.
- **`Dress` has hidden shape rules.** `Keygen` computes `17 - 3 - key_title.len()` in `usize`
  (a title over 14 bytes panics). `Rain` reads a word byte by byte (`as_bytes()[i] as char`),
  so rain words must be ASCII. `Access::word` centers with `word.len()` in bytes. The banner's
  3x5 font (`widgets::glyph`) has A to Z, 0 to 9, `!`, `:` and `¡`; no `Ñ`, no accents. The
  packet lengths feed `rng.u16(a..=b)`, which panics if `a > b`.
- **Widgets find words through a `Fiesta`.** They carry `mx: Fiesta` and call `lexico::of(mx)`
  and `widgets::dress(mx)`, in constructors and in `name()`, where no `Env` is at hand. Día de
  Muertos does not use a `Vocab`; it has its own module.
- **`Ctx` is built in 167 places** across 41 files, almost all tests. A new field there is a
  very large diff for no gain. `Env`, what the widgets are handed in `update` and `draw`, is
  written out as a literal in twelve of hollywood's tests, so it cannot gain a field either
  without editing them.
- **The holiday's share never reaches 1.** `lexico::share` is `0.5 + 0.45 x level`, 0.95 at the
  height, and it is called with a level, not with the holiday. `feed.rs` scales it again for
  daemon names (x 0.5) and errno lines (x 0.6). `Vocab::progress_label` has "uploading",
  "downloading" and "syncing" written into it, and Navidad, Patrias and Revolución share it.
- **Every rebuild makes a new scene.** `Player::rebuild` runs on `r`, on the arrow keys, when
  the playlist moves and when a reload changed the scene's options, and it calls
  `Hollywood::new` each time. Whatever a scene reads from disk it reads again there.
- **A reload that fails is tried again every second.** `app::run` records the config's mtime
  when `Config::load` fails, but not when `Player::reload` refuses the config (a theme that
  does not exist), so that error is rebuilt and shown once a second until the file is saved
  again.
- **The banner has one shape.** `Access` draws two lines of 3x5 letters, each pixel one row
  high and one or two columns wide, in a box of 13 rows; it measures the second word only and
  centers by bytes. It fills its box with `pal.ink` through `set_bg`, in one frame.
- **Widgets stretch to their pane.** `WorldMap` and `Satellites` project the world onto
  whatever rectangle they get; `Radar` fits a circle into the smaller side; `Progress` draws at
  most nine bars, from the top.
- **`SceneInfo.make` cannot fail**, and there is no per-scene validation hook for
  `oxiper config check`.
- **Live reload watches one file**, `config.toml`, by polling its mtime once a second
  (`app::run`). `Player::reload` also clears the theme picked with the arrow keys.
- **The app draws over every scene**: the easter eggs (`Eggs`) and the holiday overlay
  (`fiesta::Overlay`) in `Player::draw`. The app also follows the calendar
  (`Player::watch_calendar`, once a minute) and moves through the playlist (`app::run`,
  `headless::record`). A scene alone cannot keep any of that off the set.
- **Layout and titles run on real time** (`Hollywood::tick`): a layout change every
  `reshuffle x (0.66 to 1.66)` seconds, a new title for each pane every 8 to 22 seconds.
- **The scene rules** (`docs/SCENES.md`): never panic at any size from 1x1 or any `dt`; no
  per-frame heap allocation in hot paths; update + draw well under 1 ms at 200x60.
- **Half a cell can be painted.** `render::half_blocks` and `render::Pixels` give two square
  pixels to a cell, and under `strict` transparency a cell can only show one color.

What hollywood looks like today on a tall terminal (snaps at 60x55, 67x60 and 120x106, several
seeds): it runs and nothing is cut off. Panes mostly stack, because a pane is split side by
side only when it is physically wider than tall, or one time in five. At 60 to 75 columns a
side-by-side split gives panes of 30 to 37 columns, the same panes an 80x24 terminal has
today. The banner is the one-line form in those. The maps are the part that looks wrong: in a
pane of 34x29 the world is drawn 34 columns wide and 28 rows high, more than three times too
tall. All 34 easter eggs draw on a 67x60 screen; the three everyday ones (`sl`, `duckpond`,
`nyancat`) look right there, the train being longer than the screen is wide and so seen in
passing.

From the repo:

- CI runs `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, the tests and a
  bench on Linux and macOS, and `cargo check` on Rust 1.88.
- Five dependencies (`clap`, `clap_complete`, `fastrand`, `libc`, `toml`). This design adds
  none (C21).
- The repo is public. Docs and examples carry no personal details.
- Changes are committed and pushed to `main` once they build and pass. Every step of the work
  therefore has to leave `main` releasable ([13](#13-order-of-work)).

## 6. Approaches

### 6.1 How a preset reaches the screen

**A. One more holiday, loaded from a file. Chosen by the user.** Navidad, Fiestas Patrias and
Revolución each bring a `Vocab` and a `Dress`. The engine mixes the `Vocab` into the grammar
(`Vocab::sentence_es`: 55% of nouns, 45% of adjectives and verbs come from the holiday), fills
the blanks of canned lines with its people and things, and the widgets take their titles,
labels, rain words, market symbols and banner words from the `Dress`. A preset builds the same
two structures from a file, with a neutral Spanish base filling every field the file does not
give. While a preset is on, the calendar's holiday is ignored inside hollywood.

Why: names end up inside sentences ("compilando el kernel asíncrono de Richard Stallman"),
every widget is re-dressed, the text is Spanish, and the engine that does all this is already
written and tested.

**B. A separate layer that injects lines into the ticker, the banner and the titles.
Rejected by the user.** Names would not mix into sentences, the widgets would not change, and
the technobabble would stay English.

**C. Rewrite the holidays themselves as data files, then add presets as more data. Rejected by
the user.** About 5,000 lines of tested code refactored, with no gain for the next video.

### 6.2 How the widgets learn about the preset

(C9) Approach A leaves one mechanical question: widgets look their words up by `Fiesta`, and a
preset is not a `Fiesta`.

- **A scene-local value that stands where the `Fiesta` stands. Chosen.** hollywood gets a small
  `Copy` type, `Garb`: the day's holiday, an optional `&'static Preset`, whether the screen is
  calm (set mode), and whether shapes are kept (the maps keep the world's proportions,
  [10.8](#108-a-tall-terminal-outside-set-mode)). It derefs to `Fiesta`, so
  `mx.is(Kind::Muertos)`, `mx.level` and `mx.kind` read as they do now. `lexico::of` and
  `widgets::dress` take a `Garb` and answer with the preset's `Vocab` when there is one. The
  diff in `feed.rs` and `widgets.rs` is the type of `mx`, plus the branches that only a preset
  or the set reach. The holiday modules are not touched.
- **A new `fiesta::Kind::Preset`. Dropped.** `Kind` belongs to the whole program: every scene,
  the overlay and the eggs match on it. A variant that only hollywood understands does not
  belong there.
- **A global slot that `lexico::of` consults. Dropped.** No type changes at all, but the preset
  becomes hidden state shared by every `Hollywood` in the process, tests included.

### 6.3 How the scene learns about set mode

(C14)

- **Through `Opts`. Chosen.** `Opts` gains a typed flag (`opts.set_mode()`), false in
  `Opts::new`, set by the `Player` when it builds a scene. `Ctx` already carries `&Opts`, so
  nothing else changes and no test literal is touched.
- **A new `Ctx` field. Dropped**: 167 literals.
- **A `Scene` method called after `make`.** Workable, and the builder may prefer it. It needs an
  ordering rule (before the first `resize`) that the `Opts` route does not.

## 7. The preset file

### 7.1 Shape

(C1) A preset is a TOML file. TOML is already the config's format and already a dependency, it
has comments, and its errors come with a line number.

Three rules keep it quick to write and hard to break:

- **Flat.** Every key is at the top level. There are no tables in the file. (A key written after
  a `[table]` header silently belongs to that table; that trap is not worth having in a file
  edited between takes.)
- **Lists are blocks of lines.** A list is one multi-line literal string, one item per line,
  between `'''` and `'''`. No quotes or commas per item, and no escaping: the text can contain
  `"`, `«»` and `\` as they are. Blank lines and lines starting with `#` are skipped. A TOML
  array of strings is accepted too.
- **Everything is optional.** An empty file is a valid preset: the neutral Spanish screen.

Keys and blank names are Spanish and unaccented (`titulos`, not `títulos`), because the file is
Spanish prose with Spanish placeholders in it, and `{persona}` reads better inside a Spanish
sentence than `{person}`. (C2, kept by K1; the program's own config keeps its English keys.)

### 7.2 Example

`docs/presets/hollywood/software-libre.toml`, shipped as the documented example and compiled
into a test so that it never rots (C22):

```toml
# hollywood preset for a video about free software.
# Every key is optional. A list is one item per line between ''' and '''.

tema = "Software libre"

# --- What the camera reads: shown exactly as written -------------------------

# The ticker along the bottom (shown in capitals).
titulares = '''
1983: Stallman anuncia el proyecto GNU
1985: nace la Free Software Foundation
1989: primera versión de la GPL
1991: Torvalds publica el kernel Linux
Libertad 0: usar el programa con cualquier propósito
Libertad 1: estudiar cómo funciona y cambiarlo
Libertad 2: redistribuir copias
Libertad 3: distribuir tus versiones modificadas
GNU no es Unix
Libre como en libertad de expresión, no como en barra libre
Copyleft: lo que es libre se queda libre
{n} parches recibidos de la comunidad
'''

# The pane titles.
titulos = '''
Las cuatro libertades
Proyecto GNU, 1983
Kernel Linux, 1991
Licencia GPL v3
Copyleft
Free Software Foundation
Código fuente
'''

# The big banner of the access pane: two short lines split by " / ".
# On a narrow screen a line of several words goes onto several lines of big letters.
letrero_verde = '''
SOFTWARE / LIBRE
GNU / NO ES UNIX
CUATRO / LIBERTADES
'''
letrero_rojo = '''
CODIGO / CERRADO
'''

# --- What the small text is made of ------------------------------------------

personas = '''
Richard Stallman
Linus Torvalds
'''

# Bare names: projects, products, acronyms, places.
nombres = '''
GNU
Linux
GPL
Emacs
GCC
Debian
FSF
'''

# Common nouns, each with its article.
cosas = '''
el kernel
la licencia
el código fuente
el compilador
la comunidad
el parche
el manifiesto
el repositorio
la libertad
las cuatro libertades
'''

adjetivos = '''
libre
privativo
comunitario
auditable
con copyleft
sin puertas traseras
'''

verbos = '''
liberar
compartir
compilar
estudiar
modificar
distribuir
'''

# Whole lines for the logs. Blanks are filled at random.
frases = '''
Compilando {cosa} con GCC: {n} advertencias, 0 errores
{persona} revisa el parche {n} de {nombre}
Clonando el repositorio de {nombre}: {pct}% completado
Licencia verificada: GPL versión {d}
'''
```

A preset that only has `titulares` and `nombres` is already useful. The shortest sensible one
is about twenty lines.

### 7.3 What the camera reads is what the file says

(C5) The ticker, the pane titles and the banner are the surfaces a viewer can read. For each of
them, **if the preset gives text, the screen shows that text and nothing else there**:

| surface | key | with the key | without the key |
|---|---|---|---|
| ticker | `titulares` | only these lines, in capitals, in a shuffled round (every line once before any repeats) | the engine's mix: base headlines and generated "MISIÓN ◆ ..." lines |
| pane titles | `titulos` | each pane takes one, drawn first and bright, followed by the process name: `Las cuatro libertades · radar --sweep` | process name, then a generated noun and adjective, as on a holiday |
| banner | `letrero_verde`, `letrero_rojo` | only the pairs given, in a shuffled round; if only one color is given, only that color ever shows | the base's ACCESO CONCEDIDO / ACCESO DENEGADO |

For the ticker this was a question to the user, and it was not answered, so the default stays
(K2): with `titulares`, no generated line shares the row with them.

The reason is anti-goal 7. The grammar happily produces "advertencia: la licencia está
alucinando" or "el servidor hackeado de Richard Stallman". That is the genre, and in the small
text it is texture. In the three places a viewer can read, next to real names, in a video that
may be serious, it should be the author's choice. A preset that wants generated lines on the
ticker simply leaves `titulares` out.

The `babble` option does not touch these surfaces: at `babble = 0` the titles, the headlines
and the banners of a preset are still shown.

The alternative, mixing generated lines into the ticker even when headlines are given (what the
holidays do, 60/40 at their height), gives more variety and less control.

Everything else on the screen is texture, in set mode too (K6): the labels of the progress
bars, the login attempts that scroll under the banner, the lines of the log. One part of that
texture is kept away from real people. The radar's list puts a name beside a bearing and a
range, and the sequence aligner puts one beside a score and a kinship; under a preset those
names come from `nombres`, never from `personas` ([8.2](#82-what-each-key-feeds)).

### 7.4 Keys

| key | what to write | rules |
|---|---|---|
| `tema` | The topic, a few words. | A string, not a list, up to 24 characters. Used in the banner's top caption and the key block's header. Without it both are the base's own; the file's name is never shown (K11). |
| `titulares` | Headlines for the ticker. | Up to 100 characters each. Blanks allowed. Shown in capitals. |
| `titulos` | Pane titles. | Up to 40 characters; 28 or fewer fit most panes whole. Blanks allowed, filled once when a pane takes the title. |
| `letrero_verde` | Banner pairs shown in green. | `LINE ONE / LINE TWO`, optionally `/ a caption` for the bottom of the frame. See [7.5](#75-the-banner). |
| `letrero_rojo` | Banner pairs shown in red. | Same. |
| `personas` | People, as they should be read: `Richard Stallman`. | Up to 28 characters. |
| `nombres` | Bare proper names: projects, products, acronyms, places. No article. | Up to 20 characters. Those of 9 or fewer also name the radar's contacts (`EMACS-417`). |
| `cosas` | Common nouns **with their article**: `el kernel`, `la licencia`, `las cuatro libertades`. | The article gives the gender and number; see below. |
| `adjetivos` | Adjectives in the masculine singular, or short phrases. | See below. |
| `verbos` | Infinitives. | See below. |
| `frases` | Whole lines for the logs. | Up to 120 characters. Blanks allowed. |

**Nouns carry their article (C3).** This is how the gender table is avoided: a Spanish speaker
writes "la licencia" without thinking. `el` and `la` make a singular noun that the grammar can
use as a subject, with everything agreeing ("la licencia polimórfica fue liberada con éxito").
`los` and `las` make a plural, which the grammar cannot conjugate for; plurals are used where
no agreement is needed: in `{cosa}` blanks and in "de ..." tails ("el servidor de las cuatro
libertades"). An entry without one of the four articles is skipped with a warning. A feminine
noun that takes `el` is marked: `el área (f)`.

A proper name that should be the subject of sentences goes in `cosas` with its article
(`la GPL`, `la FSF`); `nombres` is for names used bare.

**Adjectives (C4).** A single word ending in `-o` gets its feminine automatically (`privativo`,
`privativa`). Anything else, words not ending in `-o` and phrases of several words (`con
copyleft`, `sin puertas traseras`), is used unchanged for both genders. A phrase whose first
word has to agree marks the vowel with `*`, the engine's own notation: `protegid* por
copyleft`. The docs warn about the few adjectives this gets wrong (`español`, `trabajador`:
feminine by adding `-a`) and suggest a phrase instead.

**Verbs (C4).** The engine needs three forms of each verb: infinitive, gerund, participle. The
author writes the infinitive and the loader derives the rest:

- `-ar`: always regular (`liberar`, `liberando`, `liberado`).
- `-er`, `-ir`: the regular rule (`compartir`, `compartiendo`, `compartido`), plus the vowel-stem
  rule (`distribuir`, `distribuyendo`, `distribuido`; `leer`, `leyendo`, `leído`), plus a
  built-in table of the common irregular families matched by ending so that compounds are
  covered (`-scribir`: `escrito`; `-poner`: `puesto`; `-hacer`: `hecho`; `-volver`, `-solver`:
  `vuelto`, `resuelto`; `abrir`, `cubrir`, `romper`, `ver`, `decir`; the stem-changers
  `convertir`, `pedir`, `seguir`, `servir`, `elegir`, `corregir`, `medir`, `repetir`, `dormir`,
  `morir` and their families).
- Three forms written out override everything: `proveer | proveyendo | provisto`.
- Anything that is not an infinitive (reflexives included) is skipped with a warning.

The table lives in the program, not in the preset, which is what the requirement asks. Its
extent is the builder's ([12](#12-left-to-the-builder)); the test corpus in
[11.2](#112-the-preset-loader) is the floor.

**Blanks.** In `titulares`, `titulos` and `frases`:

| blank | becomes | engine key |
|---|---|---|
| `{persona}` | one of `personas` | `{s}` |
| `{nombre}` | one of `nombres` | `{t}` |
| `{cosa}` | one of `cosas`, with its article | `{o}` |
| `{usuario}` | a login name derived from `personas` | `{u}` |
| `{n}` | a number, 2 to 9998 | `{n}` |
| `{d}` | a digit, 1 to 9 | `{l}` |
| `{pct}` | a number, 1 to 99 | `{p}` |
| `{hex}` | four hex digits | `{x}` |

The loader rewrites them to the engine's one-letter keys (`Vocab::expand`). A line with an
unknown blank or a stray brace is skipped with a warning; a raw `{...}` never reaches the
screen.

A line never comes out as "de el kernel" (C30). `Auditoría de {cosa}` is the natural way to
write such a line, so it is allowed, and the engine contracts where the blank is filled: a
thing that starts with `el ` right after `de ` becomes `del ...`, and after `a ` becomes
`al ...`, the contraction `Vocab::de_np` already makes for generated phrases. The holidays'
`things` carry no article, so nothing changes for them.

### 7.5 The banner

Each line of `letrero_verde` or `letrero_rojo` is one banner: two lines of big letters, and an
optional caption for the bottom edge of the frame. The top edge carries `tema`.

**What can be written.**

- Big letters are drawn in the widget's 3x5 font. The loader upper-cases them and strips
  accents (`CÓDIGO` becomes `CODIGO`, as on any sign in capitals).
- Big letters can be A to Z, `Ñ`, 0 to 9, space, and `! ¡ ? ¿ : - . +` (C29). The font has
  A to Z, the digits, `!`, `:` and `¡` today; it gains the rest, which is what a title about
  technology needs (`¿QUE ES / LINUX?`, `C++`, `WEB 3.0`, `GPL-3`).
- **`Ñ` is drawn, never folded to `N`.** "AÑO" must not become "ANO" on camera, and
  `CONTRASEÑA`, `DISEÑO` and `ESPAÑOL` are words a Spanish video about technology uses. Its
  glyph is a bar over a short N (rows `111`, `000`, `110`, `101`, `101`). A banner with a
  character the font still lacks is skipped with a warning that says which character.
- Up to 12 characters per line, and no word longer than 12; 10 or fewer is the advice. A line
  may be several words.

**How it is drawn (C28).** Today the widget knows two sizes and one arrangement: two lines, one
word each, in a box of 13 rows, or else one line of ordinary text. On a tall screen that wastes
the height, and on a low or narrow pane the big letters are simply gone. Under a preset and in
set mode the banner is fitted to its pane instead:

1. *Lines can wrap.* A line of several words may go onto several lines of big letters, a break
   only between words: `NO ES UNIX` can be drawn as `NO ES` over `UNIX`. Two lines of the
   file are never joined into one, a word is never broken, and a banner is at most four lines
   of letters high.
2. *The letters come in eight sizes.* A letter is 3 by 5 pixels with one pixel between letters
   and one between lines. What changes is the size of a pixel:

   | size | a pixel is (columns x rows) | a letter takes (columns) | box height in rows for 2, 3, 4 lines of letters | drawn with |
   |---|---|---|---|---|
   | x6 | 6 x 3 | 24 | 35, 53, 71 | full blocks |
   | x5 | 5 x 2.5 | 20 | 30, 45, 60 | half blocks |
   | x4 | 4 x 2 | 16 | 24, 36, 48 | full blocks |
   | x3 | 3 x 1.5 | 12 | 19, 28, 37 | half blocks |
   | x2 | 2 x 1 | 8 | 13, 19, 25 | full blocks; today's wide form |
   | narrow | 1 x 1 | 4 | 13, 19, 25 | full blocks; today's narrow form |
   | compact | 1 x 0.5 | 4 | 8, 11, 14 | half blocks (`▀ ▄ █`) |
   | pocket | 0.5 x 0.5 | 2 | 8, 11, 14 | quadrant blocks (`▘ ▝ ▖ ▗` and the rest) |

   The pixels of x6 to x2 and of compact are physically square; those of narrow and pocket are
   twice as tall as wide, the look of today's narrow form. A box of `N` lines of letters is
   `2 + ceil((6N - 1) x h)` rows high, `h` being the pixel's height in rows, and
   `2 + (columns per letter) x (letters in its longest line)` columns wide, the frame included.
3. *The biggest that fits wins.* The sizes are tried from x6 down. At each one, every line of
   the file is wrapped to the number of letters that fit across the pane; if no word is too
   long and the box is not higher than the pane, that is the banner. So a wider letter is
   always preferred to a taller box, and wrapping is what lets a narrow, tall pane keep wide
   letters.
4. *Below pocket* (a pane under 8 rows, or too narrow for the longest word at two columns a
   letter) the banner is the one line of ordinary text it is today, and under 3 rows or 10
   columns there is none.

Some cases, by the inside of the pane (computed from the rule above):

| pane | `SOFTWARE / LIBRE` | `GNU / NO ES UNIX` | `CUATRO / LIBERTADES` |
|---|---|---|---|
| 66x19, the banner band of a 67x60 screen | x2, two lines, box 66x13 | x2, three lines (`GNU`, `NO ES`, `UNIX`), box 42x19 | narrow, two lines, box 42x13 |
| 119x28, the banner band of 120x106 | x3, two lines, box 98x19 | x3, three lines, box 62x28 | x2, two lines, box 82x13 |
| 131x44, the banner pane of 213x60 | x4, two lines, box 130x24 | x4, three lines, box 82x36 | x3, two lines, box 122x19 |
| 73x21, the banner pane of 120x33 | x2, two lines, box 66x13 | x2, three lines, box 42x19 | narrow, two lines, box 42x13 |
| 49x15, the banner pane of 80x24 | narrow, two lines, box 34x13 | narrow, two lines, box 42x13 | narrow, two lines, box 42x13 |
| 33x9, a small pane outside set mode | pocket, two lines, box 18x8 | pocket, two lines, box 22x8 | pocket, two lines, box 22x8 |

Two things to read off it. A word of eight letters gets the wide x2 letters from 67 columns up
and the narrow ones below, so on a small tall screen the advice is words of seven letters or
fewer. And a long word costs more than a long line: `LIBERTADES` drops a size where
`NO ES UNIX` wraps and keeps it.

The captions go on the frame as today, when they fit in it. Letters and box are centered by
characters, not bytes, which is what lets `¡`, `¿` and `Ñ` in. In `strict` transparency a cell
shows one color, so half-block letters are drawn as a glyph in the banner's color and never as
a blend.

**Where this applies.** Under a preset, and in set mode with or without one. On an ordinary day
and on a holiday outside set mode, `Access` draws exactly what it draws today, through the code
it has today.

### 7.6 Room for sections

Sections are not built. The format leaves room in the obvious way: later, a `[seccion.<name>]`
table can carry any of the same keys, overriding the top level while that section is on. Since
v1 files are flat, any table in a file is by definition from the future: the v1 loader warns
that it does not know it and ignores it. Nothing in the loader, in `Preset` or in the scene
assumes that a file yields exactly one vocabulary forever.

## 8. From file to screen

### 8.1 The neutral Spanish base

A new static `Vocab` (working name `BASE`, in `src/scenes/hollywood/neutro.rs`), written by
hand like the holidays' but about nothing in particular: Spanish technobabble, no holiday, no
country. It is what the screen says when a preset is on and has nothing to say.

- It fills **every** field of `Vocab` and `Dress`, and it satisfies every shape rule in
  [5](#5-constraints). It is tested like the holidays' vocabularies.
- It has no English. `canned_en` holds Spanish lines; `mundane_es` is the whole of `mundane`.
  Its errno messages, its packet lines and its labels are Spanish too, since under a preset
  they are the only ones shown ([8.3](#83-a-preset-inside-the-engine)).
- Beside the `Vocab` it brings the two small lists a preset needs and `Vocab` has no field
  for: neutral call signs for the radar and the aligner (`NODO`, `ECO`, `VECTOR`; no people),
  and the Spanish verbs of a transfer (`subiendo`, `descargando`, `respaldando`,
  `transfiriendo`, `restaurando`, `sincronizando`).
- Its canned lines and headlines lean on blanks, because that is how a short preset still gets
  its names everywhere: "Aviso de {persona}: reinicio programado en {n} minutos" needs nothing
  from the preset but a person. Its `things` carry their articles, as a preset's do, and the
  contraction of [7.4](#74-keys) serves its lines too.
- Its grammar lists (`nouns`, `adjs`, `verbs`, `states`, `errors`, `units`) can be the plain
  tables `lexico.rs` already has (`NOUNS`, `ADJS`, `VERBS`, `STATES`, `ERRORS`, `UNITS`), made
  visible to it.
- Rough floor for the new writing: 40 canned lines, 24 headlines, 16 people, 12 each of the
  smaller lists, 24 errno lines, 12 call signs. Its content is the builder's.

The base is only used under a preset. Set mode without a preset keeps today's English
technobabble (U).

### 8.2 What each key feeds

(C6, C7) A preset becomes one `Vocab` (with its `Dress`) plus the few things `Vocab` has no
field for: the pane titles, the banner rounds, the round of `frases` for the set's log, the
contact names and the transfer verbs. Working name for the whole: `Preset`.

| from | goes to | policy |
|---|---|---|
| `titulares` | `headlines`; the ticker | replaces the base |
| `titulos` | the preset's title list | replaces generated titles |
| `letrero_verde`, `letrero_rojo` | `dress.granted`, `dress.denied` (the first pair) and the preset's banner rounds | replaces the base |
| `personas` | `people` (`{persona}`); `users` and `dress.users` (login names: `stallman`); "de ..." tails. Not the radar, not the aligner | replaces the base; topped up from the base where a minimum is needed |
| `nombres` | `places` (`{nombre}`); "de ..." tails; and, as slugs: `holy` (identifiers: `gnu_servidor`), `passwords`, `files`, `dirs`, `daemons` (`linuxd`), `hosts`, `locals`, `dress.rain_words`, `dress.sat_names` (`EMACS-3`), `dress.symbols`, the process names in `feeds` (`tail -f /var/log/gnu.log`), `domain`; and, in upper case, the preset's contact names (the radar's list: `EMACS-417`; the aligner's subjects) | replaces the base; topped up where a minimum is needed (5 satellites, 8 symbols with at least one bullish, 8 contact names) |
| `cosas` | `nouns` (singular, article stripped, gender kept); `things` (`{cosa}`, with article); "de ..." tails (`del kernel`, `de la licencia`); slugs as above | blended by the engine: it takes 55% of nouns from the vocabulary and the rest from its plain table |
| `adjetivos` | `adjs` | blended by the engine (45%) |
| `verbos` | `verbs` | blended by the engine (45%) |
| `frases` | `canned_es` (and `canned_en`); the preset's round for the set's log pane ([10.2](#102-the-widget-pool)) | added to the base's canned lines, not replacing them: four phrases alone would repeat all day, and the base's lines are filled with the preset's names anyway |
| `tema` | `dress.granted[2]`, `dress.denied[2]` (the banner's top caption), `pem` | replaces the base's; without it, the base's stay |
| nothing | `errnos`, `errs`, `states`, `errors`, `units`, `mundane`, `packets`, `sniff`, `dna_head`, `dna_kin`, `tree_end`, `phase`, `mark`, the remaining labels of `Dress`, `days` (none), `songs` (none) | the base |

The rule behind the last row: **the file can only reach fields whose shape the loader checks.**
The structural strings of `Dress` (the randomart's frame, the column headers) are never taken
from user text.

Slugs are the ASCII forms used where the widgets need ASCII: lower case, accents stripped,
spaces as `_`. A word with `ñ` gets no slug; it is left out of identifiers, rain and symbols
rather than misspelled. The generated domain ends in `.example` (reserved by RFC 2606), so the
sniffer never shows a real one.

Contact names (K6, C34) are the upper-case slugs of the `nombres` of nine characters or fewer.
The radar lists a contact as the name and its number, `EMACS-417`, in the thirteen columns it
has; the aligner writes the bare name. Fewer than eight, and the base's call signs fill in.
A person's name never stands beside an invented bearing, range, score or kinship.

Every list in the finished `Vocab` is non-empty: the preset's entries if it gave any that
survived validation, else the base's.

### 8.3 A preset inside the engine

(C8) A preset is a holiday with no date. Inside hollywood it differs from Navidad in these ways
and no others:

1. **No `Kind`.** So no signature widgets, no garland on the title bars, no holiday palette, no
   songs in the logs, no headlines of particular days, no Santos Inocentes. The widget pool is
   the ordinary nineteen.
2. **Always at its height.** `level` is 1 wherever the engine scales with the holiday (eleven
   contacts on the radar, three drops in ten spelling a word).
3. **No English prose (K7).** A holiday's share of the text is `0.5 + 0.45 x level`, never more
   than 0.95, and the rest is the English `babble`. A preset's share is 1: every sentence,
   title, label, error and errno message comes from the preset's `Vocab`, and logs say
   "desde", never "from". What stays English is the boilerplate of the tools being imitated,
   which is not prose and which a Spanish-speaking admin reads in English every day: `INFO`,
   `WARN`, `Call Trace:`, `Kernel panic - not syncing`, `[ OK ]`, the words of the packets
   (`Standard query`, `Echo (ping) request`), the keywords of the code listings.
4. **The readable surfaces are verbatim** ([7.3](#73-what-the-camera-reads-is-what-the-file-says)).
5. **The ordinary palette.** The classic greens, or the theme's colors. The "holiday gold"
   (`Pal::oro`), which the holidays keep in its own hue under every theme, is the palette's
   amber under a preset, so a custom theme controls every color.
6. **The ordinary map.** Traces run between the world's cities, not home to Mexico.
7. **The holiday is ignored, forced or not.** With a preset on, hollywood dresses for the
   preset even if `fiestas=navidad` forces a holiday. The app still hangs that holiday's
   overlay on top, which is what forcing it asked for.
8. **Call signs on the radar and in the aligner**, not people
   ([8.2](#82-what-each-key-feeds)).

Everything else that the dressed widgets do for a holiday, they do for a preset: the radar's
contacts walking in to the center, the star at the head of a progress bar, words falling in
the rain.

How: under a preset, `Garb` ([6.2](#62-how-the-widgets-learn-about-the-preset)) derefs to a
`Fiesta` of `Kind::None` at level 1, so every `mx.is(..)` is false and every `mx.level` is 1.
It answers `lexico::of` and `widgets::dress` with the preset's `Vocab`. `Hollywood` builds its
`Garb` from the preset it loaded, or from `ctx.fiesta` as today when there is none. `Pal::new`
is told about the preset so that it skips the holiday tints (inside hollywood the holiday
counts as `Fiesta::NONE` while a preset is on). The pane titles (`Hollywood::retitle`), the
ticker (`Hollywood::feed_tape`) and the banner (`Access`) each gain a preset branch for
[7.3](#73-what-the-camera-reads-is-what-the-file-says).

**How the share becomes 1.** Level 1 is not enough: at level 1 the holidays' code still lets
English through in five places. Each gets a branch that only a preset reaches, and the holiday
path beside it stays as it is, drawing the numbers it draws today:

| where | what leaks at level 1 | under a preset |
|---|---|---|
| `Vocab::mixed`, `Vocab::mixed_title` (through `feed::say`, `feed::title`, `Hollywood::retitle`, `Hollywood::feed_tape`) | 1 line in 20 is `babble`'s, since `share(1)` is 0.95 | `Vocab::sentence` and `Vocab::title` are called directly |
| every `rng.f32() < lexico::share(level)` in `feed.rs` and `widgets.rs` (labels, identifiers, locals, kernel errors, tree names, passwords, login names) | 1 in 20 from the English tables | the test is skipped: always the vocabulary |
| daemon names, `0.5 x share` | half are `babble::DAEMONS` | always the vocabulary's |
| errno lines, `0.6 x share` | 4 in 10 are the real errno table, with its English messages | always the vocabulary's |
| `Vocab::progress_label` | "uploading", "downloading", "syncing" among its verbs | a preset's own label: the same shape, verbs from the preset's Spanish list |

One way to write it that keeps the diff small: `Garb` answers `takes(rng, k)`, which for a
holiday is the `rng.f32() < k x lexico::share(level)` there is today, number for number, and
for a preset is `true` without drawing. The builder may write it otherwise; what is fixed is
the table and the test of [11.3](#113-the-engine-with-a-preset).

**Random numbers.** Ordinary days and holidays must draw exactly the numbers they draw today.
Anything a preset or set mode needs at random comes either from inside a branch that only they
reach, or from a generator of their own.

### 8.4 Loading and validation

**Where files live (C11).** A new hollywood option, `preset` (string, default `""`, none):

```toml
[scenes.hollywood]
preset = "software-libre"
```

or `-o hollywood.preset=software-libre` for one run.

- A bare name is `<config dir>/presets/hollywood/<name>.toml`, where the config dir is the
  folder of the default config file (`$XDG_CONFIG_HOME/oxiper`, else `~/.config/oxiper`). The
  `hollywood/` level costs nothing now and keeps other scenes' presets apart later.
- A value with a `/` or ending in `.toml` is a path, used as given (a leading `~/` is
  expanded; a relative path is relative to where oxiper was started).
- As with any `-o` value, a bare name that TOML would read as a number or a boolean has to be
  quoted. An unquoted `-o hollywood.preset=2026` arrives as an integer. Today that is a
  warning ("expected string"), a pause of 1.2 seconds, and then `Opts::s` quietly answers `""`
  for it: no preset, on camera. So a `preset` that is set and is not a string is an error like
  a missing file (K10), and its message says to quote the name.

**Reading.** Loading is a total function from bytes to either a `Preset` and a list of warnings,
or one error. It never panics, whatever the bytes.

*Errors* (the preset cannot be used at all): the option is not a string; the file cannot be
read; it is not UTF-8; it is over 256 KiB; it is not valid TOML.

*Warnings* (one entry is dropped and the rest is used), each naming the key and the entry
(`titulares[4]: ...`):

- an unknown key (a likely typo), a key of the wrong type, or a table;
- an entry that is too long for its key, or beyond the 400th of its list;
- a character that does not take exactly one cell: control characters, wide characters
  (`render::is_wide`), combining marks (the loader first composes the Spanish ones, so a file
  saved in decomposed form still works);
- an unknown blank, a stray `{` or `}`, or a `*` outside `adjetivos`;
- a noun without its article, a word that is not an infinitive, a verb the loader will not
  guess;
- a banner line the font cannot draw or that is too long, or a banner without its two lines.

Before use, runs of spaces are collapsed and entries trimmed. A BOM and CRLF line endings are
tolerated.

**The last gate.** One function checks a whole `Vocab` against every rule the engine relies on:
no empty list the engine picks from, the `Dress` shape rules of [5](#5-constraints), the
banner's glyphs, `mundane_es <= mundane.len()`, packet lengths in order. (At most, not less
than: the base and every preset have `mundane_es == mundane.len()`, all Spanish, while the
holidays' own tests go on asserting the strict `<` for theirs. K12.) The loader runs it on
what it built and turns a failure into an error. The same function runs in the tests over the
base and over the three holidays, which is also how the checker itself is checked.

**Becoming `'static` (C10).** Strings are interned: each distinct string is leaked once and
reused on every later load. The slices and the `Preset` itself are leaked on each successful
parse. This is a deliberate, bounded leak: a few kilobytes per edit of a file that is edited by
hand a few dozen times in a session. The alternative is owned strings throughout `Vocab`,
`Dress` and the widgets, which is the refactor the user turned down.

**The last good preset (K5).** A scene is built again on every `r`, `s`, arrow key, playlist
step and option change ([5](#5-constraints)), and each time it needs its preset. Reading the
file there, as the first version of this design did, has two faults. A file caught half
written, or broken since the last look, would put the generic English screen on camera at the
next `r` (anti-goal 10), since only start-up and reload refuse a bad preset. And every rebuild
would parse and leak again.

So every load goes through one process-wide cache, keyed by the file's path:

- An entry holds the file's stamp (mtime and length) as last seen, the last `Preset` that
  loaded from that path with its warnings, and the reason of the last failure if the file is
  bad right now.
- A load first looks at the stamp. Unchanged: the entry answers, and the file is not read,
  parsed or leaked. `r` pressed a thousand times costs a thousand `stat` calls.
- Changed: the file is read and parsed. If it is good it becomes the entry's preset. If it is
  not, the entry keeps the preset it had and records the reason and the new stamp, so the bad
  file is not parsed again until it is saved again.
- A file that comes back empty (no key at all) after one that had keys counts as a failed
  read, with the reason "the file is empty". An editor that truncates before it writes
  produces exactly that for a moment, and an empty file is otherwise a valid preset: the
  neutral base, in other words the generic screen. The price: a preset cannot be emptied on
  purpose while oxiper runs. That needs a restart or another `preset`.
- The answer of a load says which of three things happened: a good preset as on disk; the last
  good preset with the reason the file is bad now; or an error, when the path has never
  loaded. Only the last one can stop a start-up or refuse a reload
  ([8.6](#86-errors)).

The cache is shared state, and [6.2](#62-how-the-widgets-learn-about-the-preset) turned a global
slot down for that reason. The difference is the key: two scenes or two tests only meet in it
if they name the same file, and then they should get the same answer.

### 8.5 Checking a preset without running it

No new command (C21). The existing ones already reach a preset through `-o`:

```sh
oxiper config check -o hollywood.preset=borrador.toml        # every error and warning
oxiper snap -s hollywood -o hollywood.preset=borrador.toml \
            -o set_mode=true --size 67x60 --at 20             # one frame, as text: on its side
oxiper snap -s hollywood -o hollywood.preset=borrador.toml \
            -o set_mode=true --size 120x33 --at 20            # and lying down
```

`config check` lists a preset's error or its warnings among its problems, each prefixed with
the preset's name, and exits non-zero if there are any. This is the loop an assistant drafting
a preset runs before handing it over.

### 8.6 Errors

(C12)

| | at start-up | while running | `config check` |
|---|---|---|---|
| **error**: not a string, missing, unreadable, too big, not TOML, emptied | oxiper does not start. One line on stderr with the path and the reason (a TOML error with its line). Same for `snap`, `record` and `--saver`. | Nothing on screen changes, now or at the next `r`, `s` or arrow key: the last good preset stays ([8.4](#84-loading-and-validation)). The HUD shows the reason for 5 seconds, once per bad save and once more for each scene built while the file is bad. The next save is tried again. | listed; exit 1 |
| **error in a preset just named** (`preset = "otro"` saved into the config, and `otro` has never loaded) | | The reload is refused like one with an unknown theme: the config in use stays, the HUD shows the reason, and the config's mtime is recorded so that the refusal is not repeated every second (K9; the same fix serves the themes). | |
| **warning**: an entry dropped | Printed with the config's warnings before the terminal is taken (stderr, then the existing 1.2 s pause). The preset loads without the entry. | The preset loads without the entry. The HUD says how many warnings. | each listed; exit 1 |

The preset is checked whenever the option is set, whether or not hollywood is in the playlist:
the arrow keys reach every scene (the themes are checked the same way).

Refusing to start is the answer to anti-goal 4: the moment to find out that a preset is broken
is at the keyboard, not in the edit. It is not a crash: nothing has been drawn yet, and the
message says what to fix.

HUD messages are one line of 60 characters or fewer with the specific part first (`preset:
line 12: expected a string`); the HUD is right-aligned, and a longer message loses its head.

The checks mirror the themes: a check beside `scene_theme_problems` that `Player::new` and
`Player::reload` treat as fatal, that `config check` adds to its list, and whose warnings `main`
prints with the config's. It asks the cache, so it is fatal only for a path that has never
loaded. `Hollywood::new` asks the cache too and takes whatever preset it answers with. If it
answers with none after the check passed (the file of a preset that never loaded vanished in
between), the scene starts without a preset and says so at its first `refresh`. It never
panics.

`app::run` records the config's mtime when `Player::reload` returns an error, as it already
does when `Config::load` does (K9).

### 8.7 Live edits

(C13) The scene watches its own file. `Scene` gains one method with a default, so no other
scene changes:

```rust
/// Called about once a second of real time by the interactive loop. A scene
/// that reads a file besides its options looks whether it changed and takes it
/// in. `Some(Ok(note))` and `Some(Err(why))` go to the HUD; `None`: nothing new.
fn refresh(&mut self, _ctx: &Ctx) -> Option<Result<String, String>> {
    None
}
```

`app::run` calls it next to its own check of `config.toml`, paused or not. hollywood asks the
cache of [8.4](#84-loading-and-validation) for its preset, which costs one `stat` when nothing
changed.

- A new good preset: the panes are re-dressed where they stand (`Hollywood::dress`, the path a
  holiday takes at midnight: same rectangles, same widget kinds, fresh widgets with the new
  words), the ticker starts over, and the HUD says `preset reloaded` (with the number of
  warnings, if any).
- The file is bad: the scene keeps the `Preset` it has, which is also what the cache would
  hand to a scene built now, and the HUD shows the reason. The scene remembers which failure
  it has reported, so the reason is shown once, not every second. A scene built later, by `r`
  for instance, reports it once more at its first `refresh`: the author has just looked at the
  screen, and what is on it is not what is in the file.

A half-written file is just an error followed, a second later, by a success. So is a file
caught empty in the middle of a save.

Why not make the preset file part of the config's own reload: `Player::reload` forgets the
theme picked with the arrow keys, and rebuilds the scene when options changed. Saving a preset
must do neither.

Changing *which* preset is on (`preset = "otro"` in `config.toml`) is an option change: it goes
through the config reload, which rebuilds the scene as any option change does. In set mode the
rebuilt scene has the same layout ([10.3](#103-the-layout)).

`snap`, `record` and `bench` never call `refresh`; they stay reproducible.

## 9. Set mode: switching and the app

### 9.1 The setting

(C14) A new global setting, beside `fiestas` and `eggs`:

```toml
set_mode = false   # on a film set: hollywood holds still, nothing follows the calendar
set_eggs = true    # in set mode, may the easter eggs play?
```

Both are top-level keys, so they also work as `-o set_mode=true`, in `[saver]`, and through
live reload. `oxiper config init` documents them with the rest.

The key **`s`** toggles set mode while oxiper runs, with a HUD note (`set mode on · seed 4821`,
`set mode off`). The key **`e`** switches the easter eggs off and back on
([9.3](#93-easter-eggs-on-the-set)). Both join the help overlay (`?`), the `--help` key list
and the README's table.

The running state is the `Player`'s. It starts as the config says; `s` flips it; a config
reload changes it only if the config's own `set_mode` value changed in that reload. So the last
word wins: editing an unrelated setting does not undo the key, and editing `set_mode` does take
effect.

Toggling rebuilds the current scene (`Player::rebuild`), with the flag passed through `Opts`. A
scene therefore only has to know how to start in set mode, never how to enter it halfway. The
seed is kept, unless set mode is going on and the seed in use is not one that can be written
down ([9.4](#94-a-seed-that-can-be-written-down)).

### 9.2 What the app does in set mode

(C15, U1)

| | normally | in set mode |
|---|---|---|
| **Calendar** | `cfg.fiestas.resolve()`, looked at again every minute. | Only a forced holiday counts: `Setting::Fixed(f)` gives `f`; `Auto` and `Off` give `Fiesta::NONE`. Nothing starts at midnight. (U) |
| **Overlay** | Drawn for the day's holiday. | Follows the same rule, so it is only there for a forced holiday. |
| **Easter eggs** | Rolled about once an hour; `E` plays one. | The same, unless `set_eggs` is off or `e` has switched them off. (U) |
| **Playlist** | Moves to the next scene after `duration`. | Stays on the current scene. The arrow keys still change it. |
| **Seed** | `r` and the arrow keys draw a new one. | They draw one that can be written down, and the HUD shows it. (K3) |
| **Keys, HUD, live reload** | | Unchanged. Everything a key or a save does is the author's own doing. |

The playlist is not on the user's list. It follows from anti-goal 1: with `scene = "all"` set
mode would otherwise leave hollywood after a minute. The same rule applies in
`headless::record`, through one shared predicate on `Player`.

The app-level half applies to every scene. Other scenes do nothing else with the flag in v1;
the docs say so.

### 9.3 Easter eggs on the set

(U1, C23, C24) The first version of this design kept every egg off the set. The user wants
them there, and wants to be able to switch them off. The existing settings do the second only
by losing the eggs everywhere: `eggs = "none"` and `egg_rate = 0` are global. So:

- **A setting for the set alone**, `set_eggs` (boolean, default `true`). A shoot that must not
  be interrupted sets it to `false`, or runs with `-o set_eggs=false`, and the eggs outside set
  mode are untouched.
- **A key, `e`**, that switches the eggs off, or back on, for as long as oxiper runs, in any
  mode. The HUD says `eggs off` or `eggs on`.
- **One flag behind both**, in the `Player`. It is set whenever set mode goes on or off: to
  `set_eggs` when it goes on, to on when it goes off. `e` flips it. A reload sets it again
  only if `set_eggs` changed in that reload. (The same "last word wins" as the `s` key.)

When the flag is off:

- nothing is rolled, and a forced `egg_rate` plays nothing;
- `E` plays nothing, and the HUD answers `eggs off` so that the key is not taken for broken;
- the egg that is playing is cut at once. This is why the key exists: a train has started
  across a take, the author presses `e`, and it is gone. (`Eggs` gains the one-line method
  that drops the active egg. The first design needed it to clear the set on entering set
  mode; now it is only called here.)

When the flag is on, the eggs are what `eggs` and `egg_rate` say, with the rules they have
today: about 1.2 an hour, never in the first minute, never over the help overlay.

**Which eggs can play.** A holiday's eggs keep to their days, and in set mode the day is no
holiday unless one is forced. So with `fiestas` on `auto` or `none` the set gets the three
everyday eggs, `sl`, `duckpond` and `nyancat`, on Christmas Day as on any other. With a forced
holiday it gets that holiday's eggs too, on the days and as often as that day would have them.
Nothing is added to `Eggs` for this: it already takes the day from the `Player`, and in set
mode the `Player`'s day is the forced one or none.

**An egg is not calmed.** It runs on real time, as always, and in its own colors; `set_pace`
and `set_dim` are hollywood's. An egg is meant to be seen.

**On a tall screen.** Every egg was run on a 67x60 screen, on a day it comes on: all 34 draw
there. The three that can play on an ordinary set look right at 60x55 and 67x60. The train of
`sl` is 83 columns long, longer than a narrow tall screen is wide, so it is seen in passing and
never whole, which is what a train does. A test keeps this true
([11.6](#116-set-mode-in-the-app)). If a holiday's egg turns out to look wrong on a tall
screen, the fix belongs in that egg's module and is not part of this work: it can only show on
a set where that holiday was forced.

### 9.4 A seed that can be written down

(K3, C33) In set mode the layout is a function of the seed
([10.3](#103-the-layout)), and the author chooses a layout by pressing `r` until one looks
right. To have it again tomorrow the author needs its number. Today there is none to give:
`r` takes the next 64-bit number from the player's generator, and no value of the `seed`
setting leads to it.

So in set mode a new seed is drawn the other way round:

- The `Player` draws a number `n` from 1 to 9999, then does with `n` exactly what start-up does
  with `seed = n`: a generator started from `n`, the shuffle of the playlist if `shuffle` is
  on (its numbers are drawn and thrown away; the playlist in use is not reordered), then the
  scene's seed. From there the generator is the player's.
- This happens on `r` and on the arrow keys in set mode, at start-up in set mode when `seed`
  is not set, and when `s` turns set mode on while the seed in use has no such number. A
  `seed` from the config or `-o` is such a number already, for the first scene of the run.
- The HUD shows it for four seconds: `hollywood · seed 4821` after `r`,
  `set mode on · seed 4821` after `s`, `set mode · seed 4821` at start-up (when `hud` is on;
  never in `--saver`).
- `seed = 4821` in the config, or `-o seed=4821`, on a terminal of the same size, brings the
  same panes back. With several scenes in the playlist the number brings back the first scene
  of the run; on the set that is `scene = "hollywood"`.

Outside set mode `r` and the arrow keys do what they do today, number for number, and the HUD
says `reseeded` as it does now.

## 10. Set mode in hollywood

### 10.1 Summary

| | normally | in set mode |
|---|---|---|
| layout | a split, a merge or a reshuffle every 10 to 25 s | frozen (U); made for the screen's shape, tall or wide (U) |
| pane titles | a new one every 8 to 22 s | fixed (U) |
| panes | up to `panes` (6) | 3 or 4 (U), on a tall screen as on a wide one (K13) |
| always there | nothing | the banner pane and one log pane (U) |
| widgets | any of nineteen | the big visual ones, and one slow log (U) |
| banner | two lines, one or two widths | fitted to its pane: lines wrap, eight sizes ([7.5](#75-the-banner)) |
| pace | `speed` | `speed x set_pace` (U: slower) |
| brightness | the palette | faded by `set_dim` (U: a bit dimmer) |
| blinking | banners, map, radar, new panes | none (U) |
| holiday | the calendar's | only a forced one (U; done by the app) |
| easter eggs | about one an hour | the same, with a switch (U; done by the app) |
| text | English, or the preset's | the same (U) |

The options `panes` and `reshuffle` mean nothing in set mode. `babble` works as usual.

### 10.2 The widget pool

(C16, U3, C27) The user approved "radar, map, bars, banner, rain" in and "hex dump, PEM keys,
DNA, sniffer" out, and later one log pane, slowed down. The widgets in between are a call:

| in | out |
|---|---|
| access banner (U), in every layout | hex dump (U) |
| one log (U), in every layout: the mission log (C27) | PEM keys (U) |
| radar (U) | DNA aligner (U) |
| world map (U) | sniffer (U) |
| progress bars (U) | the other text feeds: log tail, code, errno, tree, passwords and randomart (C: one slow log is enough text) |
| rain (U) | `htop` (C: half of it is a process table that re-sorts itself) |
| bandwidth graph (C: one big moving shape) | satellites, market (C: small tables of numbers) |

Seven kinds, no pane repeating another's. The five that are not in every layout are one
constant; moving a widget across is a one-line change.

**The banner pane is always there (U3).** In the first design it was one of six kinds the seed
might or might not pick, and a preset's banners were a matter of luck. Now every set layout has
the `Access` pane, sized for big letters ([10.3](#103-the-layout)).

**One log pane is always there (U3).** The user asked set mode to keep the technobabble, and
the log is where a preset's `frases` are read. Which log, and how it behaves (C27):

- It is the mission log (`mission-log`; the preset's name for it under a preset). Of the eleven
  text feeds it is the slowest already and the plainest to read: a clock, sometimes a green
  `[ OK ]`, one sentence, and now and then a phase line in capitals. No process ids, no host
  names, and no red: the syslog tail spends 25 columns on its prefix and paints one line in
  twelve red.
- Lines arrive one at a time, one line each scene second, which is one every two seconds on
  the clock at the default `set_pace`. A sentence that wraps onto three lines takes six
  seconds to come in. There is never more than one new line in a frame, whatever the frame's
  length (today a feed catches up with as many as sixty).
- Under a preset that gives `frases`, one sentence in three is one of them, taken in a shuffled
  round, so every phrase comes by before any repeats. The other two are the engine's, as in
  any log: the base's lines filled with the preset's names, and generated sentences. That is
  the technobabble the user asked to keep; the log is texture, not one of the verbatim
  surfaces of [7.3](#73-what-the-camera-reads-is-what-the-file-says). An author who wants the
  log closer to the script writes more `frases`.
- Without a preset it is the English mission log, at the same pace.

The login attempts that scroll under the banner get the same pace and the same one-line rule.
They are the only other scrolling text on a set screen.

### 10.3 The layout

(U2, C17, C25, C26, K13) In set mode the layout is a pure function of the seed and the terminal
size. Nothing else goes into it: not the preset, not the clock, not the generator the words
use.
**Editing a preset, or switching to another, cannot move a pane** (anti-goal 8).

It is made for the shape of the screen. Below, `C` and `R` are the columns and rows of the
area the panes tile (the terminal less the ticker's row), and a pane's size includes its title
bar and left edge, as everywhere in hollywood.

**Two shapes.** The screen is *tall* when it is physically taller than wide, `2R > C`, and
*wide* otherwise. 67x60, 120x106 and a half-screen terminal of 106x60 are tall; 213x60, 120x33
and 80x24 are wide.

```
tall, 67x60 (one seed)                 wide, 120x33 (one seed)

┌────────────────────────────────┐    ┌─────────────────────────────┬─────────────────┐
│ banner                 67x20   │    │ banner             74x22    │ radar    46x16  │
│                                │    │                             │                 │
│                                │    │                             │                 │
├────────────────────────────────┤    │                             ├─────────────────┤
│ radar                  67x16   │    ├─────────────────────────────┤ map      46x16  │
│                                │    │ log                74x10    │                 │
├────────────────────────────────┤    │                             │                 │
│ map                    67x16   │    └─────────────────────────────┴─────────────────┘
│                                │      the ticker
├────────────────────────────────┤
│ log                    67x7    │
└────────────────────────────────┘
  the ticker
```

**A tall screen is a stack of bands**, each as wide as the screen and each one pane (K13):

- *The banner band.* Its height is the title row plus the box of three lines of big letters at
  the biggest size in which eight letters fit across ([7.5](#75-the-banner)): 20 rows under 99
  columns, 29 rows from 99, 37 from 131, 46 from 163. Never more than 40% of `R`, never under
  14 rows. The yardstick is fixed, eight letters and three lines, and not the preset's longest
  banner, so the words never decide where a pane goes.
- *The log band.* An eighth of `R`, between 7 and 14 rows.
- *Two shape bands* share the rest evenly. With less than 18 rows left there is one; with less
  than 9, none.
- *Their order.* The banner band is the first or the second from the top; the seed picks which,
  and the order of the others. (C25: a screen on its side behind a presenter shows its upper
  half above the shoulders more often than its lower half. One constant.)

At 9:16, the shape of a monitor on its side, this comes out even: a third of the height for
the banner, an eighth for the log, and two bands each about a quarter as high as the screen is
wide, which is the height at which a full-width band is the shape of the world map.

**A wide screen is two columns:**

- *The side column* is 38% of `C`, at least 30 columns; the main column is the rest. If that
  would leave the main column under 44 columns there is no side column.
- *The main column* is the banner pane over the log pane. The log takes three tenths of `R`,
  between 7 and 14 rows.
- *The side column* holds two panes where there is room: one above the other when the column
  is physically taller than wide (`2R >= ` its columns) and `R` is at least 24; side by side
  when it is wider than tall and at least 60 columns. Otherwise it holds one.
- *The seed picks* which side the main column is on.

**Widgets go where their shape fits (C26).** The seed shuffles the five kinds that are not in
every layout, and the shape panes take from that shuffle in order, each taking the first kind
that suits it:

| kind | wants | suits a pane whose physical shape (columns / (2 x rows)) is |
|---|---|---|
| radar | a physically square area: twice as many columns as rows, plus 24 columns for its list from 50 columns | 0.6 to 2.4 |
| world map | the shape of the world: four times as many columns as rows | 1.2 or more |
| rain | anything; it is the one that likes a tall pane | any |
| bandwidth graph | wide or square | 0.6 or more |
| progress bars | wide; a label and a bar need 40 columns | 1.0 or more, and 40 columns |

If nothing that is left suits a pane, it takes the next kind anyway.

**A band is never divided (K13).** The first version of this design gave a shaped widget its
natural width inside one band of a tall screen and put a neighbor beside it, which made a
fifth pane. The user approved "3 or 4 panes, big", so every band holds exactly one pane, as
wide as the screen. A shaped widget draws in its band as it would in any pane of that size,
and no widget is changed for it:

- *The radar* fits its circle to the band's height and draws it in the middle of the columns
  left of its list, which stays at the right edge from 50 columns, as today. The circle is
  round at any width. The "suits" table keeps the radar out of a band that is too flat for
  it (more than 2.4 times as wide as tall, physically).
- *The map* keeps the world's proportions by the rule of
  [10.8](#108-a-tall-terminal-outside-set-mode), which every map follows on the set: the
  largest rectangle in its area that is between three and six times as many columns as rows,
  centered. In the 67x16 bands of a 67x60 screen that is the whole band; in a band only 9
  rows high the map is narrower than the band, with a margin on each side.
- *The rain, the graph and the bars* fill the width.

**How many panes.** Tall: banner, log and two bands: 4. Wide: banner, log and one or two in
the side column: 3 or 4. "Few and big" holds on both: at 67x60 every pane is as wide as the
screen, and the only one lower than 16 rows is the log.

| screen | panes (one seed) |
|---|---|
| 60x55 | banner 60x20, radar 60x13, progress 60x14, log 60x7 |
| 67x60 | banner 67x20, radar 67x16, map 67x16, log 67x7 |
| 120x106 | banner 120x29, radar 120x31, map 120x32, log 120x13 |
| 100x30 | banner 62x20 over log 62x9; radar 38x15 over rain 38x14 |
| 120x33 | banner 74x22 over log 74x10; radar 46x16 over map 46x16 |
| 213x60 | banner 132x45 over log 132x14; radar 81x30 over map 81x29 |
| 240x35 | banner 149x24 over log 149x10; rain 46x34 beside bandwidth 45x34 |

**Below the minimums** panes are dropped, the banner pane last:

| what goes | on a tall screen | on a wide screen |
|---|---|---|
| the second shape pane | `R` under 41 | a side column with room for one pane only (see above) |
| every shape pane | `R` under 30 | `C` under 74 |
| the log pane | `R` under 21 | `R` under 21 |

A pane other than the banner's is never narrower than 30 columns or lower than 7 rows,
hollywood's minimums today: a side column needs 7 rows, the bands of a tall screen need 30
columns, and where those are missing the banner pane is the whole screen. The banner itself
then goes down its sizes to one line of text ([7.5](#75-the-banner)), and a pane under 3x3 is
not drawn, as today. The sizes the docs recommend are 60x55 or more for a tall screen and
100x30 or more for a wide one.

**The rest of the function.**

- Everything random in it comes from a generator of its own, seeded from `ctx.seed`: the
  shuffle of the kinds, the order of the bands, the sides, each widget's own seed, the order
  in which the titles are dealt.
- Each pane's title is assigned once, by position, panes counted from the top and then from
  the left: the i-th pane takes the i-th of the shuffled `titulos`, or a generated one.
- It changes only when the author acts: `r` or an arrow key (a new seed), `s`, or a resize of
  the terminal, which lays the screen out again from the same seed and the new size, with
  fresh widgets.
- `Hollywood::tick` skips the layout timer and the title timers, and `ensure_signature` is not
  called ([10.6](#106-set-mode-with-a-forced-holiday)).

Because it is a function of the seed, a layout can be kept: the HUD gives the number
([9.4](#94-a-seed-that-can-be-written-down)), and `seed = 4821` gives the same panes every time
on the same terminal.

Outside set mode nothing about the layout changes; ordinary days keep their single generator
and their numbers.

### 10.4 Slower and dimmer

(C18) Two new hollywood options, because both are tuned by eye through the camera and no
default can be checked without the real set:

| option | default | |
|---|---|---|
| `set_pace` | `0.5` | In set mode, the pace of the widgets and the ticker, on top of `speed` (0.1 to 1) |
| `set_dim` | `0.25` | In set mode, how far the colors fade toward the background (0 = as usual, 0.9 = barely there) |

`set_pace` multiplies `dt` in `Hollywood::update`, next to `speed`. The ticker goes from 14 to
7 cells a second at the defaults. The `+` and `-` keys still work on top.

`set_dim` is applied once, where the palette is built (`Pal::new`): every tone, the ramp and
the garland colors are faded toward the background with the fade the palette already has. On a
terminal without RGB colors that fade does nothing; such a setup can use a dimmer theme.

### 10.5 No blinking, no flashes

(C19, K8, C35) The rule: in set mode nothing switches on and off, and nothing large appears,
disappears or changes color in one frame. What changes, eases.

**The clock these times run on.** A widget is only ever handed scene time: the `dt` of
`Hollywood::update`, already multiplied by `speed` and `set_pace`. `Env` does not change and no
new parameter reaches `update` or `draw`. So every time below is a time on the scene's clock,
chosen for the default `set_pace` of 0.5, and the second figure is what a wall clock shows at
that default. At `set_pace = 1` everything in the table is twice as quick, the banner's stay
included; that is what the option is for.

Every source in the pool and the chrome, and what becomes of it:

| where | today | in set mode: scene seconds (on the wall at 0.5) |
|---|---|---|
| pane border (`Hollywood::draw`) | a new pane's border is bright for a second | panes are born with a settled border |
| access banner (`Access`) | up for 1.8 s every 3 to 10 s; blinks at 5 Hz in its last half second | comes in over 0.5 s (1 s), stays 6 to 10 s (12 to 20 s), leaves over 0.5 s (1 s), with 2 to 4 s (4 to 8 s) between; the first within 2 s (4 s); never blinks |
| the banner's box | filled with `pal.ink` by `set_bg`, in one frame | cross-faded, see below |
| the log, and the logins under the banner (`Feed`) | 1.6 and 3 lines a second, and up to 60 lines in a long frame | 1 line a second (one every 2 s); never two in a frame |
| world map (`WorldMap`): arrival | the whole arc turns red in one frame, its marker alternates between two glyphs, and 1.4 s later `arcs.retain` drops both at once | the arc keeps its color and the marker is one glyph; both fade to the background over 1.5 s (3 s), and the arc is dropped when nothing of it shows |
| radar (`Radar`): center | alternates between two glyphs about three times a second when dressed | one glyph |
| radar: list | bearing and range are rewritten every frame | a contact's line is rewritten when the sweep passes it |
| progress (`Progress`): a finished bar | the whole bar turns green in one frame, and 1.4 s later is replaced by a new bar already up to 30% full | the bar keeps its colors and only the tag (`COMPLETO`) appears; after 3 s (6 s) its two rows fade out over 0.5 s (1 s) and the new bar starts empty |
| progress: its place in the pane | up to nine bars from the top, whatever the height | every third row when the pane has 27 rows or more, up to twelve bars: a tall pane is not left half empty |
| bandwidth (`Bandwidth`): the figures | rewritten ten times a second | once a second (every 2 s); the graph moves as before |

**The banner's cross-fade.** Fading only the letters would leave a solid box of `pal.ink`
appearing in one frame, with the log gone under it. So the banner comes in in two halves, and
leaves the same way backwards. In the first half the text of the log inside the box's
rectangle fades to the background (the cells are already drawn; `Access` fades their color).
In the second half the box is drawn: its background from the pane's background to `pal.ink`,
its frame, captions and letters from the background to their colors. No frame shows a box at
full strength where the frame before showed none. The fades are `Pal::fade`, which blends
toward the terminal's background when it is known and darkens toward black when it is not; on
a terminal without RGB colors they are coarse, and the docs say to film with one that has
them.

What is left alone, on purpose: the radar's contacts still brighten as the sweep passes and
fade over three seconds; the rain's glyphs change shape, not brightness; the ticker moves a
cell at a time. Those are single cells and single rows, eased by the slower pace, and they are
what makes the screen look alive.

Widgets learn that they are calm from their `Garb`, which they already hold.

### 10.6 Set mode with a forced holiday

`set_mode = true` with `fiestas = "navidad"` is the author asking for a holiday on the set. It
gets the set layout, the fixed titles, the pace and the dimming, and that holiday's easter
eggs. The banner pane and the log pane are there as always, dressed for the holiday, and the
first shape pane holds one of the holiday's signature widgets, picked by the layout's own
generator (C31). `ensure_signature`, which outside set mode turns the biggest pane into a
signature widget, is not used: the biggest pane of a set layout is the banner's.

What it does not get in v1 is calm versions of the holiday's own things: Navidad's lights still
chase along the title bars, the piñata still bursts, and on a forced `inocentes` the prank
still takes over the screen. The app's overlay for that holiday moves as it always does.
Calming all of that is real work in the holiday modules. The user was asked whether a holiday
will be filmed soon and did not say, so the default stands (K4;
[16](#16-questions-answered), question 3).

With a preset on, the holiday is ignored inside hollywood anyway
([8.3](#83-a-preset-inside-the-engine), point 7).

### 10.7 Every combination

"Eggs" below means: if the eggs are on ([9.3](#93-easter-eggs-on-the-set)).

| preset | set mode | `fiestas` | hollywood shows | the app draws on top |
|---|---|---|---|---|
| no | off | any | as today | as today |
| yes | off | any | the preset; no holiday dress | the overlay and eggs of the day's (or the forced) holiday, as today |
| no | on | `auto`, `none` | the ordinary English screen in a set layout | the everyday eggs |
| no | on | forced | that holiday in a set layout ([10.6](#106-set-mode-with-a-forced-holiday)) | its overlay; its eggs and the everyday ones |
| yes | on | `auto`, `none` | the preset in a set layout | the everyday eggs |
| yes | on | forced | the preset in a set layout | the forced holiday's overlay; its eggs and the everyday ones |

On a tall terminal every row is the same row; what changes with the shape is the layout in set
mode ([10.3](#103-the-layout)) and, outside it, the maps
([10.8](#108-a-tall-terminal-outside-set-mode)).

### 10.8 A tall terminal outside set mode

(C32) Presets work without set mode, and the screen on its side may well run ordinary hollywood
between shoots. So the ordinary scene was looked at on tall terminals too
([5](#5-constraints)). It holds up: panes mostly stack, the titles and the ticker fit, nothing
is cut that an 80x24 terminal does not cut today. Two things are poor. One is fixed wherever a
preset is on, the other on every tall terminal.

**The banner** falls to its one-line form in the narrow panes. Under a preset the fitted
banner of [7.5](#75-the-banner) takes care of it, in any mode. Without a preset the ordinary
banner stays exactly as it is (anti-goal 5).

**The maps are stretched.** `WorldMap` and `Satellites` draw the world onto whatever rectangle
they are given, and on a tall terminal the panes are often physically taller than wide: the
world comes out three times too tall. The fix is in the two widgets: when told to keep
shapes, the map is drawn in the largest rectangle inside its area that is between three and
six times as many columns as rows (physically 3:2 to 3:1), centered. A 34x28 area gives a map
of 34x11; a 140x10 area gives one of 60x10; a 120x30 area is left as it is.

They are told to keep shapes by their `Garb`
([6.2](#62-how-the-widgets-learn-about-the-preset)), when any of these holds as the widget is
made:

- a preset is on;
- set mode is on;
- the terminal is clearly a screen on its side: three rows or more for every two columns
  (`3 x rows >= 2 x columns`; physically 4:3 or taller). 67x60 and 120x106 are; 80x24 and
  213x60 are not, and neither is 106x60, a terminal filling half of a landscape screen, which
  must not change because of this.

After a resize that crosses that line, the widgets already on screen keep what they were told
and the ones made later follow the new shape. Nothing is re-rolled for it.

**Why this is allowed.** Anti-goal 5 says an ordinary day does not move when neither feature is
on, and this moves it: on a tall terminal, on an ordinary day, the two maps are drawn
differently. The call: the user asked for the screen on its side to work, a world three times
too tall is the one thing on it that reads as broken, and the change cannot reach a terminal
that is not tall. The fingerprint of [11.1](#111-ordinary-days-and-holidays-do-not-move) is
taken at a landscape size and at 67x60, from the code as it is, and both must still match
afterwards: on a tall terminal the rectangles, the kinds, the titles, the ticker and every
line of every feed stay what they are. Only the drawing of the two maps changes there, and
that was never in the fingerprint.

**What was looked at and left alone.** The splitting rule: at 60 to 75 columns it makes panes
of 30 to 37 columns, as an 80x24 terminal has today, and with a 6-pane limit a tall screen is
not short of big panes. The pane titles: a title of `titulos` is cut at the pane's edge like
any title. The ticker: one row at any width.

## 11. Testing

### 11.1 Ordinary days and holidays do not move

This comes first, before any other change (C20).

- **A fingerprint taken from the code as it is.** A test that runs hollywood for a couple of
  simulated minutes on an ordinary day and at the height of Muertos, Navidad, Patrias and
  Revolución, with a fixed seed, and hashes what it produced. It is taken at two sizes: a
  landscape one (140x40) and a tall one (67x60), the second because of
  [10.8](#108-a-tall-terminal-outside-set-mode). The expected values are recorded from the
  unmodified code and never change afterwards. It hashes only output that does not pass
  through `libm`, because CI runs on Linux and macOS: pane rectangles, widget kinds, pane
  titles, the ticker's text and the lines of each text feed. (The radar and the maps use `sin`
  and `cos`.)
- **The existing tests are not edited.** hollywood has dozens; they build widgets and scenes
  from a `Fiesta`, they write `Env` out field by field, and they must compile and pass as they
  stand. So must the tests of the eggs and of the app. (Hints: constructors that take
  `impl Into<Garb>`, with `From<Fiesta>`; `Hollywood` keeping its `fiesta` field.)
- **`r` outside set mode.** With a fixed `seed`, the seeds that `r` and the arrow keys produce
  are the ones they produce today.

### 11.2 The preset loader

Pure functions over text; no files needed.

- Each key in both forms (block and array); comments and blank lines inside a block.
- Nouns: `el`, `la`, `los`, `las`, `(f)`, a missing article.
- Adjectives: `-o`, invariable, phrases, an explicit `*`.
- Verbs, at least: `liberar`, `compartir`, `distribuir` (`distribuyendo`, `distribuido`),
  `leer` (`leyendo`, `leído`), `extraer` (`extrayendo`, `extraído`), `distinguir`
  (`distinguiendo`), `escribir` and `describir` (`-scrito`), `abrir` (`abierto`), `hacer` and
  `deshacer` (`-hecho`), `componer` (`compuesto`), `resolver` (`resuelto`), `devolver`
  (`devuelto`), `ver` (`visto`), `romper` (`roto`), `decir` (`diciendo`, `dicho`), `convertir`
  (`convirtiendo`), `pedir` (`pidiendo`), `seguir` (`siguiendo`), `elegir` (`eligiendo`),
  `corregir` (`corrigiendo`), `servir` (`sirviendo`), `dormir` (`durmiendo`); three written
  forms; a reflexive (skipped).
- Blanks: each one rewritten; an unknown one and a stray brace skip the line.
- `de {cosa}` and `a {cosa}`: over 10,000 fillings, never "de el" or "a el"; "del kernel",
  "al kernel", "de la licencia", "Del kernel" at the start of a line.
- Banners: accents stripped; `Ñ` kept; `¡ ¿ ? - . +` accepted; a character without a glyph, a
  line or a word over 12, and a banner of one line each skipped with their warning.
- Every glyph of the font is distinct from every other, and `Ñ` from `N`.
- `tema`: given, it is the caption; missing, the caption is the base's, and the file's name
  appears nowhere in the `Preset`.
- Contact names: upper case, from `nombres` of nine characters or fewer, at least eight with
  the base's; none of them is one of the `personas`.
- Every warning in [8.4](#84-loading-and-validation) fires for its case and names the entry.
- Every error fires for its case; a TOML error carries its line; a `preset` that is an integer
  or a boolean is an error that says to quote it.
- An empty file is the base and nothing else.
- **Hostile input never panics**: empty strings, one character, 10,000 characters, emoji, wide
  and combining characters, right-to-left text, NUL, braces, only plurals, only `ñ` words, 5,000
  entries, a 1 MiB file, random bytes. Each either loads to a `Vocab` that passes the last gate
  or fails with an error.
- The example preset in `docs/` (through `include_str!`) loads with no warnings.

### 11.3 The engine with a preset

- The last gate passes for the base and the three holidays (unchanged statics) and for every
  `Vocab` a preset test builds. The base and the presets have `mundane_es == mundane.len()`.
- The lexicon's existing checks, run over the base and over the example preset: 10,000
  sentences with no `{`, `}`, `*` or double space; titles in lower case; identifiers and
  passwords ASCII; headlines in capitals.
- **No English prose under a preset.** The check has to be one that the leaks of
  [8.3](#83-a-preset-inside-the-engine) cannot pass, which "nothing from `babble`'s tables and
  no ` from `" was not: `uploading` is in no table of `babble`, and neither is
  `No such file or directory`. It has three parts:
  1. *What is drawn.* The scene runs under the example preset and under an empty one, outside
     set mode, long enough for all nineteen widgets to have been on screen (kinds forced in
     turn, several minutes each). Every frame's text is collected, the known boilerplate is
     removed (a short list written in the test: `Kernel panic - not syncing`, `Call Trace:`,
     `Tainted:`, `[ OK ]`, `[permission denied]`, the level tags, the fixed words of the
     packet lines), and what is left is split into words. None may be on a stop list of
     English words that are neither Spanish words nor keywords of the code listings: `the`,
     `of`, `and`, `with`, `from`, `into`, `your`, `our`, `their`, `this`, `that`, `these`,
     `each`, `every`, `please`, `unable`, `failed`, `cannot`, `not`, `such`, `uploading`,
     `downloading`, `syncing`.
  2. *Where it comes from.* Over 10,000 draws of each, under a preset: no errno line is one of
     `feed::ERRNOS`, no daemon one of `babble::DAEMONS`, no login one of `feed::USERS`, and no
     sentence, title or label has gone through `babble`.
  3. *It can fail.* The same check is run once more with the preset's branches switched off: a
     test-only `Garb` that hands out the example preset's `Vocab` but answers for its share as
     a holiday at its height does (level 1 and nothing more, which is where the first design
     stopped). There it must fail, and among its failures must be a sentence of `babble`, one
     of the three verbs, and a message of the real errno table. A check that cannot catch the
     leak it was written for is not a check.
- Never "de el" or "a el" in anything a preset produces.
- No name of `personas` beside a number that was made up: over the radar's list and the
  aligner's lines, every name is a contact name.
- Every ordinary widget, dressed by a preset, at every size the existing test uses (1x1 to
  120x40) and at 34x29, 60x54, 67x59 and 120x105, for forty steps: no panic, nothing drawn
  outside its pane.
- The scene with a preset: the ticker shows the file's headlines in capitals and nothing else;
  pane titles come from `titulos`, first and bright; the banner shows the file's pairs in the
  right color, and only one color when only one is given; names from `nombres` fall in the
  rain.
- A preset wins over the holiday: with `ctx.fiesta` at the height of Navidad or Muertos, no
  garland, no signature widget, the ordinary palette.
- `babble = 0` with a preset shows the base's plain lines.
- **The fitted banner** ([7.5](#75-the-banner)), as a pure function from the words and the
  pane's inner size to a size, the lines and the box:
  - every row of the table of cases in 7.5;
  - for every pair of the example preset and every inner size from 1x1 to 150x60: the box is
    inside the pane; no word is broken; no two lines of the file share a line; at most four
    lines; and no bigger size would also have fitted;
  - drawn: nothing outside the pane at any size; the letters of each size are the font's
    pixels (a rendering of `AÑO` at each of the eight sizes is compared with the glyphs);
    under `strict` no cell of the letters is a blend;
  - with no preset and outside set mode, `Access` draws cell for cell what it draws today (the
    existing banner tests, unedited, are that check).

### 11.4 Live edits and the last good preset

With files in a temporary directory:

- An edit shows up after `refresh`, with the same rectangles and kinds as before.
- A broken file gives `Some(Err(..))`, the old words stay, and the next `refresh` without a new
  save is `None`.
- A fixed file loads again.
- A deleted file is an error, not a panic, and the old words stay.
- Loading the same content twice does not grow the interner.
- **Rebuilds do not read.** A scene built a thousand times over an unchanged file: the file is
  parsed once (the `Preset` is the same reference every time) and the interner does not grow.
- **A bad file does not reach the screen by `r`.** Load a good preset, break the file, build a
  new scene: it has the good preset, and its first `refresh` says why the file is bad. The
  same for a file truncated to nothing, and for one deleted.
- **A bad file is still fatal when nothing good came before.** A path that has never loaded
  and is broken, empty of TOML or missing: the check that `Player::new` runs is an error. An
  empty file that has never loaded is the base, with no error.
- A config reload that names a preset that has never loaded and is broken is refused: the
  scene, the config in use and the old preset stay.
- Two paths do not share an entry; the same path through two scenes does.

### 11.5 Set mode in hollywood

The sizes below are the tall ones 44x31, 60x55, 67x60, 90x80, 120x106 and 130x110, the wide
ones 80x24, 100x30, 120x33, 213x60, 240x35 and 240x65, and the in-between 106x60 and 100x50.

**The layout, as a pure function** of seed and size (no scene needed), over those sizes and a
hundred seeds each:

- The panes tile the area exactly: no overlap, no gap, the ticker's row left free.
- There is exactly one banner pane, and exactly one log pane whenever `R` is 21 or more.
- Every kind is in the pool; none twice.
- Tall: every pane is as wide as the screen, so no two share a row (K13); the banner band is
  the first or second; exactly 4 panes from 41 rows, never more at any size. Wide: the banner
  is above the log in one column; 3 or 4 panes from 74 columns.
- No pane but the banner's is under 30 columns or 7 rows, at any size.
- Whenever `R` is 14 or more the banner pane's inside is at least 13 rows, and on a screen of
  60 columns or more it holds eight letters in two lines at narrow or better.
- Shape: every shape pane suits its kind by the table of 10.3, unless no kind that was left
  suited it. On a tall screen a radar is therefore never in a band more than 2.4 times as
  wide as tall while another kind that suits the band is left.
- The table of examples in [10.3](#103-the-layout) for the seeds that produce it, and the
  table of minimums at its boundaries (40 and 41 rows, 29 and 30, 20 and 21; 73 and 74
  columns).
- The same seed and size give the same panes on every call; another size gives a layout for
  that size.
- Every size from 1x1 to 80x40 and the stress sizes: it returns, and with at least 3x3 there
  is a banner pane.

**The scene in set mode:**

- Ten simulated minutes (real time advancing): rectangles, kinds and titles are identical at
  the start and the end.
- Same seed and size give the same rectangles, kinds and widget seeds with no preset and with
  two different presets.
- A resize lays out again for the new size; resizing back gives the first layout again.
- With a forced holiday, the first shape pane is one of its signature widgets and the banner
  and log panes are there.

**Calm**, on the scene's clock:

- The banner: between its start and its end it is never absent for a sampled frame, and it
  stays at least 6 scene seconds. While it comes in and while it leaves, sampled at 30 frames
  a second, the background of every cell of its box moves by less than a fifth of the way
  between the pane's background and `pal.ink` from one frame to the next, and so does the
  color of its letters and its frame.
- The log and the logins: over a simulated minute at 30 frames a second, never two new lines
  in one frame, and between 20 and 40 lines in all at the default pace; one long frame (5 s)
  brings one line.
- With a preset of four `frases`, over 300 sentences of the log each phrase appears, none
  twice before all have, and between a quarter and four tenths of the sentences are phrases.
- The map: an arc's color is the same the frame before and the frame after its arrival; it is
  removed only after its fade has run; the marker shows one glyph.
- The radar: its center shows one glyph; a contact's line changes only in a frame in which the
  sweep crossed it.
- Progress: a bar that finishes does not change its bar's colors in that frame, and the bar
  that follows it starts at 0%.
- Bandwidth: its figures change at most once a scene second.
- No border is brighter than the settled color on the first frame.
- `set_dim = 0` gives the ordinary palette; `0.25` lowers the luma of every tone. `set_pace`
  halves how far the ticker moves in a second.

**Stress:** the stress sizes (1x1, 2x3, 7x2, 13x5, 300x90), the tall and wide sizes above, and
huge steps survive, with and without a preset, with and without a forced holiday.

### 11.6 Set mode in the app

- The calendar rule as a pure function: forced passes through; `auto` and `none` give
  `Fiesta::NONE`.
- The playlist predicate is false in set mode.
- **Eggs.** In set mode with `set_eggs` on, an egg can start (a forced rate starts one), and
  with `fiestas` on `auto` on a holiday's date only an everyday egg ever does. With `set_eggs`
  off: none by the roll, none by a forced rate, none by `E`. `e` flips the flag in and out of
  set mode; turning it off drops the egg that is playing in that same call. Going into set
  mode sets the flag to `set_eggs` and coming out sets it on; a reload that does not touch
  `set_eggs` leaves a key's choice alone.
- **Eggs on a tall screen.** A new test beside `eggs_survive_odd_sizes_and_huge_steps` (which
  is not edited): at 60x55, 67x60, 120x106 and 130x110, every egg draws at least one cell at
  some moment of its run and is done within 30 seconds; nothing panics.
- **Seeds.** In set mode, after `r` the HUD's number `n` and a `Player` started with
  `seed = n` have the same scene seed, with `shuffle` off and on; hollywood's rectangles and
  kinds are the same in both. The same for the number shown at start-up without a `seed`, and
  after `s`. `s` off and on again keeps the number.
- Toggling `s` keeps the theme; a reload keeps a key's choice unless `set_mode` changed in the
  config.
- A reload that the `Player` refuses is not tried again until the file changes (the mtime is
  recorded): two polls after one bad save produce one error.
- Every scene survives being built and run in set mode (the existing stress test's shape).
- `config check` reports a preset's error and its warnings.

### 11.7 By hand, on the set

What no test can say:

- With the monitor on its side and then lying down, at the font size of the shoot: the banner
  is in big letters, the radar is round, the map is the shape of the world, and no pane is
  mostly empty.
- Film ten minutes in set mode with the eggs off. Watching the footage, nothing on the
  backdrop makes you look. Then switch them on and press `E`: the egg is the only thing that
  does.
- The ticker and a pane title can be read at the framing actually used; the banner is on screen
  often enough to be caught; a line of the log can be read before the next one comes.
- Press `r` until the layout suits the framing, note the number, quit, start with `seed =`
  that number: the same panes.
- Save a preset mid-run: the words change and the panes do not move. Break it: the HUD says
  why and the screen stays, and stays after `r`.
- `oxiper bench -s hollywood` with a preset and with `set_mode=true`, at 213x60 and at
  120x106, stays within the scene budget.

## 12. Left to the builder

Decisions that are better made with the code open:

- Every working name, and how the new code is split into modules.
- The text of the neutral base ([8.1](#81-the-neutral-spanish-base)), within its floor.
- The exact patterns by which slugs become file names, daemons, hosts, satellites, symbols and
  process names ([8.2](#82-what-each-key-feeds)).
- The extent of the irregular-verb table beyond the test corpus.
- The shapes of the new glyphs (`Ñ ? ¿ - . +`), as long as each is distinct and `Ñ` is a bar
  over an N; and whether the fitted banner is drawn by one routine with a pixel size or by
  three.
- How `Garb` hands out the vocabulary's share (the `takes` of
  [8.3](#83-a-preset-inside-the-engine) is one way), and whether set mode reaches scenes
  through `Opts` or a `Scene` method ([6.3](#63-how-the-scene-learns-about-set-mode)).
- The constants of the layout that are a matter of looks and are named as such in
  [10.3](#103-the-layout): the 38% of the side column, the three tenths and the eighth of the
  log, the limits of the "suits" table. The tests of 11.5 are written against the rules, and
  the two tables of examples move with the constants.
- The exact timings of [10.5](#105-no-blinking-no-flashes) within a factor of about one and a
  half, and the wording of HUD messages, warnings and errors (60 characters, the specific part
  first).
- How the interner and the cache are kept (a `Mutex` and a map are enough).
- The shape of the fingerprint test.
- The per-key length limits, if a real preset shows one is wrong.

Not left to the builder any more, because nobody will review the choice: whether `Ñ` is drawn
(yes, C29), what happens to "de el" (contracted, C30), where the preset checks hook in
(beside the themes', [8.6](#86-errors)), whether loads are cached (yes, and why, K5).

## 13. Order of work

Each step builds, passes and can be pushed on its own, and leaves a day without either feature
exactly as it is. Step 12 is the one deliberate exception, on a tall terminal only (C32).

1. The fingerprint test, at both sizes ([11.1](#111-ordinary-days-and-holidays-do-not-move)).
2. `Garb` in place of `Fiesta` in `feed.rs`, `widgets.rs` and `make_widget`. No behavior.
3. The last gate, run over the three holidays.
4. The neutral base.
5. The loader: format, validation, derivation, interning, and the cache of the last good
   preset. Not wired to anything.
6. The fitted banner as a pure function and its drawing, and the new glyphs. Not reached by
   anything yet.
7. The `preset` option; the scene loads and dresses; the share of 1 and its test; the readable
   surfaces; the contact names; the checks at start-up and in `config check`; the config's
   mtime on a refused reload.
8. `Scene::refresh` and live edits.
9. `set_mode` and `set_eggs` in the config and the app: calendar, overlay, playlist, eggs, the
   `s` and `e` keys, the seed that can be written down.
10. The set layout as a pure function, with its tests. Not reached by anything yet.
11. Set mode in hollywood: the layout wired in, the log pane, then pace and dimming, then the
    calm widgets.
12. The maps that keep their shape ([10.8](#108-a-tall-terminal-outside-set-mode)), and the
    eggs' test on tall screens.
13. Docs and the example preset.

Where it lands:

| file | what changes |
|---|---|
| `src/scene.rs` | `Opts` carries set mode; `Scene::refresh` |
| `src/config.rs` | the `set_mode` and `set_eggs` settings |
| `src/app.rs` | set mode in `Player` (calendar, playlist, toggle, reload); the eggs' flag; the seed that can be written down; the `s` and `e` keys and the help overlay; the `refresh` poll; the preset check in `new` and `reload`; the mtime of a refused reload |
| `src/main.rs` | the key list; preset problems in `config check`; preset warnings at start-up |
| `src/headless.rs` | `record` uses the shared playlist predicate |
| `src/eggs/mod.rs` | a way to drop the egg that is playing; the test on tall screens |
| `src/scenes/hollywood/mod.rs` | the three options; `Garb`; the preset in `new`, `retheme`, `refresh`; titles and ticker; set mode in `resize`, `tick` and `update`; `Pal` (preset, dim) |
| `src/scenes/hollywood/lexico.rs` | `of` takes a `Garb`; the contraction in `expand`; the plain tables visible to the base |
| `src/scenes/hollywood/feed.rs` | the type of `mx`; the preset's branches (share, labels, errnos, daemons, the aligner's names); the calm feed; the round of `frases` |
| `src/scenes/hollywood/widgets.rs` | the type of `mx`; the new glyphs; `Access` under a preset and on the set (rounds, the fitted banner, the cross-fade); the map's routes under a preset; the radar's contact names; the calm branches of `WorldMap`, `Radar`, `Progress`, `Bandwidth`; the maps that keep their shape |
| `src/scenes/hollywood/preset.rs` (new) | the format, the loader, the last gate, the cache |
| `src/scenes/hollywood/neutro.rs` (new) | the base |
| `src/scenes/hollywood/letrero.rs` (new) | the fitted banner: wrapping, sizes, drawing |
| `src/scenes/hollywood/foro.rs` (new) | the set layout, a pure function |
| the holiday modules (`navidad.rs`, `patrias.rs`, `revolucion.rs`, `muertos.rs`, `posada.rs`, `grito.rs`, `archivo.rs`, `mictlan.rs`, `rostro.rs`, `inocentes.rs`) | nothing |
| the eggs (`src/eggs/*.rs` but `mod.rs`) | nothing |

## 14. Docs

- **`README.md`.** A section, "On camera", after "Fiestas": what set mode does and how to
  switch it; what a preset is, with a ten-line example and a pointer to the guide; the eggs on
  the set and the two ways to silence them; and the practical notes for the set machine:
  - *A monitor on its side.* Rotate the display in the system's settings, not the terminal;
    oxiper only sees columns and rows. `tput cols; tput lines` in the fullscreen terminal says
    what it has. Set mode lays out for a tall screen when there are more than half as many rows
    as columns.
  - *The font is the terminal's.* A bigger font means fewer, bigger cells: on a 1080x1920
    monitor about 120x106 with an ordinary font and about 67x60 with a big one. Aim for 60x55
    or more on a tall screen and 100x30 or more on a wide one; below that panes are dropped
    ([10.3](#103-the-layout)). The table of banner sizes, in short: from 67 columns a word of
    eight letters is in wide letters; shorter words are bigger.
  - *Preview without the monitor*: `oxiper snap -s hollywood -o set_mode=true --size 67x60`.
  - *Colors.* A terminal with RGB colors, or the fades are coarse.
  - *Keep it running.* Keep the window focused or set `unfocused_fps`; turn off the system's
    own screen blanking.
  - *Keep a layout.* `r` until it suits the framing, then `seed =` the number the HUD showed.

  `s` and `e` in the key table. `set_mode` and `set_eggs` in "Make it yours". hollywood's
  option table regenerated with `oxiper docs`.
- **`docs/PRESETS.md`** (new). The reference and the authoring guide, in English with Spanish
  examples, written so that an assistant given a video's script can produce a correct preset in
  one pass: every key with its rules and limits; the noun, adjective and verb rules with the
  cases they get wrong; the blanks; what to pull from a script (the people; the names; ten to
  twenty nouns with their articles; ten to twenty headlines of 60 characters or fewer that
  state facts from the video; six to ten titles; two to four banner pairs; six to twelve
  `frases`, since the set's log shows them often); how the banner is fitted, with the advice
  that follows from it (words of seven letters or fewer for a small tall screen; a long line
  wraps, a long word shrinks); the warning about what generated text does with real names,
  that the readable surfaces are verbatim, and that the radar and the aligner never show a
  person; the two checking commands of [8.5](#85-checking-a-preset-without-running-it), with a
  tall `--size` beside the wide one.
- **`docs/presets/hollywood/software-libre.toml`** (new). The example of
  [7.2](#72-example).
- **`docs/SCENES.md`.** `ctx.opts.set_mode()` and what a scene owes the set (nothing pops up);
  `refresh` in the lifecycle; under "Holidays", that hollywood's maps keep their shape on a
  tall terminal.
- **`src/scenes/hollywood/mod.rs`.** A paragraph each for presets and set mode in the module
  docs, in the voice of the holiday paragraphs.
- **`src/eggs/mod.rs`.** The module docs say an egg can be switched off with `eggs = "none"` or
  `egg_rate = 0`; they gain `set_eggs` and the `e` key.
- **`oxiper --help`**, the help overlay and `oxiper config init` pick up the keys and the
  settings from the code.

## 15. Decision log

Every call in this design and who made it: **U** the user (in this round of answers; the
user's earlier decisions are in sections 2, 3 and 6.1, and are marked U wherever they come up
again), **C** the design's author, **K** the coordinator. A call that a later one reversed
keeps its row, says so, and points to the row that replaced it.

| | call | why | the alternative, and what became of the call |
|---|---|---|---|
| U1 | easter eggs play in set mode, and can be switched off | the user's answer | reverses the eggs half of C15; how, C23 and C24 |
| U2 | the set screen is mainly a monitor on its side, sometimes a landscape one; the layout is designed for the tall one and works on both | the user's answer; it also answers the old question about the terminal's size (unknown: design for a range) | C25, C26, C28, C32 |
| U3 | in set mode the banner pane is always there, and so is one log pane, slowed down | the user approved the coordinator's proposal; the user had asked set mode to keep the technobabble, and `frases` live in the logs | reverses part of C16 and the banner clause of C17; how, C27 |
| U4 | build everything, with no further review | the user's answer | nothing is left as a question; see section 16 |
| K1 | keys and blanks stay Spanish | the default of old question 1, unanswered | confirms C2 |
| K2 | with `titulares`, the ticker shows only those | the default of old question 2, unanswered | confirms C5 |
| K3 | after `r`, the HUD shows a seed that can be written into the config | old question 4, unanswered: a layout picked with `r` has to be keepable | how, C33 |
| K4 | a forced holiday in set mode is not calmed in v1 | old question 3, unanswered | 10.6 stands |
| K5 | one process-wide cache of the last good preset, keyed by path, stamped by mtime and length; a bad or emptied file keeps the last good one; fatal only when nothing good came before | every rebuild re-read the file: a bad save reached the camera at the next `r`, and every rebuild leaked | amends C10, C12 and C13 |
| K6 | the rest of the set's text is texture; the radar never shows `personas` | a real name beside an invented bearing reads as a claim | how, C34 |
| K7 | "no English" means no English prose; the tools' boilerplate stays; the share of 1 is reached by branches only a preset takes, and the test must be able to fail | at level 1 the holidays' code still lets English through in five places | amends C8; the table in 8.3, the test in 11.3 |
| K8 | calm timings are on the scene's clock, tuned for `set_pace` 0.5, both figures stated; the map's arc and the banner's box join the audit | widgets only get scene time; two flashes were missed | amends C19; C35 |
| K9 | the config's mtime is recorded when the `Player` refuses a reload | the refusal was repeated every second | |
| K10 | a `preset` that is not a string is an error | `-o hollywood.preset=2026` meant a warning and then no preset | |
| K11 | without `tema` the captions are the base's, never the file's name | it is shown on camera | |
| K12 | the last gate allows `mundane_es <= mundane.len()` | the base wants `==`, the holidays' tests assert `<` | |
| K13 | on a tall screen every band holds exactly one pane: at most 4 panes (banner, log, two shape bands), as on a wide screen; a shaped widget draws in its full-width band as in any pane of that size | the user approved "3 or 4 panes, big"; a divided band made a fifth, and two panes of 30 columns | reverses the divided band and the fifth pane of C25, and the neighbor clause of C26. The radar's circle and the map's proportions (C32) are what keep the shapes right in a wide band |
| C1 | TOML, flat, lists as `'''` blocks | existing parser and habit; no per-item syntax to get wrong | a line-based format of our own: more code, no line numbers for free |
| C2 | Spanish keys and blanks | the file is Spanish prose | English names; decide before the first real preset exists. Confirmed by K1 |
| C3 | nouns carry their article; plurals only in blanks and tails | no gender table; the grammar is singular | |
| C4 | adjectives by their ending; verbs derived, with a built-in irregular table and a three-form override | no conjugation by hand | `-ar` only, the rest refused: safer, more typing |
| C5 | readable surfaces are verbatim when the preset provides them | anti-goal 7 | mix generated lines in, as the holidays do. Confirmed for the ticker by K2 |
| C6 | technical fields derived from `nombres`, `cosas`, `personas`; no slug for `ñ` words; `.example` domains | twenty lines of preset re-dress nineteen widgets | |
| C7 | replace for lists of names and readable text; blend for grammar; add for `frases` | short lists must not make the logs repeat | In the set's log `frases` get a fixed share, C27 |
| C8 | a preset is a dateless holiday at its height: no English, ordinary palette and map, wins over a forced holiday | "the screen talks about the video" | "No English" made precise by K7 |
| C9 | `Garb` carried by the widgets; `Preset` wraps `Vocab`; holiday modules untouched | explicit data flow, smallest safe diff | a global slot |
| C10 | interned, leaked strings | `&'static str` is everywhere | Bounded by K5: nothing is parsed unless the file changed |
| C11 | `hollywood.preset`: a name under `presets/hollywood/` in the config dir, or a path | per-video files anywhere; a home for the regulars | |
| C12 | unusable preset: no start; bad entry: warning; while running: keep what is on screen | anti-goals 3 and 4 | start anyway with a warning. Extended by K5 to every rebuild, and by K10 |
| C13 | `Scene::refresh`, polled each second; preset edits do not go through the config reload | keeps key choices; no other scene changes | It asks the cache of K5 |
| C14 | `set_mode` is global and reaches scenes through `Opts`; toggling rebuilds; a key's choice survives unrelated reloads | half of set mode is the app's | |
| C15 | in set mode: no eggs by any route, no playlist advance | anti-goal 1 | **The eggs half is reversed by U1** (see C23 and C24); the playlist half stands |
| C16 | pool: radar, map, rain, progress bars, bandwidth graph, banner | the user's five plus one; text that scrolls is out | one constant to edit. **Reversed in part by U3**: one slow log is in, and the banner is no longer drawn by lot (see C27) |
| C17 | set layout is a pure function of seed and size, from its own generator; the banner goes where big letters fit | anti-goal 8; a layout can be kept with `seed=` | The first half stands. **The banner clause is reversed by U3** (see C25): the banner has its own pane, sized for it. The tiling "as today" is replaced by C25 and C26 |
| C18 | `set_pace` 0.5 and `set_dim` 0.25 as options | tuned by eye through the camera | constants |
| C19 | calm: three widgets and the border; holiday widgets not calmed | the pool's actual blink sources | Extended by K8 and C35; the holiday half confirmed by K4 |
| C20 | fingerprint first; existing tests unedited | anti-goal 5 | The fingerprint gains a tall size, C32 |
| C21 | no new dependency, no new subcommand | `config check` and `snap` already reach a preset | |
| C22 | the example preset lives in `docs/` and in a test | it cannot rot | |
| C23 | `set_eggs` (default on) for the set alone, and the `e` key for the run; when off, `E` plays nothing and says so, and the egg that is playing is cut | `eggs` and `egg_rate` can only silence a shoot by losing the eggs everywhere; a key is the quick way mid-take | the two existing settings only: a config edit to silence a take |
| C24 | on the set an egg is as it always is: everyday eggs unless a holiday is forced, real time, own colors; a test for tall screens, and a holiday egg that looks wrong there is fixed in its own module, not here | holiday eggs keep to their days; an egg is meant to be seen; all 34 were run at 67x60 | dimmed or slowed eggs: work in 34 modules |
| C25 | the set layout has two shapes, by the screen's physical aspect: bands on a tall screen (4 or 5 panes), two columns on a wide one (3 or 4); the banner band among the top two; panes dropped below the minimums, the banner last | U2; a monitor on its side at 9:16 divides evenly into a banner, a map, a radar's square and a log | one tiling for both: small letters and a stretched map on the tall screen. **The divided band and the fifth pane are reversed by K13**: a tall screen has at most 4 panes, one to a band. The two shapes, the banner band's place and the minimums stand |
| C26 | widgets are placed by shape: the radar takes a square, the map four columns a row, and what suits the rest goes beside them | a round radar and a world of the right proportions are most of what "looks right" means here | kinds by lot, as outside set mode. **"Goes beside them" is reversed by K13**: nothing shares a band. Placing by the "suits" table stands |
| C27 | the log is the mission log; one line a scene second, never two in a frame; under a preset one sentence in three is a `frase`, in a shuffled round | U3; the calmest and plainest of the feeds; `frases` would otherwise be about one sentence in thirty | the syslog tail: more prefix, red lines |
| C28 | under a preset and on the set the banner is fitted: lines wrap between words, eight sizes from x6 to pocket, the biggest that fits; elsewhere `Access` is untouched | U2: big letters on a narrow, tall pane, and big letters at all on a low one | two more fixed sizes: simpler, and wasteful on exactly the screen the user has |
| C29 | the banner font gains `Ñ`, `?`, `¿`, `-`, `.`, `+`, and `¡` is allowed in big letters | Spanish titles about technology; the fitted banner centers by characters | refusing `Ñ`, as the first design did: no `AÑO`, no `CONTRASEÑA` |
| C30 | "de el" and "a el" are contracted where a `{cosa}` is filled | `Auditoría de {cosa}` is how such a line is written; the holidays' things have no article, so they do not change | refusing those lines with a warning |
| C31 | with a forced holiday, the first shape pane holds a signature widget; `ensure_signature` is not used on the set | it would turn the banner pane into one | |
| C32 | outside set mode a tall terminal changes one thing: the two maps keep the world's proportions (also under a preset and on the set); the fingerprint is taken at a tall size too | U2, weighed against anti-goal 5: it cannot reach a terminal that is not tall, and what the fingerprint covers does not move | leave ordinary hollywood alone: a world three times too tall on the user's main screen |
| C33 | in set mode a new seed is a number from 1 to 9999 put through start-up's own steps; it is drawn on `r`, the arrows, start-up without `seed`, and `s`; outside set mode `r` is unchanged | K3: the number on the HUD has to be one that `seed =` accepts and that leads to the same scene seed | showing the 64-bit scene seed: no setting leads to it |
| C34 | contact names from `nombres` (upper case, nine characters or fewer, the base's call signs to fill), for the radar and for the aligner | K6; the aligner puts a name beside a score and a kinship, the same fault | neutral call signs only: the radar would not talk about the video |
| C35 | calm, beyond K8's list: a finished bar keeps its colors, fades and restarts empty; bars spread over a tall pane; the bandwidth's figures once a second; a radar line rewritten as the sweep passes; the banner cross-fades with the log under it | each is a row, a pane or a box changing in one frame, or a pane left half empty on a tall screen | |

## 16. Questions, answered

The first version of this design ended with five questions, each with a default that the design
already followed. The user read a summary, answered some, and asked for the work to go ahead.
This is the record.

**From the user:**

- *Easter eggs.* Wanted in set mode, and they must be easy to switch off. The first design kept
  them out. Now: [9.3](#93-easter-eggs-on-the-set).
- *The screen.* Mainly a monitor on its side, sometimes a landscape one: the layout is to be
  thought out for the vertical monitor and to work on both. Now: [10.3](#103-the-layout),
  [7.5](#75-the-banner), [10.8](#108-a-tall-terminal-outside-set-mode).
- *What is always on screen.* The banner pane, and one log pane, slowed down (the
  coordinator's proposal, approved). Now: [10.2](#102-the-widget-pool).
- *Go ahead.* With everything, and no further review.

**The five questions:**

1. **Spanish or English keys in the preset file?** Not answered. Spanish, the default (K1).
   Changing it later breaks every preset written by then.
2. **On the ticker, only your headlines?** Not answered. Yes, the default (K2).
3. **Will a holiday be filmed in set mode soon?** Not answered. The default stands: a forced
   holiday on the set keeps its own blinking lights and bursting piñata (K4,
   [10.6](#106-set-mode-with-a-forced-holiday)). If one is to be filmed, calming it is the
   next piece of work, in the holiday modules.
4. **Does a layout have to come back on another day?** Not answered. It can: the HUD shows a
   number after `r`, and `seed =` that number brings the layout back (K3,
   [9.4](#94-a-seed-that-can-be-written-down)).
5. **About how many columns and rows will the set terminal have?** Answered by the screen: not
   known, and both shapes. So the layout is designed for a range, about 60 to 130 columns by
   55 to 110 rows on its side and 100 to 240 by 30 to 65 lying down, and the compact
   big-letter banner this question suggested is built, with seven other sizes
   ([7.5](#75-the-banner)).

**Still open:** nothing that stops the work. Three things can only be judged through the
camera, and each has an option or a constant to turn: how slow (`set_pace`), how dim
(`set_dim`), and whether the banner band belongs in the upper half of a screen on its side
([10.3](#103-the-layout)).
