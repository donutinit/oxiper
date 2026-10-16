//! Seaweed, kelp, grasses, anemones, corals and sponges, continued.

use super::*;

pub const PLANTS: &[Plant] = &[
    Plant::sprite(
        "sea-palm",
        &[
            r"
 ╲╲ ╲│╱ ╱╱
  ╲╲╲│╱╱╱
    ╲│╱
     ┃
     ┃
    ▟┃▙
",
            r"
 ╱╱ ╲│╱ ╲╲
  ╱╱╱│╲╲╲
    ╲│╱
     ┃
     ┃
    ▟┃▙
",
        ],
        &[r"
 yy yyy yy
  yyyyyyy
    yyy
     y
     y
    kyk
"],
    )
    .color('y')
    .fps(0.25)
    .weight(5)
    .biomes(KELP)
    .traits(SHELTER),
    Plant::sprite(
        "surfgrass",
        &[
            r"
 │ │ │ │ │ │
 │╲│╱│╲│╱│╲│
 ┃┃ ┃┃ ┃┃ ┃┃
",
            r"
 │ │ │ │ │ │
 │╱│╲│╱│╲│╱│
 ┃┃ ┃┃ ┃┃ ┃┃
",
            r"
 ╲ │ ╱ │ ╲ │
 │╱│╲│╱│╲│╱│
 ┃┃ ┃┃ ┃┃ ┃┃
",
        ],
        &[],
    )
    .color('G')
    .fps(0.3)
    .weight(6)
    .biomes(KELP | OCEAN)
    .traits(SHELTER),
    Plant::sprite(
        "coralline-algae",
        &[
            r"
  ▗▟▓▙▖
 ▟▓█▓█▓▙
 ▜█▓█▓█▛
",
            r"
  ▗▟▒▙▖
 ▟▒█▒█▒▙
 ▜█▒█▒█▛
",
        ],
        &[r"
  11111
 1111111
 1111111
"],
    )
    .tint("1:mMrR")
    .color('m')
    .fps(0.25)
    .weight(5)
    .biomes(KELP | ARCTIC),
    Plant::sprite(
        "sea-lettuce",
        &[
            r"
  ╭∼∼╮
 ╱∼∼∼∼╲∼╮
 ╲∼∼╱∼∼∼∼╲
  ╲╱∼∼∼∼∼╱
   │╲∼∼╯
   │
",
            r"
   ╭∼∼╮
  ╱∼∼∼∼╲ ╮
 ╭╲∼∼╱∼∼∼╲
 ╲∼∼∼∼∼╱╱
   ╰∼∼╯│
     │
",
        ],
        &[],
    )
    .color('G')
    .fps(0.3)
    .weight(5)
    .biomes(KELP | ARCTIC),
    Plant::sprite(
        "sargassum",
        &[
            r"
   ○  ○   ○
  ╲○╲│╱○ ╱○╱
 ○ ╲╲│╱╱○
  ╲○╲╲│╱╱○╱
    ╲╲│╱╱
     ╲│╱
      ┃
",
            r"
  ○   ○  ○
 ╱○╲│╱○╲ ○╲
  ○╱╱│╲╲○
 ╲○╱╱│╲╲○╲
    ╱╱│╲╲
     ╱│╲
      ┃
",
        ],
        &[
            r"
   Y  Y   Y
  yYyyyY yYy
 Y yyyyyY
  yYyyyyyYy
    yyyyy
     yyy
      y
",
            r"
  Y   Y  Y
 yYyyyYy Yy
  YyyyyyY
 yYyyyyyYy
    yyyyy
     yyy
      y
",
        ],
    )
    .color('y')
    .fps(0.2)
    .weight(6)
    .biomes(OCEAN)
    .traits(SHELTER),
    Plant::sprite(
        "deep-kelp-frond",
        &[
            r"
  ▗
  ▐▙
  ▜▌
  ▐▙
  ▜▌
   ▐
   ▌
   ┃
",
            r"
   ▖
  ▟▌
  ▐▛
  ▟▌
  ▐▛
  ▐
   ▌
   ┃
",
            r"
  ▗
  ▟▌
  ▐▌
  ▟▌
  ▐▛
   ▐
   ▌
   ┃
",
        ],
        &[],
    )
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(OCEAN | KELP)
    .traits(SHELTER),
    Plant::sprite(
        "sea-grass-tuft",
        &[
            r"
 ╲│╱
 ┃│┃
",
            r"
 ╱│╲
 ┃│┃
",
            r"
 │╲│
 ┃│┃
",
        ],
        &[],
    )
    .color('g')
    .fps(0.3)
    .weight(7)
    .biomes(OCEAN | REEF | FRESH),
    Plant::sprite(
        "sea-oat-tuft",
        &[
            r"
 ∙ ∙ ∙
 ┃╲│╱┃
",
            r"
  ∙ ∙ ∙
 ┃╱│╲┃
",
        ],
        &[],
    )
    .color('y')
    .fps(0.3)
    .weight(4)
    .biomes(OCEAN)
    .traits(SHELTER),
    Plant::sprite(
        "bladder-wrack",
        &[
            r"
  ○  ○  ○
  │╲ │  │╱
  ○│ ○│ ○│
   ╲ │ ╱
    ╲│╱
     ┃
",
            r"
 ○  ○  ○
 ╲│  │ ╱│
 │○ │○ │○
  ╲ │ ╱
   ╲│╱
    ┃
",
        ],
        &[
            r"
  Y  Y  Y
  yy y  yy
  Yy Yy Yy
   y y y
    yyy
     y
",
            r"
 Y  Y  Y
 yy  y yy
 yY yY yY
  y y y
   yyy
    y
",
        ],
    )
    .color('y')
    .fps(0.25)
    .weight(5)
    .biomes(OCEAN | KELP)
    .traits(SHELTER),
    Plant::sprite(
        "sea-lettuce-patch",
        &[
            r"
 ╭≈╮ ╭≈╮
 ≋≋≋≋≋≋≋≋
",
            r"
 ╭∼╮ ╭∼╮
 ∼≋≋∼≋≋∼
",
        ],
        &[],
    )
    .color('G')
    .fps(0.25)
    .weight(4)
    .biomes(OCEAN | KELP | TANK),
    Plant::sprite(
        "sea-lily",
        &[
            r"
 ╲ ╲│┃│╱ ╱
  ╲╲│┃│╱╱
   ╲│┃│╱
    ╲┃╱
     ┃
     ┃
    ╭╯
    ┃
   ▗▟▙▖
",
            r"
 ╱ ╱│┃│╲ ╲
  ╱╱│┃│╲╲
   ╱│┃│╲
    ╱┃╲
     ┃
     ╰╮
      ┃
     ╭╯
    ▗▟▙▖
",
        ],
        &[],
    )
    .color('w')
    .fps(0.15)
    .weight(5)
    .biomes(DEEP),
    Plant::sprite(
        "crinoid-stalk-pair",
        &[
            r"
 ╲│╱    ╲│╱
 ─┼─    ─┼─
  │      │
  ┃      ┃
  │      │
 ╱┃╲    ╱┃╲
",
            r"
 ╱│╲    ╱│╲
 ┄┼┄    ┄┼┄
  │      │
  ┃      ┃
  │      │
 ╱┃╲    ╱┃╲
",
        ],
        &[r"
 WwW    WwW
 WWW    WWW
  w      w
  w      w
  w      w
 WwW    WwW
"],
    )
    .color('w')
    .fps(0.25)
    .weight(3)
    .biomes(DEEP),
    Plant::sprite(
        "glass-sponge",
        &[
            r"
   ╭───╮
  ╱┼│┼│┼╲
 │┼─┼─┼─┼│
 │┼│┼│┼│┼│
 │┼─┼─┼─┼│
  ╲┼│┼│┼╱
   ╰───╯
",
            r"
   ╭───╮
  ╱╳│╳│╳╲
 │┼─╳─╳─┼│
 │╳│┼│┼│╳│
 │┼─╳─╳─┼│
  ╲╳│╳│╳╱
   ╰───╯
",
        ],
        &[r"
   WWWWW
  WcWcWcW
 WcWcWcWcW
 WcWcWcWcW
 WcWcWcWcW
  WcWcWcW
   WWWWW
"],
    )
    .color('W')
    .fps(0.25)
    .weight(5)
    .biomes(DEEP)
    .traits(SHELTER),
    Plant::sprite(
        "venus-flower-basket",
        &[
            r"
   ╭──╮
  ╱╳╳╳╳╲
 │╳╳╳╳╳╳│
  │╳╳╳╳│
  │╳╳╳╳│
  ╰╲──╱╯
    ┃┃
",
            r"
   ╭──╮
  ╱┼┼┼┼╲
 │┼┼┼┼┼┼│
  │┼┼┼┼│
  │┼┼┼┼│
  ╰╲──╱╯
    ┃┃
",
        ],
        &[r"
   WWWW
  WWWWWW
 WwwwwwwW
  WwwwwW
  WwwwwW
  WWWWWW
    WW
"],
    )
    .color('W')
    .fps(0.25)
    .weight(3)
    .biomes(DEEP)
    .traits(SHELTER),
    Plant::sprite(
        "bamboo-coral",
        &[
            r"
  ◦   ◦
  ┃╲ ╱┃
  ╪┃ ┃╪
  ┃╲ ╱┃
  ╪┃ ┃╪
  ┃  ┃
  ╪  ╪
",
            r"
  ○   ○
  ┃╱ ╲┃
  ╪┃ ┃╪
  ┃╱ ╲┃
  ╪┃ ┃╪
  ┃  ┃
  ╪  ╪
",
        ],
        &[r"
  1   1
  11 11
  11 11
  11 11
  11 11
  1  1
  1  1
"],
    )
    .tint("1:wWcC")
    .color('w')
    .fps(0.25)
    .weight(5)
    .biomes(DEEP),
    Plant::sprite(
        "black-coral",
        &[
            r"
  ╲  ╲  ╱ ╱
   ╲  ╲╱ ╱
  ╲ ╲ ┃┃ ╱  ╱
   ╲ ╲┃┃╱  ╱
    ╲ ┃┃ ╱╱
     ╲┃┃╱
      ┃┃
",
            r"
   ╲  ╲  ╱ ╱
   ╲  ╲╱ ╱
  ╲ ╲ ┃┃ ╱ ╱
   ╲ ╲┃┃╱ ╱
    ╲ ┃┃ ╱╱
     ╲┃┃╱
      ┃┃
",
        ],
        &[],
    )
    .color('K')
    .fps(0.25)
    .weight(5)
    .biomes(DEEP)
    .traits(SHELTER),
    Plant::sprite(
        "giant-tube-worms",
        &[
            r"
 ✱    ✱  ✱
 ┃  ✱ ┃  ┃
 ┃┃ ┃ ┃┃ ┃✱
 ┃┃ ┃ ┃┃ ┃┃
 ┃┃ ┃ ┃┃ ┃┃
 ▟▙ ▟ ▟▙ ▟▙
",
            r"
 ✲    ✲  ✲
 ┃  ✲ ┃  ┃
 ┃┃ ┃ ┃┃ ┃✲
 ┃┃ ┃ ┃┃ ┃┃
 ┃┃ ┃ ┃┃ ┃┃
 ▟▙ ▟ ▟▙ ▟▙
",
        ],
        &[r"
 R    R  R
 W  R W  W
 WW W WW WR
 WW W WW WW
 WW W WW WW
 WW W WW WW
"],
    )
    .color('W')
    .fps(0.2)
    .weight(6)
    .biomes(DEEP)
    .traits(SHELTER),
    Plant::sprite(
        "sea-pen",
        &[
            r"
    ◦
   ╱│╲
  ╱╱│╲╲
 ╱╱╱│╲╲╲
 ╲╲╲│╱╱╱
   ╲│╱
    ┃
    ┃
",
            r"
     ◦
   ╱│╲
  ╱╱│╲╲
 ╱╱╱│╲╲╲
 ╲╲╲│╱╱╱
   ╲│╱
    ┃
    ┃
",
        ],
        &[
            r"
    1
   111
  11111
 1111111
 1111111
   111
    1
    1
",
            r"
     1
   111
  11111
 1111111
 1111111
   111
    1
    1
",
        ],
    )
    .tint("1:mMwWcC")
    .color('g')
    .fps(0.1)
    .weight(5)
    .biomes(DEEP | ARCTIC)
    .traits(GLOW),
    Plant::sprite(
        "carnivorous-sponge",
        &[
            r"
 ◦  ◦  ◦  ◦
 ┃╲ ┃╲ ┃╲ ┃╲
 ┃ ╲┃ ╲┃ ╲┃ ╲
 ╰─╯╰─╯╰─╯╰─╯
    ╲  ╱
     ┃┃
",
            r"
  ◦  ◦  ◦  ◦
 ╱┃ ╱┃ ╱┃ ╱┃
 ╱ ┃╱ ┃╱ ┃╱ ┃
 ╰─╯╰─╯╰─╯╰─╯
    ╲  ╱
     ┃┃
",
        ],
        &[
            r"
 c  c  c  c
 ww ww ww ww
 w ww ww ww w
 wwwwwwwwwwww
    w  w
     ww
",
            r"
  c  c  c  c
 ww ww ww ww
 w ww ww ww w
 wwwwwwwwwwww
    w  w
     ww
",
        ],
    )
    .color('w')
    .fps(0.1)
    .weight(3)
    .biomes(DEEP),
    Plant::sprite(
        "vent-tube-worms",
        &[
            r"
   ◖◗  ◖◗
   ┃┃  ┃┃  ◖◗
  ◖◗┃┃ ┃┃  ┃┃
  ┃┃┃┃▁┃┃◖◗┃┃
 ▗▟▙▟▙▟▙▟▙▟▙▟▙▖
",
            r"
   ◗◖  ◗◖
   ┃┃  ┃┃  ◗◖
  ◗◖┃┃ ┃┃  ┃┃
  ┃┃┃┃▁┃┃◗◖┃┃
 ▗▟▙▟▙▟▙▟▙▟▙▟▙▖
",
        ],
        &[r"
   RR  RR
   ww  ww  RR
  RRww ww  ww
  wwwwwwwRRww
 wwwwwwwwwwwwww
"],
    )
    .color('w')
    .fps(0.25)
    .weight(4)
    .biomes(DEEP)
    .traits(SHELTER),
    Plant::sprite(
        "pondweed",
        &[
            r"
  ╭○╮ ╭○╮
 ╭○●○○○○●○╮
  ╲ │ │ ╱
   ╲│▁│╱
    │││
    │││
",
            r"
   ╭○╮ ╭○╮
  ╭○●○○○○●○╮
  ╱ │ │ ╲
   ╲│▁│╱
    │││
    │││
",
        ],
        &[],
    )
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(FRESH)
    .traits(SHELTER),
    Plant::sprite(
        "lily-pad-stem",
        &[
            r"
  ▗▄▄▄▄▄▄▖
 ▐▓▓▓▓◣▓▓▓▌
     ┃
     ┃
     ┃
     ┃
",
            r"
  ▗▄▄▄▄▄▄▖
 ▐▓▓▓▓▓◣▓▓▌
     ┃
    ╭╯
    ┃
    ╰╮
",
        ],
        &[
            r"
  GggggggG
 gGGGGwGGGg
     g
     g
     g
     g
",
            r"
  GggggggG
 gGGGGGwGGg
     g
    gg
    g
    gg
",
        ],
    )
    .color('g')
    .fps(0.15)
    .weight(6)
    .biomes(FRESH),
    Plant::sprite(
        "water-lily-bloom",
        &[
            r"
    ◢▲◣
  ╭─◖✿◗─╮
 ▐▓▓▓▓▓▓▓▌
     ┃
     ┃
",
            r"
    ◣▲◢
  ╭─◖❀◗─╮
 ▐▓▓▓▓▓▓▓▌
     ┃
     ┃
",
        ],
        &[r"
    MWM
  ggMyMgg
 gGGGGGGGg
     g
     g
"],
    )
    .color('g')
    .fps(0.15)
    .weight(3)
    .biomes(FRESH)
    .traits(SHELTER),
    Plant::sprite(
        "cattail-roots",
        &[
            r"
   ▮   ▮
   ▮   ▮   ▮
   │  ╱│   │
   │ ╱ │  ╱
  ╲│╱  │ ╱
   │   │╱
",
            r"
   ▮   ▮
   ▮   ▮  ▮
   │  ╱│  │
   │ ╱ │ ╱
  ╲│╱  │╱
   │   │
",
        ],
        &[
            r"
   y   y
   y   y   y
   g  gg   g
   g g g  g
  ggg  g g
   g   gg
",
            r"
   y   y
   y   y  y
   g  gg  g
   g g g g
  ggg  gg
   g   g
",
        ],
    )
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(FRESH)
    .traits(SHELTER),
    Plant::sprite(
        "elodea",
        &[
            r"
 ╲╱╲╱╲╱╲╱
  ╲││││╱
   ╲││╱
    ││
    ││
",
            r"
 ╱╲╱╲╱╲╱╲
  ╱││││╲
   ╱││╲
    ││
    ││
",
        ],
        &[],
    )
    .color('G')
    .fps(0.3)
    .weight(5)
    .biomes(FRESH | TANK)
    .traits(SHELTER),
    Plant::sprite(
        "bulrush",
        &[
            r"
  ▮   ▮   ▮
  │  ╲│  ╱│
 ╲│  ╲│  ╱│
  │╱  │ ╱ │
  │ ╲ │╱  │
  │  ╲││  │
",
            r"
  ▮   ▮   ▮
  │  ╱│  ╲│
 ╱│  ╱│  ╲│
  │╲  │ ╲ │
  │ ╱ │╲  │
  │  ╱││  │
",
        ],
        &[],
    )
    .color('g')
    .fps(0.25)
    .weight(3)
    .biomes(FRESH)
    .traits(SHELTER),
    Plant::sprite(
        "dead-mans-fingers",
        &[
            r"
 ╭╮  ╭╮  ╭╮
 ┃┃  ┃┃  ┃┃
 ╲╲╲ ┃┃ ╱╱╱
  ╲╲╲┃┃╱╱╱
    ╲┃┃╱
     ┃┃
",
            r"
 ╭╮  ╭╮  ╭╮
 ┃┃  ┃┃  ┃┃
 ╲╲┃ ┃┃ ┃╱╱
  ╲╲┃┃┃┃╱╱
    ╲┃┃╱
     ┃┃
",
        ],
        &[r"
 11  11  11
 11  11  11
 111 11 111
  11111111
    1111
     11
"],
    )
    .tint("1:wWyYmM")
    .color('w')
    .fps(0.25)
    .weight(5)
    .biomes(ARCTIC | KELP),
    Plant::sprite(
        "frilled-anemone",
        &[
            r"
 ╲╲╿│╿│╿╱╱
 ╲╲╿│╿│╿╱╱
  ╲╲│╿│╱╱
   ╰───╯
    ┃┃┃
",
            r"
 ╱╱│╿│╿│╲╲
 ╱╱│╿│╿│╲╲
  ╱╱╿│╿╲╲
   ╰───╯
    ┃┃┃
",
        ],
        &[r"
 111111111
 111111111
  1111111
   wwwww
    111
"],
    )
    .tint("1:wWcCmM")
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(ARCTIC)
    .traits(SHELTER | STING),
    Plant::sprite(
        "plumose-anemone",
        &[
            r"
 ╲∙╲│╱∙╱
 ∙╲╲│╱╱∙
 │╲╲│╱╱│
  ╲╲│╱╱
   ┃┃┃
   ┃┃┃
",
            r"
 ╱∙╲│╱∙╲
 ∙╱╱│╲╲∙
 │╱╱│╲╲│
  ╱╱│╲╲
   ┃┃┃
   ┃┃┃
",
        ],
        &[r"
 1111111
 1111111
 1111111
  11111
   111
   111
"],
    )
    .tint("1:wWyYcC")
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(ARCTIC)
    .traits(SHELTER | STING),
    Plant::sprite(
        "arctic-laminaria",
        &[
            r"
 ╲ │╲│╱│ ╱
  ╲│ │ │╱
   ╲╲│╱╱
    ╲│╱
     ┃
     ┃
     ┃
",
            r"
 ╱ │╲│╱│ ╲
  │╱ │ ╲│
   ╱╱│╲╲
    ╲│╱
     ┃
     ┃
     ┃
",
        ],
        &[],
    )
    .color('y')
    .fps(0.2)
    .weight(6)
    .biomes(ARCTIC | KELP)
    .traits(SHELTER),
    Plant::sprite(
        "cold-water-coral",
        &[
            r"
  ✱   ✱   ✱
 ╲│╱ ╲│╱ ╲│╱
  ╰╮  │  ╭╯
   ╰╮ │ ╭╯
    ╰╮│╭╯
     ╰┼╯
",
            r"
  ✧   ✧   ✧
 ╱│╲ ╱│╲ ╱│╲
  ╰╮  │  ╭╯
   ╰╮ │ ╭╯
    ╰╮│╭╯
     ╰┼╯
",
        ],
        &[r"
  1   1   1
 111 111 111
  11  1  11
   11 1 11
    11111
     111
"],
    )
    .tint("1:wWcC")
    .color('w')
    .fps(0.25)
    .weight(4)
    .biomes(ARCTIC | DEEP)
    .traits(SHELTER),
    Plant::sprite(
        "sea-raspberry",
        &[
            r"
 ◦●◉●◦
 ●◉●◉●◉
 ╰●◉●●╯
",
            r"
 ◦◉●◉◦
 ◉●◉●◉●
 ╰◉●◉◉╯
",
        ],
        &[r"
 11111
 111111
 111111
"],
    )
    .tint("1:rRmM")
    .color('g')
    .fps(0.25)
    .weight(3)
    .biomes(ARCTIC | KELP),
    Plant::sprite(
        "amazon-sword",
        &[
            r"
 ╲  ╲ │ ╱  ╱
  ╲  ╲│╱  ╱
   ╲ ╲│╱ ╱
    ╲ │ ╱
     ╲│╱
      ┃
",
            r"
  ╲ ╲ │ ╱ ╱
  ╱  ╲│╱  ╲
   ╱ ╲│╱ ╲
    ╱ │ ╲
     ╲│╱
      ┃
",
        ],
        &[],
    )
    .color('g')
    .fps(0.25)
    .weight(7)
    .biomes(TANK)
    .traits(SHELTER),
    Plant::sprite(
        "java-fern",
        &[
            r"
  ╭∼╮ ╭∼╮
 ╱≈≈≈╲≈≈≈╲
 ╲≈≈≈╱╲≈≈≈╱
  ╰┃╯  ╰┃╯
   ╲╲  ╱╱
    ╲╲╱╱
",
            r"
  ╭∼╮ ╭∼╮
 ╲≈≈≈╱≈≈≈╱
 ╱≈≈≈╲╲≈≈≈╲
  ╰┃╯  ╰┃╯
   ╱╱  ╲╲
    ╱╱╲╲
",
        ],
        &[],
    )
    .color('G')
    .fps(0.2)
    .weight(7)
    .biomes(TANK)
    .traits(SHELTER),
    Plant::sprite(
        "anubias",
        &[
            r"
 ◢  ◣ ◢  ◣
 ╲╲╱╱  ╲╲╱╱
  ┃┃    ┃┃
  ┃┃    ┃┃
  ╰╲▄▄▄▄╱╯
",
            r"
 ◥  ◤ ◥  ◤
 ╲╲╱╱  ╲╲╱╱
  ┃┃    ┃┃
  ┃┃    ┃┃
  ╰╲▄▄▄▄╱╯
",
        ],
        &[r"
 G  G G  G
 gggg  gggg
  gg    gg
  gg    gg
  gggggggg
"],
    )
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(TANK)
    .traits(SHELTER),
    Plant::sprite(
        "java-moss",
        &[
            r"
 ▗▟▓▓▒▓▒▙▖
 ▓▒▓▒▓▓▒▓▒
",
            r"
 ▗▟▒▓▓▒▓▙▖
 ▒▓▒▓▒▓▓▒▓
",
        ],
        &[],
    )
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(TANK)
    .traits(SHELTER),
    Plant::sprite(
        "marimo",
        &[
            r"
  ▗▟▓▓▙▖
 ▐▓▒▓▓▒▓▌
  ▝▜▓▓▛▘
",
            r"
  ▗▟▒▓▙▖
 ▐▒▓▓▒▓▒▌
  ▝▜▓▒▛▘
",
        ],
        &[r"
  ggGGgg
 gGGGGGGg
  ggGGgg
"],
    )
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(TANK | FRESH),
    Plant::sprite(
        "plastic-plant",
        &[r"
 ╲│╱ ╲│╱
 ╲│╱ ╲│╱
  │ │ │
  ╲ │ ╱
   ╲│╱
  ▐███▌
"],
        &[r"
 111 111
 111 111
  1 1 1
  1 1 1
   111
  wwwww
"],
    )
    .tint("1:MCYR")
    .color('g')
    .fps(0.0)
    .weight(3)
    .biomes(TANK)
    .traits(SHELTER),
    Plant::sprite(
        "hairgrass",
        &[
            r"
 ╷ ╷ ╷ ╷ ╷
 │ │ │ │ │
 │ │ │ │ │
",
            r"
  ╷ ╷ ╷ ╷ ╷
 ╱ ╱ ╱ ╱ ╱
 │ │ │ │ │
",
        ],
        &[],
    )
    .color('G')
    .fps(0.3)
    .weight(5)
    .biomes(TANK),
    Plant::sprite(
        "water-wisteria",
        &[
            r"
  ╲ ╲│╱ ╱
  ╲╲╲│╱ ╱
   ╲╲╲│╱
    ╲╲│
     ╲│
      ┃
",
            r"
  ╲ ╲│╱╱ ╱
  ╲╲╲│╱╱
   ╲╲╲│╱
    ╲╲│
     ╲│
      ┃
",
        ],
        &[],
    )
    .color('g')
    .fps(0.25)
    .weight(4)
    .biomes(TANK | FRESH)
    .traits(SHELTER),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_plants_drawn(super::PLANTS);
    }
}
