//! Swimmers that are not fish: jellyfish, cephalopods, seahorses, shrimps and
//! the other soft things of the sea, drawn in box drawing, blocks and braille.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "moon-jellyfish",
        &[
            r"
  ▂▄▆▆▆▄▂
▗▟▓▒░░░▒▓▙▖
▐▒░░○○○░░▒▌
▝▀▀▀▀▀▀▀▀▀▘
  ╎ ╏ ╎ ╏
 ╏ ╎ ╏ ╎ ╏
",
            r"
   ▃▅▇▅▃
  ▟▓▒░▒▓▙
  ▐▒○○○▒▌
  ▝▀▀▀▀▀▘
   ╏ ╎ ╏
  ╎ ╏ ╎ ╏
",
            r"
  ▂▄▆▆▆▄▂
▗▟▓▒░░░▒▓▙▖
▐▒░░○○○░░▒▌
▝▀▀▀▀▀▀▀▀▀▘
 ╏ ╎ ╏ ╎ ╏
  ╎ ╏ ╎ ╏
",
        ],
        &[
            r"
  1111111
11111111111
1111WWW1111
11111111111
  1 1 1 1
 1 1 1 1 1
",
            r"
   11111
  1111111
  11WWW11
  1111111
   1 1 1
  1 1 1 1
",
            r"
  1111111
11111111111
1111WWW1111
11111111111
 1 1 1 1 1
  1 1 1 1
",
        ],
    )
    .critter()
    .traits(STING)
    .tint("1:cCwWmM")
    .color('c')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.04, 0.15)
    .fps(0.15)
    .weight(8)
    .biomes(OCEAN | REEF | KELP | ARCTIC),
    Species::new(
        "lions-mane-jellyfish",
        &[
            r"
    ▂▃▅▆▆▆▆▆▅▃▂
  ▗▟▓▒░░░░░░░▒▓▙▖
 ▟▓▒░░░░░░░░░░░▒▓▙
▐▓▒▒░░░░░░░░░░░▒▒▓▌
▝▀▜▄▄▄▄▄▄▄▄▄▄▄▄▄▛▀▘
 ⡇⢸⠸⡇⢸⠸⡇⢸⠸⡇⢸⠸⡇⢸
⢸⡇⠸⢸⡇⠸⢸⡇⠸⢸⡇⠸⢸⡇⠸
 ⠸⡇⢸⠸⡇⢸⠸⡇⢸⠸⡇⢸⠸⡇
  ⢸⡇ ⠸⢸ ⡇⢸ ⠸⡇ ⢸
   ⠸⡇  ⢸  ⡇  ⠸
",
            r"
    ▂▃▅▆▆▆▆▆▅▃▂
  ▗▟▓▒░░░░░░░▒▓▙▖
 ▟▓▒░░░░░░░░░░░▒▓▙
▐▓▒▒░░░░░░░░░░░▒▒▓▌
▝▀▜▄▄▄▄▄▄▄▄▄▄▄▄▄▛▀▘
 ⢸⡇⠸⢸⡇⠸⢸⡇⠸⢸⡇⠸⢸⡇
⡇⢸⠸⡇⢸⠸⡇⢸⠸⡇⢸⠸⡇⢸⠸
 ⢸⡇⠸⢸⡇⠸⢸⡇⠸⢸⡇⠸⢸
   ⡇⢸ ⠸⡇ ⢸⠸ ⡇⢸
  ⠸⡇  ⢸⡇  ⠸  ⡇
",
        ],
        &[
            r"
    11111111111
  111111111111111
 11111111111111111
1111111111111111111
1111111111111111111
 22222222222222
222222222222222
 22222222222222
  22 22 22 22 2
   22  2  2  2
",
            r"
    11111111111
  111111111111111
 11111111111111111
1111111111111111111
1111111111111111111
 22222222222222
222222222222222
 2222222222222
   22 22 22 22
  22  22  2  2
",
        ],
    )
    .critter()
    .traits(STING)
    .tint("1:rRyYmM 2:yYrRmM")
    .color('y')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.03, 0.1)
    .fps(0.12)
    .weight(4)
    .biomes(OCEAN | KELP | ARCTIC),
    Species::new(
        "box-jellyfish",
        &[
            r"
 ▄▄▄▄▄▄▄
▐▒░░░░░▒▌
▐░◉░░░◉░▌
▝▀▜▄▄▄▛▀▘
 ╎ ╎ ╎ ╎
 ╏ ╏ ╏ ╏
 ╎ ╎ ╎ ╎
 ┆ ┆ ┆ ┆
",
            r"
 ▄▄▄▄▄▄▄
▐▒░░░░░▒▌
▐░◉░░░◉░▌
▝▀▜▄▄▄▛▀▘
  ╏ ╏ ╏ ╏
  ╎ ╎ ╎ ╎
  ┆ ┆ ┆ ┆
  ╎ ╎ ╎
",
        ],
        &[
            r"
 1111111
111111111
11W111W11
111111111
 1 1 1 1
 1 1 1 1
 1 1 1 1
 1 1 1 1
",
            r"
 1111111
111111111
11W111W11
111111111
  1 1 1 1
  1 1 1 1
  1 1 1 1
  1 1 1
",
        ],
    )
    .critter()
    .traits(STING)
    .tint("1:wWcC")
    .color('c')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.05, 0.18)
    .fps(0.2)
    .weight(4)
    .biomes(REEF | OCEAN),
    Species::new(
        "portuguese-man-o-war",
        &[
            r"
       ▗▄▄▖
  ▗▄▄▟██▓▓██▙▄▄▖
▗▟▓▒▒░░░░░░░░▒▒▓▙▖
▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
   ╎ ╏ ╎ ╏ ╎ ╏
    ╏ ╎ ╏ ╎ ╏
     ╎ ╏ ╎ ╏
      ╏ ╎ ╏
       ╎ ╏
",
            r"
       ▗▄▄▖
  ▗▄▄▟██▓▓██▙▄▄▖
▗▟▓▒▒░░░░░░░░▒▒▓▙▖
▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
   ╏ ╎ ╏ ╎ ╏ ╎
    ╎ ╏ ╎ ╏ ╎
     ╏ ╎ ╏ ╎
      ╎ ╏ ╎
       ╏ ╎
",
        ],
        &[r"
       1111
  11111111111111
111111111111111111
111111111111111111
   2 2 2 2 2 2
    2 2 2 2 2
     2 2 2 2
      2 2 2
       2 2
"],
    )
    .critter()
    .traits(STING)
    .tint("1:bBmMcC 2:bBcB")
    .color('b')
    .swim(Swim::Drift)
    .zone(Zone::Top)
    .speed(0.04, 0.12)
    .fps(0.15)
    .weight(3)
    .biomes(OCEAN | REEF),
    Species::new(
        "comb-jelly",
        &[
            r"
 ▗▄▆▆▄▖
▟▛┇╏┇╏▜▙
█▌╏┇╏┇▐█
▜▙┇╏┇╏▟▛
 ▝▀▜▛▀▘
  ╎  ╎
  ╏  ╏
   ╎ ╏
",
            r"
 ▗▄▆▆▄▖
▟▛╏┇╏┇▜▙
█▌┇╏┇╏▐█
▜▙╏┇╏┇▟▛
 ▝▀▜▛▀▘
  ╏  ╏
  ╎  ╎
   ╏ ╎
",
        ],
        &[r"
 wwwwww
wwRYGCww
wwRYGCww
wwRYGCww
 wwwwww
  c  c
  c  c
   c c
"],
    )
    .critter()
    .traits(GLOW)
    .color('w')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.04, 0.12)
    .fps(0.3)
    .weight(6)
    .biomes(DEEP | OCEAN),
    Species::new(
        "atolla-jellyfish",
        &[
            r"
   ▂▄▆▆▆▆▆▄▂
 ▗▟▓▒▒▒▒▒▒▒▓▙▖
▐▓▒░░◦░◦░◦░░▒▓▌
▝▀▀▜▄▄▄▄▄▄▄▛▀▀▘
  ⡇⢸ ⠸⡇ ⢸⠸⡇
   ⢸  ⡇ ⠸ ⢸
       ╎
       ╏
",
            r"
   ▂▄▆▆▆▆▆▄▂
 ▗▟▓▒▒▒▒▒▒▒▓▙▖
▐▓▒░░◦░◦░◦░░▒▓▌
▝▀▀▜▄▄▄▄▄▄▄▛▀▀▘
  ⢸⡇ ⡇⢸ ⠸⡇⢸
   ⡇  ⢸ ⡇ ⠸
       ╏
       ╎
",
        ],
        &[r"
   111111111
 1111111111111
11111R1R1R11111
111111111111111
  11 11 111
   1  1 1 1
       1
       1
"],
    )
    .critter()
    .traits(GLOW)
    .tint("1:rRmM")
    .color('r')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.04, 0.12)
    .fps(0.15)
    .weight(5)
    .biomes(DEEP),
    Species::new(
        "upside-down-jellyfish",
        &[
            r"
≀⌇≀⌇≀⌇≀⌇≀⌇≀⌇≀⌇≀
 ⌇≀⌇≀⌇≀⌇≀⌇≀⌇≀⌇≀
  ▝▜▓▒▒▒▒▒▒▒▒▓▛▘
",
            r"
⌇≀⌇≀⌇≀⌇≀⌇≀⌇≀⌇≀⌇
 ≀⌇≀⌇≀⌇≀⌇≀⌇≀⌇≀⌇
  ▝▜▓▒▒▒▒▒▒▒▒▓▛▘
",
        ],
        &[r"
111111111111111
 11111111111111
  11111111111111
"],
    )
    .critter()
    .traits(STING)
    .tint("1:gGcCyY")
    .color('g')
    .swim(Swim::Drift)
    .zone(Zone::Lower)
    .speed(0.03, 0.09)
    .fps(0.12)
    .weight(4)
    .biomes(REEF | TANK | FRESH),
    Species::new(
        "fried-egg-jellyfish",
        &[
            r"
   ▂▄▆▆▆▆▆▄▂
▗▟▓▒░▗▄▄▄▖░▒▓▙▖
▐▓▒░░▐███▌░░▒▓▌
▝▜▀▀▀▀▝▀▘▀▀▀▀▛▘
  ╎╏╎╏╎╏╎╏╎╏
  ╏╎ ╏╎╏╎ ╎╏
   ╎╏ ╎╏╎╏ ╎
",
            r"
   ▂▄▆▆▆▆▆▄▂
▗▟▓▒░▗▄▄▄▖░▒▓▙▖
▐▓▒░░▐███▌░░▒▓▌
▝▜▀▀▀▀▝▀▘▀▀▀▀▛▘
  ╏╎╏╎╏╎╏╎╏╎
  ╎╏ ╎╏╎╏ ╏╎
   ╏╎ ╏╎╏╎ ╏
",
        ],
        &[r"
   111111111
11111YYYYY11111
11111YYYYY11111
111111YYY111111
  2222222222
  22 2222 22
   22 2222 2
"],
    )
    .critter()
    .traits(STING)
    .tint("1:wWcC")
    .color('w')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.04, 0.14)
    .fps(0.15)
    .weight(5)
    .biomes(OCEAN | REEF),
    Species::new(
        "siphonophore",
        &[
            r"
  ╭─╮    ╭─╮    ╭─╮    ▗▄▄▖
⠒⠒◖●◗⠒⠒⠒⠒◖●◗⠒⠒⠒⠒◖●◗⠒⠒⠒⠒▐◉◉▌
  ╰─╯    ╰─╯    ╰─╯    ▝▀▀▘
  ╎ ╎    ╎ ╎    ╎ ╎
   ╏      ╏      ╏
",
            r"
  ╭─╮    ╭─╮    ╭─╮    ▗▄▄▖
⠒⠒◖●◗⠒⠒⠒⠒◖●◗⠒⠒⠒⠒◖●◗⠒⠒⠒⠒▐◉◉▌
  ╰─╯    ╰─╯    ╰─╯    ▝▀▀▘
   ╎ ╎    ╎ ╎    ╎ ╎
  ╏      ╏      ╏
",
        ],
        &[
            r"
  111    111    111    1111
111W111111W111111W111111WW1
  111    111    111    1111
  1 1    1 1    1 1
   1      1      1
",
            r"
  111    111    111    1111
111W111111W111111W111111WW1
  111    111    111    1111
   1 1    1 1    1 1
  1      1      1
",
        ],
    )
    .critter()
    .traits(GLOW | STING)
    .tint("1:cCmMyY")
    .color('c')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.04, 0.1)
    .fps(0.2)
    .weight(3)
    .biomes(DEEP),
    Species::new(
        "salp-chain",
        &[
            r"
▗▄▄▄▄▖▗▄▄▄▄▖▗▄▄▄▄▖▗▄▄▄▄▖▗▄▄▄▄▖
◖░●░░◗◖░●░░◗◖░●░░◗◖░●░░◗◖░●░░◗
▝▀▀▀▀▘▝▀▀▀▀▘▝▀▀▀▀▘▝▀▀▀▀▘▝▀▀▀▀▘
",
            r"
▗▄▄▄▄▖▗▄▄▄▄▖▗▄▄▄▄▖▗▄▄▄▄▖▗▄▄▄▄▖
◖░░●░◗◖░░●░◗◖░░●░◗◖░░●░◗◖░░●░◗
▝▀▀▀▀▘▝▀▀▀▀▘▝▀▀▀▀▘▝▀▀▀▀▘▝▀▀▀▀▘
",
        ],
        &[
            r"
111111111111111111111111111111
11W11111W11111W11111W11111W111
111111111111111111111111111111
",
            r"
111111111111111111111111111111
111W11111W11111W11111W11111W11
111111111111111111111111111111
",
        ],
    )
    .critter()
    .traits(FILTER)
    .tint("1:cCwW")
    .color('c')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.08, 0.25)
    .fps(0.25)
    .weight(4)
    .biomes(OCEAN | DEEP),
    Species::new(
        "common-octopus",
        &[
            r"
   ▗▄▄▄▄▄▖
 ▗▟▓▓▒▒▒▒▓▙▖
▟▓▒▒░░░░░▒▓▓▙
█▒░░░░░░░◉░▒█▌
▜▙▒░░░▄▄▄▄▄▟▛▘
 ▝▀▜▙▟▛▀▀▀▘
 ╭╯╰╮╭╯╰╮╭╮
╭╯╭╯╰╮╭╯╰╮╰╮
╰╮╰╮ ╰╯ ╭╯╭╯
",
            r"
   ▗▄▄▄▄▄▖
 ▗▟▓▓▒▒▒▒▓▙▖
▟▓▒▒░░░░░▒▓▓▙
█▒░░░░░░░◉░▒█▌
▜▙▒░░░▄▄▄▄▄▟▛▘
 ▝▀▜▙▟▛▀▀▀▘
  ╭╯╰╮╭╯╰╮
 ╭╯╭╯╰╮╭╯╰╮
  ╰╮╰╮╭╯╭╯
",
        ],
        &[
            r"
   1111111
 11111111111
1111111111111
11111111141111
11111111111111
 1111111111
 1111111111
111111111111
1111 11 1111
",
            r"
   1111111
 11111111111
1111111111111
11111111141111
11111111111111
 1111111111
  11111111
 1111111111
  11111111
",
        ],
    )
    .critter()
    .traits(PREDATOR | INK | CURIOUS | SHY | NOCTURNAL)
    .home("rock,cave,amphora,bottle,pot")
    .tint("1:rRyYmM")
    .color('r')
    .swim(Swim::Dart)
    .zone(Zone::Any)
    .speed(0.2, 1.1)
    .fps(0.3)
    .weight(6)
    .biomes(REEF | OCEAN | KELP | TANK),
    Species::new(
        "blue-ringed-octopus",
        &[
            r"
  ▗▄▄▄▖
▗▟▒○▒○▒▙▖
▐▒○▒▒▒▒◉▌
▝▜▄▒▒▒▄▛▘
╭╯╰╮╭╯╰╮
╰╮╭╯╰╮╭╯
",
            r"
  ▗▄▄▄▖
▗▟▒○▒○▒▙▖
▐▒○▒▒▒▒◉▌
▝▜▄▒▒▒▄▛▘
 ╭╯╰╮╭╯╰╮
 ╰╮╭╯╰╮╭╯
",
        ],
        &[r"

   B B
  B    W



"],
    )
    .critter()
    .traits(DEFENDED | SHY | NOCTURNAL)
    .home("rock,cave,amphora,bottle,pot")
    .color('y')
    .swim(Swim::Hover)
    .zone(Zone::Any)
    .speed(0.1, 0.4)
    .fps(0.2)
    .weight(3)
    .biomes(REEF | TANK),
    Species::new(
        "dumbo-octopus",
        &[
            r"
      ▗▄▄▄▖
  ▗▄▖▟▓▒▒▒▓▙▗▄▖
 ▟▒░▒▐▒◉░◉▒▌▒░▒▙
 ▝▀▀▜▙▄▄▄▄▄▟▛▀▀▘
    ╭╯╭╯╰╮╰╮
    ╰╮╰╮╭╯╭╯
",
            r"
  ▗▄▖ ▗▄▄▄▖ ▗▄▖
 ▟▒░▒▟▓▒▒▒▓▙▒░▒▙
 ▝▀▜▙▐▒◉░◉▒▌▟▛▀▘
     ▝▜▄▄▄▛▘
    ╭╯╭╯╰╮╰╮
    ╰╮╰╮╭╯╭╯
",
            r"
      ▗▄▄▄▖
     ▟▓▒▒▒▓▙
▗▄▄▖ ▐▒◉░◉▒▌ ▗▄▄▖
▟▒░▒▙▝▜▄▄▄▛▘▟▒░▒▙
    ╭╯╭╯╰╮╰╮
    ╰╮╰╮╭╯╭╯
",
            r"
      ▗▄▄▄▖
  ▗▄▖▟▓▒▒▒▓▙▗▄▖
 ▟▒░▒▐▒◉░◉▒▌▒░▒▙
 ▝▀▀▜▙▄▄▄▄▄▟▛▀▀▘
    ╭╯╭╯╰╮╰╮
    ╰╮╰╮╭╯╭╯
",
        ],
        &[
            r"
      11111
  1111111111111
 111111414111111
 111111111111111
    11111111
    11111111
",
            r"
  111 11111 111
 111111111111111
 111111414111111
     1111111
    11111111
    11111111
",
            r"
      11111
     1111111
1111 1141411 1111
11111111111111111
    11111111
    11111111
",
            r"
      11111
  1111111111111
 111111414111111
 111111111111111
    11111111
    11111111
",
        ],
    )
    .critter()
    .traits(SHY | CURIOUS)
    .home("rock,cave,pot")
    .tint("1:mMrRwW 4:wW")
    .color('m')
    .swim(Swim::Hover)
    .zone(Zone::Lower)
    .speed(0.06, 0.25)
    .fps(0.3)
    .weight(5)
    .biomes(DEEP),
    Species::new(
        "vampire-squid",
        &[
            r"
     ▗▄▄▄▖
   ▗▟▓▒▒▒▓▙▖
 ▗▟▓▒▒▒▒▒▒▒▒▙▖
▐▓▒▒▒▒▒◉▒▒▒▒▒▒▓▙
▝▜▀▀▜▙▄▄▄▄▟▛▀▀▀▘
  ╰╮╰╮╰╮╰╮╭╯
",
            r"
     ▗▄▄▄▖
   ▗▟▓▒▒▒▓▙▖
 ▗▟▓▒▒▒▒▒▒▒▒▙▖
▐▓▒▒▒▒▒◉▒▒▒▒▒▒▓▙
▝▜▀▀▜▙▄▄▄▄▟▛▀▀▀▘
   ╰╮╰╮╰╮╭╯╭╯
",
        ],
        &[
            r"
     11111
   111111111
 1111111111111
1111111B11111111
1111111111111111
  1111111111
",
            r"
     11111
   111111111
 1111111111111
1111111B11111111
1111111111111111
   1111111111
",
        ],
    )
    .critter()
    .traits(GLOW | SHY | NOCTURNAL)
    .tint("1:rRkmM")
    .color('r')
    .swim(Swim::Hover)
    .zone(Zone::Lower)
    .speed(0.06, 0.2)
    .fps(0.2)
    .weight(3)
    .biomes(DEEP),
    Species::new(
        "common-squid",
        &[
            r"
 ◢▄▄▄▄▄▄▄▖
◀██▓▒░░◉▐═╾╌╌▸
 ◥▀▀▀▀▀▀▀▘
",
            r"
 ◢▄▄▄▄▄▄▄▖
◀██▓▒░░◉▐═╾━╌▸
 ◥▀▀▀▀▀▀▀▘
",
            r"
 ◢▄▄▄▄▄▄▄▖
◀██▓▒░░◉▐╾╌╌╌╌▸
 ◥▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
 111111111
1111111k122222
 111111111
",
            r"
 111111111
1111111k122222
 111111111
",
            r"
 111111111
1111111k1222222
 111111111
",
        ],
    )
    .critter()
    .traits(PREDATOR | INK | NOCTURNAL)
    .tint("1:wWmMcC 2:wmyr")
    .color('w')
    .swim(Swim::Dart)
    .zone(Zone::Any)
    .speed(0.4, 1.8)
    .fps(0.35)
    .school(3, 7)
    .weight(6)
    .biomes(OCEAN | KELP | REEF),
    Species::new(
        "firefly-squid",
        &[
            r"
 ▗▄▄▄▖
◀▐⣿⣶◉▌═╾▸
 ▝▀▀▀▘
",
            r"
 ▗▄▄▄▖
◀▐⣶⣿◉▌╾╌▸
 ▝▀▀▀▘
",
        ],
        &[r"

    W

"],
    )
    .critter()
    .traits(GLOW | NOCTURNAL)
    .color('B')
    .swim(Swim::Dart)
    .zone(Zone::Any)
    .speed(0.3, 1.2)
    .fps(0.3)
    .school(4, 9)
    .weight(4)
    .biomes(DEEP | OCEAN),
    Species::new(
        "cuttlefish",
        &[
            r"
   ▗▄▄▄▄▄▄▄▖
 ▗▟▓▒▒░▒▒░▒▓▙▖
◖▐▒░▒░▒░▒░▒▓█◉▌═╾╌╾
 ▝▜▄▄▄▄▄▄▄▄▄▟▛▘╾╌
   ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
   ▗▄▄▄▄▄▄▄▖
 ▗▟▓▒░▒▒░▒▒▓▙▖
◖▐▒▒░▒░▒░▒░▓█◉▌═╾╌╾
 ▝▜▄▄▄▄▄▄▄▄▄▟▛▘╌╾
   ▝▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
   111111111
 1111111111111
1111111111111412222
 1111111111111122
   11111111111
"],
    )
    .critter()
    .traits(PREDATOR | INK | CURIOUS)
    .home("rock,cave,amphora")
    .tint("1:yYmMwr 2:yYwW 4:WG")
    .color('y')
    .swim(Swim::Hover)
    .zone(Zone::Any)
    .speed(0.1, 0.5)
    .fps(0.3)
    .weight(5)
    .biomes(REEF | OCEAN | KELP | TANK),
    Species::new(
        "chambered-nautilus",
        &[
            r"
   ▃▅▆██▆▅▃
 ▗▟▓▒░░░░▒▓▙▖
▟▓▒░╭────╮░▒▓▙
▐█▒░│╭──╮│░▒█▙▄▄
▐█▓▒│╰──╯│░▒██◉▌╾╌╾
▝▜▙▒╰────╯▒▟▛▀▀╌╾
  ▝▀▜▄▄▄▄▛▀▘
",
            r"
   ▃▅▆██▆▅▃
 ▗▟▓▒░░░░▒▓▙▖
▟▓▒░╭────╮░▒▓▙
▐█▒░│╭──╮│░▒█▙▄▄
▐█▓▒│╰──╯│░▒██◉▌╾╌╾
▝▜▙▒╰────╯▒▟▛▀▀╾╌
  ▝▀▜▄▄▄▄▛▀▘
",
        ],
        &[r"
   11111111
 111111111111
11111111111111
1111111111111122
11111111111111W2222
11111111111111222
  1111111111
"],
    )
    .critter()
    .traits(DEFENDED | NOCTURNAL | SCAVENGER)
    .tint("1:yYwW 2:wWyY")
    .color('y')
    .swim(Swim::Hover)
    .zone(Zone::Any)
    .speed(0.08, 0.3)
    .fps(0.2)
    .weight(4)
    .biomes(REEF | OCEAN | DEEP),
    Species::new(
        "seahorse",
        &[
            r"
   ▗▄▖
  ▟▓◉▙▄▄▸
  ▜▓▓▛▀▘
 ░▝▜▓▌
⠇░▐▒▒▙▖
 ░▐▒▒▒▌
  ▐▒▒▛
  ▝▜▓▌
  ╭╯▐▌
  ╰─╯
",
            r"
   ▗▄▖
  ▟▓◉▙▄▄▸
  ▜▓▓▛▀▘
 ▒▝▜▓▌
⠸▒▐▒▒▙▖
 ▒▐▒▒▒▌
  ▐▒▒▛
  ▝▜▓▌
  ╭╯▐▌
  ╰─╯
",
            r"
   ▗▄▖
  ▟▓◉▙▄▄▸
  ▜▓▓▛▀▘
 ░▝▜▓▌
⠇▒▐▒▒▙▖
 ░▐▒▒▒▌
  ▐▒▒▛
  ▝▜▓▌
  ╭╯▐▌
  ╰─╯
",
        ],
        &[r"
   111
  1141111
  111111
 21111
2211111
 211111
  1111
  1111
  1111
  111
"],
    )
    .critter()
    .traits(SHY)
    .home("grass,eelgrass")
    .tint("1:yYrRmM 4:wW")
    .color('y')
    .swim(Swim::Hover)
    .zone(Zone::Any)
    .speed(0.05, 0.2)
    .fps(0.25)
    .school(1, 2)
    .weight(7)
    .biomes(REEF | KELP | TANK),
    Species::new(
        "leafy-sea-dragon",
        &[
            r"
    ≀   ⌇≀  ⌇    ≀
 ≀⌇ ╱╲≀⌇╱╲≀⌇╱╲ ⌇≀
◖▒▓▒▓▓▒▓▒▓▓▒▓▒◉▙▄▄▬▸
 ≀⌇ ╲╱≀⌇╲╱≀⌇╲╱ ⌇≀
    ⌇   ≀⌇  ≀    ⌇
",
            r"
   ⌇   ≀⌇  ≀    ⌇
 ⌇≀ ╲╱⌇≀╲╱⌇≀╲╱ ≀⌇
◖▒▓▒▓▓▒▓▒▓▓▒▓▒◉▙▄▄▬▸
 ⌇≀ ╱╲⌇≀╱╲⌇≀╱╲ ≀⌇
    ≀   ⌇≀  ⌇    ≀
",
        ],
        &[
            r"
    2   22  2    2
 22 2222222222 22
11111111111111411111
 22 2222222222 22
    2   22  2    2
",
            r"
   2   22  2    2
 22 2222222222 22
11111111111111411111
 22 2222222222 22
    2   22  2    2
",
        ],
    )
    .critter()
    .traits(SHY)
    .home("kelp,wrack")
    .tint("1:yYgG 2:gGyY 4:wW")
    .color('g')
    .swim(Swim::Hover)
    .zone(Zone::Any)
    .speed(0.04, 0.15)
    .fps(0.2)
    .weight(3)
    .biomes(KELP | REEF),
    Species::new(
        "weedy-sea-dragon",
        &[
            r"
    ≀⌇  ⌇≀  ⌇
 ≀⌇╱╲▒▓▓▒▓▒▓╱╲≀ ⌇
◖▒▓▒▓▓▒▓▒▓▓▒▓◉▙▄▄▬▸
 ⌇≀╲╱▒▓▒▓▓▒▓╲╱⌇ ≀
",
            r"
   ⌇≀  ≀⌇  ≀
 ⌇≀╲╱▒▓▓▒▓▒▓╲╱⌇ ≀
◖▒▓▒▓▓▒▓▒▓▓▒▓◉▙▄▄▬▸
 ≀⌇╱╲▒▓▒▓▓▒▓╱╲≀ ⌇
",
        ],
        &[
            r"
    22  22  2
 22221111111222 2
1111111111111411111
 22221111111222 2
",
            r"
   22  22  2
 22221111111222 2
1111111111111411111
 22221111111222 2
",
        ],
    )
    .critter()
    .traits(SHY)
    .home("kelp,wrack")
    .tint("1:yYrR 2:yYgG 4:wW")
    .color('y')
    .swim(Swim::Hover)
    .zone(Zone::Any)
    .speed(0.04, 0.15)
    .fps(0.2)
    .weight(3)
    .biomes(KELP),
    Species::new(
        "cleaner-shrimp",
        &[
            r"
      ▗▄▄▄▖  ╱
 ▗▄▄▟▒▒▒▒◉▌╱╌╌
╭▐▒▓▒▒▒▒▒▙▘
 ▝▀╯╱╱ ╲╲
",
            r"
      ▗▄▄▄▖  ╱
 ▗▄▄▟▒▒▒▒◉▌╱╌╌
╭▐▒▓▒▒▒▒▒▙▘
 ▝▀╯╲╲ ╱╱
",
        ],
        &[r"
      11111  W
 1111111141WWW
11111111111
 111WW WW
"],
    )
    .critter()
    .traits(CLEANER | SHY)
    .home("anemone,coral")
    .tint("1:wWrR 4:kW")
    .color('w')
    .swim(Swim::Dart)
    .zone(Zone::Lower)
    .speed(0.1, 0.7)
    .fps(0.4)
    .weight(6)
    .biomes(REEF | TANK),
    Species::new(
        "krill-swarm",
        &[
            r"
≈▒●▸
",
            r"
∼▒●▸
",
        ],
        &[r"
1141
"],
    )
    .critter()
    .traits(FILTER)
    .tint("1:rRmwy")
    .color('r')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.3, 0.9)
    .fps(0.4)
    .school(6, 12)
    .weight(6)
    .biomes(OCEAN | ARCTIC | KELP),
    Species::new(
        "sea-angel",
        &[
            r"
  ╲│╱
 ▗▄▟▒▙▄▖
◖▒▓▒●▒▓▒▌▸
 ▝▀▀▀▀▀▀▘
",
            r"
 ▗▄▄▄▄▄▄▖
◖▒▓▒●▒▓▒▌▸
 ▝▄▜▒▛▄▀▘
  ╱│╲
",
        ],
        &[
            r"
  www
 wwwwwww
wwwwRwwwwW
 wwwwwwww
",
            r"
 wwwwwwww
wwwwRwwwwW
 wwwwwwww
  www
",
        ],
    )
    .critter()
    .traits(PREDATOR)
    .color('w')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.05, 0.2)
    .fps(0.35)
    .weight(5)
    .biomes(ARCTIC | OCEAN),
    Species::new(
        "sea-butterfly",
        &[
            r"
◢◣
◖●▸
",
            r"
◖●▸
◥◤
",
        ],
        &[
            r"
11
1W1
",
            r"
1W1
11
",
        ],
    )
    .critter()
    .traits(FILTER)
    .tint("1:cCmMbB")
    .color('c')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.06, 0.25)
    .fps(0.45)
    .school(4, 9)
    .weight(5)
    .biomes(ARCTIC | OCEAN | KELP),
    Species::new(
        "spanish-dancer",
        &[
            r"
   ▂▄▆▆▆▆▄▂
 ╭≋▓▒░▒░▒▓≋╮ ╱
◖≋▒░▒▓▒▓▒░▒≋◉◗
 ╰≋▓▒░▒░▒▓≋╯
   ▔▀▀▀▀▀▔
",
            r"
   ▂▄▆▆▆▆▄▂
 ╭∼▓▒░▒░▒▓∼╮ ╱
◖∼▒░▒▓▒▓▒░▒∼◉◗
 ╰∼▓▒░▒░▒▓∼╯
   ▔▀▀▀▀▀▔
",
        ],
        &[r"
   11111111
 1Y1111111Y1 Y
1Y111111111YW1
 1Y1111111Y1
   Y11111Y
"],
    )
    .critter()
    .traits(DEFENDED | NOCTURNAL)
    .tint("1:rRmM")
    .color('r')
    .swim(Swim::Wave)
    .zone(Zone::Lower)
    .speed(0.1, 0.4)
    .fps(0.35)
    .weight(3)
    .biomes(REEF | TANK),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_species_drawn(super::SPECIES);
    }
}
