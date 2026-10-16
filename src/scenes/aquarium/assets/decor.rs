//! Props and scenery for the sea floor.

use super::*;

pub const DECOR: &[Decor] = &[
    Decor::new(
        "treasure-chest",
        &[
            r"

   ╭───────╮
 ╭─╯▒▒▒▒▒▒▒╰─╮
 ┣━╋━━━━━━━╋━┫
 ┃░┃▒▒▐◆▌▒▒┃░┃
 ┃░┃▒▒▒▒▒▒▒┃░┃
 ╰━┻━━━━━━━┻━╯
",
            r"
   ╭───────╮
 ╭─╯▒▒▒▒▒▒▒╰─╮
 ╰━┻━━━━━━━┻━╯
  ▗▟█✦█▟█✦█▙▖
 ┃░┃▒▒▐◆▌▒▒┃░┃
 ┃░┃▒▒▒▒▒▒▒┃░┃
 ╰━┻━━━━━━━┻━╯
",
            r"
   ╭───────╮
 ╭─╯░░░░░░░╰─╮
 ╰━┻▗▟█✦█▙▖┻━╯
  ▗▟█✦█▟█✦█▙▖
 ┃░┃▒▒▐◆▌▒▒┃░┃
 ┃░┃▒▒▒▒▒▒▒┃░┃
 ╰━┻━━━━━━━┻━╯
",
        ],
        &[
            r"



 KKKKKKKKKKKKK
 K K  KYK  K K
 K K       K K
 KKKKKKKKKKKKK
",
            r"


 KKKKKKKKKKKKK
  YYYWYYYWYYY
 K K  KYK  K K
 K K       K K
 KKKKKKKKKKKKK
",
            r"


 KKKYYYWYYYKKK
  YYYWYYYWYYY
 K K  KYK  K K
 K K       K K
 KKKKKKKKKKKKK
",
        ],
    )
    .color('y')
    .fps(0.04)
    .bubbles(6, 1)
    .weight(7)
    .biomes(REEF | OCEAN | TANK)
    .traits(SHELTER),
    Decor::new(
        "coin-hoard",
        &[
            r"
        ✦
      ▗▟◉▙▖  ∙
   ◦ ▗▟█◎█▙▖ ✧
  ▗▟█◎█◉█◎█▙▖▗▖
 ▟█◉█◎█◉█◎█◉█▙▟▙
▟█◎█◉█◎█◉█◎█◉█◎█▙
▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
            ✧
      ▗▟◎▙▖ ✦
   ✧ ▗▟█◉█▙▖  ∙
  ▗▟█◉█◎█◉█▙▖▗▖
 ▟█◎█◉█◎█◉█◎█▙▟▙
▟█◉█◎█◉█◎█◎█◉█◎█▙
▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
        W
        W    W
   W    Y    W
     Y W Y
   W Y W Y W
  Y W Y W Y W Y
",
            r"
            W
        Y   W
   W    W    W
     W Y W
   Y W Y W Y
  W Y W Y W Y W
",
        ],
    )
    .color('y')
    .fps(0.06)
    .weight(4)
    .biomes(REEF | OCEAN),
    Decor::new(
        "gold-bars",
        &[r"
        ▗▄▄▄▄▄▄▖
        ▟██✦███▙
        ▝▀▀▀▀▀▀▘
   ▗▄▄▄▄▄▄▖▗▄▄▄▄▄▄▖
   ▟████✦█▙▟██████▙
   ▝▀▀▀▀▀▀▘▝▀▀▀▀▀▀▘
"],
        &[r"
        yyyyyyyy
        YYYWYYYY
        yyyyyyyy
   yyyyyyyyyyyyyyyy
   YYYYYWYYYYYYYYYY
   yyyyyyyyyyyyyyyy
"],
    )
    .color('y')
    .weight(3)
    .biomes(REEF | OCEAN | TANK),
    Decor::new(
        "message-bottle",
        &[r"
     ▄▄
     ▐▌
    ╭╯╰╮
   ╭╯  ╰╮
  ╭╯    ╰╮
  │░ ▗▄▖ │
  │░ ▐▒▌ │
  │░ ▐▒▌ │
  │░ ▝▀▘ │
  ╰──────╯
"],
        &[r"
     yy
     yy



     WWW
     WWW
     WWW
     WWW
"],
    )
    .color('g')
    .bubbles(5, 0)
    .weight(4)
    .biomes(KELP | OCEAN | REEF)
    .traits(SHELTER),
    Decor::new(
        "bottle-lying",
        &[r"
   ╭───────╮
 ╭─╯░░     ╰───╮▄▄
 │░░  ○ ○ ○     ██
 ╰─╮░░     ╭───╯▀▀
   ╰───────╯
"],
        &[r"

                yy
                yy
                yy
"],
    )
    .color('g')
    .weight(5)
    .biomes(KELP | OCEAN | FRESH | ARCTIC)
    .traits(SHELTER),
    Decor::new(
        "old-boot",
        &[r"
  ╭────╮
  ┃▒▒▒▒┃
  ┃░░░░┃
  ┃ ∙∙ ┃
  ┃    ╰───╮
  ┃▒░░░░░░░╰─╮
  ┃▒▒▒▒▒▒▒▒▒▒│
  ╰━━━━━━━━━━━╯
"],
        &[r"



    WW



  KKKKKKKKKKKKK
"],
    )
    .color('y')
    .weight(5)
    .biomes(KELP | OCEAN | FRESH)
    .traits(SHELTER),
    Decor::new(
        "skull-and-crossbones",
        &[r"
       ▗▄▄▄▄▄▄▄▖
●●    ▟█████████▙  ●●
  ╲╲  █▌ ▐███▌ ▐█╱╱
    ╲╲█▙▄▟█▀█▙▄▟█
      ╲▝▜█▀█▀█▛▘
        ▝▀▀▀▀▀▘
         ╱╳╲
       ╱╱   ╲╲
     ╱╱       ╲╲
   ●●           ●●
"],
        &[r"
       WWWWWWWWW
ww    WWWWWWWWWWW  ww
  ww  WWWWWWWWWWWww
    wwWWWWWWWWWWW
      wWWWWWWWWW
        WWWWWWW
         www
       ww   ww
     ww       ww
   ww           ww
"],
    )
    .weight(4)
    .biomes(OCEAN | REEF | TANK),
    Decor::new(
        "bleached-skull",
        &[r"
  ▗▄▄▄▄▄▖
 ▟███████▙
 █▌ ▐█▌ ▐█
 █▙▄▟█▙▄▟█
  ▝▜█▀█▛▘
 ░▒▝▀▀▀▘▒░
"],
        &[r"
  WWWWWWW
 WWWWWWWWW
 WWWWWWWWW
 WWWWWWWWW
  WWWWWWW
 wwWWWWWww
"],
    )
    .weight(5)
    .biomes(TANK | KELP | DEEP)
    .traits(SHELTER),
    Decor::new(
        "ships-wheel",
        &[r"
       ╻
   ◦ ╭─┃─╮ ◦
    ╭╯╲┃╱╰╮
 ╺━━┥─ ◉ ─┝━━╸
    ╰╮╱┃╲╭╯
   ◦ ╰─┃─╯ ◦
       ╹
"],
        &[r"
       Y
   W       W

 YYY   R   YYY

   W       W
"],
    )
    .color('y')
    .weight(3)
    .biomes(OCEAN | REEF | TANK),
    Decor::new(
        "bronze-bell",
        &[r"
       ╭╮
     ▗▄██▄▖
    ▟██▓▓██▙
   ▟██▓▒▒▓██▙
   ██▓▒░░▒▓██
  ▟██▓▒░░▒▓██▙
 ▗▟▀▀▀▀▀▀▀▀▀▀▙▖
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▘
       ◖◗
"],
        &[r"
       KK
     YYYYYY
    YYYYYYYY
   YYYYYYYYYY
   YYYYYYYYYY
  YYYYYYYYYYYY
 yyyyyyyyyyyyyy
 yyyyyyyyyyyyyy
       KK
"],
    )
    .color('y')
    .bubbles(5, 4)
    .weight(3)
    .biomes(OCEAN | ARCTIC | REEF)
    .traits(SHELTER),
    Decor::new(
        "diver-helmet",
        &[r"
       ▄▄
    ▗▄▟██▙▄▖
   ▟████████▙
  ▐██╭────╮██▌
  ▐██│░░✦░│██▌
  ▐██╰────╯██▌
   ▜████████▛
 ▗▄▟▀▀▀▀▀▀▀▀▙▄▖
 ▐██∙██████∙██▌
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▘
"],
        &[r"
       KK
    YYYYYYYY
   YYYYYYYYYY
  YYYKKKKKKYYY
  YYYKCCWCKYYY
  YYYKKKKKKYYY
   YYYYYYYYYY
 yyyyyyyyyyyyyy
 yyyWyyyyyyWyyy
 yyyyyyyyyyyyyy
"],
    )
    .color('y')
    .bubbles(7, 0)
    .weight(4)
    .biomes(OCEAN | REEF | TANK)
    .traits(SHELTER),
    Decor::new(
        "ship-anchor",
        &[r"
        ┏━┓
        ┗┳┛
    ╺━━━━╋━━━━╸
         ┃
         ┃
         ┃
   ▲     ┃     ▲
   ┃     ┃     ┃
   ┗┓    ┃    ┏┛
    ┗┓   ┃   ┏┛
     ┗━━━╋━━━┛
"],
        &[],
    )
    .color('K')
    .weight(6)
    .biomes(OCEAN | ARCTIC | KELP | TANK),
    Decor::new(
        "chain-anchor",
        &[r"
        ┏━┓
        ┗┳┛
    ╺━━━━╋━━━━╸
         ┃
         ┃
         ┃
   ▲     ┃     ▲
   ┃     ┃     ┃
   ┗┓    ┃    ┏┛
    ┗┓   ┃   ┏┛
     ┗━━━╋━━━┛━○━○━○━○━○━○
"],
        &[],
    )
    .color('K')
    .weight(4)
    .biomes(OCEAN | ARCTIC | KELP),
    Decor::new(
        "cannon",
        &[r"
       ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
  ◖▄▄▄▟█████████████████████████████▙▄▄▄▖
  ◗▀▀▀▜▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▛▀▀▀▘
       ▝▜▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▛▘
     ╭─┬─╮              ╭─┬─╮
     ├─◉─┤              ├─◉─┤
     ╰─┴─╯              ╰─┴─╯
"],
        &[r"




      yyy                yyy
     yyyyy              yyyyy
     yyyyy              yyyyy
"],
    )
    .color('K')
    .weight(4)
    .biomes(OCEAN | REEF)
    .traits(SHELTER),
    Decor::new(
        "barrel-stack",
        &[r"
      ▗▄▄▄▄▄▄▄▖
     ▟▀▀▀▀▀▀▀▀▀▙
     ▐▒░▒▒░▒▒░▒▌
     ▐░▒▒░▒▒░▒▒▌
     ▙▄▄▄▄▄▄▄▄▄▟
 ▗▄▄▄▄▄▄▄▖  ▗▄▄▄▄▄▄▄▖
▟▀▀▀▀▀▀▀▀▀▙▟▀▀▀▀▀▀▀▀▀▙
▐▒░▒▒░▒▒░▒▌▐░▒▒░▒▒░▒▒▌
▐░▒▒░▒▒░▒▒▌▐▒░▒▒░▒▒░▒▌
▙▄▄▄▄▄▄▄▄▄▟▙▄▄▄▄▄▄▄▄▄▟
 ▝▀▀▀▀▀▀▀▘  ▝▀▀▀▀▀▀▀▘
"],
        &[r"

     KKKKKKKKKKK


     KKKKKKKKKKK

KKKKKKKKKKKKKKKKKKKKKK


KKKKKKKKKKKKKKKKKKKKKK
"],
    )
    .color('y')
    .weight(5)
    .biomes(OCEAN | REEF | KELP)
    .traits(SHELTER),
    Decor::new(
        "giant-clam",
        &[
            r"


     ╭──────────╮
   ╭─╯▒▒░│▒▒│░▒▒╰─╮
  ╱▒░│▒░│▒▒▒│░▒│░▒╲
 ▕▒▒░│▒░│▒▒▒▒│░▒│░▒▒▏
 ╰◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡╯
",
            r"

     ╭──────────╮
   ╭─╯▒▒░│▒▒│░▒▒╰─╮
  ╱▒░│▒░│▒▒▒│░▒│░▒╲
 ▕▓▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▓▏
  ≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼
 ╰◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡╯
",
            r"
     ╭──────────╮
   ╭─╯▒▒░│▒▒│░▒▒╰─╮
  ╱▒░│▒░│▒▒▒│░▒│░▒╲
 ▕▓▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▓▏
  ≈∼≈∼≈∼ ◉◉ ∼≈∼≈∼≈
 ╭▓▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▓╮
 ╰◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡◡╯
",
        ],
        &[
            r"
",
            r"





  mmmmmmmmmmmmmmmm
",
            r"




  mmmmmmWWmmmmmmmm
",
        ],
    )
    .fps(0.035)
    .bubbles(9, 3)
    .weight(6)
    .biomes(REEF)
    .traits(SHELTER),
    Decor::new(
        "temple-ruins",
        &[r"
   ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
   ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▙▖
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
    ▐░┃▒▌   ▐░┃▒▌   ▐░┃▒▌
    ▐░┃▒▌   ▐░┃▒▌   ▐░┃▒▌       ▗▖
    ▐░┃▒▌   ▐░┃▒▌   ▐░┃▒▌      ▗▟▒▙
    ▐░┃▒▌   ▐░┃▒▌   ▝▀▀▘      ▐░┃▒▌
    ▐░┃▒▌   ▐░┃▒▌       ▗▟▒▙  ▐░┃▒▌
  ▗▄▟▀▀▀▀▙▄▟▀▀▀▀▙▄▖  ▗▄▟▀▀▀▀▙▖▗▟▀▀▀▙▖
 ▐▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▌▂▂▐▓▓▓▓▓▓▓▓▓▓▓▌
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
"],
        &[r"









  gg   ggg    g   gg      gg
 ggg ggg   gg   gggg   ggggggg
"],
    )
    .color('w')
    .weight(4)
    .biomes(REEF | OCEAN)
    .traits(SHELTER),
    Decor::new(
        "standing-columns",
        &[r"
▗▄▄▄▄▄▄▄▄▖  ▗▄▄▄▄▄▄▄▄▖  ▗▄▄▄▄▄▄▄▄▖
▐▀▀▀▀▀▀▀▀▌  ▐▀▀▀▀▀▀▀▀▌  ▐▀▀▀▀▀▀▀▀▌
 ▝▜▄▄▄▄▛▘    ▝▜▄▄▄▄▛▘    ▝▜▄▄▄▄▛▘
  ▐░┃┃┃▒▌     ▐░┃┃┃▒▌     ▐░┃┃┃▒▌
  ▐░┃┃┃▒▌     ▐░┃┃┃▒▌     ▐░┃┃┃▒▌
  ▐░┃┃┃▒▌     ▐░┃┃┃▒▌     ▐░┃┃┃▒▌
  ▐░┃┃┃▒▌     ▐░┃┃┃▒▌     ▐░┃┃┃▒▌
 ▗▟▀▀▀▀▀▀▙▖  ▗▟▀▀▀▀▀▀▙▖  ▗▟▀▀▀▀▀▀▙▖
 ▐████████▌  ▐████████▌  ▐████████▌
▐▓▓▓▓▓▓▓▓▓▓▌▐▓▓▓▓▓▓▓▓▓▓▌▐▓▓▓▓▓▓▓▓▓▓▌
"],
        &[r"









  gg   gg   g gg    gg   g  gg g
"],
    )
    .color('w')
    .weight(4)
    .biomes(REEF | OCEAN | TANK)
    .traits(SHELTER),
    Decor::new(
        "broken-columns",
        &[r"
                    ▗▄▖
         ▗▟▖        ▐▒▌▖
   ▗▟▖   ▐░┃▜▖      ▐░┃▌
  ▗▟░┃▙  ▐░┃▒▌     ▗▟░┃▒▌
  ▐░┃┃▒▌ ▐░┃▒▌     ▐░┃┃▌
  ▐░┃┃▒▌ ▐░┃▒▌     ▐░┃┃▒▌
 ▗▟▀▀▀▀▙▖▗▟▀▀▙▖    ▗▟▀▀▀▙▖ ▗▄▄▄▄▄▖
 ▐██████▌▐████▌    ▐█████▌▐▒▒▒▒▒▒▌
▐▓▓▓▓▓▓▓▓▌▓▓▓▓▓▌  ▐▓▓▓▓▓▓▓▌▝▀▀▀▀▀▘
"],
        &[r"








 gg  gg   ggg   g   gg    gg  gg
"],
    )
    .color('w')
    .weight(5)
    .biomes(REEF | OCEAN)
    .traits(SHELTER),
    Decor::new(
        "moai-head",
        &[r"
  ▗▄▄▄▄▄▄▖
 ▗▟██████▙▖
 ██▛▀▀▀▀▜██
▐██▌    ▐██▌
▐██▌ ▟▙ ▐██▌
▐██▌ ██ ▐██▌
▐██▌ ▜▛ ▐██▌
 ██▌▀▀▀▀▐██
 ▜█▙▄▄▄▄▟█▛
 ▐████████▌
▐██████████▌
▝▀▀▀▀▀▀▀▀▀▀▘
"],
        &[r"










  gg    gg
 ggg ggg  gg
"],
    )
    .color('w')
    .weight(4)
    .biomes(REEF | OCEAN)
    .traits(SHELTER),
    Decor::new(
        "statue-head",
        &[r"
   ▗▄▄▄▄▄▄▖
  ▟▓▒▓▒▒▓▒▓▙
  ▐▓▀▀▀▀▀▀▓▌
  ▐▌▂▂  ▂▂▐▌
  ▐▌  ▐▌  ▐▌
  ▐▌  ▟▙  ▐▌
  ▝▙ ╺━━╸ ▟▘
   ▝▜▄▄▄▄▛▘
    ▐▒▒▒▒▌
  ▗▄▟████▙▄▖
▐████████████▌
▝▀▀▀▀▀▀▀▀▀▀▀▀▘
"],
        &[r"
   KKKKKKKK
  KKKKKKKKKK
"],
    )
    .color('w')
    .weight(4)
    .biomes(REEF | OCEAN | KELP)
    .traits(SHELTER),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_decor_drawn(super::DECOR);
    }
}
