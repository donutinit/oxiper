//! Props and scenery for the sea floor, continued.

use super::*;

pub const DECOR: &[Decor] = &[
    Decor::new(
        "old-tire",
        &[r"
    ▄▄▄▄▄▄▄▄▄
  ▟██▀▀▀▀▀▀▀██▙
 ▟█▀ ╭─────╮ ▀█▙
 █▌  │?????│  ▐█
 ▜█▄ ╰─────╯ ▄█▛
  ▜██▄▄▄▄▄▄▄██▛
    ▀▀▀▀▀▀▀▀▀
"],
        &[r"
    KKKKKKKKK
  KKK       KKK
 KK  kkkkkkk  KK
"],
    )
    .color('k')
    .weight(6)
    .biomes(FRESH)
    .traits(SHELTER),
    Decor::new(
        "shopping-cart",
        &[r"
 ╭───╮
 ╰╮  ┏━━━━━━━━━━━━┓
  ╰╮ ┃┼┼┼┼┼┼┼┼┼┼┼┼┃
   ╲ ┃┼┼┼┼┼┼┼┼┼┼┼┼┃
    ╲┗┷┷┷┷┷┷┷┷┷┷┷┷┛
     ◉           ◉
"],
        &[r"
 yyyyy
 yy
  yy
   y
    y
     K           K
"],
    )
    .color('w')
    .weight(4)
    .biomes(FRESH)
    .traits(SHELTER),
    Decor::new(
        "rusty-bicycle",
        &[r"
          ▄▄▄▄             ╭─╮
            ╲━━━━━━━━━━━━━━━│
             ╲              │
     ╭─────╮  ╲          ╭──┬──╮
    ╭╯ ╲│╱ ╰╮  ╲╱▁▁▁▁▁▁▁╭╯ ╲│╱ ╰╮
    │ ──◉── │━━━◉       │ ──◉── │
    ╰╮ ╱│╲ ╭╯               ╰╮ ╱│╲ ╭╯
     ╰─────╯                 ╰─────╯
"],
        &[r"
          KKKK             rrr
            rrrrrrrrrrrrrrrrr
             r              r
     kkkkkkk  r          kkkkkkk
    kkkkkkkkk  rrrrrrrrrkkkkkkkkk
    kkkkkkkkkrrry       kkkkkkkkk
    kkkkkkkkk               kkkkkkkkk
     kkkkkkk                 kkkkkkk
"],
    )
    .color('y')
    .weight(4)
    .biomes(FRESH),
    Decor::new(
        "river-boulders",
        &[r"
       ▗▄▄▄▄▄▖
    ▗▄▟█▓▒▒▓██▙▄▖       ▗▄▄▖
  ▗▟██▓▒░░ ∙░▒▓██▙▖  ▗▟█▓▒▓▙▖
 ▟███▓▒░  ∙   ░▒▓██▙▟██▓▒░ ▒▓▙▖
▟████▓▓▒▒░░ ░░▒▒▓▓███████▓▒▒░▒▓█▙
"],
        &[r"
       KKKKKK
    KKKKkkkkkKKKK       KKKK
  KKkkkk   k kkkkKK  KKkkkkK
 KKkkkk     k   kkkkKKKKkkk   k
"],
    )
    .color('k')
    .weight(8)
    .biomes(FRESH)
    .traits(SHELTER),
    Decor::new(
        "sunken-canoe",
        &[r"
                                    ▗▟
    ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟▛▘
 ▟██▀▀═════════════════════════▀▀██▘
 ▝▜██▄▄▃▃▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▂▃▃▄▄██▛▘
"],
        &[r"
                                    yy
    yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
 yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
 yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
"],
    )
    .color('y')
    .weight(5)
    .biomes(FRESH)
    .traits(SHELTER),
    Decor::new(
        "sunken-car",
        &[r"
          ▗▄▄▄▄▄▄▄▄▄▄▖
        ▗▟▀▘????│????▝▀▙▖
   ▗▄▄▄▟▀▄▄▄▄▄▄▄│▄▄▄▄▄▄▄▝▀▙▄▄▖
  ▟█▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█▙
  █▌ ◉                    ◈  ▐█
  ▜█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█▛
    ◖██◗                ◖██◗
"],
        &[r"
          rrrrrrrrrrr
        rrrr    r    rrrr
   rrrrrrrrrrrrrrrrrrrrrrrrrrr
  rrrrrrrrrrrrrrrrrrrrrrrrrrrrrr
  rr Y                    Y  rr
  rrrrrrrrrrrrrrrrrrrrrrrrrrrrr
    KKKK                KKKK
"],
    )
    .color('y')
    .weight(4)
    .biomes(FRESH)
    .traits(SHELTER),
    Decor::new(
        "ice-rocks",
        &[r"
          ◢◣
         ◢██◣        ◢◣
     ◢◣ ◢█▓▒█◣      ◢██◣ ◢◣
    ◢██◣█▓▒░▒█◣    ◢█▓▒█◣██◣
   ◢█▓▒███▓▒░░▒████▓▒░▒███▓▒█◣
  ◢██▓▒░▒▓██▓▒░░▒▓██████▓▒░░▒▓█◣
"],
        &[r"
          WW
         WWWW        WW
     WW WWccW      WWWW WW
    WWWWWccccWW    WWccWWWWW
   WWccccccccccWW  ccccccccccW
  WWWccccccccccccccccccccccccccW
"],
    )
    .color('c')
    .weight(8)
    .biomes(ARCTIC)
    .traits(SHELTER),
    Decor::new(
        "ice-arch",
        &[r"
          ▄▄▄▄▄▄▄▄▄▄▄▄
      ▗▟███▓▓▒▒░░▒▒▓▓███▙▖
    ▗▟██▓▒░            ░▒▓██▙▖
   ▗██▓▒░ ??????????????? ░▒▓██▖
   ███▒░ ????????????????? ░▒███
  ▐██▓░ ??????????????????? ░▓██▌
 ▟███▓▒░????????????????????░▒▓███▙
"],
        &[r"
          WWWWWWWWWWWW
      WWWWWWccccccccccWWWW
    WWWWccc            cccWWWW
   WWWccc                  cccWW
   WWccc                    ccWW
  WWcc                       ccWW
 WWWccc                       cccWW
"],
    )
    .color('c')
    .weight(3)
    .biomes(ARCTIC)
    .traits(SHELTER),
    Decor::new(
        "expedition-ship",
        &[r"
              ╷            ╷            ╷
             ◢█◣          ◢█◣          ◢█◣
            ◢███◣        ◢███◣        ◢███◣
           ◢█████◣      ◢█████◣      ◢█████◣
           ▀▀▀╽▀▀▀      ▀▀▀╽▀▀▀      ▀▀▀╽▀▀▀
      ◢◣       ╽            ╽            ╽       ▄▄▄
      ◥█◣  ▂▂▂▂▂╿▂▂▂▂▂▂▂▂▂▂▂╿▂▂▂▂▂▂▂▂▂▂▂╿▂▂▂▂▂ ▐▛▜▌
      ◥███▟◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ ◉ █▙▟▌
    ▗▄▟████▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▜█▙▖
    ▝▜█████▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟█▛▘
      ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
"],
        &[r"
              Y            Y            Y
             WWW          WWW          WWW
            WWWWW        WWWWW        WWWWW
           WWWWWWW      WWWWWWW      WWWWWWW
           WWWwWWW      WWWwWWW      WWWwWWW
      WW       w            w            w       rrr
      WWW  yyyyywyyyyyyyyyyywyyyyyyyyyyywyyyyyy rrr
      WWWWWK K K K K K K K K K K K K K K K K K rr
    WWWWWWWWccccccccccccccccccccccccccccccccccccccWWW
    WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
      CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
"],
    )
    .color('y')
    .weight(2)
    .biomes(ARCTIC)
    .traits(SHELTER),
    Decor::new(
        "frozen-wreck",
        &[r"
            ◢█
            ██◣
    ▗▄▄▖    ██▙▂      ▗▄▄▖
  ▗▟████▙▄▄▄██▛▀▀▀▄▄▄▟████▙▖
 ▟██▓▒░ ?   ?   ?   ?  ░▒▓██▙
 ██▓▒░ ?   ?  ?  ?   ?  ░▒▓██
 ▜██▓▒░?▁▁▁?▁▁▁▁?▁▁▁▁▁░▒▓██▛
  ▀▜█▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▙▟▛▀
    ▝ ▘ ▝▘ ▝ ▘ ▝ ▘ ▝▘ ▘
"],
        &[r"
            WW
            WWW
    WWWW    WWyy      WWWW
  WWWWWWWWyyyWWyyyyyyyWWWWWW
 WWWccc                     c
 WWccc
 WWWccc   yyy yyyy yyyyy ccc
  WWWWWWccWWccWWccWWccWWc
    C C  CC C  C C C C
"],
    )
    .color('y')
    .weight(3)
    .biomes(ARCTIC)
    .traits(SHELTER),
    Decor::new(
        "pirate-ship-ornament",
        &[
            r"
                  ╷▶
        ╷         │         ╷
       ◢█◣       ◢█◣       ◢█◣
      ◢███◣     ◢███◣     ◢███◣
     ◢█████◣   ◢█████◣   ◢█████◣
     ▀▀▀╽▀▀▀   ▀▀▀╽▀▀▀   ▀▀▀╽▀▀▀
  ▂▂▂▂▂▂╿▂▂▂▂▂▂▂▂╿▂▂▂▂▂▂▂▂╿▂▂▂▂▂▂
 ▟█ ◦  ◦  ◦  ◦  ◦  ◦  ◦  ◦  █▙▂▂
 ▜█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█▛ ▝▀▚▖
  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
",
            r"
                  ╷▷
        ╷         │         ╷
       ◢█◣       ◢█◣       ◢█◣
      ◢███◣     ◢███◣     ◢███◣
     ◢█████◣   ◢█████◣   ◢█████◣
     ▀▀▀╽▀▀▀   ▀▀▀╽▀▀▀   ▀▀▀╽▀▀▀
  ▂▂▂▂▂▂╿▂▂▂▂▂▂▂▂╿▂▂▂▂▂▂▂▂╿▂▂▂▂▂▂
 ▟█ ◦  ◦  ◦  ◦  ◦  ◦  ◦  ◦  █▙▂▂
 ▜█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄█▛ ▝▀▚▖
  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
",
        ],
        &[
            r"
                  KR
        K         k         K
       WWW       WWW       WWW
      WWWWW     WWWWW     WWWWW
     WWWWWWW   WWWWWWW   WWWWWWW
     RRRkRRR   RRRkRRR   RRRkRRR
  yyyyyykyyyyyyyykyyyyyyyykyyyyyy
 RRRYyYYyYYyYYyYYyYYyYYyYYyYYyRR
 RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR yy
  YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
",
            r"
                  KY
        K         k         K
       WWW       WWW       WWW
      WWWWW     WWWWW     WWWWW
     WWWWWWW   WWWWWWW   WWWWWWW
     RRRkRRR   RRRkRRR   RRRkRRR
  yyyyyykyyyyyyyykyyyyyyyykyyyyyy
 RRRYyYYyYYyYYyYYyYYyYYyYYyYYyRR
 RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR yy
  YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
",
        ],
    )
    .color('y')
    .fps(0.1)
    .weight(4)
    .biomes(TANK)
    .traits(SHELTER),
    Decor::new(
        "diver-with-chest",
        &[
            r"
     ▄▄▄▄
   ▗▟████▙▖
   ▐█◖◗██▌          ▟▀▀▀▀▀▀▀▜▙
   ▝▜████▛▘━━━╾─╮   ▐▌═╤══╤═▐▌
   ▗▟▀██▀▙▖     ╰─  ▐▌ ◈    ▐▌
  ▟▛ ████ ▜▙        ▝▀▀▀▀▀▀▀▀▀
     ▐▌▐▌
    ▟█▛▜█▙
",
            r"
     ▄▄▄▄
   ▗▟████▙▖             ▃▃
   ▐█◖◗██▌          ▟▀▀▀▀▀▀▀▜▙
   ▝▜████▛▘━━━╾─╮   ▐▌═╤══╤═▐▌
   ▗▟▀██▀▙▖     ╰╴  ▐▌ ◈    ▐▌
  ▟▛ ████ ▜▙        ▝▀▀▀▀▀▀▀▀▀
     ▐▌▐▌
    ▟█▛▜█▙
",
            r"
     ▄▄▄▄
   ▗▟████▙▖            ▃▃▃
   ▐█◖◗██▌          ▟▀▀▀▀▀▀▀▜▙
   ▝▜████▛▘━━━╾─╮   ▐▌═╤══╤═▐▌
   ▗▟▀██▀▙▖     ╰╴  ▐▌ ◈    ▐▌
  ▟▛ ████ ▜▙        ▝▀▀▀▀▀▀▀▀▀
     ▐▌▐▌
    ▟█▛▜█▙
",
        ],
        &[
            r"
     kkkk
   kkWWWWkk
   kkWWWWkk          yyyyyyyyy
   kkkkkkkkwwwwwww   yyYyyyyyy
   kkkkkkkk     ww  yyyYYYyyyy
  kkkkkkkkkk        yyyyyyyyy
     kkkk
    kkkkkk
",
            r"
     kkkk
   kkWWWWkk             YY
   kkWWWWkk          yyyyyyyyy
   kkkkkkkkwwwwwww   yyYyyyyyy
   kkkkkkkk     ww  yyyYYYyyyy
  kkkkkkkkkk        yyyyyyyyy
     kkkk
    kkkkkk
",
            r"
     kkkk
   kkWWWWkk            YYY
   kkWWWWkk          yyyyyyyyy
   kkkkkkkkwwwwwww   yyYyyyyyy
   kkkkkkkk     ww  yyyYYYyyyy
  kkkkkkkkkk        yyyyyyyyy
     kkkk
    kkkkkk
",
        ],
    )
    .color('y')
    .fps(0.04)
    .bubbles(7, 0)
    .weight(5)
    .biomes(TANK),
    Decor::new(
        "air-stone",
        &[
            r"
     ◦
    ∙
      ○
   ◦   ∙
  ∙  ○
    ∙◦∙
  ▗▄▟███▙▄▖
 ▟█▓▒░▒▓▒░▒█▙▄▂
",
            r"
      ∙
   ◦     ◦
     ○
    ∙   ∙
  ○  ∙
   ∙◦∙
  ▗▄▟███▙▄▖
 ▟█▓▒░▒▓▒░▒█▙▄▂
",
            r"
    ∙    ○
      ◦
   ∙     ∙
     ○  ◦
   ◦
    ∙○∙
  ▗▄▟███▙▄▖
 ▟█▓▒░▒▓▒░▒█▙▄▂
",
        ],
        &[
            r"
     C
    C
      C
   C   C
  C  C
    CCC
  kkkkkkkkk
 kkkkkkkkkkkkkk
",
            r"
      C
   C     C
     C
    C   C
  C  C
   CCC
  kkkkkkkkk
 kkkkkkkkkkkkkk
",
            r"
    C    C
      C
   C     C
     C  C
   C
    CCC
  kkkkkkkkk
 kkkkkkkkkkkkkk
",
        ],
    )
    .fps(0.12)
    .bubbles(6, 0)
    .weight(6)
    .biomes(TANK),
    Decor::new(
        "tank-amphora",
        &[r"
    ▄▀▀▀▄
    ▐▄▄▄▌
   ▟▛▀▀▀▜▙
  ▟█ ◇◇◇ █▙
 ▐█▌ ▓▓▓ ▐█▌
  ▜█ ◆◆◆ █▛
  ▝▜█▄▄▄█▛▘
    ▝▀▀▀▘
"],
        &[r"
    RRRRR
    RRRRR
   RRRRRRR
  RRRYYYRRR
 RRR YYY RRR
  RRRYYYRRR
  RRRRRRRRR
    RRRRR
"],
    )
    .color('y')
    .weight(5)
    .biomes(TANK)
    .traits(SHELTER),
    Decor::new(
        "tank-bridge",
        &[r"
              ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
         ▗▄▟▀▀▀╥─╥─╥─╥─╥─╥─╥▀▀▀▙▄▖
       ▗▟▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▙▖
      ▐█▌????????????????????????▐█▌
     ▟▀▀▙???▗▄▄▄▄▄▄▄▄▄▄▄▄▖????▟▀▀▙
     ▐██▌??▟▀            ▀▙??▐██▌
    ▗▟██▙?▟▀???????????????▀▙?▟██▙▖
   ▐██████▀?????????????????▀██████▌
"],
        &[r"
              RRRRRRRRRRRRRRRR
         RRRRRRRYRYRYRYRYRYRYRRRRR
       RRRRRRRRRRRRRRRRRRRRRRRRRRRRR
      YRY                        YRY
     RRRRR   RRRRRRRRRRRRRR    RRR
     RRRRR  RR            RR  RRR
    RRRRRR RR              RR RRRRR
   YYYYYYYRR                RRYYYYYY
"],
    )
    .color('r')
    .weight(4)
    .biomes(TANK)
    .traits(SHELTER),
    Decor::new(
        "tank-pagoda",
        &[
            r"
         ▲
        ▟█▙
      ▂▟███▙▂
     ▟▀▀▀▀▀▀▀▙
       ▐▌◈▐▌
    ▂▟▀▀▀▀▀▀▀▀▙▂
   ▟▀▀▀▀▀▀▀▀▀▀▀▀▙
     ▐▌◈ ◈ ◈▐▌
  ▂▟▀▀▀▀▀▀▀▀▀▀▀▀▙▂
 ▟▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▙
    ▐▌◈  ▟▙  ◈▐▌
    ▐█▄▄▄██▄▄▄█▌
",
            r"
         △
        ▟█▙
      ▂▟███▙▂
     ▟▀▀▀▀▀▀▀▙
       ▐▌◇▐▌
    ▂▟▀▀▀▀▀▀▀▀▙▂
   ▟▀▀▀▀▀▀▀▀▀▀▀▀▙
     ▐▌◇ ◇ ◇▐▌
  ▂▟▀▀▀▀▀▀▀▀▀▀▀▀▙▂
 ▟▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▙
    ▐▌◇  ▟▙  ◇▐▌
    ▐█▄▄▄██▄▄▄█▌
",
        ],
        &[
            r"
         Y
        RRR
      RRRRRRR
     RRRRRRRRR
       wYwY
    RRRRRRRRRRR
   RRRRRRRRRRRRRR
     wYYYYYYw
  RRRRRRRRRRRRRRRR
 RRRRRRRRRRRRRRRRRR
    wYY  RR  YYw
    wwwwwwwwwwww
",
            r"
         y
        RRR
      RRRRRRR
     RRRRRRRRR
       wywy
    RRRRRRRRRRR
   RRRRRRRRRRRRRR
     wyyyyyyw
  RRRRRRRRRRRRRRRR
 RRRRRRRRRRRRRRRRRR
    wyy  RR  yyw
    wwwwwwwwwwww
",
        ],
    )
    .color('r')
    .fps(0.1)
    .weight(4)
    .biomes(TANK)
    .traits(SHELTER),
    Decor::new(
        "ceramic-castle",
        &[
            r"
   ▲         ▲         ▲
  ▟█▙       ▟█▙       ▟█▙
 ▐▛▀▜▌     ▐▛▀▜▌     ▐▛▀▜▌
 ▐▌◖◗▌▄▄▄▄▄▐▌◖◗▌▄▄▄▄▄▐▌◖◗▌
 ▐▌  ▌█████▐▌  ▌█████▐▌  ▌
 ▐█▄▄▌██▟▙██▐█▄▄▌██▟▙██▐█▄▄▌
",
            r"
   △         △         △
  ▟█▙       ▟█▙       ▟█▙
 ▐▛▀▜▌     ▐▛▀▜▌     ▐▛▀▜▌
 ▐▌◖◗▌▄▄▄▄▄▐▌◖◗▌▄▄▄▄▄▐▌◖◗▌
 ▐▌  ▌█████▐▌  ▌█████▐▌  ▌
 ▐█▄▄▌██▟▙██▐█▄▄▌██▟▙██▐█▄▄▌
",
        ],
        &[
            r"
   R         R         R
  BBB       BBB       BBB
 bbbbb     bbbbb     bbbbb
 bbYYb bbbbbbbYYb bbbbbbbY
 bb  b ccccc bb  b ccccc b
 bbbbb cckkcc bbbbb cckkcc b
",
            r"
   Y         Y         Y
  BBB       BBB       BBB
 bbbbb     bbbbb     bbbbb
 bbYYb bbbbbbbYYb bbbbbbbY
 bb  b ccccc bb  b ccccc b
 bbbbb cckkcc bbbbb cckkcc b
",
        ],
    )
    .color('c')
    .fps(0.1)
    .weight(4)
    .biomes(TANK)
    .traits(SHELTER),
    Decor::new(
        "ruined-tower",
        &[r"
   ▄    ▄ ▄
  ▐█▄▄▟█▌██▖
  ▐█▛▀▀▀▜?█▌
  ▐█ ▝▘  ?█▌
  ▐█  ▄▄▄  █▌
  ▐█ ▐▌░▐▌ █▌
 ▗▟█ ▐▙▄▟▌?██▖
▟████▄▄▄▄▄▄████▙
"],
        &[r"
   w    w w
  wwwwwwwwww
  wwwwwwww w
  ww kk    w
  ww   kkk  w
  ww  kkkk  w
 wwww kkkk  ww
wwwwwwwwwwwwwwww
"],
    )
    .color('w')
    .weight(4)
    .biomes(TANK | REEF | KELP)
    .traits(SHELTER),
    Decor::new(
        "sand-castle",
        &[
            r"
        ▲▶
    ▄   │   ▄
   ▐█▌▄▟█▙▄▐█▌
   ▐▌▀▀▓▓▓▀▀▐▌
  ▄█▌ ▟▛▀▜▙ ▐█▄
 ▟▓▓▓▓▓▙▟▛▓▓▓▓▓▙
",
            r"
        ▲▷
    ▄   │   ▄
   ▐█▌▄▟█▙▄▐█▌
   ▐▌▀▀▓▓▓▀▀▐▌
  ▄█▌ ▟▛▀▜▙ ▐█▄
 ▟▓▓▓▓▓▙▟▛▓▓▓▓▓▙
",
        ],
        &[
            r"
        yR
    y   y   y
   yyyyyyyyyyy
   yyyyyyyyyyy
  yyyykkkkyyyyy
 yyyyyyyyyyyyyyy
",
            r"
        yR
    y   y   y
   yyyyyyyyyyy
   yyyyyyyyyyy
  yyyykkkkyyyyy
 yyyyyyyyyyyyyyy
",
        ],
    )
    .color('y')
    .fps(0.1)
    .weight(3)
    .biomes(TANK)
    .traits(SHELTER),
    Decor::new(
        "wreck-with-flag",
        &[
            r"
                ┃▀▀▀▜▙
                ┃▀▀▘
                ┃
     ▗▄         ┃
    ▟█▀▀▀▀▀▀▀▀▀▜█▙▃
   ▟█▛ ?    ?  ?   ▀▜█▙▃▂
  ▟█▛ ?   ?   ?  ?   ?  ▀▜█▙▃
  ▜█▙▄?▄▄▄?▄▄▄?▄▄▄?▄▄▄▄▄▄▄██▛▘
",
            r"
                ┃▀▀▀▀▜▙
                ┃▀▀▀▘
                ┃
     ▗▄         ┃
    ▟█▀▀▀▀▀▀▀▀▀▜█▙▃
   ▟█▛ ?    ?  ?   ▀▜█▙▃▂
  ▟█▛ ?   ?   ?  ?   ?  ▀▜█▙▃
  ▜█▙▄?▄▄▄?▄▄▄?▄▄▄?▄▄▄▄▄▄▄██▛▘
",
            r"
                ┃▀▀▜▙
                ┃▀▀▀▘
                ┃
     ▗▄         ┃
    ▟█▀▀▀▀▀▀▀▀▀▜█▙▃
   ▟█▛ ?    ?  ?   ▀▜█▙▃▂
  ▟█▛ ?   ?   ?  ?   ?  ▀▜█▙▃
  ▜█▙▄?▄▄▄?▄▄▄?▄▄▄?▄▄▄▄▄▄▄██▛▘
",
        ],
        &[
            r"
                 KKKKK
                 KKK
",
            r"
                 kKKKKK
                 KKKK
",
            r"
                 KKKk
                 kKKK
",
        ],
    )
    .color('y')
    .fps(0.08)
    .bubbles(12, 6)
    .weight(4)
    .biomes(OCEAN | REEF)
    .traits(SHELTER),
    Decor::new(
        "sunken-sword",
        &[r"
    ◖◗
   ▄▟▙▄
   ▀▐▌▀
    ▐▌
    ▐▌
 ▂▃▄▐▌▄▃▂
"],
        &[r"
    YY
   yyyy
   rrrr
    ww
    ww
 yyyyyyyy
"],
    )
    .weight(3)
    .biomes(KELP | OCEAN | TANK),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_decor_drawn(super::DECOR);
    }
}
