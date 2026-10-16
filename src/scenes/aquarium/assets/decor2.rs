//! Props and scenery for the sea floor, continued.

use super::*;

pub const DECOR: &[Decor] = &[
    Decor::new(
        "amphora-pair",
        &[r"
   ▄▄▄▄
  ╭▀██▀╮
 ╭╯▓██▓╰╮
 │▒▓██▓▒│  ╭──────╮ ▗▄▖
 ╰╮▒▓▓▒╭╯ ╭╯▒▓▓▓▒░░╰▄▟█▌
  ╰╮▒▒╭╯  ╰╮▒▓▓▒░░╭╯▀▜█▌
   ╰──╯    ╰──────╯ ▝▀▘
"],
        &[r"
   YYYY
  yYrrYy
 yyrrrryy
 yrrrrrry   yyyyyyyy yy
 yyrrrryy yyrrrrrrryyyyy
  yyrryy  yyrrrrrryyyyyy
   yyyy    yyyyyyyy yyy
"],
    )
    .color('y')
    .weight(5)
    .biomes(REEF | OCEAN | TANK)
    .traits(SHELTER),
    Decor::new(
        "atlantis-skyline",
        &[r"
                     ╻
        ▲          ╱▔▀▔╲          ▲
       ╱█╲      ▗▄▄▟▛◇◇▜▙▄▄▖     ╱█╲      ▲
  ▲    ▐█▌    ▗▟▀╱▔▔▔▔▔▔▔╲▀▙▖    ▐█▌     ╱█╲
 ╱█╲   ▐█▌    ▐▌ ▯  ▯  ▯  ▯ ▐▌   ▐█▌  ▲  ▐█▌
 ▐█▌▄▄ ▐█▌▄▄  ▐▌ ▯  ▯  ▯  ▯ ▐▌▄▄ ▐█▌ ╱█╲ ▐█▌
 ▐█▌██▄▐█▌██▄ ▐▌ │  │  │  │ ▐▌██▄▐█▌▄▐█▌▄▐█▌
▁▐█▌████████▜█▙▄▟▄▄▟▄▄▟▄▄▟▄▟█▛███████████▐█▌▁
"],
        &[r"
                     W
        w          cWWWc          w
       www      cccccyyccccc     www      w
  w    www    cccccccccccccccc   www     www
 www   www    cc wc  wc  wc  wc cc   www  w
 www cc www cc  cc w  w  w  w cc cc www www
 wwwwwww wwww  cc c  c  c  c  cc wwwww www w
"],
    )
    .color('c')
    .weight(3)
    .biomes(REEF | OCEAN)
    .traits(SHELTER),
    Decor::new(
        "coral-wreck",
        &[
            r"
                       ╽
         ▗▟▙▖          ┃╲
        ▟▜██▙▖         ┃▒╲        ▗▟▙▖
       ▟██▜█▛▙▖     ⌠  ┃▒▒╲      ▟▜██▙▖
      ▐▜█▟▙█▜█▌  ▁▁▁⌡▁▁┃▁▁▁▁╲▁▁  ▐█▙▜█▌
      ▝▜▙██▟▛▜▄▄▟▀▔░░░░░░░░░░░▔▀▙▄▟▜██▛▘
    ▁▁▁▝▀▜▛▀▄▟▀╱ ░░ ◯ ░░░ ◯ ░░░ ╲▀▙▄▀▘▁▁
   ╱▒▓▒░░▒▒▓▒░▒▒▒░░▒▒▒▓▒░▒▒░▒▒▒▓▒░▒▒▓▒▒░╲
  ▕▒▒▓▒▒░▒▒░▒▒▒▓▒▒░▒▒░▒▒▒▒▓▒▒░▒▒░▒▒▓▒▒░▒▒▏
   ╲▂▃▂▂▁▂▃▂▂▃▂▁▂▂▃▂▂▃▂▂▁▃▂▂▂▃▂▂▃▂▂▁▃▂▂╱
    ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
",
            r"
                       ╽
         ▗▟▙▖          ┃╲
        ▟▜██▙▖         ┃▒╲        ▗▟▙▖
       ▟██▜█▛▙▖     ⌡  ┃▒▒╲      ▟▜██▙▖
      ▐▜█▟▙█▜█▌  ▁▁▁⌠▁▁┃▁▁▁▁╲▁▁  ▐█▙▜█▌
      ▝▜▙██▟▛▜▄▄▟▀▔░░░░░░░░░░░▔▀▙▄▟▜██▛▘
    ▁▁▁▝▀▜▛▀▄▟▀╱ ░░ ◯ ░░░ ◯ ░░░ ╲▀▙▄▀▘▁▁
   ╱▒▓▒░░▒▒▓▒░▒▒▒░░▒▒▒▓▒░▒▒░▒▒▒▓▒░▒▒▓▒▒░╲
  ▕▒▒▓▒▒░▒▒░▒▒▒▓▒▒░▒▒░▒▒▒▒▓▒▒░▒▒░▒▒▓▒▒░▒▒▏
   ╲▂▃▂▂▁▂▃▂▂▃▂▁▂▂▃▂▂▃▂▂▁▃▂▂▂▃▂▂▃▂▂▁▃▂▂╱
    ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
",
        ],
        &[r"
                       y
         mrrm          y
        rmmrrm         yy        rmmr
       mrrmmrrm     g  yyy      mrrmmr
      mrrmmrrmm     g   yyyyyyy   mrmmr
      rmmrrmmryyyyyyyyyyyyyyyyyyyyyyyyrm
    yyyyrmmryyyyyyyyyyyyyyyyyyyyyyyyyyyy
"],
    )
    .color('y')
    .fps(0.1)
    .weight(4)
    .biomes(REEF)
    .traits(SHELTER),
    Decor::new(
        "sunken-galleon",
        &[
            r"
                  ╿                            ╻
                 ╱┃▒╲                   ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
                ╱▒┃▒▒▖       ╿          ▐▛▀▀▀▀▀▀▀▀▀▀▀▀▀▀▜▌
               ╱▒▒┃▒ ▝       ┃          ▐▌▫ ▫ ▫ ▫ ▫ ▫ ▫ ▐▌
               ▔▔▔┃▔╲       ╱┃╲         ▐▌▁▁▁▁▁▁▁▁▁▁▁▁▁▁▐▌
     ╲            ┃          ┃    ⌠     ▐▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▌
       ╲ ▗▄▄▄▄▄▄▄▄┻▄▄▄▄▄▄▄▄▄▄┻▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▙▖
         ▗▟▀ ◯   ◯   ◯   ◯  ░░░░░░░░  ◯   ◯   ◯   ◯   ◯▜▙▖
        ▗▟▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▜▙
       ▝▙▟▓▒▒░░▒▒░▒▒▒░░▒▒▒▒░░▒▒░▒▒▒▒░░▒▒░▒▒▒▒░▒▒▒▒░░▒▓▟▛
         ▝▀▙▃▂▂▃▂▂▂▃▂▂▃▂▂▂▃▂▂▃▂▂▂▃▂▂▃▂▂▂▃▂▂▃▂▂▃▃▂▂▃▂▟▀▘
           ▔▔▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▔▔
",
            r"
                  ╿                            ╻
                 ╱┃▒╲                   ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
                ╱▒┃▒▒▖       ╿          ▐▛▀▀▀▀▀▀▀▀▀▀▀▀▀▀▜▌
               ╱▒▒┃▒ ▝       ┃          ▐▌▫ ▫ ▫ ▫ ▫ ▫ ▫ ▐▌
               ▔▔▔┃▔╲       ╱┃╲         ▐▌▁▁▁▁▁▁▁▁▁▁▁▁▁▁▐▌
     ╲            ┃          ┃    ⌡     ▐▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▌
       ╲ ▗▄▄▄▄▄▄▄▄┻▄▄▄▄▄▄▄▄▄▄┻▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▙▖
         ▗▟▀ ◯   ◯   ◯   ◯  ░░░░░░░░  ◯   ◯   ◯   ◯   ◯▜▙▖
        ▗▟▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▜▙
       ▝▙▟▓▒▒░░▒▒░▒▒▒░░▒▒▒▒░░▒▒░▒▒▒▒░░▒▒░▒▒▒▒░▒▒▒▒░░▒▓▟▛
         ▝▀▙▃▂▂▃▂▂▂▃▂▂▃▂▂▂▃▂▂▃▂▂▂▃▂▂▃▂▂▂▃▂▂▃▂▂▃▃▂▂▃▂▟▀▘
           ▔▔▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▔▔
",
        ],
        &[r"
                                               Y
                 wwww
                wwwwww
               wwwwwww                    C C C C C C C
               wwwwww
                                  g

             k   k   k   k            k   k   k   k   k
"],
    )
    .color('y')
    .fps(0.1)
    .weight(3)
    .biomes(OCEAN)
    .traits(SHELTER),
    Decor::new(
        "sunken-plane",
        &[
            r"
                                                ▗▟
                                               ▗▟▛
                   ▁▁▁▁▁▁                     ▗▟▛
                  ▟▒▒▒▒▒▒▙                   ▟▛
    ▁▁▂▃▄▄▄▄▄▄▄▄▟▀▀▀▀▀▀▀▀▜▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▛▀
  ▗▟▀╭──╮ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ▔▀▀▙▖
 ▐▌ ◉│  │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▜▌
  ▝▙▄╰──╯▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▛
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
 ⌠ ⌡⌠ ⌡  ⌠  ⌡⌠  ⌡ ⌠   ⌡ ⌠  ⌡⌠ ⌡  ⌠ ⌡⌠  ⌡ ⌠ ⌡
",
            r"
                                                ▗▟
                                               ▗▟▛
                   ▁▁▁▁▁▁                     ▗▟▛
                  ▟▒▒▒▒▒▒▙                   ▟▛
    ▁▁▂▃▄▄▄▄▄▄▄▄▟▀▀▀▀▀▀▀▀▜▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▛▀
  ▗▟▀╭──╮ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ◦ ▔▀▀▙▖
 ▐▌ ◉│  │░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▜▌
  ▝▙▄╰──╯▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▛
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
 ⌡ ⌠⌡ ⌠  ⌡  ⌠⌡  ⌠ ⌡   ⌠ ⌡  ⌠⌡ ⌠  ⌡ ⌠⌡  ⌠ ⌡ ⌠
",
        ],
        &[r"
                                                ww
                                               www
                   wwwwww                     www
                  wwwwwwww                   ww
    wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
  wwwwwwwcccccccc c c c c c c c c c c c c c c
 wwwCwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
  wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
   wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
 g gggg  g  gggg  g g   g g  gggg  g gggg  g
"],
    )
    .color('w')
    .fps(0.1)
    .weight(3)
    .biomes(OCEAN)
    .traits(SHELTER),
    Decor::new(
        "cargo-container",
        &[r"
   ┏━━━━━━━━━━━━━━━━━━━━━━━┳━━┓
   ┃╎╎╎╎╎╎╎╎░╎╎╎╎╎╎╎╎╎╎╎╎╎┃╟╢┃
   ┃╎╎╎╎╎╎╎╎╎╎╎╎╎╎▒╎╎╎╎╎╎╎┃╟╢┃
   ┗━━━━━━━━━━━━━━━━━━━━━━━┻━━┛
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━┳━━┓
┃╎╎░▒╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎┃╟╢┃
┃╎╎╎░╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎╎▒░╎╎╎╎┃╟╢┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━┻━━┛
"],
        &[r"
   wwwwwwwwwwwwwwwwwwwwwwwwwwww
   w11111111r11111111111111www
   w11111111111111r1111111wwww
   wwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
w11rr1111111111111111111111111w
w111r11111111111111111111rr1111
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
"],
    )
    .tint("1:rbygc")
    .weight(4)
    .biomes(OCEAN)
    .traits(SHELTER),
    Decor::new(
        "lighthouse-ruin",
        &[r"
       ▗▄▖ ▄▖▗▄
       ▐▛▙▟▜▌ ▜▌
       ▐▌ ?  ▐▌▌
      ▗▟▀▀▀▀▀▀▀▙▖
      ▐▌ ▯    ▒▐▌
     ▗▟▀▀▀▀▀▀▀▀▀▙▖
     ▐▌   ▯    ▒▐▌
    ▗▟▀▀▀▀▀▀▀▀▀▀▀▙▖
    ▐▌     ▯     ▐▌
   ▗▟▀▀▀▀▀▀▀▀▀▀▀▀▀▙▖
  ▗▟▒░▒▓▒░▒▒▓▒░▒▒▓▒▙▖
 ▟▓▒▒▓▒▒▓░▒▒▓▒▒▓▒▓▒▒▓▙
"],
        &[r"
        Y
       YYYYYY
       Y
      RRRRRRRRRR
      RRRRRRRRRR
     RRRRRRRRRRRRR
     RRRRRRRRRRRRR
    wwwwwwwwwwwwwww
    wwwwwwwwwwwwwww
   RRRRRRRRRRRRRRRRR
  kkkkkkkkkkkkkkkkkkk
 kkkkkkkkkkkkkkkkkkkkk
"],
    )
    .weight(3)
    .biomes(OCEAN | REEF | ARCTIC)
    .traits(SHELTER),
    Decor::new(
        "rock-pile",
        &[r"
         ▗▄▄▖
    ▗▄▄▟▀▓▒▒▀▙▖
  ▗▟▀▒▒▓▓▓▒░▒▒▀▙▄▖
 ▟▀▒▓▓▓▀▄▄▒▒▒▓▓▒▒▀▙▖
▐▓▒▒▒▓▓▌▐██▌▓▓▒▒░▒▒▓▜▙
▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
"],
        &[r"
         wwww
    kkkwwkkkkkk
  kkkkkkkkkkkkkkkk
 kkkkkkwkkkkkkkkkwkk
kkkkkkkkkkwwkkkkkkkkkk
"],
    )
    .color('k')
    .weight(8)
    .biomes(KELP | REEF | OCEAN | FRESH)
    .traits(SHELTER),
    Decor::new(
        "rowboat-wreck",
        &[
            r"
                  ╱
                 ╱  ⌠
 ▗▄▄▄▄▄▄▖▄▄▄▄▄▄╱▄▄▖▄▄▄▄▄▖
 ▐▀▒▒░░▒▀▀▀▀▀▀▀▀▀▀▀▒░░▒▒▌
  ▝▙▄▒▒▓▒▒▒░▒▒▒░▒▒▓▒▒▄▟▘
    ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
                  ╱
                 ╱  ⌡
 ▗▄▄▄▄▄▄▖▄▄▄▄▄▄╱▄▄▖▄▄▄▄▄▖
 ▐▀▒▒░░▒▀▀▀▀▀▀▀▀▀▀▀▒░░▒▒▌
  ▝▙▄▒▒▓▒▒▒░▒▒▒░▒▒▓▒▒▄▟▘
    ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
                  y
                 y  g
 yyyyyyyyyyyyyyyyyyyyyyyy
 yyrryyyyyyyyyyyyyyyyyyry
  yyyyyyyyyyyyyyyyyyyyyy
    yyyyyyyyyyyyyyyyyy
"],
    )
    .color('y')
    .fps(0.1)
    .weight(4)
    .biomes(KELP)
    .traits(SHELTER),
    Decor::new(
        "crab-trap",
        &[r"
   ╭─────────────╮
  ╱ ╳ ╳ ╳ ╳ ╳ ╳ ╱│
 ┌─────────────┐ │
 │ ╳ ╳ ╳ ╳ ╳ ╳ │ │
 │ ╳   ◠◠  ╳ ╳ │╱
 └───┴▄▄▄▄┴────┘
"],
        &[r"
   kkkkkkkkkkkkkkk
  k y y y y y y kk
 kkkkkkkkkkkkkkk k
 k y y y y y y k k
 k y   yy  y y kk
 kkkkkyyyyykkkkk
"],
    )
    .color('k')
    .weight(5)
    .biomes(KELP)
    .traits(SHELTER),
    Decor::new(
        "lobster-pot",
        &[r"
    ╭──╌╌╌──╮
  ╭╯│ │ │ │ │╰╮
 ╱  │ │ │ │ │  ╲
┣━━━╋━╋━╋━╋━╋━━━┫
│  ▗▙▟▙▟▙▟▙▟▖  │
╰─┴─────────┴─╯
"],
        &[r"
    yyyyyyyyy
  yyyyyyyyyyyyy
 yy  y y y y y
kkkkkkkkkkkkkkkkk
k  yyyyyyyyyyy
kkkkkkkkkkkkkkk
"],
    )
    .color('y')
    .weight(5)
    .biomes(KELP)
    .traits(SHELTER),
    Decor::new(
        "rock-arch",
        &[r"
          ▗▄▄▄▄▄▄▄▄▄▄▖
       ▗▄▟▀▒▒▓▓▒░▒▒▓▓▀▙▄▖
      ▟▀▒▒▓▓▀▀▀▀▀▀▀▀▓▓▒▒▀▙
     ▟▀▓▓▒▛▀?????????▀▜▒▓▓▀▙
    ▐▒▓▓▀▘???????????▝▀▓▓▒▒▌
   ▐▒▓▒▌?????????????▐▒▓▒▒▌
   ▐▓▒▒▌?????????????▐▒▒▓▒▌
  ▗▟▒▒▓▌?????????????▐▓▒▒▒▙▖
 ▟▀▀▀▀▀▘?????????????▝▀▀▀▀▀▙
"],
        &[r"
          wwwwwwww
       kkkwwkkkkkkkkwwk
      kkkkkkkkkkkkkkkkkkkk
     gkkkkkkk
"],
    )
    .color('k')
    .weight(3)
    .biomes(KELP | REEF)
    .traits(SHELTER),
    Decor::new(
        "cave-mouth",
        &[r"
         ▗▄▄▄▄▄▄▄▖
      ▗▄▟▀▒▒▓▒▒▓▒▀▙▄▖
    ▗▟▀▒▓▓▒▒░▒▒▓▓▒▒░▀▙▖
   ▟▀▒▓▗▄▄▄▄▄▄▄▄▄▖▓▒▒▀▙
  ▟▒▓▒▐▛▀       ▀▜▌▒▓▒▙
 ▐▓▒▒▌▛  ░░░░░░░  ▜▐▒▒▓▌
▟▀▀▀▀▘   ▒▒▒▒▒▒▒   ▝▀▀▀▀▙
"],
        &[r"
         wwwwwwww
      kkkwwkkkkwkkkkk
    kkkkkkkkkkkkkkkkkkk
   kkkkkKKKKKKKKKKkkkkk
  kkkkkKKK       KKKkkk
 kkkkkKK  KKKKKKK  KKkkk
"],
    )
    .color('k')
    .weight(3)
    .biomes(KELP | DEEP)
    .traits(SHELTER),
    Decor::new(
        "black-smoker",
        &[
            r"
       ░▒░
      ▒▓▓▒░
     ░▒▓▒▒░
       ░▒▓░
      ▗▟▀▀▙▖
       ▐▓▒▌
       ▐▒▓▌ ▗▖
      ▗▟▓▒▓▙▟▒▙▖
     ▟▓▒▓▓▒▓▒▓▓▙
   ▗▟▒▓▒░▒▓▒▓▒▒▓▙▖
 ▗▟▓▒▓▒▓░▒▓▒▓░▒▓▒▙▖
",
            r"
        ░▒░
     ░▒▓▒░
      ▒▓▒▒▒
      ░▒▓▒
      ▗▟▀▀▙▖
       ▐▓▒▌
       ▐▒▓▌ ▗▖
      ▗▟▓▒▓▙▟▒▙▖
     ▟▓▒▓▓▒▓▒▓▓▙
   ▗▟▒▓▒░▒▓▒▓▒▒▓▙▖
 ▗▟▓▒▓▒▓░▒▓▒▓░▒▓▒▙▖
",
            r"
      ░▒░
       ▒▓▒░▒
     ░▒▓▒▒▒░
      ░▒▓░▒
      ▗▟▀▀▙▖
       ▐▓▒▌
       ▐▒▓▌ ▗▖
      ▗▟▓▒▓▙▟▒▙▖
     ▟▓▒▓▓▒▓▒▓▓▙
   ▗▟▒▒▓▒░▒▓▒▓▒▒▓▙▖
 ▗▟▓▒▓▒▓░▒▓▒▓░▒▓▒▙▖
",
        ],
        &[
            r"
       KKK
      KKKKK
     KKKKKK
       KKKK
      kRRRRk
       kkkk
       kkkk kk
      yyyyyykkkk
     yyyyyyyyyyy
   yyyyyyyyyyyyyyy
 yyyyyyyyyyyyyyyyyy
",
            r"
        KKK
     KKKKK
      KKKKK
      KKKK
      kRRRRk
       kkkk
       kkkk kk
      yyyyyykkkk
     yyyyyyyyyyy
   yyyyyyyyyyyyyyy
 yyyyyyyyyyyyyyyyyy
",
            r"
      KKK
       KKKKK
     KKKKKKK
      KKKKK
      kRRRRk
       kkkk
       kkkk kk
      yyyyyykkkk
     yyyyyyyyyyy
   yyyyyyyyyyyyyyyy
 yyyyyyyyyyyyyyyyyy
",
        ],
    )
    .color('k')
    .fps(0.05)
    .bubbles(8, 4)
    .weight(7)
    .biomes(DEEP)
    .traits(GLOW),
    Decor::new(
        "whale-skeleton",
        &[r"
   ▗▄▄▄▄▄▄▄▄▄▖
 ▗▟▀▒▒▒▒▒▒▒▒▒▀▙▖
▟▀ ◉ ▒▒▒▒▒▒▒▒▒▒▀▙▄▄▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▄▃▂▂▁▁
▐▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▌      │  │  │  │  │  │  │  │  │  ╰
▝▀▜▛▜▛▜▛▜▛▜▛▜▛▀▀▘       │  │  │  │  │  │  │  ╰  ╰
                        │  │  │  │  ╰  ╰  ╰
                        ╰  ╰  ╰  ╰
"],
        &[r"


   k              WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW




"],
    )
    .color('w')
    .weight(3)
    .biomes(DEEP)
    .traits(SHELTER),
    Decor::new(
        "sunken-submarine",
        &[r"
                   ▗▄▖
                   ▐▌▙▄
             ▁▁▁▁▁▁▐▌▟▙▁▁▁▁▁▁▁▁
   ▁▁▄▄▄▄▄▟▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▙▄▄▄▄▖
  ▐▌▒░  ◯    ◯    ◯    ◯    ◯    ░▒▒▐▌╾╾
  ▝▙▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▘
          ▀▙▄▟▀               ▀▙▄▟▀
"],
        &[r"
                   yyy
                   yyyy
             yyyyyyyyyyyyyyyyyy
   yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
  yyyyyY    Y    Y    Y    Y    yyyyyyyy
  yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
          kkkkkk               kkkk
"],
    )
    .color('k')
    .weight(3)
    .biomes(DEEP | OCEAN)
    .traits(SHELTER),
    Decor::new(
        "lost-rov",
        &[
            r"
       ┃
       ┃
   ╔═══╩═══╗
  ╔╝▛▀▀▀▀▀▜╚╗
  ║▐ ◉   ◉ ▌║═╾╾
  ╚╗▙▄▄▄▄▄▟╔╝╾
   ╚═╦═══╦═╝
    ◖▐▌  ▐▌◗
",
            r"
       ┃
       ┃
   ╔═══╩═══╗
  ╔╝▛▀▀▀▀▀▜╚╗
  ║▐ ○   ○ ▌║═╾╾
  ╚╗▙▄▄▄▄▄▟╔╝╾
   ╚═╦═══╦═╝
    ◗▐▌  ▐▌◖
",
        ],
        &[r"
       w
       w
   wwwwwwwww
  wwwwwwwwwww
  wwWCwwwWCwwwww
  wwwwwwwwwwww
   wwwwwwwww
    wwww  ww
"],
    )
    .color('y')
    .fps(0.15)
    .weight(4)
    .biomes(DEEP)
    .traits(SHELTER | GLOW),
    Decor::new(
        "deep-cable",
        &[r"
                     ▁▁▁
  ╭──╮      ╭──╮  ▗▟◉▙▖  ╭──╮      ╭──╮
━━╯  ╰━━━━━━╯  ╰━━╯   ╰━━╯  ╰━━━━━━╯  ╰━
"],
        &[r"

                     Y
"],
    )
    .color('k')
    .weight(4)
    .biomes(DEEP),
    Decor::new(
        "pillow-lava",
        &[r"
     ▗▄▄▄▖
  ▗▄▟▀▓▒▀▙▄▄▄▄▖
 ▟▀▒▓▒▒▓▀▒▒▓▒▒▀▙▖
▐▓▒▒▓▓▒▒▓▒▒▓▓▒▒▓▒▌
▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
"],
        &[r"

      Rr
    R  R   R
 R  RR  R  RR  R
"],
    )
    .color('k')
    .weight(5)
    .biomes(DEEP)
    .traits(SHELTER),
    Decor::new(
        "lava-vent",
        &[
            r"
         ░▒░
        ░▒▓▒░
         ▒░▒

         ∙
         ▗▄▄▖
       ▗▟▀▓▓▀▙▖
      ▟▀ ◆▒▒◆ ▀▙
    ▗▟▒▓▒▒▓▒▓▒▒▓▙▖
  ▗▟▓▒▓▒░▒▓▒▓▒░▒▓▙▖
",
            r"

        ░▒▒░
         ░▒░
          ▒
       ∙
         ▗▄▄▖
       ▗▟▀▓▒▀▙▖
      ▟▀ ◇▓▓◇ ▀▙
    ▗▟▒▓▒▒▓▒▓▒▒▓▙▖
  ▗▟▓▒▓▒░▒▓▒▓▒░▒▓▙▖
",
            r"

          ░
        ░▒▓▒░
         ░▒░  ▒
            ∙
         ▗▄▄▖
       ▗▟▀▒▓▀▙▖
      ▟▀ ◆▒▓◆ ▀▙
    ▗▟▒▓▒▒▓▒▓▒▒▓▙▖
  ▗▟▓▒▓▒░▒▓▒▓▒░▒▓▙▖
",
        ],
        &[
            r"
         KKK
        KKKKK
         KKK

         Y
         kkkk
       kkkRRkkk
      kk RRRR kk
    kkkkkRkkkRkkkk
  kkkkkkkkkkkkkkkkk
",
            r"

        KKKK
         KKK
          K
       Y
         kkkk
       kkkRRkkk
      kk rRRr kk
    kkkkRkkkkRkkkk
  kkkkkkkkkkkkkkkkk
",
            r"

          K
        KKKKK
         KKK  K
            Y
         kkkk
       kkkRRkkk
      kk RRRR kk
    kkkkkRkkkkRkkk
  kkkkkkkkkkkkkkkkk
",
        ],
    )
    .color('k')
    .fps(0.05)
    .bubbles(10, 5)
    .weight(5)
    .biomes(DEEP)
    .traits(GLOW),
    Decor::new(
        "sunken-log",
        &[
            r"
   ▗▄▄▄▖▄▄▄▄▄▄▄▄▄▄▄▄⌠▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
  ▟▀▄▄▄▀▙▒▓▒▒░▒▓▒▒▓░▒▒▓▒░▒▒▓▒▒░▒▓▒░▒▀▙
 ▐▌▐▓◉▓▌▐▒▒▓▒▒░▒▒▓▒▒░▒▓▒▒▓▒▒░▒▒▓▒▒░▒▒▐▌
 ▝▙▄▀▀▀▄▟▒▒▓▒░▒▒▓▒▒▓▒▒░▒▒▓▒░▒▒▓▒▒░▒▄▟▘
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
   ▗▄▄▄▖▄▄▄▄▄▄▄▄▄▄▄▄⌡▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
  ▟▀▄▄▄▀▙▒▓▒▒░▒▓▒▒▓░▒▒▓▒░▒▒▓▒▒░▒▓▒░▒▀▙
 ▐▌▐▓◉▓▌▐▒▒▓▒▒░▒▒▓▒▒░▒▓▒▒▓▒▒░▒▒▓▒▒░▒▒▐▌
 ▝▙▄▀▀▀▄▟▒▒▓▒░▒▒▓▒▒▓▒▒░▒▒▓▒░▒▒▓▒▒░▒▄▟▘
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
   yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
  yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
 yyyyYyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
 yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
   yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
"],
    )
    .color('y')
    .fps(0.1)
    .weight(7)
    .biomes(FRESH)
    .traits(SHELTER),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_decor_drawn(super::DECOR);
    }
}
