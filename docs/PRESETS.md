# hollywood presets

A preset is a small TOML file of Spanish words that the hollywood scene weaves into its
technobabble: the people, the names and the things a video is about. It decides what the
three readable surfaces say (the ticker, the pane titles and the big banner), word for word,
and it feeds the logs, the login attempts, the file names and the rest of the small text.

This guide is the reference for every key and the rules the loader enforces, and a recipe for
writing a preset from a video's script. The file it describes is in Spanish, with Spanish keys
and blanks; the guide is in English. The example that ships with oxiper is
[`docs/presets/hollywood/software-libre.toml`](presets/hollywood/software-libre.toml).

- [Turning a preset on](#turning-a-preset-on)
- [The shape of the file](#the-shape-of-the-file)
- [What the camera reads](#what-the-camera-reads)
- [The keys](#the-keys)
- [Blanks](#blanks)
- [The banner](#the-banner)
- [Real names](#real-names)
- [Writing a preset from a script](#writing-a-preset-from-a-script)
- [Checking a preset](#checking-a-preset)
- [Errors and warnings](#errors-and-warnings)

## Turning a preset on

A preset is a hollywood option:

```toml
[scenes.hollywood]
preset = "software-libre"
```

or `-o hollywood.preset=software-libre` for one run.

- A bare name is `presets/hollywood/<name>.toml` in the config folder, the folder of the default
  config file: `$XDG_CONFIG_HOME/oxiper`, else `~/.config/oxiper`. `software-libre` is
  `~/.config/oxiper/presets/hollywood/software-libre.toml`.
- A value with a `/` in it or ending in `.toml` is a path, used as given. A leading `~/` is
  your home; a relative path is relative to where oxiper was started.
- A name that TOML would read as a number or a boolean has to be quoted: `-o
  hollywood.preset='"2026"'`. Unquoted it arrives as a number, and oxiper refuses to start
  ("expected a string, found integer: quote the name").

The preset is checked whenever the option is set, at start-up and at every reload of the
config. A preset with an error stops oxiper from starting (see
[Errors and warnings](#errors-and-warnings)).

Edit the file while oxiper runs and save it: within a second the panes are dressed again in
the new words, where they stand, and the HUD says `preset reloaded` (with the number of
warnings, if any). A save that breaks the file changes nothing on screen: the last good preset
stays, and the HUD shows the reason for five seconds. The next save is tried again.

## The shape of the file

- **Flat.** Every key is at the top level. There are no tables: a `[table]` header is warned
  about and ignored, with everything under it (those are kept for a later version).
- **Lists are blocks of lines.** A list is one multi-line literal string, one item per line,
  between `'''` and `'''`. No quotes or commas per item and no escaping: `"`, `«»` and `\`
  are written as they are. Blank lines and lines that start with `#` are skipped.
- **Arrays work too.** `nombres = ['GNU', 'Linux']` is the same list as a block.
- **Everything is optional.** An empty file is a valid preset: the neutral Spanish screen,
  about nothing in particular. A preset with only `titulares` and `nombres` is already useful.

```toml
# A comment line outside the blocks.
tema = "Software libre"

nombres = '''
GNU
Linux
# A line that starts with # is skipped, inside a block too.
GPL
'''

personas = ['Richard Stallman']
```

What the loader does with the text before it looks at it:

- Every entry is trimmed and runs of spaces become one.
- A BOM at the start, CRLF line endings and accents saved in decomposed form (`o` followed by
  a combining accent) are read as if the file were clean.
- Every character has to take exactly one cell of the terminal. An entry with a control
  character, an emoji, a CJK character, a zero-width character or a stray combining mark is
  dropped with a warning.
- A key written twice is a TOML error, and the file does not load.
- A list keeps its first 400 entries; one warning names the rest. A file over 256 KiB is not
  read.
- `{` and `}` mean something only in `titulares`, `titulos` and `frases` ([blanks](#blanks)),
  and `*` only in `adjetivos`. Anywhere else either one drops the entry with a warning.
- A block cannot contain `'''` (it would end the block).

Keys and blanks are Spanish and unaccented: `titulos`, not `títulos`. An unknown key is a
warning, and most often a typo.

## What the camera reads

Three surfaces are the ones a viewer can read. For each of them, **if the preset gives text,
the screen shows that text and nothing else there**:

| surface | key | with the key | without it |
|---|---|---|---|
| the ticker along the bottom | `titulares` | only these lines, in capitals, in a shuffled round: every line once before any repeats | the neutral base's headlines and generated lines |
| the pane titles | `titulos` | each pane takes one, drawn first and bright, followed by the process name: `Las cuatro libertades · radar --barrido` | the process name and a generated name |
| the big banner of the access pane | `letrero_verde`, `letrero_rojo` | only the pairs given, in a shuffled round; with only one of the two keys, only that color ever shows | `ACCESO` over `CONCEDIDO` or `DENEGADO` |

The `babble` option does not touch these three: at `babble = 0` a preset's headlines, titles
and banners are still shown.

Everything else on the screen is texture: the logs, the labels of the progress bars, the login
attempts that scroll under the banner, the file and host names. The engine builds it from the
preset's words and its own grammar, in Spanish. What stays English is the boilerplate of the
tools being imitated (`INFO`, `WARN`, `[ OK ]`, `Kernel panic - not syncing`, the words of
network packets), which a Spanish-speaking admin reads in English every day.

Under a preset the scene ignores the holiday, even one forced with `fiestas`: no holiday
widgets, colors or songs. The app still draws a forced holiday's decorations and its easter
eggs on top.

## The keys

| key | what to write | limit per entry | blanks | where it shows |
|---|---|---|---|---|
| `tema` | the topic, a few words; a string, not a list | 24 characters | no | the banner's top edge, the key block's header |
| `titulares` | headlines for the ticker | 100 characters; 60 or fewer is the advice | yes | the ticker, in capitals |
| `titulos` | pane titles | 40 characters; 28 or fewer fit most panes whole | yes | the title bars |
| `letrero_verde` | banners shown in green | 70 characters for the whole line; see [the banner](#the-banner) | no | the access pane |
| `letrero_rojo` | banners shown in red | the same | no | the access pane |
| `personas` | people, as they should be read | 28 characters | no | the logs, login names |
| `nombres` | bare proper names: projects, products, acronyms, places | 20 characters | no | the logs, identifiers, the radar |
| `cosas` | common nouns **with their article** | 40 characters, article included | no | the logs, `{cosa}` |
| `adjetivos` | adjectives in the masculine singular, or short phrases | 40 characters | no (`*` marks agreement) | the logs |
| `verbos` | infinitives | 60 characters | no | the logs, the progress bars |
| `frases` | whole lines for the logs | 120 characters | yes | the logs |

An entry over its limit is dropped with a warning; the rest of the list is used.

### `tema`

```toml
tema = "Software libre"
```

The caption on the top edge of the banner's frame, as written (not in capitals), when it fits
in the frame. Without it the frame keeps the base's caption, `AUTENTICACIÓN`. The file's name
is never shown.

### `titulares`

```toml
titulares = '''
1983: Stallman anuncia el proyecto GNU
1991: Torvalds publica el kernel Linux
GNU no es Unix
{n} parches recibidos de la comunidad
'''
```

The ticker shows these and nothing else, in capitals, every line once before any repeats.
Blanks are filled each time the line comes round. Write facts from the video: this is the
surface that reads like news.

### `titulos`

```toml
titulos = '''
Las cuatro libertades
Proyecto GNU, 1983
Código fuente
'''
```

A pane's title bar shows the title first and bright, then ` · ` and the name of the process the
pane imitates, dim. The bright part ends at the first `·` of the line, so **a title should not
contain `·`**: whatever follows it is drawn dim, as if it were the process name. A title longer
than the pane is cut at its edge.

Outside set mode a pane takes the next title of a shuffled round each time it is titled (every
8 to 22 seconds). In set mode the titles are dealt once, in an order the seed shuffles, one per
pane; a pane beyond the list gets a generated name. Blanks are filled when a pane takes the
title.

### `letrero_verde`, `letrero_rojo`

```toml
letrero_verde = '''
SOFTWARE / LIBRE
GNU / NO ES UNIX / desde 1983
'''
letrero_rojo = '''
CODIGO / CERRADO
'''
```

One banner per line: its top line and its bottom line of big letters, split by ` / `, and
optionally a third part, the caption for the bottom edge of the frame. Both colors go into one
shuffled round. The rules are in [the banner](#the-banner).

### `personas`

```toml
personas = '''
Richard Stallman
Linus Torvalds
'''
```

People, written the way they should be read. They fill `{persona}`, they make the "de ..."
tails of generated sentences (`el parche de Richard Stallman`), and the last word of each, in
lower case and without accents, becomes a login name: `stallman`, for `{usuario}`, the login
attempts under the banner and the users of the process table. A last word with `ñ` makes no
login name.

People never stand beside an invented number: the radar's contacts and the sequence aligner's
subjects never come from `personas` (see [real names](#real-names)).

### `nombres`

```toml
nombres = '''
GNU
Linux
GPL
Emacs
Debian
'''
```

Bare proper names, used without an article: `de GNU`. They fill `{nombre}`, and in ASCII form
(lower case, accents stripped, spaces as `_`) they name things on the screen: identifiers
(`gnu_servidor`), files (`gnu.tar.gz`), daemons (`linuxd`), hosts, the processes of the logs
(`tail -f /var/log/linux.log`), the words falling in the rain, satellites (`EMACS-3`), the
symbols of the market (names of six letters or fewer) and the sniffer's domain, which always
ends in `.example`. A name with `ñ` is left out of all of these rather than misspelled.

The names of nine characters or fewer in that ASCII form, in capitals, are the radar's contacts
(`EMACS-417`) and the aligner's subjects; with fewer than eight, the base's call signs fill in.
A name that is also a word of one of the `personas` is never a contact.

A proper name that should be the subject of sentences goes in `cosas`, with its article:
`la GPL`, `la FSF`.

### `cosas`

```toml
cosas = '''
el kernel
la licencia
el código fuente
las cuatro libertades
la GPL
el área (f)
'''
```

Common nouns, **each with its article**: `el`, `la`, `los` or `las`, in any case (`La GPL` is
read as `la GPL`). The article gives the gender and the number, so the grammar can make
everything agree: "la licencia polimórfica fue liberada con éxito".

- `el` and `la` make a singular noun, which the grammar uses as the subject of sentences.
- `los` and `las` make a plural. The grammar cannot conjugate for it, so a plural is used
  only where nothing has to agree: in `{cosa}` and in "de ..." tails ("el servidor de las
  cuatro libertades"). A preset whose `cosas` are all plural leaves the subjects to the base.
- A feminine noun that takes `el` is marked `(f)` at the end: `el área (f)`, `el agua (f)`.
  Without the mark it is taken as masculine ("el área cifrado"). `(f)` after any other article
  is a warning.
- What is wrong: no article (`kernel`), any other article (`un parche`, `lo nuevo`), an
  article and no noun.
- `de` and `a` before `el` contract where a blank is filled: "Auditoría de {cosa}" with
  `el kernel` comes out "Auditoría del kernel". A line never says "de el".

### `adjetivos`

```toml
adjetivos = '''
libre
privativo
auditable
con copyleft
sin puertas traseras
protegid* por copyleft
'''
```

Adjectives in the masculine singular, or short phrases.

- **One word ending in `-o`** gets its feminine by itself: `privativo` is `privativa` beside a
  feminine noun.
- **Anything else** is used unchanged for both genders: words that do not end in `-o`
  (`libre`, `auditable`), and phrases of several words (`con copyleft`).
- **A phrase whose word has to agree** marks it with `*` in place of its last `o` or `a`:
  `protegid* por copyleft` is "protegido por copyleft" or "protegida por copyleft". The `*`
  goes after the stem: `seri*`, `continu*`, `europe*`. `protegido*` and `protegida*` are
  warnings (the `*` replaces the vowel, it does not follow it), and so is a `*` in the middle
  of a word.

What these rules get wrong, and what to write instead:

- Adjectives whose feminine adds `-a`: `español`, `trabajador`, `alemán`, `burlón`. They are
  used unchanged, so a feminine noun gets "la licencia español". Write a phrase that does not
  have to agree, such as `de España`, or leave them out.
- One word ending in `-o` that does not change: `retro` would become "retra". Write a phrase
  instead: `de estilo retro`.
- The loader cannot tell `europe*` (europeo, europea) from `libre*` ("libreo"), so a `*` on a
  word that does not change is accepted and makes nonsense. Put the `*` only on a word whose
  masculine ends in `o`.
- Plural adjectives come out wrong beside singular nouns. Write the singular.

### `verbos`

```toml
verbos = '''
liberar
compartir
distribuir
escribir
proveer | proveyendo | provisto
'''
```

Infinitives, one word each, in any case (they are read in lower case). The engine needs three
forms, the infinitive, the gerund and the participle, and derives the other two:

- `-ar`: always regular. `liberar`, `liberando`, `liberado`.
- `-er` and `-ir`: the regular rule (`compartir`, `compartiendo`, `compartido`); a stem that
  ends in a vowel takes `y` (`leer`, `leyendo`, `leído`; `distribuir`, `distribuyendo`,
  `distribuido`); and a built-in table of irregular families, matched by the end of the word
  so that compounds are covered: `-scribir` (`escrito`, `descrito`), `-poner` (`compuesto`),
  `-hacer` and `-facer` (`deshecho`, `satisfecho`), `-volver` and `-solver` (`devuelto`,
  `resuelto`), `-brir` (`abierto`, `cubierto`), `-decir` (`dicho`; but `bendecido`,
  `maldecido`), `ver`, `prever`, `entrever`, `rever`, `romper` (`corromper` is regular),
  `ir`, `poder`, `morir`, `dormir`, `pudrir`, `freír`, `-eír` (`reír`, `sonreír`), `-güir`,
  `-eñir`, and the stem-changers `-vertir`, `pedir`, `medir`, `-petir`, `seguir`, `servir`,
  `hervir`, `-legir`, `-regir`, `sentir`, `mentir`, `-ferir`, `-gerir`, `herir`, `-querir`,
  `venir`, `vestir`, `gemir`, `rendir`, `-cebir`, `derretir`, `erguir` and their families.
  `reir`, `freir` and `sonreir` written without the accent are read as `reír`, `freír` and
  `sonreír`.
- **Three forms written out override everything:** `proveer | proveyendo | provisto`. The
  gerund ends in `-ndo` and the participle in `-o`, masculine singular.

Write three forms where the derived ones are wrong or not the ones you want: `embestir`
would come out "embestiendo" (write `embestir | embistiendo | embestido`), `henchir`
"henchiendo"; `imprimir` gives "imprimido" and `proveer` "proveído", both correct, where you
may prefer `impreso` and `provisto`.

What is wrong: a reflexive (`liberarse`; write `liberar`), anything that is not one word ending
in `-ar`, `-er`, `-ir` or `-ír` (`liberado`, `poner en marcha`), and three forms that do not
have those endings.

### `frases`

```toml
frases = '''
Compilando {cosa} con GCC: {n} advertencias, 0 errores
{persona} revisa el parche {n} de {nombre}
Clonando el repositorio de {nombre}: {pct}% completado
Licencia verificada: GPL versión {d}
'''
```

Whole lines for the logs. They join the base's lines rather than replace them: a handful of
phrases alone would repeat all day, and the base's lines are filled with the preset's names
anyway. In set mode the log pane takes every third sentence from `frases`, in a shuffled round,
so every phrase comes by before any repeats; the other two are the engine's. An author who
wants the set's log closer to the script writes more `frases`.

## Blanks

In `titulares`, `titulos` and `frases`:

| blank | becomes |
|---|---|
| `{persona}` | one of `personas` |
| `{nombre}` | one of `nombres` |
| `{cosa}` | one of `cosas`, with its article |
| `{usuario}` | a login name made from `personas` (`stallman`) |
| `{n}` | a number, 2 to 9998 |
| `{d}` | a digit, 1 to 9 |
| `{pct}` | a number, 1 to 99 |
| `{hex}` | four hex digits |

A blank whose key the preset leaves out is filled from the neutral base (invented people,
neutral names and things). A line with an unknown blank (`{fecha}`), a `{` without its `}` or a
`}` without its `{` is dropped with a warning: a raw brace never reaches the screen.

A blank is filled as it is: `{cosa}` at the start of a title gives "el kernel ...", in lower
case. Start a title with a word of your own, or with `{persona}` or `{nombre}`.

## The banner

Each line of `letrero_verde` or `letrero_rojo` is one banner: `TOP / BOTTOM`, or
`TOP / BOTTOM / caption`.

- The line is split on ` / ` (a space, a slash, a space), at most three parts: a caption may
  contain a slash, and even ` / `. A slash without spaces is part of a line of letters, which
  cannot draw it (`GNU/LINUX / LIBRE` is a warning that names `/`).
- **The two lines of letters** are drawn in a small pixel font. They are upper-cased and their
  accents stripped (`código` is `CODIGO`, as on any sign in capitals), and `Ñ` is drawn as
  `Ñ`, never folded to `N`. What the font has: `A` to `Z`, `Ñ`, `0` to `9`, the space and
  `! ¡ ? ¿ : - . +`. Nothing else: a comma, an apostrophe, a slash or a letter outside Spanish
  drops the banner with a warning that names the character.
- Each line of letters is 12 characters at most, spaces included, and so no word is longer than
  12; 10 or fewer is the advice. A line may be several words.
- **The caption** is shown as written, on the bottom edge of the frame, when it fits there; 40
  characters at most. A banner without a caption has none. The top edge carries `tema`.
- Both lines are needed: `SOFTWARE LIBRE` alone, without ` / `, is a warning.

**How it is fitted.** The banner is fitted to its pane. A line of several words may wrap onto
several lines of letters, only between words (`NO ES UNIX` can become `NO ES` over `UNIX`); two
lines of the file are never joined, a word is never broken, and a banner is at most four lines
of letters high. The letters come in eight sizes, and the biggest that fits wins: at each size,
from the biggest down, every line is wrapped to the letters that fit across the pane, and if no
word is too long and the box is not taller than the pane, that is the banner.

| size | a letter takes (columns) | box height in rows for 2, 3, 4 lines of letters |
|---|---|---|
| x6 | 24 | 35, 53, 71 |
| x5 | 20 | 30, 45, 60 |
| x4 | 16 | 24, 36, 48 |
| x3 | 12 | 19, 28, 37 |
| x2 | 8 | 13, 19, 25 |
| narrow | 4 | 13, 19, 25 |
| compact | 4 | 8, 11, 14 |
| pocket | 2 | 8, 11, 14 |

A box is two columns wider than its longest line of letters, the frame included. Below pocket
the banner is one line of ordinary text, and in a pane under 3 rows or 10 columns there is
none. Some cases, by the inside of the pane:

| pane | `SOFTWARE / LIBRE` | `GNU / NO ES UNIX` | `CUATRO / LIBERTADES` |
|---|---|---|---|
| 66x19, the banner of a 67x60 screen in set mode | x2, two lines | x2, three lines | narrow, two lines |
| 119x28, the banner of 120x106 | x3, two lines | x3, three lines | x2, two lines |
| 131x44, the banner of 213x60 | x4, two lines | x4, three lines | x3, two lines |
| 73x21, the banner of 120x33 | x2, two lines | x2, three lines | narrow, two lines |
| 49x15, the banner of 80x24 | narrow, two lines | narrow, two lines | narrow, two lines |
| 33x9, a small pane outside set mode | pocket, two lines | pocket, two lines | pocket, two lines |

What follows from it:

- **A long word costs more than a long line.** `LIBERTADES` drops a size where `NO ES UNIX`
  wraps and keeps it.
- **A word of eight letters gets the wide x2 letters from 67 columns up**, and the narrow ones
  below. For a small tall screen (a monitor on its side with a big font, 60 to 67 columns),
  keep the words to seven letters or fewer.
- Short words get bigger letters on every screen.

## Real names

The grammar happily writes "advertencia: la licencia está alucinando" or "el servidor
hackeado de Richard Stallman". That is the genre, and in the small text it is texture; next to
a real person, in a video that may be serious, it is not something to leave to chance. So:

- **The readable surfaces are verbatim.** The ticker, the titles and the banner show only what
  the file says, when it says anything. Nothing generated shares them.
- **The texture is generated.** The logs, the login attempts, the labels and the "de ..."
  tails put the words of `personas`, `nombres` and `cosas` into sentences no one checked. A
  person in `personas` can appear in any of them, and as a login name (`stallman`) under the
  banner. If a person must not be put into generated sentences, leave them out of `personas`
  and name them only in `titulares` and `titulos` (as text, not as a blank).
- **The radar and the aligner never show a person.** They put a name beside an invented
  bearing, range, score or kinship, so their names come from `nombres` (of nine characters or
  fewer, and never a word of a person's name) or from the base's call signs.

## Writing a preset from a script

What to pull from a video's script, in the order of the file. Every key is optional; leave out
what the video does not have rather than inventing it.

1. **`tema`**: the topic in a few words, 24 characters at most.
2. **`titulares`**: ten to twenty headlines that state facts from the video, 60 characters or
   fewer each. Dates, names, numbers, quotations. Blanks are allowed but rarely needed.
3. **`titulos`**: six to ten titles, 28 characters or fewer, no `·`. Names of chapters,
   documents, projects, places in the story.
4. **`letrero_verde`** (and **`letrero_rojo`** if the story has an opposite): two to four
   banners, `TOP / BOTTOM`, words of seven letters or fewer where you can, only the characters
   the font has, an optional caption after a second ` / `.
5. **`personas`**: the people the video names, as they should be read. Read
   [real names](#real-names) first.
6. **`nombres`**: the projects, products, acronyms, organizations and places, bare. Short ones
   (nine characters or fewer) become the radar's contacts.
7. **`cosas`**: ten to twenty common nouns of the subject, each with `el`, `la`, `los` or
   `las`; mostly singular; `(f)` after `el` for a feminine noun such as `área`.
8. **`adjetivos`**: six to twelve, masculine singular; `*` in a phrase whose word agrees.
9. **`verbos`**: six to twelve infinitives the subject uses; three forms where the derived
   ones are wrong.
10. **`frases`**: six to twelve log lines in the voice of a system at work ("Compilando
    {cosa}...", "{persona} revisa..."), since the set's log shows them often. Start each with
    a capital or a blank that is a name.

Before handing it over, check every entry against these, which are the warnings people hit
most:

- every noun in `cosas` has its article;
- no `*` outside `adjetivos`, and in `adjetivos` it replaces a final `o` or `a`;
- no `{`, `}` outside `titulares`, `titulos` and `frases`, and only the eight blanks inside;
- every verb is one infinitive, not reflexive;
- every banner has ` / ` between its two lines, 12 characters or fewer per line, and only
  `A`-`Z`, `Ñ`, digits, spaces and `! ¡ ? ¿ : - . +` in them (no commas);
- no emoji, and every entry within its key's limit.

Then run the [checks](#checking-a-preset). A clean preset prints `ok`.

A small one, ready to load:

```toml
tema = "Software libre"

titulares = '''
1983: Stallman anuncia el proyecto GNU
1991: Torvalds publica el kernel Linux
GNU no es Unix
'''

titulos = '''
Las cuatro libertades
Código fuente
Licencia GPL v3
'''

letrero_verde = '''
SOFTWARE / LIBRE
GNU / NO ES UNIX
'''

nombres = '''
GNU
Linux
GPL
Emacs
'''

cosas = '''
el kernel
la licencia
el código fuente
el compilador
'''

frases = '''
Compilando {cosa} con GCC: {n} advertencias, 0 errores
Clonando el repositorio de {nombre}: {pct}% completado
'''
```

## Checking a preset

No command of its own: the usual ones take the preset through `-o`.

```sh
oxiper config check -o hollywood.preset=borrador.toml        # every error and warning
oxiper snap -s hollywood -o hollywood.preset=borrador.toml \
            -o set_mode=true --size 67x60 --at 20             # one frame, as text: on its side
oxiper snap -s hollywood -o hollywood.preset=borrador.toml \
            -o set_mode=true --size 120x33 --at 20            # and lying down
```

`config check` lists the preset's error or each of its warnings, prefixed with the path of the
config file and the preset's name, and exits with 1 if there is any:

```
~/.config/oxiper/config.toml: borrador: cosas[1]: no article: write el, la, los or las
~/.config/oxiper/config.toml: borrador: titulos[2]: unknown blank {x}
oxiper: 2 problem(s)
```

A clean preset prints `ok` and exits with 0. `snap` prints the warnings and draws the frame
anyway; add `--ansi` for color, and try other `--size`s and `--at` times. This is the loop to
run on a draft until `config check` is clean.

## Errors and warnings

**An error** means the preset cannot be used at all: the `preset` option is not a string; the
file is missing, cannot be read or is not a regular file; it is not UTF-8; it is over 256 KiB;
it is not valid TOML (the message gives the line). At start-up, and for `snap`, `record` and
`--saver`, oxiper does not start: one line on stderr says which file and why. While oxiper runs, a file that has
loaded once and is broken now keeps its last good preset on screen; a preset newly named in a
reloaded config that cannot load is refused like a bad theme, and the config in use stays. A
file that comes back empty while oxiper runs counts as a failed read ("the file is empty"),
because editors empty a file for a moment as they save it; emptying a preset on purpose needs a
restart.

**A warning** drops one entry and keeps the rest. Each names the key and the entry, counted
from 1 with blank and comment lines not counted (`titulares[4]: 104 characters, 100 at most`),
and fits one line of 60 characters. Warnings come key by key, the keys in alphabetical order.
At start-up they are printed before the screen is taken, with the config's own warnings; while
running, the HUD says how many. What warns:

- an unknown key, a key of the wrong type (`tema` that is not a string, a list that is a
  number), a `[table]`;
- an entry over its key's limit, or past the 400th of its list;
- a character that does not take exactly one cell;
- an unknown blank, a stray `{` or `}`, a brace outside the keys with blanks, a `*` outside
  `adjetivos` or in the wrong place there;
- a noun without its article, `(f)` after an article other than `el`, a verb that is not one
  infinitive, three forms that are not infinitive, gerund and participle;
- a banner without its two lines, with a character the font cannot draw, with a line or a
  word over 12 characters, or with a caption over 40.
