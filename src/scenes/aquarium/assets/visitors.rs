//! Passers-by: boats, birds, whales, submarines, divers and stranger things.

use super::*;

pub const VISITORS: &[Visitor] = &[
    Visitor::new(
        "sailboat",
        Lane::Boat,
        &[
            r"
        |>
       /|\
      / | \
     /__|__\
  ___\__|__/___
  \____________/
",
            r"
        |)
       /|\
      / | \
     /__|__\
  ___\__|__/___
  \____________/
",
        ],
        &[
            r"
        yR
       WyW
      W y W
     WWWyWWW
  111111y111111
  11111111111111
",
            r"
        yR
       WyW
      W y W
     WWWyWWW
  111111y111111
  11111111111111
",
        ],
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
         |>
         |\
         | \
         |__\
 ,____===|=====____,
  \__/         \__/
",
            r"
         |)
         |\
         | \
         |__\
 ,____===|=====____,
  \__/         \__/
",
        ],
        &[
            r"
         wR
         wW
         w W
         wWWW
 11111wwwwwwwww11111
  1111         1111
",
            r"
         wR
         wW
         w W
         wWWW
 11111wwwwwwwww11111
  1111         1111
",
        ],
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
        o  /
       /|\/
  ,___/_|_\___.
   `-._______.-'
",
            r"
        o
       /|\ \
  ,___/_|_\ \_.
   `-._______.-'
",
        ],
        &[
            r"
        Y  w
       wwww
  1111111111111
   1111111111111
",
            r"
        Y
       www w
  111111111 111
   1111111111111
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
        o
   \   /|\   /
    \_/_|_\_/
 ,___\_____/___.
  `-.________.-'
",
            r"
        o
    /  /|\  \
   /\_/_|_\_/\
 ,___/_____\___.
  `-.________.-'
",
        ],
        &[
            r"
        Y
   w   www   w
    wwwwwwwww
 111111111111111
  11111111111111
",
            r"
        Y
    w  www  w
   wwwwwwwwwww
 111111111111111
  11111111111111
",
        ],
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
       o  /
    ___/|\/___
 ><_____|_____>
",
            r"
       o \
    ___/|\ \___
 ><_____|_\___>
",
        ],
        &[
            r"
       Y  w
    wwwwwwwwww
 WW11111111111W
",
            r"
       Y w
    wwwwww wwww
 WW11111111111W
",
        ],
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
       o
     _/|\_____
 ~'-(_____o_)=>'~
",
            r"
       o
     _/|\_____
 ~-.(_____o_)=>-~
",
            r"
       o
     _/|\_____
 ~.-(_____o_)=>.~
",
        ],
        &[
            r"
       Y
     wwwwwwwww
 CCC111111Y1111CC
",
            r"
       Y
     wwwwwwwww
 CCC111111Y1111CC
",
            r"
       Y
     wwwwwwwww
 CCC111111Y1111CC
",
        ],
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
             ,-.
     o  o   ( o>
    /|\/|\   \ |
  ,-'-'-'-.__/ |
 (            _.'
  `-.._____..-'
",
            r"
             ,-.
     o  o   ( o>
    \|/\|/   \ |
  ,-'-'-'-.__/ |
 (            _.'
  `-.._____..-'
",
        ],
        &[
            r"
             WWW
     1  1   WkWY
    111111   W W
  WWWWWWWWWWWWWW
 WWWWWWWWWWWWWWWW
  WWWWWWWWWWWWW
",
            r"
             WWW
     1  1   WkWY
    111111   W W
  WWWWWWWWWWWWWW
 WWWWWWWWWWWWWWWW
  WWWWWWWWWWWWW
",
        ],
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
              ,
             /|
     o      / |
    /|\____/  |
  ,--/_|_\____\|
  \___________/ o
",
            r"
              ,
             /|
     o      / |
    /|\____/  |
  ,--/_|_\____\:
  \___________/ o
",
        ],
        &[
            r"
              w
             ww
     Y      w w
    wwwwwwww  w
  11111111111111
  1111111111111 Y
",
            r"
              w
             ww
     Y      w w
    wwwwwwww  w
  11111111111111
  1111111111111 Y
",
        ],
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
          |>???????????|[x]?????????|>
         )_)??????????)_)??????????)_)
        )___)????????)___)????????)___)
 |=|???)_____)\?????)_____)\?????)_____)\
 _________|____________|____________|______,.-'
 \_o__o__o__o__o__o__o__o__o__o__o__o__o__o__/
",
            r"
          |)???????????|[x]?????????|)
         )_)??????????)_)??????????)_)
        )___)????????)___)????????)___)
 |=|???)_____)\?????)_____)\?????)_____)\
 _________|____________|____________|______,.-'
 \_o__o__o__o__o__o__o__o__o__o__o__o__o__o__/
",
        ],
        &[
            r"
          yR           yWWW         yR
         KKK          KKK          KKK
        KKKKK        KKKKK        KKKKK
 yRy   KKKKKKKK     KKKKKKKK     KKKKKKKK
 yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
 yyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyy
",
            r"
          yR           yWWW         yR
         KKK          KKK          KKK
        KKKKK        KKKKK        KKKKK
 yRy   KKKKKKKK     KKKKKKKK     KKKKKKKK
 yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
 yyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyWyyy
",
        ],
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
     .    o  O
      o  (   )
        .==..==.
  ______|##||##|__
 /  o  o  o  o  o \__
 \__________________/
",
            r"
        O  o  .
       (   )  o
        .==..==.
  ______|##||##|__
 /  o  o  o  o  o \__
 \__________________/
",
        ],
        &[
            r"
     K    Y  K
      Y  K   K
        KKKKKKKK
  RRRRRRRRRRRRRRRR
 1  Y  Y  Y  Y  Y 111
 11111111111111111111
",
            r"
        K  Y  K
       K   K  Y
        KKKKKKKK
  RRRRRRRRRRRRRRRR
 1  Y  Y  Y  Y  Y 111
 11111111111111111111
",
        ],
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
                                 .=====.
                       ______    |  #  |   ______
                 ______|[][][]|__|_____|___|[][][]|____
          _______|  o  o  o  o  o  o  o  o  o  o  o  |_______
     ____/ o  o  o  o  o  o  o  o  o  o  o  o  o  o  o  o   \__
   /____________________________________________________________/
",
            r"
                                 .=====.
                       ______    |  #  |   ______
                 ______|[]|[]|__|_____|___|[]|[]|____
          _______|  o  o  o  o  o  o  o  o  o  o  o  |_______
     ____/ o  o  o  o  o  o  o  o  o  o  o  o  o  o  o  o   \__
   /____________________________________________________________/
",
        ],
        &[
            r"
                                 KKKKKKK
                       wwwwww    R  R  R   wwwwww
                 wwwwwwwCCCCCCwwwwwwwwwwwwwwCCCCCCwwwww
          wwwwwwww  C  C  C  C  C  C  C  C  C  C  C  wwwwwwww
     wwwww C  C  C  C  C  C  C  C  C  C  C  C  C  C  C  C   www
   11111111111111111111111111111111111111111111111111111111111111
",
            r"
                                 KKKKKKK
                       wwwwww    R  R  R   wwwwww
                 wwwwwwwCCwCCwwwwwwwwwwwwwwCCwCCwwwww
          wwwwwwww  C  C  C  C  C  C  C  C  C  C  C  wwwwwwww
     wwwww C  C  C  C  C  C  C  C  C  C  C  C  C  C  C  C   www
   11111111111111111111111111111111111111111111111111111111111111
",
        ],
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
     .
    _[=]_???[##][##]????[##][##][##]????????[##][##]????[##][#
  |_o_o_o_|?[##][##][##][##][##][##][##][##][##][##][##][##][#
  | o o o |?[##][##][##][##][##][##][##][##][##][##][##][##][#
 ||_______|___________________________________________________
 \____________________________________________________________
",
            r"
     o
    _[=]_???[##][##]????[##][##][##]????????[##][##]????[##][#
  |_o_o_o_|?[##][##][##][##][##][##][##][##][##][##][##][##][#
  | o o o |?[##][##][##][##][##][##][##][##][##][##][##][##][#
 ||_______|___________________________________________________
 \____________________________________________________________
",
        ],
        &[
            r"
     K
    wCCCw   33336666    555522225555        66663333    222255
  wwCwCwCww 22225555222244441111444411113333555522225555111144
  w C C C w 11114444111133336666333366662222444411114444666633
 wwwwwwwwww
 1111111111111111111111111111111111111111111111111111111111111
",
            r"
     K
    wCCCw   33336666    555522225555        66663333    222255
  wwCwCwCww 22225555222244441111444411113333555522225555111144
  w C C C w 11114444111133336666333366662222444411114444666633
 wwwwwwwwww
 1111111111111111111111111111111111111111111111111111111111111
",
        ],
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
        &[r"
              __
         __.-'__|__
  _____.'__ o o o o`-.__
 \_________________________>
  `-.____________________.-'
"],
        &[r"
              WW
         WWWWWWWWWW
  WWWWWWWWW C C C CWWWWW
 111111111111111111111111111
  11111111111111111111111111
"],
    )
    .tint("1:bBkKrR")
    .speed(0.7, 1.3)
    .weight(5)
    .traits(SCARY)
    .biomes(OCEAN | REEF | KELP),
    Visitor::new(
        "paddle-steamer",
        Lane::Boat,
        &[
            r"
         ||   ||
         ||   ||    _
  .--.  _||___||__|_|_
 ( /|\ )| o o o o o o |
  `-|-' |______________|\
~~~~~~~~\______________/
",
            r"
         ||   ||
         ||   ||    _
  .--.  _||___||__|_|_
 (-(+)-)| o o o o o o |
  `-|-' |______________|\
~~~~~~~~\______________/
",
        ],
        &[
            r"
         RR   RR
         RR   RR    R
  1111  wwwwwwwwwwwwww
 1 111 1w C C C C C C w
  11111 wwwwwwwwwwwwwwwww
yyyyyyyyyyyyyyyyyyyyyyyy
",
            r"
         RR   RR
         RR   RR    R
  1111  wwwwwwwwwwwwww
 1111111w C C C C C C w
  11111 wwwwwwwwwwwwwwwww
yyyyyyyyyyyyyyyyyyyyyyyy
",
        ],
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
        &[r"
            []
         ___||___
    ____|_o_o_o_|__
  _|__o__o__o__o__|__
 |__________________| \__
 \_____________________\_/
"],
        &[r"
            ww
         wwwwwwww
    wwwwwwCwCwCwwww
  wwwwCwwCwwCwwCwwwww
 RRRRRRRRRRRRRRRRRRRR RRR
 RRRRRRRRRRRRRRRRRRRRRRRRR
"],
    )
    .speed(0.4, 0.8)
    .weight(4)
    .traits(SCARY)
    .biomes(ARCTIC),
    Visitor::new(
        "tugboat",
        Lane::Boat,
        &[
            r"
        .  o
       ( )(  )
      _|##|_
   ___|_o_o_|__
 _/_o__o__o__o_\_
 \______________/
",
            r"
        o  .
       (  )( )
      _|##|_
   ___|_o_o_|__
 _/_o__o__o__o_\_
 \______________/
",
        ],
        &[
            r"
        K  C
       K KK  K
      RRRRRR
   wwwwwCwCwwww
 111C11C11C11C111
 1111111111111111
",
            r"
        C  K
       K  KK K
      RRRRRR
   wwwwwCwCwwww
 111C11C11C11C111
 1111111111111111
",
        ],
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
        &[r"
          o      .:::.
   __    /|\   .:::::::.
  |__|__/_|_\_|:::::::::|__
  \________________________/
"],
        &[r"
          w      wwwwK
   ww    www   wwwwwwKKK
  wwwwwwwwwwwwwwwwwwwKKKwKK
  11111111111111111111111111
"],
    )
    .tint("1:rRyYgG")
    .speed(0.2, 0.5)
    .weight(5)
    .biomes(FRESH),
    Visitor::new(
        "trawler",
        Lane::Boat,
        &[
            r"
          |         /
    |\    |      ___
    | \   |     |_o_|
  --+--\__|_____| o |__
   \__o___o__o__o_o___/
    \________________/
",
            r"
          |         \
    |\    |      ___
    | \   |     |_o_|
  --+--\__|_____| o |__
   \__o___o__o__o_o___/
    \________________/
",
        ],
        &[
            r"
          w         w
    ww    w      www
    w w   w     wwYww
  wwwwwwwwwwwwwww Y www
   111Y111Y11Y11Y1Y1111
    111111111111111111
",
            r"
          w         w
    ww    w      www
    w w   w     wwYww
  wwwwwwwwwwwwwww Y www
   111Y111Y11Y11Y1Y1111
    111111111111111111
",
        ],
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
            |
         .--+--.
         |#|#|#|
 ,       `--+--'            ,<
 \(o)_(o)_(o)_(o)_(o)_(o)_(o)_/'
  `-/-/-/-/-/-/-/-/-/-/-'
",
            r"
            |
         .--+--.
         |#|#|#|
 ,       `--+--'            ,<
 \(o)_(o)_(o)_(o)_(o)_(o)_(o)_/'
  `-\-\-\-\-\-\-\-\-\-\-'
",
        ],
        &[
            r"
            y
         wwwwwww
         w1w1w1w
 w       wwwwwww            ww
 yy2yyy2yyy2yyy2yyy2yyy2yyy2yyyy
  yyyyyyyyyyyyyyyyyyyyyyy
",
            r"
            y
         wwwwwww
         w1w1w1w
 w       wwwwwww            ww
 yy2yyy2yyy2yyy2yyy2yyy2yyy2yyyy
  yyyyyyyyyyyyyyyyyyyyyyy
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
              _
             | |_
         ____| |_|__
  ,-.  ,'    ___    `-._
 (-+-)=|  o  o  o  o   |=>
  `-'  `-._____________.-'
",
            r"
              _
             | |_
         ____| |_|__
  ,-.  ,'    ___    `-._
 (-|-)=|  o  o  o  o   |=>
  `-'  `-._____________.-'
",
        ],
        &[
            r"
              Y
             K KY
         YYYYK KYKYY
  YKY  YY    YYY    YKYY
 YKKKYYK  C  C  C  C   KYY
  YKY  YKYYYYYYYYYYYYYYYKY
",
            r"
              Y
             K KY
         YYYYK KYKYY
  YKY  YY    YYY    YKYY
 YKKKYYK  C  C  C  C   KYY
  YKY  YKYYYYYYYYYYYYYYYKY
",
        ],
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
    ______________________
  ,'  ::::::::::::::::    `.
 (_________________________)
       `-._      _.-'
         .-'`--'`-.
        (   (O)    )
         `-.____.-'
",
            r"
    ______________________
  ,'  ::::::::::::::::    `.
 (_________________________)
       `-._      _.-'
         .-'`--'`-.
        (   (o)    )
         `-.____.-'
",
        ],
        &[
            r"
    RRRRRRRRRRRRRRRRRRRRRR
  RR  WWWWWWWWWWWWWWWW    RR
 RRRRRRRRRRRRRRRRRRRRRRRRRRR
       wwww      wwww
         wwwwwwwwww
        w   wYw    w
         wwwwwwwwww
",
            r"
    RRRRRRRRRRRRRRRRRRRRRR
  RR  WWWWWWWWWWWWWWWW    RR
 RRRRRRRRRRRRRRRRRRRRRRRRRRR
       wwww      wwww
         wwwwwwwwww
        w   wyw    w
         wwwwwwwwww
",
        ],
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
            o
          .--. o
  \\     /(o) )>
   \\\_.'[##]'--->
   ///  `-.__/
",
            r"
            o
          .--.  o
   \\    /(o) )>
  ///_.'[##]'--->
  \\\   `-.__/
",
        ],
        &[
            r"
            C
          bbbb C
  BB     BbCb bb
   BBBbbbYYYYbbbbb
   BBB  bbbbbB
",
            r"
            C
          bbbb  C
   BB    BbCb bb
  BBBbbbYYYYbbbbb
  BBB   bbbbbB
",
        ],
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
                      ,\
                     /  \
 \ /      _.-------'    `--.___
   V.--.-'                   `-.__
 ,--'    `.   ___     (o)  ___   `.
 `.  `-.__ `-'   `-._        `--._ |
   `-._       `--..______..`---'
       `---..__________..---'
",
            r"
                      ,\
                     /  \
          _.-------'    `--.___
    .--.-'                   `-.__
 ,--'    `.   ___     (o)  ___   `.
 `.  `-.__ `-'   `-._        `--._ |
  /\`-._       `--..______..`---'
       `---..__________..---'
",
        ],
        &[
            r"
                      KK
                     K  K
 K K      KKKKKKKKKK    KKKKKKK
   KKKKKKK                   KKKKK
 KKKK    KK   KKK     KWK  KKK   KK
 WW  WWWWW WWW   WWWW        WWWWW W
   WWWW       WWWWWWWWWWWWWWWWWW
       WWWWWWWWWWWWWWWWWWWWWW
",
            r"
                      KK
                     K  K
          KKKKKKKKKK    KKKKKKK
    KKKKKK                   KKKKK
 KKKK    KK   KKK     KWK  KKK   KK
 WW  WWWWW WWW   WWWW        WWWWW W
  WWWWWW       WWWWWWWWWWWWWWWWWW
       WWWWWWWWWWWWWWWWWWWWWW
",
        ],
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
                         ,-----.
                    _.--'       `--._
 \ /    __..--'    o            `-._
   V.--'  ,-----.                    `-.
 ,--'`.,'  .-.  `.   .  .  .  .  .  . `.
 `.  `-.__.' / '--`-.__.__.__.__.__.__.`)
   `-._    /  /   `--...____________..-'
       `--/  /
          `-'
",
            r"
                         ,-----.
                    _.--'       `--._
        __..--'    o            `-._
   >.--'  ,-----.                    `-.
 ,--'`.,'  .-.  `.   .  .  .  .  .  . `.
 `.  `-.__.' / '--`-.__.__.__.__.__.__.`)
   `-._    /  /   `--...____________..-'
       `--/  /
          `-'
",
        ],
        &[
            r"
                         bbbbbbW
                    bWbbb       bbbWb
 b b    bbWWbbb    W            bbWb
   bWbbb  bbbbbbW                    bbW
 bbbbbWbb  WbW  bW   W  W  W  W  W  W bW
 bW  bbWbbWb b bbbbbWbbWbbWbbWbbWbbWbbWbb
   BBWB    B  B   BBBWWWBBBBBBBBBBBBWWBB
       BBBB  B
          BBB
",
            r"
                         bbbbbbW
                    bWbbb       bbbWb
        bbWWbbb    W            bbWb
   bWbbb  bbbbbbW                    bbW
 bbbbbWbb  WbW  bW   W  W  W  W  W  W bW
 bW  bbWbbWb b bbbbbWbbWbbWbbWbbWbbWbbWbb
   BBWB    B  B   BBBWWWBBBBBBBBBBBBWWBB
       BBBB  B
          BBB
",
        ],
    )
    .speed(0.8, 1.4)
    .fps(0.2)
    .weight(5)
    .traits(FILTER)
    .biomes(OCEAN | REEF | KELP | ARCTIC),
    Visitor::new(
        "sperm-whale",
        Lane::Deep,
        &[
            r"
                                  __________
                       __.-------'          `-.
 \ /     _..---------'                         `.
   V.--.-'                 ,-.                  |
 ,--'  `.  ~ ~ ~ ~ ~ ~ ~  (o)                   |
 `.  `-.__.~ ~ ~ ~ ~ ~ ~ ~ ~ ~ _______           |
   `-._        ~ ~ ~ ~ ~ ~ _.-'       `---.__ _.'
       `--.__________..--'                  `-'
",
            r"
                                  __________
                       __.-------'          `-.
         _..---------'                         `.
   >.--.-'                 ,-.                  |
 ,--'  `.  ~ ~ ~ ~ ~ ~ ~  (o)                   |
 `.  `-.__.~ ~ ~ ~ ~ ~ ~ ~ ~ ~ _______           |
   `-._        ~ ~ ~ ~ ~ ~ _.-'       `---.__ _.'
       `--.__________..--'                  `-'
",
        ],
        &[
            r"
                                  wwwwwwwwww
                       wwwwwwwwwww          www
 w w     wwwwwwwwwwwww                         ww
   wwwwwww                 www                  w
 wwww  ww  K K K K K K K  www                   w
 ww  wwwwwwK K K K K K K K K K wwwwwww           w
   wwww        K K K K K K wwww       wwwwwww www
       wwwwwwwwwwwwwwwwwww                  www
",
            r"
                                  wwwwwwwwww
                       wwwwwwwwwww          www
         wwwwwwwwwwwww                         ww
   wwwwwww                 www                  w
 wwww  ww  K K K K K K K  www                   w
 ww  wwwwwwK K K K K K K K K K wwwwwww           w
   wwww        K K K K K K wwww       wwwwwww www
       wwwwwwwwwwwwwwwwwww                  www
",
        ],
    )
    .speed(0.5, 1.0)
    .fps(0.15)
    .weight(4)
    .traits(PREDATOR)
    .biomes(DEEP | OCEAN),
    Visitor::new(
        "narwhal",
        Lane::Deep,
        &[
            r"
                       _.--.__
 \ /      __..--._    _.-'   o    `-.____________________---->
   V.--'        `--'          . '  .
 ,--'.                `--.__ .  '  .-'
 `--. `-.__.......---....__.-`-'-'
     `-._ _.-'
",
            r"
                       _.--.__
          __..--._    _.-'   o    `-.____________________---->
   >.--'        `--'          . '  .
 ,--'.                `--.__ .  '  .-'
 `--. `-.__.......---....__.-`-'-'
     `-._ _.-'
",
        ],
        &[
            r"
                       WKWWKWW
 W W      WWKKWWKW    WKWK   W    WWKWWWYYYYYYYYYYYYYYYYYYYYYY
   WKWWK        WWWK          K K  K
 WWWKK                WWWKWW K  K  KWK
 WWWK WWKWWKKKKKKKWWWKKKKWWKWWWKWK
     WWKW WKWK
",
            r"
                       WKWWKWW
          WWKKWWKW    WKWK   W    WWKWWWYYYYYYYYYYYYYYYYYYYYYY
   YKWWK        WWWK          K K  K
 WWWKK                WWWKWW K  K  KWK
 WWWK WWKWWKKKKKKKWWWKKKKWWKWWWKWK
     WWKW WKWK
",
        ],
    )
    .speed(0.6, 1.2)
    .fps(0.2)
    .weight(4)
    .biomes(ARCTIC | OCEAN),
    Visitor::new(
        "beluga",
        Lane::Deep,
        &[
            r"
              _.-----._
 \ /    _..--'   ,.  o  `.
   V.--'        ( `--.__ \
 ,--'.                  `--.\
 `--._`-.__...----...__.--'/
      `-._ _.-'
",
            r"
              _.-----._
        _..--'   ,.  o  `.
   >.--'        ( `--.__ \
 ,--'.                  `--.\
 `--._`-.__...----...__.--'/
      `-._ _.-'
",
        ],
        &[
            r"
              WWWWWWWWW
 W W    WWWWWW   WW  K  WW
   WWWWW        W WWWWWW W
 WWWWW                  WWWWW
 WWWWWWWWWWWWWWWWWWWWWWWWWWW
      WWWW WWWW
",
            r"
              WWWWWWWWW
        WWWWWW   WW  K  WW
   WWWWW        W WWWWWW W
 WWWWW                  WWWWW
 WWWWWWWWWWWWWWWWWWWWWWWWWWW
      WWWW WWWW
",
        ],
    )
    .speed(0.5, 1.0)
    .fps(0.2)
    .weight(5)
    .biomes(ARCTIC),
    Visitor::new(
        "blue-whale",
        Lane::Deep,
        &[
            r"
                                                      ,--.
                                             _.------'    `--.__
 \ /                         __..-------------'      o            `-.___
   V.--.__          __...--''                                         `--.
 ,--'    `--...---''     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .       `.
 `.   `-.__                ,-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'  )
   `-._    `--..__....----'''   ......................................____.-'
       `--...___..--''
",
            r"
                                                      ,--.
                                             _.------'    `--.__
                             __..-------------'      o            `-.___
   >.--.__          __...--''                                         `--.
 ,--'    `--...---''     .  .  .  .  .  .  .  .  .  .  .  .  .  .  .       `.
 `.   `-.__                ,-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'-'  )
   `-._    `--..__....----'''   ......................................____.-'
       `--...___..--''
",
        ],
        &[
            r"
                                                      bbbW
                                             bWbbbbbbb    bbbWbb
 b b                         bbWWbbbbbbbbbbbbbb      W            bbWbbb
   bWbbWbb          bbWWWbbbb                                         bbbW
 bbbb    bbbWWWbbbbb     W  W  W  W  W  W  W  W  W  W  W  W  W  W  W       bW
 BW   BBWBB                BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB  B
   BBWB    BBBWWBBWWWWBBBBBBB   WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWBBBBWBB
       BBBWWWBBBWWBBBB
",
            r"
                                                      bbbW
                                             bWbbbbbbb    bbbWbb
                             bbWWbbbbbbbbbbbbbb      W            bbWbbb
   bWbbWbb          bbWWWbbbb                                         bbbW
 bbbb    bbbWWWbbbbb     W  W  W  W  W  W  W  W  W  W  W  W  W  W  W       bW
 BW   BBWBB                BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB  B
   BBWB    BBBWWBBWWWWBBBBBBB   WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWBBBBWBB
       BBBWWWBBBWWBBBB
",
        ],
    )
    .speed(0.4, 0.8)
    .fps(0.12)
    .weight(2)
    .traits(FILTER)
    .biomes(OCEAN | DEEP | ARCTIC),
    Visitor::new(
        "giant-whale-shark",
        Lane::Deep,
        &[
            r"
        ,-.__
     _.'      `-.____
  /|'  .  .  .  .  `-.____
 <=|  .  .  .  .  .  .  . `-.__.---.._
  \|   . .  . .  . .  . .   o  _.--'__ `>
   |`-.__  .  .  .  .  .  ____..-'
         `--..______..-'
",
            r"
        ,-.__
     _.'      `-.____
 //|'  .  .  .  .  `-.____
 <=|  .  .  .  .  .  .  . `-.__.---.._
 \\|   . .  . .  . .  . .   o  _.--'__ `>
   |`-.__  .  .  .  .  .  ____..-'
         `--..______..-'
",
        ],
        &[
            r"
        bbWbb
     bWb      bbWbbbb
  bbb  W  W  W  W  bbWbbbb
 bbb  W  W  W  W  W  W  W bbWbbWbbbWWb
  bb   W W  W W  W W  W W   W  bWbbbbb bb
   bbbWbb  W  W  W  W  W  bbbbWWbb
         bbbWWbbbbbbWWbb
",
            r"
        bbWbb
     bWb      bbWbbbb
 bbbb  W  W  W  W  bbWbbbb
 bbb  W  W  W  W  W  W  W bbWbbWbbbWWb
 bbb   W W  W W  W W  W W   W  bWbbbbb bb
   bbbWbb  W  W  W  W  W  bbbbWWbb
         bbbWWbbbbbbWWbb
",
        ],
    )
    .speed(0.4, 0.9)
    .fps(0.2)
    .weight(3)
    .traits(FILTER)
    .biomes(OCEAN | REEF | DEEP),
    Visitor::new(
        "giant-manta",
        Lane::Deep,
        &[
            r"

                  ,-.
         ___..--'''  `-.
   __.--'    o   o     `-.__
 <'~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ `>
   `--.___            ___.--'
          ``--.____.--''
",
            r"
          ___..---.
     __.-'''       `-.
   _.'     `-.         `-.
  /   ....o   o  ....    `-.__
 <'~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~`>
       `--.___ ___.--'
              ``'
",
        ],
        &[
            r"

                  KKw
         KKKwwKKKKK  KKw
   KKwKKK    W   W     KKwKK
 KKw w w w w w w w w w w w w KK
   KKKwKKK            KKKwKKK
          KKKKwKKKKwKKKK
",
            r"
          KKKwwKKKw
     KKwKKKK       KKw
   KwK     KKw         KKw
  K   wwwwW   W  wwww    KKwKK
 KKw w w w w w w w w w w w w wKK
       KKKwKKK KKKwKKK
              KKK
",
        ],
    )
    .speed(0.6, 1.2)
    .fps(0.25)
    .weight(3)
    .traits(FILTER)
    .biomes(OCEAN | REEF | DEEP),
    Visitor::new(
        "giant-squid",
        Lane::Deep,
        &[
            r"
                                  _..----.._
                              _.-'          `-.
        ,--.__             .-'                `-._
  ,~~~-'      `--.___   .-'   (O)                  `-.__
 (  ~~~--..__         `-'                              `>
  `-.__     ``--.__  ...                          _..--''
 ,~~~~-.__ ~~~~--..__`-.___ ____..-----..........-'
(  ~~~~~~`-.__         ``-'  `'
 `-.__~~~~~~~~`-.___
   ~~~`-.__~~~~~~~~
",
            r"
                                  _..----.._
                              _.-'          `-.
        ,--.__             .-'                `-._
 ,~~-'~~~~    `--.___   .-'   (O)                  `-.__
(  ~~--.__~          `-'                              `>
 `-.__ ~~~`--.__  ...                          _..--''
  ~~~~-.__~~~~--..__`-.___ ____..-----..........-'
 `-.__~~~~~~`-.__       ``-'  `'
   ~~`-.__~~~~~~~~`-.__
      ~~~`-.__~~~~~
",
        ],
        &[
            r"
                                  RRRRRRRRRR
                              RRRR          RRR
        RRRRRR             RRR                RRRR
  RrrrRR      RRRRRRR   RRR   RWR                  RRRRR
 R  rrrRRRRRR         RRR                              RR
  RRRRR     RRRRRRR  RRR                          RRRRRRR
 RrrrrRRRR rrrrRRRRRRRRRRRR RRRRRRRRRRRRRRRRRRRRRRR
R  rrrrrrRRRRR         RRRR  RR
 RRRRRrrrrrrrrRRRRRR
   rrrRRRRRrrrrrrrr
",
            r"
                                  RRRRRRRRRR
                              RRRR          RRR
        RRRRRR             RRR                RRRR
 RrrRRrrrr    RRRRRRR   RRR   RWR                  RRRRR
R  rrRRRRRr          RRR                              RR
 RRRRR rrrRRRRRR  RRR                          RRRRRRR
  rrrrRRRRrrrrRRRRRRRRRRRR RRRRRRRRRRRRRRRRRRRRRRR
 RRRRRrrrrrrRRRRR       RRRR  RR
   rrRRRRRrrrrrrrrRRRRR
      rrrRRRRRrrrrr
",
        ],
    )
    .speed(0.4, 0.9)
    .fps(0.2)
    .weight(3)
    .traits(PREDATOR | SCARY | LEGENDARY)
    .biomes(DEEP | OCEAN),
    Visitor::new(
        "sea-serpent",
        Lane::Deep,
        &[
            r"
  ,-''''-.        ,-''''-.        ,-''''-.        ,-''''-.   ,_
 /        \      /        \      /        \      /        \ _/ o`>
/          \____,/          \____,/          \____,/          \_  `~-~
",
            r"
''''-.        ,-''''-.        ,-''''-.        ,-''''-.        ,-   ,_
      \      /        \      /        \      /        \      / _/ o`>
       \____,/          \____,/          \____,/          \____,  `~-~
",
            r"
-.        ,-''''-.        ,-''''-.        ,-''''-.        ,-''''   ,_
  \      /        \      /        \      /        \      / _/ o`>
   \____,/          \____,/          \____,/          \____,/  `~-~
",
            r"
      ,-''''-.        ,-''''-.        ,-''''-.        ,-''''-.   ,_
     /        \      /        \      /        \      /        \ _/ o`>
____,/          \____,/          \____,/          \____,/  `~-~
",
        ],
        &[
            r"
  gggggggg        gggggggg        gggggggg        gggggggg   gg
 g        g      g        g      g        g      g        g gg Ygg
g          ggggggg          ggggggg          ggggggg          gg  gggg
",
            r"
gggggg        gggggggg        gggggggg        gggggggg        gg   gg
      g      g        g      g        g      g        g      g gg Ygg
       ggggggg          ggggggg          ggggggg          gggggg  gggg
",
            r"
gg        gggggggg        gggggggg        gggggggg        gggggg   gg
  g      g        g      g        g      g        g      g gg Ygg
   ggggggg          ggggggg          ggggggg          ggggggg  gggg
",
            r"
      gggggggg        gggggggg        gggggggg        gggggggg   gg
     g        g      g        g      g        g      g        g gg Ygg
gggggg          ggggggg          ggggggg          ggggggg  gggg
",
        ],
    )
    .speed(0.5, 1.0)
    .fps(0.3)
    .weight(2)
    .traits(PREDATOR | SCARY | LEGENDARY)
    .biomes(OCEAN | DEEP | KELP),
    Visitor::new(
        "rov",
        Lane::Deep,
        &[
            r"
        |
        |
         ______
    _   /      \   _
   [_]=|  (O)   |=[_]
   |  |`--------'|  |--<
   `--(__)----(__)--'
",
            r"
        |
        |
         ______
    _   /      \   _
   [_]=|  (o)   |=[_]
   |  |`--------'|  |-<<
   `--(__)----(__)--'
",
        ],
        &[
            r"
        K
        K
         YYYYYY
    Y   Y      Y   Y
   YYYYY  YWY   YYYYY
   Y  YYYYYYYYYYYY  YYYK
   YYYYYYYYYYYYYYYYYY
",
            r"
        K
        K
         YYYYYY
    Y   Y      Y   Y
   YYYYY  YWY   YYYYY
   Y  YYYYYYYYYYYY  YYKK
   YYYYYYYYYYYYYYYYYY
",
        ],
    )
    .speed(0.3, 0.7)
    .fps(0.3)
    .weight(3)
    .traits(LURE | GLOW)
    .biomes(DEEP | OCEAN | REEF),
    Visitor::new(
        "toy-submarine",
        Lane::Deep,
        &[
            r"
      _
  .--|_|-.
 (_o_o_o__)=|
  `------'
",
            r"
      _
  .--|_|-.
 (_o_o_o__)=/
  `------'
",
            r"
      _
  .--|_|-.
 (_o_o_o__)=-
  `------'
",
            r"
      _
  .--|_|-.
 (_o_o_o__)=\
  `------'
",
        ],
        &[
            r"
      Y
  YRRRYRRY
 YYCYCYCYYYRR
  YRRRRRRY
",
            r"
      Y
  YRRRYRRY
 YYCYCYCYYYRR
  YRRRRRRY
",
            r"
      Y
  YRRRYRRY
 YYCYCYCYYYRR
  YRRRRRRY
",
            r"
      Y
  YRRRYRRY
 YYCYCYCYYYRR
  YRRRRRRY
",
        ],
    )
    .speed(0.3, 0.8)
    .fps(0.4)
    .weight(6)
    .traits(LURE)
    .biomes(TANK),
    Visitor::new(
        "mermaid",
        Lane::Deep,
        &[
            r"
                    ,--.
          ~~~~~~~~~( o  )
        ~~~~~~~~~~~ `-.-'
      ,-.        _.-'|'-.__
     / ,-`-..__.'  .-'  `-'>>
 <`-' ,'  `-.__.--'
  `--'
",
            r"
                    ,--.
          ~~~~~~~~~( o  )
         ~~~~~~~~~~ `-.-'
      ,-.        _.-'|'-.__
 <`-.-/ ,-`-..__.'  .-'  `-'>>
   `--'`.  `-.__.--'
         `-.
",
        ],
        &[
            r"
                    WWWW
          YYYYYYYWWW W  W
        YYYYYYYYYWW WWWWW
      GGG        WWWWWWWWWW
     G GGGGGGGGGG  WWW  WWWWW
 GGGG GG  GGGGGGGWW
  GGGG
",
            r"
                    WWWW
          YYYYYYYWWW W  W
         YYYYYYYYWW WWWWW
      GGG        WWWWWWWWWW
 GGGGGG GGGGGGGGGW  WWW  WWWWW
   GGGGGG  GGGGGGWWW
         GGG
",
        ],
    )
    .speed(0.4, 0.9)
    .fps(0.3)
    .weight(1)
    .traits(LURE | LEGENDARY)
    .biomes(REEF | OCEAN | KELP),
    Visitor::new(
        "swimmer",
        Lane::Float,
        &[
            r"
          ___
   . '   /   \(o
 ~.'~=====~==-'~~--
  ~'~.~~'~~.~'~~~
",
            r"
               _.-.
   ' .        (o   `.
 ~.'~=====~=====~~'~-
  ~'~.~~'~~.~'~~~
",
            r"
          ___
  '  .   /   \(o
 ~'.~=====~==-'~~--
  ~.~'~~.~~'~.~~~
",
        ],
        &[
            r"
          yyy
   W W   y   y11
 CCCCyyyyyCyyyCCCyy
  CCCCCCCCCCCCCCC
",
            r"
               yyyy
   W W        11   WW
 CCCCyyyyyCyyyyyCCCCy
  CCCCCCCCCCCCCCC
",
            r"
          yyy
  W  W   y   y11
 CCCCyyyyyCyyyCCCyy
  CCCCCCCCCCCCCCC
",
        ],
    )
    .tint("1:rRbBgG")
    .speed(0.3, 0.7)
    .fps(0.3)
    .weight(6)
    .traits(LURE)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "surfer",
        Lane::Float,
        &[
            r"
      \o/
       |
      / \
 ___.-'==`-.___
  ~~~~~~~~~~~~~
",
            r"
       o/
      /|
      / \
 ___.-'==`-.___
  ~~~~~~~~~~~~~
",
        ],
        &[
            r"
      yYy
       y
      y y
 11111111111111
  CCCCCCCCCCCCC
",
            r"
       Yy
      yy
      y y
 11111111111111
  CCCCCCCCCCCCC
",
        ],
    )
    .tint("1:rRbBgG")
    .speed(0.8, 1.6)
    .fps(0.2)
    .weight(4)
    .biomes(OCEAN | REEF),
    Visitor::new(
        "rubber-duck",
        Lane::Float,
        &[r"
        __
       (o >
   _   /  |
  ( \_/   )
   `-____-'
"],
        &[r"
        YY
       Yk R
   Y   Y  Y
  Y YYY   Y
   YYYYYYYY
"],
    )
    .speed(0.2, 0.5)
    .weight(5)
    .biomes(FRESH | TANK | OCEAN | KELP),
    Visitor::new(
        "message-in-a-bottle",
        Lane::Float,
        &[
            r"
    .-----.
  ,'  ===  `-._
 (   ===       )=|
  `._______..-'
",
            r"
    .-----.
  ,'  ---  `-._
 (   ---       )=|
  `._______..-'
",
        ],
        &[
            r"
    GYYYYYG
  GG  YYY  GYGG
 G   YYY       Gyy
  GGGGGGGGGGGYG
",
            r"
    GYYYYYG
  GG  YYY  GYGG
 G   YYY       Gyy
  GGGGGGGGGGGYG
",
        ],
    )
    .speed(0.1, 0.3)
    .fps(0.1)
    .weight(4)
    .biomes(OCEAN | REEF | KELP | ARCTIC | TANK),
    Visitor::new(
        "bell-buoy",
        Lane::Float,
        &[
            r"
     *
    /|\
   (_o_)
   /___\
  |#####|
  \#####/
",
            r"
     .
    /|\
   (_o_)
   /___\
  |#####|
  \#####/
",
        ],
        &[
            r"
     Y
    www
   wwYww
   wwwww
  1111111
  1111111
",
            r"
     Y
    www
   wwYww
   wwwww
  1111111
  1111111
",
        ],
    )
    .tint("1:rRgG")
    .speed(0.1, 0.25)
    .fps(0.15)
    .weight(5)
    .biomes(OCEAN | KELP | REEF | FRESH),
    Visitor::new(
        "iceberg",
        Lane::Float,
        &[r"
            /\
           /  \         /\
        /\/    \   /\  /  \
       /        \_/  \/    \
    __/    .  '            \__
 .-'    '       .    '   .     `-.
"],
        &[r"
            CC
           C  C         CC
        CCC    C   CC  C  C
       C        CCC  CC    C
    CCC    W  W            CCC
 WCW    W       W    W   W     CCW
"],
    )
    .speed(0.1, 0.25)
    .weight(5)
    .biomes(ARCTIC),
    Visitor::new(
        "ice-floe-bear",
        Lane::Float,
        &[r"
                 __
         ___..--'o `>
    _.--'`
   (    ,-----.,..-'
   /|__/|     |/|_ \
 .-'`-'`-'---'`-'`-`-.
 `-._________________.-'
"],
        &[r"
                 WW
         WWWWWWWWk Wk
    WWWWWW
   W    WWWWWWWWWWWW
   WWWWWW     WWWW W
 CCCCCCCCCCCCCCCCCCCCC
 CCCCCCCCCCCCCCCCCCCCCCC
"],
    )
    .speed(0.1, 0.3)
    .weight(4)
    .biomes(ARCTIC),
    Visitor::new(
        "penguin-floe",
        Lane::Float,
        &[
            r"
   _      _        _
  (o>    (o>      (o>
 //\\   //\\     //\\
  V_V    V_V      V_V
 .-----------------------.
 `-._________________.-'
",
            r"
   _      _        _
  (o>    (o>      (o>
 /(_)\  /(_)\    /(_)\
  V_V    V_V      V_V
 .-----------------------.
 `-._________________.-'
",
        ],
        &[
            r"
   W      W        W
  WWY    WWY      WWY
 WWWW   WWWW     WWWW
  WWW    WWW      WWW
 CCCCCCCCCCCCCCCCCCCCCCCCC
 CCCCCCCCCCCCCCCCCCCCCCC
",
            r"
   W      W        W
  WWY    WWY      WWY
 WWWWW  WWWWW    WWWWW
  WWW    WWW      WWW
 CCCCCCCCCCCCCCCCCCCCCCCCC
 CCCCCCCCCCCCCCCCCCCCCCC
",
        ],
    )
    .speed(0.1, 0.3)
    .fps(0.15)
    .weight(4)
    .biomes(ARCTIC),
    Visitor::new(
        "raft-castaway",
        Lane::Float,
        &[
            r"
          |>
          |\
    \o/   | \
     |    |__\
    / \   |
 [=======+======]
  ~~~~~~~~~~~~~
",
            r"
          |)
          |\
     o/   | \
     |\   |__\
    / \   |
 [=======+======]
  ~~~~~~~~~~~~~
",
        ],
        &[
            r"
          RR
          ww
    wYw   w w
     w    wwww
    w w   w
 yyyyyyyyyyyyyyyy
  CCCCCCCCCCCCC
",
            r"
          RR
          ww
     Yw   w w
     ww   wwww
    w w   w
 yyyyyyyyyyyyyyyy
  CCCCCCCCCCCCC
",
        ],
    )
    .speed(0.15, 0.4)
    .fps(0.2)
    .weight(3)
    .biomes(OCEAN | REEF | KELP),
    Visitor::new(
        "floating-otter",
        Lane::Float,
        &[
            r"
         ,-.
   ,____(o o)
  (  __  \_/ )
   `-.______.-'
",
            r"
         ,-.
   ,____(o o)
  (  \/  \_/ )
   `-.______.-'
",
        ],
        &[
            r"
         yyy
   yyyyyyW Wy
  y  yy  WyW y
   yyyyyyyyyyyy
",
            r"
         yyy
   yyyyyyW Wy
  y  WW  WyW y
   yyyyyyyyyyyy
",
        ],
    )
    .speed(0.1, 0.3)
    .fps(0.15)
    .weight(4)
    .biomes(KELP | OCEAN | ARCTIC),
    Visitor::new(
        "paper-boat",
        Lane::Boat,
        &[r"
       /\
      /  \
     /    \
 ___/______\___
 \____________/
"],
        &[r"
       WW
      W  W
     W    W
 WWWWWWWWWWWWWW
 WWWWWWWWWWWWWW
"],
    )
    .speed(0.2, 0.5)
    .weight(5)
    .biomes(TANK | FRESH),
    Visitor::new(
        "deep-sea-diver",
        Lane::Floor,
        &[
            r"
    ,-~~-.
  ,'      `.  ~
  |  [ (O) | ~
  `.-.___.-'
   /|_|_|\
  / |___| \
  \ |   | /
    |___|
   /|   |\
  [__]  [__]
",
            r"
    ,-~~-.
  ,'      `.   ~
  |  [ (O) |  ~
  `.-.___.-'
   /|_|_|\
  / |___| \
  \ |   | /
    |___|
    |\  /|
    [__][__]
",
        ],
        &[
            r"
    YYWWYY
  YY      YY  W
  Y  Y YCY Y W
  YYYYYYYYYY
   yyyyyyy
  y yyyyy y
  y y   y y
    yyyyy
   KK   KK
  KKKK  KKKK
",
            r"
    YYWWYY
  YY      YY   W
  Y  Y YCY Y  W
  YYYYYYYYYY
   yyyyyyy
  y yyyyy y
  y y   y y
    yyyyy
    KK  KK
    KKKKKKKK
",
        ],
    )
    .speed(0.2, 0.5)
    .fps(0.15)
    .weight(4)
    .traits(LURE | GLOW)
    .biomes(DEEP | OCEAN | REEF | KELP | TANK),
    Visitor::new(
        "sea-rover",
        Lane::Floor,
        &[
            r"
        _____
   ___,'     `.___
  |_o_|  [ ]   |==<
 (O)(O)(O)(O)(O)
",
            r"
        _____
   ___,'     `.___
  |_o_|  [ ]   |=-<
 (o)(o)(o)(o)(o)
",
        ],
        &[
            r"
        YYYYY
   YYYYY     YYYYY
  YYWYY  Y Y   YYYK
 KKKKKKKKKKKKKKK
",
            r"
        YYYYY
   YYYYY     YYYYY
  YYWYY  Y Y   YYYK
 KKKKKKKKKKKKKKK
",
        ],
    )
    .speed(0.15, 0.4)
    .fps(0.4)
    .weight(4)
    .traits(LURE | GLOW)
    .biomes(DEEP | OCEAN),
    Visitor::new(
        "seagulls",
        Lane::Sky,
        &[
            r"
   \v/????????/v\
        -v-
 -v-??????????????\v/
           /v\
",
            r"
   -v-????????-v-
        /v\
 \v/??????????????-v-
           -v-
",
            r"
   /v\????????\v/
        -v-
 -v-??????????????/v\
           \v/
",
            r"
   -v-????????-v-
        \v/
 /v\??????????????-v-
           -v-
",
        ],
        &[
            r"




", r"




", r"




", r"




",
        ],
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
\v/
      -v-
",
            r"
-v-
      /v\
",
            r"
/v\
      -v-
",
            r"
-v-
      \v/
",
        ],
        &[
            r"


", r"


", r"


", r"


",
        ],
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
   / `.
  /    `.             ___
 | _.--._`-._..--._.-'o  `-.__
  `'    `-.__ ____    `-.  ___`=====-.
         `--'    `-.__  `-'
",
            r"
            ___
  _..---.__.'  `-.      ,-.__
 `-._  ___  `-.__ `-.--'o  ___`=====-.
     `'   ``--.__ )-'`-.______.-'
         `-._
",
        ],
        &[
            r"



                               yyyyy

",
            r"


                               yyyyy


",
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
 ~~--.__
        `~-.__          _
  .__          `~-.__ ,-' )==-
     `~-.__ ___,.-~'`-.__\
           `
",
            r"

      __.--~~-.__      _
 _.-~'     __.-~'`-.__,-' )==-
 `~--'~'~'`        `-.__ /
          `
",
        ],
        &[
            r"


                           yy


",
            r"


                           yy


",
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
                                   ___
+-----------------+             __/_o_\_____
| OXIPER AQUARIUM |-----------((___________))>|
+-----------------+                `-'   `-'
",
            r"
                                   ___
+-----------------+             __/_o_\_____
| OXIPER AQUARIUM |-----------((___________))>/
+-----------------+                `-'   `-'
",
            r"
                                   ___
+-----------------+             __/_o_\_____
| OXIPER AQUARIUM |-----------((___________))>-
+-----------------+                `-'   `-'
",
            r"
                                   ___
+-----------------+             __/_o_\_____
| OXIPER AQUARIUM |-----------((___________))>\
+-----------------+                `-'   `-'
",
        ],
        &[
            r"
                                   www
wwwwwwwwwwwwwwwwwww             wwRwCwRwwwww
w YYYYYY YYYYYYYY wwRRRRRRRRRRwwwwwwwwwwwwwwwwR
wwwwwwwwwwwwwwwwwww                wRw   wRw
",
            r"
                                   www
wwwwwwwwwwwwwwwwwww             wwRwCwRwwwww
w YYYYYY YYYYYYYY wwRRRRRRRRRRwwwwwwwwwwwwwwwwR
wwwwwwwwwwwwwwwwwww                wRw   wRw
",
            r"
                                   www
wwwwwwwwwwwwwwwwwww             wwRwCwRwwwww
w YYYYYY YYYYYYYY wwRRRRRRRRRRwwwwwwwwwwwwwwwwR
wwwwwwwwwwwwwwwwwww                wRw   wRw
",
            r"
                                   www
wwwwwwwwwwwwwwwwwww             wwRwCwRwwwww
w YYYYYY YYYYYYYY wwRRRRRRRRRRwwwwwwwwwwwwwwwwR
wwwwwwwwwwwwwwwwwww                wRw   wRw
",
        ],
    )
    .left(
        &[
            r"
         ___
   _____/_o_\__             +-----------------+
|<((___________))-----------| OXIPER AQUARIUM |
   `-'   `-'                +-----------------+
",
            r"
         ___
   _____/_o_\__             +-----------------+
/<((___________))-----------| OXIPER AQUARIUM |
   `-'   `-'                +-----------------+
",
            r"
         ___
   _____/_o_\__             +-----------------+
-<((___________))-----------| OXIPER AQUARIUM |
   `-'   `-'                +-----------------+
",
            r"
         ___
   _____/_o_\__             +-----------------+
\<((___________))-----------| OXIPER AQUARIUM |
   `-'   `-'                +-----------------+
",
        ],
        &[
            r"
         www
   wwwwwwwCwwww             wwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwww wwwwww wwwwwwww w
   www   www                wwwwwwwwwwwwwwwwwww
",
            r"
         www
   wwwwwwwCwwww             wwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwww wwwwww wwwwwwww w
   www   www                wwwwwwwwwwwwwwwwwww
",
            r"
         www
   wwwwwwwCwwww             wwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwww wwwwww wwwwwwww w
   www   www                wwwwwwwwwwwwwwwwwww
",
            r"
         www
   wwwwwwwCwwww             wwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwwwwwwwwwwwwww wwwwww wwwwwwww w
   www   www                wwwwwwwwwwwwwwwwwww
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
       =======o=======
           ___|___
 ~~--=== /  o o o  \=>
         \_________/
       ====|_____|====
",
            r"
          ---o---
           ___|___
 ~~--=== /  o o o  \=>
         \_________/
       ====|_____|====
",
        ],
        &[
            r"
       kkkkkkkCkkkkkkk

            C C C

       wwwwwwwwwwwwwww
",
            r"
          kkkCkkk

            C C C

       wwwwwwwwwwwwwww
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
        &[r"
    _.---._
  ,'  | |  `.
  `.  | |  ,'
    `.\ /,'
     [_|_]
"],
        &[r"
    1122222
  11  2 2  2
  11  2 2  2
    112 222
     yyyyy
"],
    )
    .tint("1:rRbBgm 2:yYwWcm")
    .speed(0.2, 0.5)
    .weight(4)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "blimp",
        Lane::Sky,
        &[
            r"
          .-~~~~~~~~~~~~~~~~~~-.
     /\ ,'                      `-.
    /__|                          )>
    \  |`.     o  o  o  o  o   _.-'
     \/   `-.______.__.______.-'
",
            r"
          .-~~~~~~~~~~~~~~~~~~-.
     /\ ,'                      `-.
    /__-                          )>
    \  -`.     o  o  o  o  o   _.-'
     \/   `-.______.__.______.-'
",
        ],
        &[
            r"
          wwwwwwwwwwwwwwwwwwwwww
     11 1w                      www
    1111                          ww
    1  11w     C  C  C  C  C   wwww
     ww   wwwwwwwwwwwwwwwwwwwwww
",
            r"
          wwwwwwwwwwwwwwwwwwwwww
     11 1w                      www
    1111                          ww
    1  11w     C  C  C  C  C   wwww
     ww   wwwwwwwwwwwwwwwwwwwwww
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
           /\
          /  \
          \  /
 .-~-.~.-~ \/
  ~.-~
",
            r"
           /\
          /  \
          \  /
 ~.-~-.~.-~\/
   ~-.
",
        ],
        &[
            r"
           11
          1  1
          1  1
 222222222 11
  2222
",
            r"
           11
          1  1
          1  1
 222222222211
   222
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
        .--.
       /_oo_\
 .----'------`----.
(_ o  *  o  *  o _)
   `-._______.-'
",
            r"
        .--.
       /_oo_\
 .----'------`----.
(_ *  o  *  o  * _)
   `-._______.-'
",
        ],
        &[
            r"
        GGGG
       GGYYGG
 wwwwwwwwwwwwwwwwww
YY Y  Y  Y  Y  Y YY
   wwwwwwwwwwwww
",
            r"
        GGGG
       GGYYGG
 wwwwwwwwwwwwwwwwww
YY Y  Y  Y  Y  Y YY
   wwwwwwwwwwwww
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
      \\   //
       \\_//
 <==-.__\/___o>>
",
            r"
 <==-.__/\___o>>
      //  \\
     //    \\
",
        ],
        &[
            r"
      KK   KK
       WWWWW
 WWWWWWWWWWWWkYY
",
            r"
 WWWWWWWWWWWWkYY
      WW  WW
     KK    KK
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
   \  /
 <-=\/=o>>
   `--'
",
            r"

 <-=/\=o>>
  /`--'\
",
        ],
        &[
            r"
   B  B
 BBBBBBkkk
   YYYY
",
            r"

 BBBBBBkkk
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
   .\\      //.
    `\\.__.//'
 <===-.\\//._o>
      `-..-'
",
            r"

 <===-.//\\._o>
      //`--'\\
     //      \\
",
        ],
        &[
            r"
   yyy      yyy
    yyyyyyyyyy
 yyyyyyyyyyyykk
      WWWWWW
",
            r"

 yyyyyyyyyyyykk
      yyyyyyyy
     yy      yy
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
   / `.
  /    `.             ___
 | _.--._`-._..--._.-'o  `-.__
  `'    `-.__ ____    `-.  ___`=====-.
         `--'    `-.__  `-'
",
            r"
            ___
  _..---.__.'  `-.      ,-.__
 `-._  ___  `-.__ `-.--'o  ___`=====-.
     `'   ``--.__ )-'`-.______.-'
         `-._
",
        ],
        &[
            r"
   y yy
  y    yy             yyy
 y yyyyyyyyyyyyyyyyyyyk  yyyyy
  yy    yyyyy yyyy    yyy  yyyyYYYYYyy
         yyyy    yyyyy  yyy
",
            r"
            yyy
  yyyyyyyyyyy  yyy      yyyyy
 yyyy  yyy  yyyyy yyyyyyk  yyyyYYYYYyy
     yy   yyyyyyy yyyyyyyyyyyyyyy
         yyyy
",
        ],
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
  \  /
 ==\/-(o<
   ``'
",
            r"

 ==/\-(o<
  / `' \
",
        ],
        &[
            r"
  K  K
 KKKKKWkY
   WWW
",
            r"

 KKKKKWkY
  W WW W
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
?????????????????????????,-----.
????????????????????_.--'       `--._
?\ /    __..--'    o            `-._
???V.--'  ,-----.                    `-.
?,--'`.,'  .-.  `.   .  .  .  .  .  . `.
?`.  `-.__.' / '--`-.__.__.__.__.__.__.`)
???`-._    /  /   `--...____________..-'
???????`--/  /???????????????_.--.__
??????????`-'??????????\ /_.-'  o  `-.
?????????????????????????V.-.__ __.--'
?????????????????????????????`-'`-'
",
            r"
?????????????????????????,-----.
????????????????????_.--'       `--._
????????__..--'    o            `-._
???>.--'  ,-----.                    `-.
?,--'`.,'  .-.  `.   .  .  .  .  .  . `.
?`.  `-.__.' / '--`-.__.__.__.__.__.__.`)
???`-._    /  /   `--...____________..-'
???????`--/  /???????????????_.--.__
??????????`-'??????????\ /_.-'  o  `-.
?????????????????????????>.-.__ __.--'
?????????????????????????????`-'`-'
",
        ],
        &[
            r"
                         bbbbbbW
                    bWbbb       bbbWb
 b b    bbWWbbb    W            bbWb
   bWbbb  bbbbbbW                    bbW
 bbbbbWbb  WbW  bW   W  W  W  W  W  W bW
 bW  bbWbbWb b bbbbbWbbWbbWbbWbbWbbWbbWbb
   BBWB    B  B   BBBWWWBBBBBBBBBBBBWWBB
       BBBB  B               bbbbbb
          BBB          b bbbbb  W  bbb
                         bbWbbb bbbBB
                             BBBBB
",
            r"
                         bbbbbbW
                    bWbbb       bbbWb
        bbWWbbb    W            bbWb
   bWbbb  bbbbbbW                    bbW
 bbbbbWbb  WbW  bW   W  W  W  W  W  W bW
 bW  bbWbbWb b bbbbbWbbWbbWbbWbbWbbWbbWbb
   BBWB    B  B   BBBWWWBBBBBBBBBBBBWWBB
       BBBB  B               bbbbbb
          BBB          b bbbbb  W  bbb
                         bbWbbb bbbBB
                             BBBBB
",
        ],
    )
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
????????????????????????????????????????????????????,\
???????????????????????????????????????????????????/  \
???????????????????????????????\ /      _.-------'    `--.___
?????????????????????????????????V.--.-'                   `-.__
??????????????????????,\???????,--'    `.   ___     (o)  ___   `.
?????????????????????/  \??????`.  `-.__ `-'   `-._        `--._ |
??????????_.-------'    `--.___??`-._       `--..______..`---'
????.--.-'                   `-.__???`---..__________..---'
?,--'    `.   ___     (o)  ___   `.
?`.  `-.__ `-'   `-._        `--._ |????????,\
??/\`-._       `--..______..`---'????\ /_.-'  `-.__
???????`---..__________..---'??????????V.-' (o)  `-.
???????????????????????????????????????`-..____..-'
",
            r"
????????????????????????????????????????????????????,\
???????????????????????????????????????????????????/  \
????????????????????????????????????????_.-------'    `--.___
??????????????????????????????????.--.-'                   `-.__
??????????????????????,\???????,--'    `.   ___     (o)  ___   `.
?????????????????????/  \??????`.  `-.__ `-'   `-._        `--._ |
?\ /      _.-------'    `--.___?/\`-._       `--..______..`---'
???V.--.-'                   `-.__???`---..__________..---'
?,--'    `.   ___     (o)  ___   `.
?`.  `-.__ `-'   `-._        `--._ |????????,\
???`-._       `--..______..`---'??????_.-'  `-.__
???????`---..__________..---'??????????.-' (o)  `-.
??????????????????????????????????????/\-..____..-'
",
        ],
        &[
            r"
                                                    KK
                                                   K  K
                               K K      KKKKKKKKKK    KKKKKKK
                                 KKKKKKK                   KKKKK
                      KK       KKKK    KK   KKK     KWK  KKK   KK
                     K  K      WW  WWWWW WWW   WWWW        WWWWW W
          KKKKKKKKKK    KKKKKKK  WWWW       WWWWWWWWWWWWWWWWWW
    KKKKKK                   KKKKK   WWWWWWWWWWWWWWWWWWWWWW
 KKKK    KK   KKK     KWK  KKK   KK
 WW  WWWWW WWW   WWWW        WWWWW W        KK
  WWWWWW       WWWWWWWWWWWWWWWWWW    K KKKKK  KKKKK
       WWWWWWWWWWWWWWWWWWWWWW          KKKKK KWK  KK
                                       WWWWWWWWWWWW
",
            r"
                                                    KK
                                                   K  K
                                        KKKKKKKKKK    KKKKKKK
                                  KKKKKK                   KKKKK
                      KK       KKKK    KK   KKK     KWK  KKK   KK
                     K  K      WW  WWWWW WWW   WWWW        WWWWW W
 K K      KKKKKKKKKK    KKKKKKK WWWWWW       WWWWWWWWWWWWWWWWWW
   KKKKKKK                   KKKKK   WWWWWWWWWWWWWWWWWWWWWW
 KKKK    KK   KKK     KWK  KKK   KK
 WW  WWWWW WWW   WWWW        WWWWW W        KK
   WWWW       WWWWWWWWWWWWWWWWWW       KKKKK  KKK
       WWWWWWWWWWWWWWWWWWWWWW          KKKKK KWK  K
                                       WWWWWWWWWWWW
",
        ],
    )
    .speed(1.0, 1.8)
    .fps(0.25)
    .weight(2)
    .traits(PREDATOR | SCARY)
    .biomes(OCEAN | ARCTIC | KELP),
];
