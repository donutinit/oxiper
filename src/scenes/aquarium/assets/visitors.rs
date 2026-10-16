//! Passers-by: boats, birds, whales, submarines, divers and stranger things.

use super::*;

pub const VISITORS: &[Visitor] = &[
    Visitor::new(
        "sailboat",
        Lane::Boat,
        &[
            r"
        │◣
        │█◣
       ◢│██◣
      ◢█│███◣
  ▄▄▄▄▄▄█▄▄▄▄▄▄▄▄
∼≈◥█████████████◤
",
            r"
        │◤
        │▓◣
       ◢│█▓◣
      ◢▓│▓██◣
  ▄▄▄▄▄▄█▄▄▄▄▄▄▄▄
≈∼◥█████████████◤
",
        ],
        &[r"
        yR
        yWW
       WyWWW
      WWyWWWW
  111111y11111111
cc111111111111111
"],
    )
    .tint("1:rRbBgG")
    .speed(0.4, 0.9)
    .fps(0.15)
    .weight(8)
    .biomes(OCEAN | KELP | REEF | FRESH | TANK),
    Visitor::new(
        "catamaran",
        Lane::Boat,
        &[
            r"
         │◣
         │█◣
        ◢│██◣
  ▄████▄═╪════▄████▄
∼≈◥████◤??????◥████◤
",
            r"
         │◥
         │▓◣
        ◢│█▓◣
  ▄████▄═╪════▄████▄
≈∼◥████◤??????◥████◤
",
        ],
        &[r"
         yR
         yWW
        WyWWW
  111111 y    111111
cc111111      111111
"],
    )
    .tint("1:bBcCrR")
    .speed(0.6, 1.2)
    .fps(0.15)
    .weight(5)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "canoe-paddler",
        Lane::Boat,
        &[
            r"
       ●  ╱
      ╭█╮╱
  ◢▀▀▀▀▀▀▀▀▀▀▀▀◣
∼≈◥▄▄▄▄▄▄▄▄▄▄▄▄◤
",
            r"
    ╲  ●
     ╲╭█╮
  ◢▀▀▀▀▀▀▀▀▀▀▀▀◣
≈∼◥▄▄▄▄▄▄▄▄▄▄▄▄◤
",
        ],
        &[
            r"
       Y  w
      yyyw
  11111111111111
cc111111111111
",
            r"
    w  Y
     wyyy
  11111111111111
cc111111111111
",
        ],
    )
    .tint("1:yYrRgb")
    .speed(0.3, 0.7)
    .fps(0.2)
    .weight(6)
    .biomes(FRESH | KELP | ARCTIC | OCEAN),
    Visitor::new(
        "rowboat",
        Lane::Boat,
        &[
            r"
        ●
    ╱??╭█╮??╲
   ╱?◢▀▀█▀▀◣?╲
 ◢▀▀▀▀▀▀▀▀▀▀▀▀◣
∼≈◥▄▄▄▄▄▄▄▄▄▄◤
",
            r"
        ●
    ╲??╭█╮??╱
   ╲?◢▀▀█▀▀◣?╱
 ◢▀▀▀▀▀▀▀▀▀▀▀▀◣
≈∼◥▄▄▄▄▄▄▄▄▄▄◤
",
        ],
        &[r"
        Y
    w  yyy  w
   w 111y111 w
 11111111111111
cc111111111111
"],
    )
    .tint("1:rRbBgy")
    .speed(0.3, 0.7)
    .fps(0.2)
    .weight(7)
    .biomes(FRESH | KELP | OCEAN | TANK),
    Visitor::new(
        "kayak",
        Lane::Boat,
        &[
            r"
   ⠂     ●
    ◖━━━╭█╮━━━◗
 ◀▀▀▀▀▀▀▀█▀▀▀▀▀▀▀▶
∼≈◥▄▄▄▄▄▄▄▄▄▄▄▄▄◤
",
            r"
         ●    ⠂
    ◗━━━╭█╮━━━◖
 ◀▀▀▀▀▀▀▀█▀▀▀▀▀▀▀▶
≈∼◥▄▄▄▄▄▄▄▄▄▄▄▄▄◤
",
        ],
        &[r"
         Y
    wwwwyyywwww
 11111111y11111111
cc111111111111111
"],
    )
    .tint("1:rRyYbB")
    .speed(0.5, 1.0)
    .fps(0.3)
    .weight(6)
    .biomes(FRESH | ARCTIC | KELP | OCEAN),
    Visitor::new(
        "jet-ski",
        Lane::Boat,
        &[
            r"
       ●
     ▗▟█▙▄▄◣
⠈⠁◢████████████▶
≈≈◥▀▀▀▀▀▀▀▀▀▀▀▀◤
",
            r"
       ●
     ▗▟█▙▄▄◣
⠂⠐◢████████████▶
∼≈◥▀▀▀▀▀▀▀▀▀▀▀▀◤
",
            r"
       ●
     ▗▟█▙▄▄◣
⠠⠄◢████████████▶
≈∼◥▀▀▀▀▀▀▀▀▀▀▀▀◤
",
        ],
        &[r"
       Y
     11Y1111
CC11111111111111
WWWWWWWWWWWWWWWW
"],
    )
    .tint("1:rRyYgG")
    .speed(2.5, 4.5)
    .fps(0.4)
    .weight(4)
    .traits(SCARY)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "swan-pedalo",
        Lane::Boat,
        &[
            r"
               ●◣
              ╭╯
    ●??●??????┃
   ╭█╮╭█╮????▐█▌
 ◢█████████████████◣
∼≈◥▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄◤
",
            r"
               ●◢
              ╭╯
    ●??●??????┃
   ╭▌╮╭▐╮????▐█▌
 ◢█████████████████◣
≈∼◥▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄◤
",
        ],
        &[r"
               WY
              WW
    Y  Y      W
   111111    WWW
 WWWWWWWWWWWWWWWWWWW
cc111111111111111111
"],
    )
    .speed(0.3, 0.6)
    .fps(0.15)
    .tint("1:rRbBgG")
    .weight(4)
    .biomes(FRESH | TANK),
    Visitor::new(
        "fishing-boat-angler",
        Lane::Boat,
        &[
            r"
          ╱────────╴
     ●???╱
    ╭█╮━╱
  ◢▀▀▀▀▀▀▀▀▀▀▀▀▀▀◣
∼≈◥██████████████◤
",
            r"
          ╱───────╌╴
     ◉???╱
    ╭█╮╼╱
  ◢▀▀▀▀▀▀▀▀▀▀▀▀▀▀◣
≈∼◥██████████████◤
",
        ],
        &[r"
          yyyyyyyyyy
     Y   y
    RRRyy
  1111111111111111
cc1111111111111111
"],
    )
    .tint("1:rRbBgG")
    .speed(0.2, 0.5)
    .fps(0.1)
    .weight(5)
    .traits(FISHING)
    .biomes(FRESH),
    Visitor::new(
        "pirate-ship",
        Lane::Boat,
        &[
            r"
               ┃█◦█?????????┃█◦█?????????┃█◦█
    ╻?????╺━━━━╋━━━━╸??╺━━━━╋━━━━╸??╺━━━━╋━━━━╸
 ▗▄▄▄▄▄▖??▐▓▓▓▓┃▓▓▓▓▌??▐▓▓▓▓┃▓▓▓▓▌??▐▓▓▓▓┃▓▓▓▓▌
 ▐█◦█◦█▌??◥▒▒▒▒┃▒▒▒▒◤??◥▒▒▒▒┃▒▒▒▒◤??◥▒▒▒▒┃▒▒▒▒◤
 ◢▄▄▄▄▄▄▄▄▄▄▄▄▄╿▄▄▄▄▄▄▄▄▄▄▄▄╿▄▄▄▄▄▄▄▄▄▄▄▄╿▄▄▄▄▄▄◣━╸
∼≈◥█▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪█◤
",
            r"
               ┃▛◦▜?????????┃▛◦▜?????????┃▛◦▜
    ╻?????╺━━━━╋━━━━╸??╺━━━━╋━━━━╸??╺━━━━╋━━━━╸
 ▗▄▄▄▄▄▖??▐▒▒▒▒┃▒▒▒▒▌??▐▒▒▒▒┃▒▒▒▒▌??▐▒▒▒▒┃▒▒▒▒▌
 ▐█◦█◦█▌??◥▓▓▓▓┃▓▓▓▓◤??◥▓▓▓▓┃▓▓▓▓◤??◥▓▓▓▓┃▓▓▓▓◤
 ◢▄▄▄▄▄▄▄▄▄▄▄▄▄╿▄▄▄▄▄▄▄▄▄▄▄▄╿▄▄▄▄▄▄▄▄▄▄▄▄╿▄▄▄▄▄▄◣━╸
≈∼◥█▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪██▪█◤
",
        ],
        &[r"
               yKWK         yKWK         yKWK
    Y     yyyyyyyyyyy  yyyyyyyyyyy  yyyyyyyyyyy
 yyyyyyy  wWWWWyWWWWw  wWWWWyWWWWw  wWWWWyWWWWw
 yyCyCyy  wwwwwywwwww  wwwwwywwwww  wwwwwywwwww
 yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
ccyyKyyKyyKyyKyyKyyKyyKyyKyyKyyKyyKyyKyyKyyKyyKyy
"],
    )
    .speed(0.5, 0.9)
    .fps(0.15)
    .weight(3)
    .traits(SCARY)
    .biomes(OCEAN | REEF),
    Visitor::new(
        "steamship",
        Lane::Boat,
        &[
            r"
    ╿◣???○???⠂???╿◣
    │???∘???⠒????│
   ╱│╲??▐█▌?▐█▌?╱│╲
 ▗▟▀▙▟██████████▙▟▀▖
 ▟█◦█◦█◦█◦█◦█◦█◦█▙▄◣
∼◥█████████████████◤
",
            r"
    ╿◢???⠂???○???╿◢
    │????∘???⠒???│
   ╱│╲??▐█▌?▐█▌?╱│╲
 ▗▟▀▙▟██████████▙▟▀▖
 ▟█◦█◦█◦█◦█◦█◦█◦█▙▄◣
≈◥█████████████████◤
",
        ],
        &[r"
    yR           yR
    y            y
   yyy  RRR RRR yyy
 wRRRwWWWWWWWWWWwRRw
 11Y1Y1Y1Y1Y1Y1Y1111
c1111111111111111111
"],
    )
    .tint("1:rRbBkK")
    .speed(0.5, 0.9)
    .fps(0.2)
    .weight(5)
    .traits(SCARY)
    .biomes(OCEAN | KELP | FRESH),
    Visitor::new(
        "cruise-liner",
        Lane::Boat,
        &[
            r"
                              ⠒?????????⠂
            ╿???????????????▗▄▄▄▖?????▗▄▄▄▖
          ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▐███▌▄▄▄▄▄▐███▌▄▟▀▙▄▟▀▙▄▟▀▙▄▖
       ▗▄▟█◦█◦█◦█◦█◦█◦█◦█◦█◦▐███▌█◦█◦█▐███▌◦█◦█◦█◦█◦█◦█◦█▙▄▖
     ▗▄▄▄▟█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦▙▄◣
  ∼≈◥█████████████████████████████████████████████████████████◤
",
            r"
                              ⠂?????????⠒
            ╿???????????????▗▄▄▄▖?????▗▄▄▄▖
          ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▐███▌▄▄▄▄▄▐███▌▄▟▀▙▄▟▀▙▄▟▀▙▄▖
       ▗▄▟█◦█◦█◦█◦█◦█◦█◦█◦█◦▐███▌█◦█◦█▐███▌◦█◦█◦█◦█◦█◦█◦█▙▄▖
     ▗▄▄▄▟█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦▙▄◣
  ≈∼◥█████████████████████████████████████████████████████████◤
",
        ],
        &[r"

            K               KKKKK     KKKKK
          wwwwwwwwwwwwwwwwwwYYYYYwwwwwYYYYYwRRRwRRRwRRRww
       wwwwCwCwCwCwCwCwCwCwCRRRRRwCwCwRRRRRCwCwCwCwCwCwCwwww
     wwwwwwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwCwww
  cc11111111111111111111111111111111111111111111111111111111111
"],
    )
    .tint("1:bBrRkK")
    .speed(0.4, 0.7)
    .fps(0.15)
    .weight(3)
    .traits(SCARY)
    .biomes(OCEAN | REEF),
    Visitor::new(
        "container-ship",
        Lane::Boat,
        &[
            r"
        ⠒
       ▐█▌????????▐▥▥▌▐▥▥▌▐▥▥▌????▐▥▥▌????????????▐▥▥▌▐▥▥▌
    ▟█◦█◦█◦█▙?▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌????▐▥▥▌▐▥▥▌▐▥▥▌????▐▥▥▌▐▥▥▌
    ▟█◦█◦█◦█▙?▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌
 ▛▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▜◣
∼≈◥████████████████████████████████████████████████████████◤
",
            r"
        ⠐
       ▐█▌????????▐▥▥▌▐▥▥▌▐▥▥▌????▐▥▥▌????????????▐▥▥▌▐▥▥▌
    ▟█◦█◦█◦█▙?▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌????▐▥▥▌▐▥▥▌▐▥▥▌????▐▥▥▌▐▥▥▌
    ▟█◦█◦█◦█▙?▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌▐▥▥▌
 ▛▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▜◣
≈∼◥████████████████████████████████████████████████████████◤
",
        ],
        &[r"

       KRK        333355552222    6666            44441111
    wwCwCwCww 2222555533336666    111122224444    55553333
    wwCwCwCww 33336666555522224444111133336666555522224444
 wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
cc1111111111111111111111111111111111111111111111111111111111
"],
    )
    .tint("1:bBrRkK 2:rRyY 3:gGcC 4:yYwW 5:mMrR 6:cCbB")
    .speed(0.4, 0.8)
    .fps(0.15)
    .weight(4)
    .traits(SCARY)
    .biomes(OCEAN),
    Visitor::new(
        "yacht",
        Lane::Boat,
        &[
            r"
             ╿
         ▗▄▄▟█▙▄▄▖
    ▗▄▄▟█◦█◦█◦█◦█▙▄▄▄▖
 ▗▟█████████████████████▜◣
∼≈◥██████████████████████◤
",
            r"
             ╽
         ▗▄▄▟█▙▄▄▖
    ▗▄▄▟█◦█◦█◦█◦█▙▄▄▄▖
 ▗▟█████████████████████▜◣
≈∼◥██████████████████████◤
",
        ],
        &[r"
             w
         wwwwwwwww
    wwwwwCwCwCwCwwwwww
 1111111111111111111111111
cckkkkkkkkkkkkkkkkkkkkkkkk
"],
    )
    .tint("1:bBkKrR")
    .speed(0.7, 1.3)
    .fps(0.2)
    .weight(5)
    .traits(SCARY)
    .biomes(OCEAN | REEF | KELP),
    Visitor::new(
        "paddle-steamer",
        Lane::Boat,
        &[
            r"
            ⠒????⠂
           ▐█▌??▐█▌
 ╭─┬─╮▗▄▄▟██████████▙▄▄▄▖
 ├─╋─┤▐█◦█◦█◦█◦█◦█◦█◦█◦█▌
 ╰─┴─╯◥█████████████████◤
",
            r"
            ⠂????⠒
           ▐█▌??▐█▌
 ╭─┬─╮▗▄▄▟██████████▙▄▄▄▖
 ├─╳─┤▐█◦█◦█◦█◦█◦█◦█◦█◦█▌
 ╰─┴─╯◥█████████████████◤
",
        ],
        &[r"
            w    w
           RRR  RRR
 11111WWWWWWWWWWWWWWWWWWW
 11111WwCwCwCwCwCwCwCwCWW
 11111RRRRRRRRRRRRRRRRRRR
"],
    )
    .tint("1:rRyY")
    .speed(0.4, 0.8)
    .fps(0.3)
    .weight(4)
    .traits(SCARY)
    .biomes(FRESH),
    Visitor::new(
        "icebreaker",
        Lane::Boat,
        &[
            r"
              ╿??????⠒
          ▗▄▄▟█▙▄▄▖?▐█▌
      ▗▄▄▄▟█◦█◦█◦█▙▄▄▄▖
   ▗▟█◦█◦█◦█◦█◦█◦█◦█◦█◦█▙▖
 ▟█████████████████████████◣?▗◇
∼≈◥█████████████████████████◤▫◇▫
",
            r"
              ╽??????⠐
          ▗▄▄▟█▙▄▄▖?▐█▌
      ▗▄▄▄▟█◦█◦█◦█▙▄▄▄▖
   ▗▟█◦█◦█◦█◦█◦█◦█◦█◦█◦█▙▖
 ▟█████████████████████████◣?▗◇
≈∼◥█████████████████████████◤▫◇▫
",
        ],
        &[r"

          wwwwwwwww YRY
      wwwwwwCwCwCwwwwww
   wwwCwCwCwCwCwCwCwCwCwww
 RRRRRRRRRRRRRRRRRRRRRRRRRRR WW
ccRRRRRRRRRRRRRRRRRRRRRRRRRRRWWW
"],
    )
    .speed(0.4, 0.8)
    .fps(0.15)
    .weight(4)
    .traits(SCARY)
    .biomes(ARCTIC),
    Visitor::new(
        "tugboat",
        Lane::Boat,
        &[
            r"
         ⠒
        ▐█▌
      ▗▟█◦█◦█▙▖
  ▗▄▄▄▟████████▙▄▄▖
 ▟██◉██◉██◉██◉██◉█▙◣
∼≈◥████████████████◤
",
            r"
          ∘
        ▐█▌
      ▗▟█◦█◦█▙▖
  ▗▄▄▄▟████████▙▄▄▖
 ▟██◉██◉██◉██◉██◉█▙◣
≈∼◥████████████████◤
",
        ],
        &[r"
 
        RRR
      wwwCwCwww
  WWWWWWWWWWWWWWWWW
 111k11k11k11k11k111
cc111111111111111111
"],
    )
    .tint("1:rRbBgG")
    .speed(0.4, 0.8)
    .fps(0.2)
    .weight(5)
    .traits(SCARY)
    .biomes(OCEAN | FRESH | KELP),
    Visitor::new(
        "river-barge",
        Lane::Boat,
        &[
            r"
      ●
     ╭█╮??????▗▟█▓█▙▖
  ▗▄▄▄▄▄▄▄▄▟▓█▓█▓█▓█▙▄▄▖
∼◥██████████████████████◤
",
            r"
      ◉
     ╭█╮??????▗▟█▓█▙▖
  ▗▄▄▄▄▄▄▄▄▟▓█▓█▓█▓█▙▄▄▖
≈◥██████████████████████◤
",
        ],
        &[r"
      Y
     yyy      KKKKKKK
  111111111KKKKKKKKKK111
c111111111111111111111111
"],
    )
    .tint("1:rRyYgG")
    .speed(0.2, 0.5)
    .fps(0.15)
    .weight(5)
    .biomes(FRESH),
    Visitor::new(
        "trawler",
        Lane::Boat,
        &[
            r"
                 ╿
    ╱╲????????▗▄▟█▙▄▖
   ╱??╲??????▗▟█◦█◦█▙▖
 ▗▄▄▄▄▄▄▄▄▄▄▄▟███████▙▄▄▄▖
 ▟█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█▙
∼≈◥██████████████████████◤
",
            r"
                 ╽
    ╱╲????????▗▄▟█▙▄▖
   ╱??╲??????▗▟█◦█◦█▙▖
 ▗▄▄▄▄▄▄▄▄▄▄▄▟███████▙▄▄▄▖
 ▟█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█◦█▙
≈∼◥██████████████████████◤
",
        ],
        &[r"
                 w
    yy        wwwwwww
   y  y      wwwCwCwww
 111111111111wwwwwwwww1111
 11Y1Y1Y1Y1Y1Y1Y1Y1Y1Y1Y11
cc111111111111111111111111
"],
    )
    .tint("1:rRbBgG")
    .speed(0.3, 0.7)
    .fps(0.15)
    .weight(5)
    .traits(NET | SCARY)
    .biomes(OCEAN | KELP | ARCTIC),
    Visitor::new(
        "longship",
        Lane::Boat,
        &[
            r"
            ┃
        ▐▒▒▒┃▒▒▒▌??????????╭●◣
        ▐▓▓▓┃▓▓▓▌??????????┃
 ▗▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄█
 ◥██████████████████████████◤
   ╱??╱??╱??╱??╱??╱??╱??╱
",
            r"
            ┃
        ▐▒▒▒┃▒▒▒▌??????????╭●◣
        ▐▓▓▓┃▓▓▓▌??????????┃
 ▗▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄◉▄█
 ◥██████████████████████████◤
    ╱??╱??╱??╱??╱??╱??╱??╱
",
        ],
        &[
            r"
            y
        W111y111W          YkY
        W111y111W          Y
 yy2y2y2y2y2y2y2y2y2y2y2y2yy
 yyyyyyyyyyyyyyyyyyyyyyyyyyyy
   y  y  y  y  y  y  y  y
",
            r"
            y
        W111y111W          YkY
        W111y111W          Y
 yy2y2y2y2y2y2y2y2y2y2y2y2yy
 yyyyyyyyyyyyyyyyyyyyyyyyyyyy
    y  y  y  y  y  y  y  y
",
        ],
    )
    .tint("1:rRbBgy 2:wWyY")
    .speed(0.5, 1.0)
    .fps(0.3)
    .weight(4)
    .biomes(OCEAN | ARCTIC | KELP),
    Visitor::new(
        "submarine",
        Lane::Deep,
        &[
            r"
               ╭╴
             ▗▟█▙▖
  ▗▄▄▄▄▄▄▄▄▄▄▟███▙▄▄▄▄▄▄▄▄▄▖
╳▟█◦██◦██◦██◦██◦██◦██◦██████▙◗
⠠⠂◥████████████████████████◤
",
            r"
               ╭╴
             ▗▟█▙▖
  ▗▄▄▄▄▄▄▄▄▄▄▟███▙▄▄▄▄▄▄▄▄▄▖
╋▟█◦██◦██◦██◦██◦██◦██◦██████▙◗
⠂∘◥████████████████████████◤
",
        ],
        &[r"
               w
             YYYYY
  YYYYYYYYYYYYYYYYYYYYYYYYYY
KYYCYYCYYCYYCYYCYYCYYCYYYYYYY
wwYYYYYYYYYYYYYYYYYYYYYYYYYY
"],
    )
    .speed(0.5, 1.2)
    .fps(0.4)
    .weight(5)
    .traits(LURE | GLOW)
    .biomes(OCEAN | REEF | KELP | DEEP | ARCTIC),
    Visitor::new(
        "bathyscaphe",
        Lane::Deep,
        &[
            r"
   ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
 ▗▟██████████████████████▙▖
 ▝▜██████████████████████▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
            ╲?╱
          ▗▟█◉█▙▖
           ▝▀▀▀▘
",
            r"
   ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
 ▗▟██████████████████████▙▖
 ▝▜██████████████████████▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
            ╲?╱
          ▗▟█◎█▙▖
           ▝▀▀▀▘
",
        ],
        &[r"
   RRRRRRRRRRRRRRRRRRRRRR
 RRRRRRRRRRRRRRRRRRRRRRRRRR
 RRRRRRRRRRRRRRRRRRRRRRRRRR
    RRRRRRRRRRRRRRRRRRRRRR
            w w
          wwwYwww
           wwwww
"],
    )
    .speed(0.2, 0.5)
    .fps(0.2)
    .weight(3)
    .traits(LURE | GLOW)
    .biomes(DEEP | OCEAN),
    Visitor::new(
        "scuba-diver",
        Lane::Deep,
        &[
            r"
                  ∘
                ◦
       ▗▄▄▄▄▄▖??∘
  ╲╲▄▄▟█████▙▄▄◖●
   ╱╱??▝▀▀▀▀▘?╺━
",
            r"
                 ∘
                  ◦
       ▗▄▄▄▄▄▖??◦
  ╱╱▄▄▟█████▙▄▄◖●
   ╲╲??▝▀▀▀▀▘?╺━
",
        ],
        &[r"
 
 
       YYYYYYY
  CCRRRRRRRRRRRYk
   CC  RRRRRR YY
"],
    )
    .speed(0.3, 0.8)
    .fps(0.3)
    .weight(5)
    .traits(LURE)
    .biomes(OCEAN | REEF | KELP | DEEP | TANK),
    Visitor::new(
        "orca",
        Lane::Deep,
        &[
            r"
                            ▗▟
                           ▗▟█▙
                          ▗▟███▙
                         ▗▟█████▙
          ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟██████▙▄▄▄▄▄▄▄▄▄▄▖
 ◢▙?????▗▟███▓▓▓▓▓▓▓▓▓▓▓██████████▒▒▒▒████▙▄▖
 ◥██▙▟██████████████████████████████▒▒◉█████▙◗
 ◢█▛▝▜█▙▄░░░░░░░░░░░░░░░░░░░░░░░░░░░░▟████▛▘
        ▝▀▜▙▄░░░░░░░░░░░░░░░░░░░░░░▄▟▛▀▘
                     ▗▟██▛▘
                       ▝▜▛▘
",
            r"
                            ▗▟
                           ▗▟█▙
                          ▗▟███▙
                         ▗▟█████▙
          ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟██████▙▄▄▄▄▄▄▄▄▄▄▖
 ???????▗▟███▓▓▓▓▓▓▓▓▓▓▓██████████▒▒▒▒████▙▄▖
 ◢▙▄▄▟██████████████████████████████▒▒◉█████▙◗
 ◥▀▀▝▜█▙▄░░░░░░░░░░░░░░░░░░░░░░░░░░░░▟████▛▘
        ▝▀▜▙▄░░░░░░░░░░░░░░░░░░░░░░▄▟▛▀▘
                     ▗▟██▛▘
                       ▝▜▛▘
",
        ],
        &[r"
                            KK
                           KKKK
                          KKKKKK
                         KKKKKKKK
          KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK
 KK     KKKKKwwwwwwwwwwwKKKKKKKKKKWWWWKKKKKKK
 KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKWWWKKKKKKK
 KKKKKKKKWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
        KKKKKWWWWWWWWWWWWWWWWWWWWWWWWWWW
                     KKKKKK
                       KKKK
"],
    )
    .speed(1.2, 2.2)
    .fps(0.25)
    .weight(5)
    .traits(PREDATOR | SCARY)
    .biomes(OCEAN | ARCTIC | KELP),
    Visitor::new(
        "humpback-whale",
        Lane::Deep,
        &[
            r"
                              ▗▟▖
        ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟███▙▄▄▄▄▄▄▄▄▄▄▄▖
◢▙▖??▗▟███████████████████████████████▪▪▪▪▪▪██▙▖
?◥██▙██████████████████████████████████▒▒●███▙◗
?◢█▛▝▜█▙▄░░░░░░░░░░░░░░░░≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡▟▛▘
          ▝▜▙▄░░░░░░░░░░░░≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡▄▟▛▘
               ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
                               ▗▟██▙▖
                            ▗▟███▛▘
                        ▗▟███▛▘
                     ▗▟██▛▘
                   ▝▀▀▘
",
            r"
                              ▗▟▖
        ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟███▙▄▄▄▄▄▄▄▄▄▄▄▖
?????▗▟███████████████████████████████▪▪▪▪▪▪██▙▖
◢▙▄▄▟██████████████████████████████████▒▒●███▙◗
?◥▀▀▝▜█▙▄░░░░░░░░░░░░░░░░≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡▟▛▘
          ▝▜▙▄░░░░░░░░░░░░≡≡≡≡≡≡≡≡≡≡≡≡≡≡≡▄▟▛▘
               ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
                                ▗▟██▙▖
                               ▗▟███▛▘
                              ▗▟███▛▘
                            ▗▟██▛▘
                           ▝▀▀▘
",
        ],
        &[
            r"
                              bbb
        bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbWWWWWWbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbwwWbbbbb
bbbbbbbbbWWWWWWWWWWWWWWWWwwwwwwwwwwwwwwwwwwwWWW
          bbbbWWWWWWWWWWWWwwwwwwwwwwwwwwwWWWW
               WWWWWWWWWWWWWWWWWWWWWWWWWW
                               WWWWWW
                            WWWWWWW
                        WWWWWWW
                     WWWWWW
                   WWWW
",
            r"
                              bbb
        bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbWWWWWWbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbwwWbbbbb
bbbbbbbbbWWWWWWWWWWWWWWWWwwwwwwwwwwwwwwwwwwwWWW
          bbbbWWWWWWWWWWWWwwwwwwwwwwwwwwwWWWW
               WWWWWWWWWWWWWWWWWWWWWWWWWW
                                WWWWWW
                               WWWWWWW
                              WWWWWWW
                            WWWWWW
                           WWWW
",
        ],
    )
    .speed(0.8, 1.4)
    .fps(0.2)
    .weight(5)
    .traits(FILTER)
    .biomes(OCEAN | REEF | KELP | ARCTIC),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_visitors_drawn(super::VISITORS);
    }
}
