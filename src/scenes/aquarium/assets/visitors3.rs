//! Passers-by: boats, birds, whales, balloons and stranger things, continued.
//! Drawn in Unicode: blocks for the bodies, lines for wings and rigging.

use super::*;

pub const VISITORS: &[Visitor] = &[
    Visitor::new(
        "seagulls",
        Lane::Sky,
        &[
            r"
   ╲▾╱????????╱▾╲
        ─▾─
 ◜▾◝??????????????╱▾╲
           ╲▾╱
",
            r"
   ─▾─????????◜▾◝
        ╱▾╲
 ╲▾╱??????????????◜▾◝
           ─▾─
",
            r"
   ╱▾╲????????╲▾╱
        ◜▾◝
 ─▾─??????????????╲▾╱
           ╱▾╲
",
            r"
   ◜▾◝????????─▾─
        ╲▾╱
 ╱▾╲??????????????─▾─
           ◜▾◝
",
        ],
        &["\n\n\n\n", "\n\n\n\n", "\n\n\n\n", "\n\n\n\n"],
    )
    .speed(0.8, 1.6)
    .fps(0.25)
    .weight(8)
    .traits(PLUNGE)
    .biomes(OCEAN | KELP | REEF | ARCTIC | FRESH),
    Visitor::new(
        "gull-pair",
        Lane::Sky,
        &[
            r"
╲▾╱
      ─▾─
",
            r"
─▾─
      ◜▾◝
",
            r"
╱▾╲
      ╲▾╱
",
            r"
◜▾◝
      ─▾─
",
        ],
        &["\n\n", "\n\n", "\n\n", "\n\n"],
    )
    .speed(0.6, 1.2)
    .fps(0.25)
    .weight(6)
    .traits(PLUNGE)
    .biomes(OCEAN | KELP | REEF | ARCTIC | FRESH),
    Visitor::new(
        "pelican",
        Lane::Sky,
        &[
            r"
     ╱╲
   ╱▒▒▒╲
╱▓▒▒▒▒▒▒▜█████████▙▃◖●▀▀▀▀▀◥
 ▔▀▀▀▀▀▀▜████████▛▀╰━━━━━╯
?
",
            r"


▟▓▒▒▒▒▒▒▜█████████▙▃◖●▀▀▀▀▀◥
 ▀▀▀▀▀▀▀▜████████▛▀╰━━━━━╯
?
",
            r"


▃▃▃▃▃▃▃▃▜█████████▙▃◖●▀▀▀▀▀◥
▓▒▒▒▒▒▒▒▜████████▛▀╰━━━━━╯
  ╲▒▒▒▒╱
",
            r"


▟▓▒▒▒▒▒▒▜█████████▙▃◖●▀▀▀▀▀◥
 ▀▀▀▀▀▀▀▜████████▛▀╰━━━━━╯
?
",
        ],
        &[
            "\n\n\n                     kYYYYYY\n                   YYYYYYY\n\n",
            "\n\n\n                     kYYYYYY\n                   YYYYYYY\n\n",
            "\n\n\n                     kYYYYYY\n                   YYYYYYY\n\n",
            "\n\n\n                     kYYYYYY\n                   YYYYYYY\n\n",
        ],
    )
    .speed(0.7, 1.2)
    .fps(0.2)
    .weight(4)
    .traits(PLUNGE)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "albatross",
        Lane::Sky,
        &[
            r"
 ╲
  ╲╲
  ╲▁▁▂▂▃▃▄▄▅▅▆▇▜████████▙▃◖●▀▀◥
               ▝▀▀▀▀▀▀▀▀▀▘
?
",
            r"


▁▁▂▂▃▃▄▄▅▅▆▆▇▇▇▜████████▙▃◖●▀▀◥
               ▝▀▀▀▀▀▀▀▀▀▘
?
",
            r"


   ▂▃▃▄▄▅▅▆▆▇▇▇▜████████▙▃◖●▀▀◥
  ▗▞▔▔▔▔▔▔▔▔▔▔▔▝▀▀▀▀▀▀▀▀▀▘
▗▞
",
            r"


▁▁▂▂▃▃▄▄▅▅▆▆▇▇▇▜████████▙▃◖●▀▀◥
               ▝▀▀▀▀▀▀▀▀▀▘
?
",
        ],
        &[
            "\n\n\n                           kYYY\n\n\n",
            "\n\n\n                           kYYY\n\n\n",
            "\n\n\n                           kYYY\n\n\n",
            "\n\n\n                           kYYY\n\n\n",
        ],
    )
    .speed(0.9, 1.6)
    .fps(0.2)
    .weight(3)
    .traits(PLUNGE)
    .biomes(OCEAN | ARCTIC),
    Visitor::new(
        "plane-banner",
        Lane::Sky,
        &[
            r"
                               ◢◣?????▗▄▄▖
╭─────────────────╮????????????◢██▙▂▂▟████▙▃
│ OXIPER AQUARIUM │╌╌╌╌╌╌╌╌╌╌◥▄██████████████▙▶│
╰─────────────────╯??????????????▝▀▘???▝▀▘
",
            r"
                               ◢◣?????▗▄▄▖
╭─────────────────╮????????????◢██▙▂▂▟████▙▃
│ OXIPER AQUARIUM │╌╌╌╌╌╌╌╌╌╌◥▄██████████████▙▶╱
╰─────────────────╯??????????????▝▀▘???▝▀▘
",
            r"
                               ◢◣?????▗▄▄▖
╭─────────────────╮????????????◢██▙▂▂▟████▙▃
│ OXIPER AQUARIUM │╌╌╌╌╌╌╌╌╌╌◥▄██████████████▙▶─
╰─────────────────╯??????????????▝▀▘???▝▀▘
",
            r"
                               ◢◣?????▗▄▄▖
╭─────────────────╮????????????◢██▙▂▂▟████▙▃
│ OXIPER AQUARIUM │╌╌╌╌╌╌╌╌╌╌◥▄██████████████▙▶╲
╰─────────────────╯??????????????▝▀▘???▝▀▘
",
        ],
        &[
            r"
                               RR     CCCC
wwwwwwwwwwwwwwwwwww            RRRRwwwwwwwww
w YYYYYYYYYYYYYYY wRRRRRRRRRRRRwwwwwwwwwwwwwwwwk
wwwwwwwwwwwwwwwwwww              kkk   kkk
",
            r"
                               RR     CCCC
wwwwwwwwwwwwwwwwwww            RRRRwwwwwwwww
w YYYYYYYYYYYYYYY wRRRRRRRRRRRRwwwwwwwwwwwwwwwwk
wwwwwwwwwwwwwwwwwww              kkk   kkk
",
            r"
                               RR     CCCC
wwwwwwwwwwwwwwwwwww            RRRRwwwwwwwww
w YYYYYYYYYYYYYYY wRRRRRRRRRRRRwwwwwwwwwwwwwwwwk
wwwwwwwwwwwwwwwwwww              kkk   kkk
",
            r"
                               RR     CCCC
wwwwwwwwwwwwwwwwwww            RRRRwwwwwwwww
w YYYYYYYYYYYYYYY wRRRRRRRRRRRRwwwwwwwwwwwwwwwwk
wwwwwwwwwwwwwwwwwww              kkk   kkk
",
        ],
    )
    .left(
        &[
            r"
      ▗▄▄▖?????◢◣
    ▃▟████▙▂▂▟██◣????????????╭─────────────────╮
│◀▟██████████████▄◤╌╌╌╌╌╌╌╌╌╌│ OXIPER AQUARIUM │
      ▝▀▘???▝▀▘??????????????╰─────────────────╯
",
            r"
      ▗▄▄▖?????◢◣
    ▃▟████▙▂▂▟██◣????????????╭─────────────────╮
╲◀▟██████████████▄◤╌╌╌╌╌╌╌╌╌╌│ OXIPER AQUARIUM │
      ▝▀▘???▝▀▘??????????????╰─────────────────╯
",
            r"
      ▗▄▄▖?????◢◣
    ▃▟████▙▂▂▟██◣????????????╭─────────────────╮
─◀▟██████████████▄◤╌╌╌╌╌╌╌╌╌╌│ OXIPER AQUARIUM │
      ▝▀▘???▝▀▘??????????????╰─────────────────╯
",
            r"
      ▗▄▄▖?????◢◣
    ▃▟████▙▂▂▟██◣????????????╭─────────────────╮
╱◀▟██████████████▄◤╌╌╌╌╌╌╌╌╌╌│ OXIPER AQUARIUM │
      ▝▀▘???▝▀▘??????????????╰─────────────────╯
",
        ],
        &[
            r"
      CCCC     RR
    wwwwwwwwwRRRR            wwwwwwwwwwwwwwwwwww
kwwwwwwwwwwwwwwwwRRRRRRRRRRRRw YYYYYYYYYYYYYYY w
      kkk   kkk              wwwwwwwwwwwwwwwwwww
",
            r"
      CCCC     RR
    wwwwwwwwwRRRR            wwwwwwwwwwwwwwwwwww
kwwwwwwwwwwwwwwwwRRRRRRRRRRRRw YYYYYYYYYYYYYYY w
      kkk   kkk              wwwwwwwwwwwwwwwwwww
",
            r"
      CCCC     RR
    wwwwwwwwwRRRR            wwwwwwwwwwwwwwwwwww
kwwwwwwwwwwwwwwwwRRRRRRRRRRRRw YYYYYYYYYYYYYYY w
      kkk   kkk              wwwwwwwwwwwwwwwwwww
",
            r"
      CCCC     RR
    wwwwwwwwwRRRR            wwwwwwwwwwwwwwwwwww
kwwwwwwwwwwwwwwwwRRRRRRRRRRRRw YYYYYYYYYYYYYYY w
      kkk   kkk              wwwwwwwwwwwwwwwwwww
",
        ],
    )
    .speed(1.4, 2.6)
    .fps(0.5)
    .weight(5)
    .biomes(OCEAN | REEF | KELP | FRESH | TANK),
    Visitor::new(
        "helicopter",
        Lane::Sky,
        &[
            r"
        ━━━━━━━━┳━━━━━━━━
                ┃
╿━━━━━━━━━▃▅▇████████▛▀◥
          ◥▀▜░░░░░░▒▒▛▀◤
           ╺━━┻━━━━┻━━╸
",
            r"
           ─────┳─────
                ┃
╽━━━━━━━━━▃▅▇████████▛▀◥
          ◥▀▜░░░░░░▒▒▛▀◤
           ╺━━┻━━━━┻━━╸
",
        ],
        &[
            r"
        kkkkkkkkkkkkkkkkk
                k
kkkkkkkkkkRRRRRRRRRRRRRR
          RRRCCCCCCCCRRR
           kkkkkkkkkkkk
",
            r"
           kkkkkkkkkkk
                k
kkkkkkkkkkRRRRRRRRRRRRRR
          RRRCCCCCCCCRRR
           kkkkkkkkkkkk
",
        ],
    )
    .speed(1.2, 2.2)
    .fps(0.5)
    .weight(4)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "hot-air-balloon",
        Lane::Sky,
        &[
            r"
   ▗▄▟█▙▄▖
 ▗▟███████▙▖
▟█████████████▙
  ◥▀▀▀▀▀▀▀◤
    ▐▄▲▄▌
",
            r"
   ▗▄▟█▙▄▖
 ▗▟███████▙▖
▟█████████████▙
  ◥▀▀▀▀▀▀▀◤
    ▐▄△▄▌
",
        ],
        &[
            r"
   2211122
 12221112221
1122211122211
  222111222
    yyRyy
",
            r"
   2211122
 12221112221
1122211122211
  222111222
    yyYyy
",
        ],
    )
    .tint("1:rRbBgm 2:yYwWcm")
    .speed(0.2, 0.5)
    .fps(0.2)
    .weight(4)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "blimp",
        Lane::Sky,
        &[
            r"
    ◢◤????▗▄▄▟██████████████▙▄▄▖
   ◢██▙▗▟████████████████████████▙▖
   ◥██▐████████████████████████████▌
    ◥◣??▝▀▜████████████████████▛▀▘
                ▐▫◻▫◻▫◻▫◻▌
",
            r"
    ◢◤????▗▄▄▟██████████████▙▄▄▖
   ◢██▙▗▟████████████████████████▙▖
   ◥▓▓▐████████████████████████████▌
    ◥◣??▝▀▜████████████████████▛▀▘
                ▐◻▫◻▫◻▫◻▫▌
",
        ],
        &[
            r"
    11
   1111
   111
    11
                 CCCCCCCC
",
            r"
    11
   1111
   111
    11
                 CCCCCCCC
",
        ],
    )
    .tint("1:rRbBgG")
    .speed(0.3, 0.6)
    .fps(0.3)
    .weight(3)
    .biomes(OCEAN | FRESH),
    Visitor::new(
        "kite",
        Lane::Sky,
        &[
            r"
            ◢◣
           ◢██◣
           ◥██◤
          ╭╌◥◤
  ╌╌◇╌╌◇╌╌╯
",
            r"
            ◢◣
           ◢██◣
           ◥██◤
          ╭╌◥◤
  ╌╌╌◇╌╌◇╌╯
",
        ],
        &[
            r"
            12
           1122
           1122
            12
    R  R
",
            r"
            12
           1122
           1122
            12
     R  R
",
        ],
    )
    .tint("1:rRyYbB 2:wWyY")
    .speed(0.5, 1.0)
    .fps(0.3)
    .weight(3)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "ufo",
        Lane::Sky,
        &[
            r"
      ▗▄▄▄▄▄▄▖
    ▗▟▒▒▒◉◉▒▒▒▙▖
▂▄▆██████████████▆▄▂
▝▀▜██◉██◉██◉██◉██▛▀▘
     ▝▀▀▀▀▀▀▀▀▘
",
            r"
      ▗▄▄▄▄▄▄▖
    ▗▟▒▒▒◉◉▒▒▒▙▖
▂▄▆██████████████▆▄▂
▝▀▜██○██○██○██○██▛▀▘
     ▝▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
      GGGGGGGG
    GGGGGGYYGGGG
wwwwwwwwwwwwwwwwwwww
     Y  Y  Y  Y
     wwwwwwwwww
",
            r"
      GGGGGGGG
    GGGGGGYYGGGG
wwwwwwwwwwwwwwwwwwww
     Y  Y  Y  Y
     wwwwwwwwww
",
        ],
    )
    .speed(1.5, 3.5)
    .fps(0.4)
    .weight(1)
    .traits(GLOW | LEGENDARY)
    .biomes(OCEAN | ARCTIC),
    Visitor::new(
        "gannet",
        Lane::Sky,
        &[
            r"
 ╲╲            ╱╱
   ╲╲        ╱╱
  ◀▃▃▟██████▙▃◖●▬▶
     ▝▀▀▀▀▀▀▘
?
",
            r"
?
 ▁▂▃▄▅▆▇▇▇▇▆▅▄▃▂▁
  ◀▃▃▟██████▙▃◖●▬▶
     ▝▀▀▀▀▀▀▘
?
",
            r"
?
?
  ◀▃▃▟██████▙▃◖●▬▶
   ╱╱▝▀▀▀▀▀▀▘╲╲
 ╱╱            ╲╲
",
            r"
?
 ▁▂▃▄▅▆▇▇▇▇▆▅▄▃▂▁
  ◀▃▃▟██████▙▃◖●▬▶
     ▝▀▀▀▀▀▀▘
?
",
        ],
        &[
            r"
 kk            kk

              Yk
",
            r"

 kk            kk
              Yk
",
            r"


              Yk

 kk            kk
",
            r"

 kk            kk
              Yk
",
        ],
    )
    .speed(1.0, 1.8)
    .fps(0.25)
    .weight(5)
    .traits(PLUNGE)
    .biomes(OCEAN | KELP | ARCTIC),
    Visitor::new(
        "kingfisher",
        Lane::Sky,
        &[
            r"
 ▚        ▞
  ▚      ▞
 ◀▃▟████▙▃◖●▬▬▶
   ▝▀▀▀▀▘
?
",
            r"
?
▁▃▅▇▇▇▇▇▇▅▃▁
 ◀▃▟████▙▃◖●▬▬▶
   ▝▀▀▀▀▘
?
",
            r"
?
?
 ◀▃▟████▙▃◖●▬▬▶
  ▞▝▀▀▀▀▘▚
 ▞        ▚
",
            r"
?
▁▃▅▇▇▇▇▇▇▅▃▁
 ◀▃▟████▙▃◖●▬▬▶
   ▝▀▀▀▀▘
?
",
        ],
        &[
            r"
 B        B
  B      B
 BBBBBBBBBBkkkk
   YYYYYY
",
            r"

BBBBBBBBBBBB
 BBBBBBBBBBkkkk
   YYYYYY
",
            r"


 BBBBBBBBBBkkkk
  BYYYYYYB
 B        B
",
            r"

BBBBBBBBBBBB
 BBBBBBBBBBkkkk
   YYYYYY
",
        ],
    )
    .speed(0.9, 1.6)
    .fps(0.4)
    .weight(5)
    .traits(PLUNGE)
    .biomes(FRESH),
    Visitor::new(
        "osprey",
        Lane::Sky,
        &[
            r"
  ╲╲╲              ╱╱╱
    ╲╲╲          ╱╱╱
   ◀▃▃▃▟██████▙▃◖●▀◥
       ▝▀▀▀▀▀▀▘
?
",
            r"
?
 ▁▂▃▄▅▆▇▇▇▇▇▇▇▇▆▅▄▃▂▁
   ◀▃▃▃▟██████▙▃◖●▀◥
       ▝▀▀▀▀▀▀▘
?
",
            r"
?
?
   ◀▃▃▃▟██████▙▃◖●▀◥
    ╱╱╱▝▀▀▀▀▀▀▘╲╲╲
  ╱╱╱            ╲╲╲
",
            r"
?
 ▁▂▃▄▅▆▇▇▇▇▇▇▇▇▆▅▄▃▂▁
   ◀▃▃▃▟██████▙▃◖●▀◥
       ▝▀▀▀▀▀▀▘
?
",
        ],
        &[
            r"
  yyy              yyy
    yyy          yyy
   yyyyyyyyyyyyyWkkk
       WWWWWWWW
",
            r"

 yyyyyyyyyyyyyyyyyyyy
   yyyyyyyyyyyyyWkkk
       WWWWWWWW
",
            r"


   yyyyyyyyyyyyyWkkk
    yyyWWWWWWWWyyy
  yyy            yyy
",
            r"

 yyyyyyyyyyyyyyyyyyyy
   yyyyyyyyyyyyyWkkk
       WWWWWWWW
",
        ],
    )
    .speed(0.8, 1.4)
    .fps(0.2)
    .weight(4)
    .traits(PLUNGE)
    .biomes(FRESH | KELP | OCEAN),
    Visitor::new(
        "brown-pelican",
        Lane::Sky,
        &[
            r"
     ╱╲
   ╱▒▒▒╲
╱▓▒▒▒▒▒▒▜▓▓▓▓▓▓▓▓▓▙▃◖●▀▀▀▀▀◥
 ▔▀▀▀▀▀▀▜▒▒▒▒▒▒▒▒▛▀╰━━━━━╯
?
",
            r"
?
?
▟▓▒▒▒▒▒▒▜▓▓▓▓▓▓▓▓▓▙▃◖●▀▀▀▀▀◥
 ▀▀▀▀▀▀▀▜▒▒▒▒▒▒▒▒▛▀╰━━━━━╯
?
",
            r"
?
?
▃▃▃▃▃▃▃▃▜▓▓▓▓▓▓▓▓▓▙▃◖●▀▀▀▀▀◥
▓▒▒▒▒▒▒▒▜▒▒▒▒▒▒▒▒▛▀╰━━━━━╯
  ╲▒▒▒▒╱
",
            r"
?
?
▟▓▒▒▒▒▒▒▜▓▓▓▓▓▓▓▓▓▙▃◖●▀▀▀▀▀◥
 ▀▀▀▀▀▀▀▜▒▒▒▒▒▒▒▒▛▀╰━━━━━╯
?
",
        ],
        &[r"


yyyyyyyyyyyyyyyyyyyyWkYYYYYY
 yyyyyyyyyyyyyyyyyyYYYYYYY
"],
    )
    .speed(0.7, 1.2)
    .fps(0.2)
    .weight(4)
    .traits(PLUNGE)
    .biomes(OCEAN | KELP | REEF),
    Visitor::new(
        "puffin",
        Lane::Sky,
        &[
            r"
▚        ▞
 ▚      ▞
 ◀▟████▙◖●▜▶
  ▝▀▀▀▀▘
?
",
            r"
?
▃▅▇▇▇▇▇▇▅▃
 ◀▟████▙◖●▜▶
  ▝▀▀▀▀▘
?
",
            r"
?
?
 ◀▟████▙◖●▜▶
 ▞▝▀▀▀▀▘▚
▞        ▚
",
            r"
?
▃▅▇▇▇▇▇▇▅▃
 ◀▟████▙◖●▜▶
  ▝▀▀▀▀▘
?
",
        ],
        &[
            r"
K        K
 K      K
 KKKKKKKWkRR
  WWWWWW
",
            r"

KKKKKKKKKK
 KKKKKKKWkRR
  WWWWWW
",
            r"


 KKKKKKKWkRR
 KWWWWWWK
K        K
",
            r"

KKKKKKKKKK
 KKKKKKKWkRR
  WWWWWW
",
        ],
    )
    .speed(1.0, 1.7)
    .fps(0.4)
    .weight(5)
    .traits(PLUNGE)
    .biomes(ARCTIC | KELP),
    Visitor::new(
        "humpback-with-calf",
        Lane::Deep,
        &[
            r"
                       ▂▄▆██▆▄▂
◥█◣?????????????▁▃▅▇██████████████▇▅▃▂
 ◥█▙▂????▁▃▅▇████████████████████████▇▅▃▁
  ▝▀▜█▆▃▂▃▅▇████████████████████████████▇▅▃▂
 ◢█▛▔????▔▀▀▜██████████████████████◉█▛▀▀▔
◢█◤?????????????▔▀▀▜██████████████▛▀▀▔
              ◢██▙▃▃▂??▔▀▜██▛▀▔
            ◢██▀▀▀◤??????????▂▅▇███▇▅▂
           ◢▀◤???????◥█◣?▁▃▅▇██████████▇▅▃▁
                      ▝▀▜█▆▃▅▇█████████◉██▇▃
                     ◢█◤?▔▀▀▜██████████▛▀▀▔
",
            r"
                       ▂▄▆██▆▄▂
  ◢◣????????????▁▃▅▇██████████████▇▅▃▂
 ◢█▙▂????▁▃▅▇████████████████████████▇▅▃▁
  ▝▀▜█▆▃▂▃▅▇████████████████████████████▇▅▃▂
 ◥█▛▔????▔▀▀▜██████████████████████◉█▛▀▀▔
  ◥◤????????????▔▀▀▜██████████████▛▀▀▔
              ◢██▙▃▃▂??▔▀▜██▛▀▔
             ◢██▀▀◤??????????▂▅▇███▇▅▂
            ◢▀◤??????◢█▙?▁▃▅▇██████████▇▅▃▁
                      ▝▀▜█▆▃▅▇█████████◉██▇▃
                     ◥█▛?▔▀▀▜██████████▛▀▀▔
",
        ],
        &[
            r"




 BBBB    BBBBBBBBBBBBBBBBBBBBBBBBBBWBBBBB
BBB             BBBBBBBBBBBBBBBBBBBBBB
              BBBBBBB  BBBBBBBB


                                       W
                     BBB BBBBBBBBBBBBBBBBBB
",
            r"




 BBBB    BBBBBBBBBBBBBBBBBBBBBBBBBBWBBBBB
  BB            BBBBBBBBBBBBBBBBBBBBBB
              BBBBBBB  BBBBBBBB


                                       W
                     BBB BBBBBBBBBBBBBBBBBB
",
        ],
    )
    .color('b')
    .speed(0.7, 1.2)
    .fps(0.2)
    .weight(3)
    .traits(FILTER)
    .biomes(OCEAN | REEF | KELP | ARCTIC),
    Visitor::new(
        "orca-pod",
        Lane::Deep,
        &[
            r"
                                         ◣
                              ◥█◣????????▐█◣
                               ◥█▙▂▁▃▅▇██████████████▇▆▅▃
                                ▝▀▜█▆▄▅▇████░░░████░●░▇▆▃
           ◣???????????????????◢█▛▔▔▀▀▜▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▔▔
  ◢◣???????▐█◣????????????????◢█◤
 ◢█▙▂▁▃▅▇██████████████▇▆▅▃
  ▝▀▜█▆▄▅▇████░░░████░●░▇▆▃????????????????◣
 ◥█▛▔▔▀▀▜▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▔▔?????◥█◣????????▐█◣
  ◥◤?????????????????????????????◥█▙▂▁▃▅▇██████████████▇▆▅▃
                                  ▝▀▜█▆▄▅▇████░░░████░●░▇▆▃
                                 ◢█▛▔▔▀▀▜▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▔▔
                                ◢█◤
",
            r"
                                         ◣
                                ◢◣???????▐█◣
                               ◢█▙▂▁▃▅▇██████████████▇▆▅▃
                                ▝▀▜█▆▄▅▇████░░░████░●░▇▆▃
           ◣???????????????????◥█▛▔▔▀▀▜▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▔▔
◥█◣????????▐█◣??????????????????◥◤
 ◥█▙▂▁▃▅▇██████████████▇▆▅▃
  ▝▀▜█▆▄▅▇████░░░████░●░▇▆▃????????????????◣
 ◢█▛▔▔▀▀▜▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▔▔???????◢◣???????▐█◣
◢█◤??????????????????????????????◢█▙▂▁▃▅▇██████████████▇▆▅▃
                                  ▝▀▜█▆▄▅▇████░░░████░●░▇▆▃
                                 ◥█▛▔▔▀▀▜▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▔▔
                                  ◥◤
",
        ],
        &[
            r"



                                            WWW    WkW
                                       WWWWWWWWWWWWWW


              WWW    WkW
         WWWWWWWWWWWWWW

                                              WWW    WkW
                                         WWWWWWWWWWWWWW

",
            r"



                                            WWW    WkW
                                       WWWWWWWWWWWWWW


              WWW    WkW
         WWWWWWWWWWWWWW

                                              WWW    WkW
                                         WWWWWWWWWWWWWW

",
        ],
    )
    .color('K')
    .speed(1.0, 1.8)
    .fps(0.25)
    .weight(2)
    .traits(PREDATOR | SCARY)
    .biomes(OCEAN | ARCTIC | KELP),
    Visitor::new(
        "panga",
        Lane::Boat,
        &[
            r"
                     ╱
          ◉?????????╱
         ▟█▙???????╱
▐▌???????▐█▌╺━━━━━╱
▝▘▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃◢
≈∼▝▜██████████████▛▘
",
            r"
                     ╱
          ◉?????????╱
         ▟█▙???????╱
▐▌???????▐█▌╺━━━━━╱
▝▘▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃▃◢
∼≈▝▜██████████████▛▘
",
        ],
        &[
            r"
                     k
          Y         k
         yyy       k
kk       yyykkkkkkk
kk111111111111111111
CC111111111111111111
",
            r"
                     k
          Y         k
         yyy       k
kk       yyykkkkkkk
kk111111111111111111
CC111111111111111111
",
        ],
    )
    .tint("1:wWcCyY")
    .speed(0.3, 0.7)
    .fps(0.25)
    .weight(7)
    .traits(FISHING)
    .biomes(OCEAN | REEF | KELP),
    Visitor::new(
        "shrimp-trawler",
        Lane::Boat,
        &[
            r"
  ╲???????????┃???????????╱
  ┆╲??????????┃??▗▄▄▖????╱┆
  ┆?╲?????????┃??▐▒▒▌???╱?┆
  ┆??╲????????┃??▐██▌??╱??┆
  ┆?▟███◦██◦██◦█████████▙?┆
  ┆??▝▜███████████████▛▘??┆
",
            r"
  ╲???????????┃???????????╱
  ┊╲??????????┃??▗▄▄▖????╱┊
  ┊?╲?????????┃??▐░░▌???╱?┊
  ┊??╲????????┃??▐██▌??╱??┊
  ┊?▟███◦██◦██◦█████████▙?┊
  ┊??▝▜███████████████▛▘??┊
",
        ],
        &[
            r"
  w
  cw          w  wwww    wc
  c w         w  wYYw   w c
  c  w        w  wwww  w  c
  c 1111W11W11W1111111111 c
  c  1111111111111111111  c
",
            r"
  w
  cw          w  wwww    wc
  c w         w  wYYw   w c
  c  w        w  wwww  w  c
  c 1111W11W11W1111111111 c
  c  1111111111111111111  c
",
        ],
    )
    .tint("1:rRbBgG")
    .speed(0.3, 0.6)
    .fps(0.15)
    .weight(6)
    .traits(NET | SCARY)
    .biomes(OCEAN | REEF),
    Visitor::new(
        "purse-seiner",
        Lane::Boat,
        &[
            r"
         ┃
         ┣━━━━━┓??????▗▄▄▄▖
         ┃?????┃??????▐▒▒▒▌
    ▗▄▄▖?┃?????◆?░░░??▐███▌
  ▗▟████◦███◦███◦███◦████▙▖
    ▝▜█████████████████▛▘
",
            r"
         ┃
         ┣━━━━━┓??????▗▄▄▄▖
         ┃?????┃??????▐░░░▌
    ▗▄▄▖?┃?????◆?░░░??▐███▌
  ▗▟████◦███◦███◦███◦████▙▖
    ▝▜█████████████████▛▘
",
        ],
        &[
            r"

         wwwwwww      wwwww
         w     w      wYYYw
    RRRR w     R ccc  wwwww
  111111W111W111W111W111111
    111111111111111111111
",
            r"

         wwwwwww      wwwww
         w     w      wYYYw
    RRRR w     R ccc  wwwww
  111111W111W111W111W111111
    111111111111111111111
",
        ],
    )
    .tint("1:rRbBgG")
    .speed(0.25, 0.5)
    .fps(0.2)
    .weight(6)
    .traits(NET | SCARY)
    .biomes(OCEAN | KELP | ARCTIC),
    Visitor::new(
        "sport-fishing-boat",
        Lane::Boat,
        &[
            r"
        ▗▄▄▄▄▖???????????????╱
        ▐▒▒▒▒▌??????????????╱
     ▗▟▀▀▀▀▀▀▙▖????????????╱
     ▐▒▒▒▒▒▒▒▒▌▁▁▁▁▁▁▁▁▁▁▁╱
 ▗▟███◦███◦███◦███◦███◦███▙◣
≈∼≈▝▜███████████████████▛▘
",
            r"
        ▗▄▄▄▄▖???????????????╱
        ▐░░░░▌??????????????╱
     ▗▟▀▀▀▀▀▀▙▖????????????╱
     ▐░░░░░░░░▌▁▁▁▁▁▁▁▁▁▁▁╱
 ▗▟███◦███◦███◦███◦███◦███▙◣
∼≈∼▝▜███████████████████▛▘
",
        ],
        &[
            r"
        wwwwww               k
        wCCCCw              k
     wwwwwwwwww            k
     wCCCCCCCCwwwwwwwwwwwwk
 11111W111W111W111W111W11111
CCC11111111111111111111111
",
            r"
        wwwwww               k
        wCCCCw              k
     wwwwwwwwww            k
     wCCCCCCCCwwwwwwwwwwwwk
 11111W111W111W111W111W11111
CCC11111111111111111111111
",
        ],
    )
    .tint("1:bBcCrR")
    .speed(0.35, 0.8)
    .fps(0.25)
    .weight(7)
    .traits(FISHING | SCARY)
    .biomes(OCEAN | REEF),
    Visitor::new(
        "lobster-boat",
        Lane::Boat,
        &[
            r"
            ╽????????????????╱
            ┃???????????????╱
         ▗▄▄▄▄▄▖???????????╱
 ▛▀▜▛▀▜??▐▒▒▒▒▒▌▁▁▁▁▁▁▁▁▁▁╱
▗▟██████◦██◦██◦████████████▙◣
≈∼▝▜█████████████████████▛▘
",
            r"
            ╿????????????????╱
            ┃???????????????╱
         ▗▄▄▄▄▄▖???????????╱
 ▛▀▜▛▀▜??▐░░░░░▌▁▁▁▁▁▁▁▁▁▁╱
▗▟██████◦██◦██◦████████████▙◣
∼≈▝▜█████████████████████▛▘
",
        ],
        &[
            r"
            k                k
            k               k
         wwwwwww           k
 yyyyyy  wCCCCCwwwwwwwwwwwk
11111111W11W11W11111111111111
CC1111111111111111111111111
",
            r"
            k                k
            k               k
         wwwwwww           k
 yyyyyy  wCCCCCwwwwwwwwwwwk
11111111W11W11W11111111111111
CC1111111111111111111111111
",
        ],
    )
    .tint("1:wWbBgG")
    .speed(0.25, 0.6)
    .fps(0.25)
    .weight(7)
    .traits(FISHING)
    .biomes(KELP | ARCTIC | OCEAN),
    Visitor::new(
        "rowboat-angler",
        Lane::Boat,
        &[
            r"
       ▃▄▃?????????╱
        ◉?????????╱
       ▟█▙???????╱
       ▐█▌╺━━━━━╱
 ◣▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄◢
  ▝▜███████████▛▘??≈
",
            r"
       ▃▄▃?????????╱
        ◉?????????╱
       ▟█▙???????╱
       ▐█▌╺━━━━━╱
 ◣▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄◢
  ▝▜███████████▛▘??∼
",
        ],
        &[
            r"
       yyy         k
        Y         k
       RRR       k
       RRRkkkkkkk
 11111111111111111
  111111111111111  C
",
            r"
       yyy         k
        Y         k
       RRR       k
       RRRkkkkkkk
 11111111111111111
  111111111111111  C
",
        ],
    )
    .tint("1:rRgGyY")
    .speed(0.2, 0.4)
    .fps(0.2)
    .weight(6)
    .traits(FISHING)
    .biomes(FRESH),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_visitors_drawn(super::VISITORS);
    }
}
