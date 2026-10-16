//! Seaweed, kelp, grasses, anemones, corals and sponges.

use super::*;

pub const PLANTS: &[Plant] = &[
    Plant::stalk("sea-whip", "\\", " /", 4, 10)
        .color('r')
        .fps(0.25)
        .weight(6)
        .biomes(REEF),
    Plant::stalk("grape-algae", "8", " 8", 3, 8)
        .color('g')
        .fps(0.2)
        .weight(4)
        .biomes(REEF | TANK),
    Plant::stalk("giant-kelp-stipe", "}", " {", 8, 22)
        .color('y')
        .fps(0.2)
        .weight(7)
        .biomes(KELP),
    Plant::stalk("eelgrass", "|", " |", 3, 8)
        .color('g')
        .fps(0.3)
        .weight(8)
        .biomes(FRESH | OCEAN),
    Plant::stalk("vallisneria", "\\|", " |/", 4, 12)
        .color('G')
        .fps(0.3)
        .weight(7)
        .biomes(FRESH | TANK),
    Plant::stalk("sargassum-vine", "o", " )", 4, 12)
        .color('y')
        .fps(0.25)
        .weight(7)
        .biomes(OCEAN),
    Plant::stalk("red-ribbon-algae", "{", " }", 3, 9)
        .color('r')
        .fps(0.25)
        .weight(5)
        .biomes(KELP | ARCTIC | OCEAN),
    Plant::stalk("hornwort", "\\|/", " | ", 3, 8)
        .color('g')
        .fps(0.25)
        .weight(6)
        .biomes(FRESH),
    Plant::stalk("cabomba", "}|{", " ~|~", 4, 10)
        .color('g')
        .fps(0.3)
        .weight(6)
        .biomes(TANK | FRESH),
    Plant::stalk("rotala", "-|", " |-", 4, 11)
        .color('R')
        .fps(0.25)
        .weight(5)
        .biomes(TANK),
    Plant::stalk("ludwigia", "<|", " |>", 4, 10)
        .color('r')
        .fps(0.25)
        .weight(5)
        .biomes(TANK | FRESH),
    Plant::stalk("bamboo-coral-stalk", "=|=", "-|-", 4, 14)
        .color('W')
        .fps(0.1)
        .weight(5)
        .biomes(DEEP),
    Plant::stalk("wire-coral", "s", " s", 5, 18)
        .color('K')
        .fps(0.1)
        .weight(4)
        .biomes(DEEP),
    Plant::stalk("ribbon-kelp", ")", " (", 5, 14)
        .color('y')
        .fps(0.2)
        .weight(6)
        .biomes(KELP | ARCTIC),
    Plant::stalk("water-milfoil", "=|=", " ~ ", 3, 9)
        .color('g')
        .fps(0.25)
        .weight(5)
        .biomes(FRESH),
    Plant::stalk("chara", "*", " +", 2, 6)
        .color('G')
        .fps(0.2)
        .weight(4)
        .biomes(FRESH),
    Plant::stalk("turtle-grass", "|", " )", 2, 6)
        .color('g')
        .fps(0.3)
        .weight(7)
        .biomes(REEF | OCEAN),
    Plant::stalk("sea-lettuce-frill", "~", " ~", 2, 5)
        .color('G')
        .fps(0.3)
        .weight(4)
        .biomes(KELP | ARCTIC),
    Plant::sprite(
        "brain-coral",
        &[r"
    _.-~~-._
  ,'~S~~S~S~`.
 (~S~S~~S~S~S~)
  `.~S~S~~S~,'
    `-.__.-'
"],
        &[r"
    11111111
  111111111111
 11111111111111
  111111111111
    11111111
"],
    )
    .tint("1:yYgGmM")
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(REEF),
    Plant::sprite(
        "staghorn-coral",
        &[r"
  \    Y   /
   Y  /|  Y
   |\/ | /
    \\ |/ Y
     \\|/ /
      \|//
       ||
"],
        &[r"
  1    1   1
   1  11  1
   111 1 1
    11 11 1
     1111 1
      1111
       11
"],
    )
    .tint("1:yYmMwW")
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(REEF),
    Plant::sprite(
        "elkhorn-coral",
        &[r"
   _,-~-.   .-~-,_
 ,-'_,-~-\_/-~-,_`-,
  `-~ _,-'|`-,_ ~-'
     ,-'  |  `-,
          |
         /|\
"],
        &[r"
   111111   111111
 1111111111111111111
  111 111111111 111
     111  1  111
          1
         111
"],
    )
    .tint("1:yYrR")
    .color('g')
    .fps(0.25)
    .weight(4)
    .biomes(REEF),
    Plant::sprite(
        "sea-fan",
        &[
            r"
   .-~~-.
 .'|\||/|`.
 |-+-+-+-+-|
 |-+-+-+-+-|
  \-+-+-+-/
   `\ | /'
     \|/
      |
",
            r"
    .-~~-.
  .'|\||/|`.
  |-+-+-+-+-|
  |-+-+-+-+-|
   \-+-+-+-/
    `\ | /'
      \|/
       |
",
        ],
        &[
            r"
   111111
 1111111111
 11111111111
 11111111111
  111111111
   11 1 11
     111
      1
",
            r"
    111111
  1111111111
  11111111111
  11111111111
   111111111
    11 1 11
      111
       1
",
        ],
    )
    .tint("1:rRmMyY")
    .color('g')
    .fps(0.15)
    .weight(6)
    .biomes(REEF),
    Plant::sprite(
        "mushroom-coral",
        &[r"
  .-====-.
 (========)
  `-.__.-'
     ||
"],
        &[r"
  11111111
 1111111111
  11111111
     11
"],
    )
    .tint("1:gGmMcC")
    .color('g')
    .fps(0.25)
    .weight(5)
    .biomes(REEF),
    Plant::sprite(
        "bubble-coral",
        &[
            r"
  oOo Oo
 OoOoOoOo
  oOoOoO
   `||'
",
            r"
 oOo  Oo
  OoOoOoO
  oOoOoO
   `||'
",
        ],
        &[
            r"
  111 11
 11111111
  111111
   1111
",
            r"
 111  11
  1111111
  111111
   1111
",
        ],
    )
    .tint("1:wWyYcC")
    .color('g')
    .fps(0.1)
    .weight(4)
    .biomes(REEF),
    Plant::sprite(
        "pillar-coral",
        &[r"
 ()  ()
 ||  ||  ()
 ||()||  ||
 ||||||()||
 ||||||||||
"],
        &[r"
 11  11
 11  11  11
 111111  11
 1111111111
 1111111111
"],
    )
    .tint("1:yYwW")
    .color('g')
    .fps(0.25)
    .weight(4)
    .biomes(REEF),
    Plant::sprite(
        "table-coral",
        &[r"
  ____..-~~-..____
 (_____________,-'_)
        ||||
        ||||
       _||||_
"],
        &[r"
  1111111111111111
 1111111111111111111
        1111
        1111
       111111
"],
    )
    .tint("1:yYgGcC")
    .color('g')
    .fps(0.25)
    .weight(4)
    .biomes(REEF),
    Plant::sprite(
        "tube-coral",
        &[r"
  ()  ()   ()
  ||  ||   ||
  ||  ||() ||
 ()|| ||||_||
 ||||_||||||||
"],
        &[r"
  11  11   11
  11  11   11
  11  1111 11
 1111 1111111
 1111111111111
"],
    )
    .tint("1:mMrRyY")
    .color('g')
    .fps(0.25)
    .weight(4)
    .biomes(REEF),
    Plant::sprite(
        "finger-coral",
        &[r"
 , , , ,,
 | | | ||
 |,|,|,||,
 |||||||||
"],
        &[r"
 1 1 1 11
 1 1 1 11
 111111111
 111111111
"],
    )
    .tint("1:yYwWmM")
    .color('g')
    .fps(0.25)
    .weight(5)
    .biomes(REEF),
    Plant::sprite(
        "christmas-tree-worms",
        &[
            r"
  /\    /\
 /@@\  /@@\
 \@@/ /\\@@\
  ||  \@@/
  ||   ||
",
            r"
  /\    /\
 /@@\  /@@\
 \@@/ /\\@@\
  ||  \@@/
  ||   ||
",
        ],
        &[r"
  11    11
 1111  1111
 1111 111111
  11  1111
  11   11
"],
    )
    .tint("1:rRyYbB")
    .color('g')
    .fps(0.3)
    .weight(5)
    .biomes(REEF),
    Plant::sprite(
        "feather-duster-worm",
        &[
            r"
  \\|//
 --\\|//--
 --{***}--
    |||
    |||
",
            r"
   \|/
  -\|/-
  -{*}-
    |||
    |||
",
            r"
    |
    |
    |
    |||
    |||
",
        ],
        &[
            r"
  11w11
 1111w1111
 111111111
    www
    www
",
            r"
   1w1
  11w11
  11111
    www
    www
",
            r"
    w
    w
    w
    www
    www
",
        ],
    )
    .tint("1:rRmMyY")
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(REEF),
    Plant::sprite(
        "barrel-sponge",
        &[r"
 .-~~~~~-.
 |       |
 |   ~   |
 |  ~ ~  |
 `-.___.-'
"],
        &[r"
 111111111
 1       1
 1   1   1
 1  1 1  1
 111111111
"],
    )
    .tint("1:yYrRmM")
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(REEF),
    Plant::sprite(
        "tube-sponges",
        &[r"
 ()  ()
 ||  ||  ()
 ||  ||  ||
 ||()||  ||
 ||||||()||
"],
        &[r"
 11  11
 11  11  11
 11  11  11
 111111  11
 1111111111
"],
    )
    .tint("1:yYmMrR")
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(REEF),
    Plant::sprite(
        "vase-sponge",
        &[r"
  \     /
  |\   /|
  | \_/ |
   \   /
    `-'
"],
        &[r"
  1     1
  11   11
  1 111 1
   1   1
    111
"],
    )
    .tint("1:yYrRmM")
    .color('g')
    .fps(0.25)
    .weight(5)
    .biomes(REEF),
    Plant::sprite(
        "pink-anemone",
        &[
            r"
 \|/\|/\|/
 /|\|/\|/\
  \_____/
   |___|
",
            r"
 /|\/|\/|\
 \|/|\/|\/
  \_____/
   |___|
",
        ],
        &[r"
 121121121
 121211211
  1222221
   22222
"],
    )
    .tint("1:mMrRyY 2:yw")
    .color('g')
    .fps(0.3)
    .weight(7)
    .biomes(REEF | TANK),
    Plant::sprite(
        "bubble-tip-anemone",
        &[
            r"
 o o o o o
 |o|o|o|o|
  \|\|/|/
   \___/
    |_|
",
            r"
  o o o o o
  |o|o|o|o|
   \|\|/|/
    \___/
     |_|
",
        ],
        &[
            r"
 1 1 1 1 1
 111111111
  1111111
   12221
    121
",
            r"
  1 1 1 1 1
  111111111
   1111111
    12221
     121
",
        ],
    )
    .tint("1:rRmMyY 2:yY")
    .color('g')
    .fps(0.3)
    .weight(6)
    .biomes(REEF | TANK),
    Plant::sprite(
        "sea-grass-meadow",
        &[
            r"
 | \ | / | \ |
 |\ \|/ /|\ \|
 || \||/ ||\ |
",
            r"
 \ | / | \ | /
 /| \|/ |/ |\|
 ||/ || \|| /|
",
        ],
        &[],
    )
    .color('g')
    .fps(0.3)
    .weight(7)
    .biomes(REEF | OCEAN),
    Plant::sprite(
        "fire-coral",
        &[r"
  ,  ,  ,
 /|\/|\/|\
 |||||||||
 `======='
"],
        &[r"
  1  1  1
 111111111
 111111111
 111111111
"],
    )
    .tint("1:yYrR")
    .color('g')
    .fps(0.25)
    .weight(3)
    .biomes(REEF),
    Plant::sprite(
        "sea-rod-fans",
        &[
            r"
  \ | /   \ | /
   \|/     \|/
 \  |  \ /  |  /
  \ |   |   | /
   \|   |   |/
    |   |   |
",
            r"
  / | \   / | \
   \|/     \|/
 /  |  \ /  |  \
  / |   |   | \
   |/   |   \|
    |   |   |
",
        ],
        &[r"
  1 1 1   1 1 1
   111     111
 1  1  1 1  1  1
  1 1   1   1 1
   11   1   11
    1   1   1
"],
    )
    .tint("1:rRmMyY")
    .color('g')
    .fps(0.15)
    .weight(4)
    .biomes(REEF),
    Plant::sprite(
        "giant-kelp",
        &[
            r"
  \ /
  }O
  { )
   } \
  ( }O
   ) {
  }  /
  { )O
   }(
   |
",
            r"
  \ /
  O{
  ( }
  / {
 O{ )
  } (
  \  {
  O( }
   )}
   |
",
        ],
        &[
            r"
  y y
  yG
  y y
   y y
  y yG
   y y
  y  y
  y yG
   yy
   y
",
            r"
  y y
  Gy
  y y
  y y
 Gy y
  y y
  y  y
  Gy y
   yy
   y
",
        ],
    )
    .color('y')
    .fps(0.2)
    .weight(6)
    .biomes(KELP),
    Plant::sprite(
        "bull-kelp",
        &[
            r"
 \|\|/|/
  \\|//
   (O)
    ) (
   (  )
    ) (
   (  )
    )(
    ||
",
            r"
 \|\|/|/
  \\|//
   (O)
   ( )
    ) (
   (  )
    ) (
    )(
    ||
",
        ],
        &[
            r"
 yyyyyyy
  yyyyy
   yGy
    y y
   y  y
    y y
   y  y
    yy
    yy
",
            r"
 yyyyyyy
  yyyyy
   yGy
   y y
    y y
   y  y
    y y
    yy
    yy
",
        ],
    )
    .color('y')
    .fps(0.2)
    .weight(6)
    .biomes(KELP | ARCTIC),
    Plant::sprite(
        "feather-boa-kelp",
        &[r"
  ,-,-,
 /|/|/|\
 ||||||||
  \||||/
   |||
   |||
"],
        &[r"
  11111
 1111111
 11111111
  111111
   111
   111
"],
    )
    .tint("1:yYgG")
    .color('y')
    .fps(0.25)
    .weight(5)
    .biomes(KELP),
    Plant::sprite(
        "sea-palm",
        &[
            r"
 \\ \|/ //
  \\\|///
    \|/
     |
     |
    _|_
",
            r"
 // \|/ \\
  ///|\\\
    \|/
     |
     |
    _|_
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
    .biomes(KELP),
    Plant::sprite(
        "surfgrass",
        &[
            r"
 | | | | | |
 |\|/|\|/|\|
 || || || ||
",
            r"
 | | | | | |
 |/|\|/|\|/|
 || || || ||
",
        ],
        &[],
    )
    .color('G')
    .fps(0.3)
    .weight(6)
    .biomes(KELP | OCEAN),
    Plant::sprite(
        "coralline-algae",
        &[r"
  .oOo.
 oOoOoOo
 OoOoOoO
"],
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
  ,~~,
 /~~~~\~,
 \~~/~~~~\
  `/~~~~~/
   |`~~'
   |
",
            r"
   ,~~,
  /~~~~\ ,
 ,\~~/~~~\
 \~~~~~/'/
   `~~'|
     |
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
   o  o   o
  \o\|/o /o/
 o \\\|///o
  \o\\|//o/
    \\|//
     \|/
      |
",
            r"
  o   o  o
 /o\|/o\ o\
  o///|\\\ o
 \o//|\\o\
    //|\\
     \|/
      |
",
        ],
        &[
            r"
   Y  Y   Y
  yYyyyY yYy
 Y yyyyyyyY
  yYyyyyyYy
    yyyyy
     yyy
      y
",
            r"
  Y   Y  Y
 yYyyyYy Yy
  Yyyyyyyy Y
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
    .biomes(OCEAN),
    Plant::sprite(
        "deep-kelp-frond",
        &[
            r"
  ,
  )\
  (/
  )\
  (/
   )
   (
   |
",
            r"
   ,
  /(
  \)
  /(
  \)
  (
   )
   |
",
        ],
        &[],
    )
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(OCEAN | KELP),
    Plant::sprite(
        "sea-grass-tuft",
        &[
            r"
 \|/
 |||
",
            r"
 /|\
 |||
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
        &[r"
 ' ' '
 |\|/|
"],
        &[],
    )
    .color('y')
    .fps(0.3)
    .weight(4)
    .biomes(OCEAN),
    Plant::sprite(
        "bladder-wrack",
        &[
            r"
  o  o  o
  |\ |  |/
  o| o| o|
   \ | /
    \|/
     |
",
            r"
 o  o  o
 \|  | /|
 |o |o |o
  \ | /
   \|/
    |
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
    .biomes(OCEAN | KELP),
    Plant::sprite(
        "sea-lettuce-patch",
        &[r"
 ,~, ,~,
 ~~~~~~~~
"],
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
 \ \|||/ /
  \\|||//
   \|||/
    |||
    ||
    ||
    ||
    ||
    /\
",
            r"
 / /|||\ \
  //|||\\
   /|||\
    |||
     ||
    ||
     ||
    ||
    /\
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
        &[r"
 \|/    \|/
 -+-    -+-
  |      |
  |      |
  |      |
 /|\    /|\
"],
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
        &[r"
   .-~-.
  /+|+|+\
 |+-+-+-+|
 |+|+|+|+|
 |+-+-+-+|
  \+|+|+/
   `---'
"],
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
    .biomes(DEEP),
    Plant::sprite(
        "venus-flower-basket",
        &[r"
   _.._
  /|/\|\
 ( (++) )
  |++++|
  |++++|
  `\__/'
    ||
"],
        &[r"
   WWWW
  WWWWWW
 W WwwW W
  WwwwwW
  WwwwwW
  WWWWWW
    WW
"],
    )
    .color('W')
    .fps(0.25)
    .weight(3)
    .biomes(DEEP),
    Plant::sprite(
        "bamboo-coral",
        &[r"
  Y   Y
  |\ /|
  =| |=
  |\ /|
  =| |=
  |  |
  =  =
"],
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
        &[r"
  \  \  / /
   \  \/ /
  \ \ || /  /
   \ \||/  /
    \ || //
     \||/
      ||
"],
        &[r"
  K  K  K K
   K  KK K
  K K KK K  K
   K KKKK  K
    K KK KK
     KKKK
      KK
"],
    )
    .color('K')
    .fps(0.25)
    .weight(5)
    .biomes(DEEP),
    Plant::sprite(
        "giant-tube-worms",
        &[
            r"
 *    *  *
 |  * |  |
 || | || |*
 || | || ||
 || | || ||
 || | || ||
",
            r"
 *    *  *
 |  * |  |
 || | || |*
 || | || ||
 || | || ||
 || | || ||
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
    .biomes(DEEP),
    Plant::sprite(
        "sea-pen",
        &[
            r"
    ,
   /|\
  //|\\
 ///|\\\
 \\\|///
   \|/
    |
    |
",
            r"
     ,
   /|\
  //|\\
 ///|\\\
 \\\|///
   \|/
    |
    |
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
    .biomes(DEEP | ARCTIC),
    Plant::sprite(
        "carnivorous-sponge",
        &[
            r"
 o  o  o  o
 |\ |\ |\ |\
 | \| \| \| \
 \_/\_/\_/\_/
    \  /
     ||
",
            r"
  o  o  o  o
 /| /| /| /|
 / |/ |/ |/ |
 \_/\_/\_/\_/
    \  /
     ||
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
        &[r"
   ()  ()
   ||  ||  ()
  ()|| ||  ||
  ||||_||()||
 _||||||||||||_
"],
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
    .biomes(DEEP),
    Plant::sprite(
        "pondweed",
        &[
            r"
  ,o, ,o,
 (oooooooo)
  \ | | /
   \|_|/
    |||
    |||
",
            r"
   ,o, ,o,
  (oooooooo)
  / | | \
   \|_|/
    |||
    |||
",
        ],
        &[],
    )
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(FRESH),
    Plant::sprite(
        "lily-pad-stem",
        &[
            r"
  _.----._
 (___,____)
     |
     |
     |
     |
",
            r"
  _.----._
 (____,___)
     |
     |
     |
     |
",
        ],
        &[
            r"
  GggggggG
 gGGGwGGGGg
     g
     g
     g
     g
",
            r"
  GggggggG
 gGGGGwGGGg
     g
     g
     g
     g
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
    \|/
  .-(*)-.
 (_______)
     |
     |
",
            r"
    \|/
  .-(*)-.
 (_______)
     |
     |
",
        ],
        &[r"
    MgM
  ggMyMgg
 MgggggggM
     g
     g
"],
    )
    .color('g')
    .fps(0.15)
    .weight(3)
    .biomes(FRESH),
    Plant::sprite(
        "cattail-roots",
        &[
            r"
   H   H
   H   H   H
   |  /|   |
   | / |  /
  \|/  | /
   |   |/
",
            r"
   H   H
   H   H  H
   |  /|  |
   | / | /
  \|/  |/
   |   |
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
    .biomes(FRESH),
    Plant::sprite(
        "elodea",
        &[
            r"
 \/\/\/\/
  \||||/
   \||/
    ||
    ||
",
            r"
 /\/\/\/\
  /||||\
   /||\
    ||
    ||
",
        ],
        &[],
    )
    .color('G')
    .fps(0.3)
    .weight(5)
    .biomes(FRESH | TANK),
    Plant::sprite(
        "bulrush",
        &[r"
  |   |   |
  |  \|  /|
 \|  \|  /|
  |/  | / |
  | \ |/  |
  |  \||  |
"],
        &[],
    )
    .color('g')
    .fps(0.25)
    .weight(3)
    .biomes(FRESH),
    Plant::sprite(
        "dead-mans-fingers",
        &[r"
 ()  ()  ()
 ||  ||  ||
 \\\ || ///
  \\\||///
    \||/
     ||
"],
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
 \\|||||//
 \\|||||//
  \\|||//
   `---'
    |||
",
            r"
 //|||||\\
 //|||||\\
  //|||\\
   `---'
    |||
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
    .biomes(ARCTIC),
    Plant::sprite(
        "plumose-anemone",
        &[
            r"
 \'\|/'/
 '\\|//'
 |\\|//|
  \\|//
   |||
   |||
",
            r"
 /'\|/'\
 '//|\\'
 |//|\\|
  //|\\
   |||
   |||
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
    .biomes(ARCTIC),
    Plant::sprite(
        "arctic-laminaria",
        &[
            r"
 \ |\|/| /
  \| | |/
   \\|//
    \|/
     |
     |
     |
",
            r"
 / |\|/| \
  |/ | \|
   //|\\
    \|/
     |
     |
     |
",
        ],
        &[],
    )
    .color('y')
    .fps(0.2)
    .weight(6)
    .biomes(ARCTIC | KELP),
    Plant::sprite(
        "cold-water-coral",
        &[r"
  ,*, ,*, ,*,
 ( * ) * ) * )
  `*'  `*'  `*'
    \  |  /
     \ | /
      \|/
"],
        &[r"
  111 111 111
 1 1 1 1 1 1 1
  111  111  111
    1  1  1
     1 1 1
      111
"],
    )
    .tint("1:wWcC")
    .color('w')
    .fps(0.25)
    .weight(4)
    .biomes(ARCTIC | DEEP),
    Plant::sprite(
        "sea-raspberry",
        &[r"
 ooOoo
 OoOoOo
 `oOoo'
"],
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
 \  \ | /  /
  \  \|/  /
   \ \|/ /
    \ | /
     \|/
      |
",
            r"
  \ \ | / /
  /  \|/  \
   / \|/ \
    / | \
     \|/
      |
",
        ],
        &[],
    )
    .color('g')
    .fps(0.25)
    .weight(7)
    .biomes(TANK),
    Plant::sprite(
        "java-fern",
        &[
            r"
  ,~, ,~,
 /~~~\~~~\
 \~~~/\~~~/
  `|'  `|'
   \\  //
    \\//
",
            r"
  ,~, ,~,
 \~~~/~~~/
 /~~~\\~~~\
  `|'  `|'
   //  \\
    //\\
",
        ],
        &[],
    )
    .color('G')
    .fps(0.2)
    .weight(7)
    .biomes(TANK),
    Plant::sprite(
        "anubias",
        &[r"
 (  ) (  )
 \\//  \\//
  ||    ||
  ||    ||
  \\____//
"],
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
    .biomes(TANK),
    Plant::sprite(
        "java-moss",
        &[r"
 ,~'~,~'~,
 ~'~,~'~,~
"],
        &[],
    )
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(TANK),
    Plant::sprite(
        "marimo",
        &[r"
  .-~~-.
 (~~~~~~)
  `-~~-'
"],
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
 \|/ \|/
 \|/ \|/
  | | |
  \ | /
   \|/
  [___]
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
    .biomes(TANK),
    Plant::sprite(
        "hairgrass",
        &[
            r"
 ' ' ' ' '
 | | | | |
 | | | | |
",
            r"
  ' ' ' ' '
  | | | | |
 | | | | |
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
        &[r"
  \ \|/ /
  \\\|/ /
   \\\|/
    \\|
     \|
      |
"],
        &[],
    )
    .color('g')
    .fps(0.25)
    .weight(4)
    .biomes(TANK | FRESH),
];
