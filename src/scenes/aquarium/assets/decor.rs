//! Props and scenery for the sea floor.

use super::*;

pub const DECOR: &[Decor] = &[
    Decor::new(
        "treasure-chest",
        &[
            r"


   .-------.
  /=========\
 |-----------|
 |  [ (_) ]  |
 |___________|
",
            r"

   .-------.
  /=========\
 |$*$*$*$*$*$|
 |-----------|
 |  [ (_) ]  |
 |___________|
",
            r"
   .-------.
  /=========\
  '---------'
 .$*$*$*$*$*$.
 |-----------|
 |  [ (_) ]  |
 |___________|
",
        ],
        &[
            r"




 kkkkkkkkkkkkk
    K KKK K
",
            r"



  YYYYYYYYYYY
 kkkkkkkkkkkkk
    K KKK K
",
            r"



  YYYYYYYYYYY
 kkkkkkkkkkkkk
    K KKK K
",
        ],
    )
    .color('y')
    .fps(0.04)
    .bubbles(6, 2)
    .weight(7)
    .biomes(REEF | OCEAN | TANK),
    Decor::new(
        "coin-hoard",
        &[
            r"
        $
      .$*$.  o
   . ($*$*$*) *
  ($*$)$*$($*$*)
 ($*$*$*$*$*$*$*)
(_$*$*$*$*$*$*$*_)
",
            r"
       *
      .$*$.  o
   . ($*$*$*) $
  ($*$)$*$($*$*)
 ($*$*$*$*$*$*$*)
(_$*$*$*$*$*$*$*_)
",
        ],
        &[
            r"
        Y
       YWY   W
      YWYWYW  W
   YWY YWY YWYW
  YWYWYWYWYWYWYW
  YWYWYWYWYWYWYW
",
            r"
       W
       YWY   W
      YWYWYW  Y
   YWY YWY YWYW
  YWYWYWYWYWYWYW
  YWYWYWYWYWYWYW
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
     ___
    /___/|
 ___|___|/|
/___/|___|/
|___|/|___|
"],
        &[r"
     YYY
    YYYYYy
 YYYyYYYyYy
YYYYYyYYYyY
yYYYyYyYYYy
"],
    )
    .color('y')
    .weight(3)
    .biomes(REEF | OCEAN | TANK),
    Decor::new(
        "message-bottle",
        &[r"
    ||
   .''.
  /    \
 |  ~~  |
 | /__/ |
 |______|
"],
        &[r"
    yy

  w
    WW
   w  w
"],
    )
    .color('g')
    .bubbles(4, 0)
    .weight(4)
    .biomes(KELP | OCEAN | REEF),
    Decor::new(
        "bottle-lying",
        &[r"
   .-.
 _/   `--.__
(_  o o o  _)>
  `-.___.-'
"],
        &[r"


    W W W
"],
    )
    .color('g')
    .weight(5)
    .biomes(KELP | OCEAN | FRESH | ARCTIC),
    Decor::new(
        "old-boot",
        &[r"
 .----.
 |====|
 |    |
 |    `-.__
 |        __)
 '--------'
"],
        &[r"

 wwwwww


 k        kkk
 kkkkkkkkkk
"],
    )
    .color('y')
    .weight(5)
    .biomes(KELP | OCEAN | FRESH),
    Decor::new(
        "skull-and-crossbones",
        &[r"
 ()_   .-----.   _()
   \\ / o   o \ //
    \\\   v   ///
    ///\ VVV /\\\
 ()' // `-----' \\ `()
"],
        &[r"

        K   K
          K
"],
    )
    .weight(4)
    .biomes(OCEAN | REEF | TANK),
    Decor::new(
        "bleached-skull",
        &[r"
  .-''-.
 / o  o \
 \  /\  /
  |VVVV|
"],
        &[r"

   K  K
"],
    )
    .weight(5)
    .biomes(TANK | KELP | DEEP),
    Decor::new(
        "ships-wheel",
        &[r"
   \|/
 -- O --
   /|\
"],
        &[r"

    R
"],
    )
    .color('y')
    .weight(3)
    .biomes(OCEAN | REEF | TANK),
    Decor::new(
        "bronze-bell",
        &[r"
   _||_
  /    \
 / .--. \
/__|__|__\
   `--'
"],
        &[r"
   kkkk
"],
    )
    .color('y')
    .bubbles(5, 4)
    .weight(3)
    .biomes(OCEAN | ARCTIC | REEF),
    Decor::new(
        "diver-helmet",
        &[r"
   _____
  /  _  \
 | (( )) |
 |  `-'  |
 '--|_|--'
"],
        &[r"


   CC CC
"],
    )
    .color('y')
    .bubbles(4, 0)
    .weight(4)
    .biomes(OCEAN | REEF | TANK),
    Decor::new(
        "ship-anchor",
        &[r"
     .-.
     `-'
   ___|___
      |
 (\   |   /)
  \`._|_.'/
   `-._.-'
"],
        &[r"

"],
    )
    .color('k')
    .weight(6)
    .biomes(OCEAN | ARCTIC | KELP | TANK),
    Decor::new(
        "chain-anchor",
        &[r"
      .-.
      `-'
    ___|___
       |
  (\   |   /)       _   _
   \`._|_.'/ _ _ _ ( ) ( )
    `-._.-'(_) (_) `-' `-'
"],
        &[r"

"],
    )
    .color('k')
    .weight(4)
    .biomes(OCEAN | ARCTIC | KELP),
    Decor::new(
        "cannon",
        &[r"
          ___________
 ______.-'___________`-.
(_O_)==|=[_____________]==>
 `-'  |__|          |__|
"],
        &[r"

"],
    )
    .color('k')
    .weight(4)
    .biomes(OCEAN | REEF),
    Decor::new(
        "barrel-stack",
        &[r"
  .--.  .--.
  |==|  |==|
 .--. \/ .--.
 |==|.--.|==|
 |==||==||==|
 '--'|==|'--'
"],
        &[r"

   kk    kk

  kk      kk
  kk  kk  kk
      kk
"],
    )
    .color('y')
    .weight(5)
    .biomes(OCEAN | REEF | KELP),
    Decor::new(
        "giant-clam",
        &[
            r"


     _.-~~~~-._
   .'/ / || \ \'.
  / / /  ||  \ \ \
 (_(_(_______)_)_)
   `-.________.-'
",
            r"

     _.-~~~~-._
   .'/ / || \ \'.
  / / /  ||  \ \ \
 (~ ~ ~ ~ ~ ~ ~ ~ )
 (_(_(_______)_)_)
   `-.________.-'
",
            r"
     _.-~~~~-._
   .'/ / || \ \'.
  / / /  ||  \ \ \
 /_/_/___||___\_\_\
 (~ ~ ~ ( O ) ~ ~ )
 (_(_(_______)_)_)
   `-.________.-'
",
        ],
        &[
            r"


        mmmm
",
            r"

        mmmm


  m m m m m m m m
",
            r"
        mmmm



  m m m   W   m m
",
        ],
    )
    .fps(0.035)
    .bubbles(9, 3)
    .weight(6)
    .biomes(REEF),
    Decor::new(
        "temple-ruins",
        &[r"
   _____________________
  /_____________________\
  |=====================|__
   |#|???|#|???|#|???|#|
   |#|???|#|???|#|???|#|
   |#|???|#|???|#|???|#|
   |#|???|#|???|#|??
   |#|???|#|???|#|
  _|#|_?_|#|_?_|#|_    _
 [______________________][_]
"],
        &[r"



    K     K     K     K
    K     K     K     K
    K     K     K     K
    K     K     K
    K     K     K
    K     K     K
"],
    )
    .weight(4)
    .biomes(REEF | OCEAN),
    Decor::new(
        "standing-columns",
        &[r"
 ______     ______     ______
|______|   |______|   |______|
 | || |     | || |     | || |
 | || |     | || |     | || |
 | || |     | || |     | || |
 | || |     | || |     | || |
_|_||_|_   _|_||_|_   _|_||_|_
|_______| |_______|   |_______|
"],
        &[r"

"],
    )
    .weight(4)
    .biomes(REEF | OCEAN | TANK),
    Decor::new(
        "broken-columns",
        &[r"
                 ___
       _        /_ /
      /_|      | |/|
  ___ | |  __  | |
 /_ /\| | /_/\ | |
 |_|/ |_| | |  |_|
[_____] [____] [____]
"],
        &[r"

"],
    )
    .weight(5)
    .biomes(REEF | OCEAN),
    Decor::new(
        "moai-head",
        &[r"
    _.---._
   /=======\
  |  ,_ _,  |
 (|    L    |)
  |   ---   |
  |  \___/  |
   \       /
 ___|_____|___
"],
        &[r"

"],
    )
    .weight(4)
    .biomes(REEF | OCEAN),
    Decor::new(
        "statue-head",
        &[r"
    ,~~~~,
   /~~~~~~\
  | -    - |
  |   <    |
  |  ____  |
   \  \/  /
    |    |
  __|____|__
"],
        &[r"
     kkkk
    kkkkkk
"],
    )
    .weight(4)
    .biomes(REEF | OCEAN | KELP),
    Decor::new(
        "amphora-pair",
        &[r"
   _||_
  /_||_\
 (  \/  )          __
 |  /\  |    ___  /_/\
 |      |   /__ \ \_\/
  \    /   (_/_/'
   '--'
"],
        &[r"

"],
    )
    .color('y')
    .weight(5)
    .biomes(REEF | OCEAN | TANK),
    Decor::new(
        "atlantis-skyline",
        &[r"
                    |
       .          /\|/\          |
      /_\    ____/_/\_\___      /^\      .
  |   | |   /  ()  ()  ()  \   | |     /_\
 /^\  |_|  |  ____________ |  |_|     | |
 |_|  | |  | |? ? ? ? ? ?| | | |  ___ |_|
 | |__| |__| |? ? ? ? ? ?| |_| |_|   |_|
 |_|??|_|__|_|_____________|_|___|___|_|
"],
        &[r"
                    C
       y            C            C
                                 W       y
  C   C C                      C C
  W   C C  C               C  C C     C C
 C C  C C  C C           C C C C      C C
 C C  C C  C C           C C C C C   C C
 C C  C C  C C             C C   C   C C
"],
    )
    .color('c')
    .weight(3)
    .biomes(REEF | OCEAN),
    Decor::new(
        "coral-wreck",
        &[r"
                       |
          @%           |\
         %@@%@         | \        %@%
        @%%@@%         |  \      @%%@@
       %@@%%@%@     ___|___\_   %@@%@%
      %@%@%@%@@_.--'  ?    ?  `--.%%@%
    ___%@%@%%__/ ?   ?   __   ?   \_%@_
   /   ?    ?  \  ?    ?|__|  ?    ? \%|
  /  ?   ?   ?  `-.__ ?   ?     ?  __.-'|
 |__?___?___?___?___?___?___?___?___?___/
   `-----------------------------------'
"],
        &[r"

          mr
         rmmrm                    rmr
        mrrmmr                   mrrmm
       rmmrrmrm                 rmmrmr
      rmrmrmrmm                   rrmr
       rmrmrr                       rm
                                      r
"],
    )
    .color('y')
    .weight(4)
    .biomes(REEF),
    Decor::new(
        "sunken-galleon",
        &[r"
              |              |
             /|\            /|\
            / | \          / | \         ,---------------,
           /__|__\        /__|__\        |[] [] [] [] [] |
              |              |           |[]_[]_[]_[]_[] |
              |              |           |_______________|
   ,.----------------------------------------------------.
 ,'    o     o     o     ????????    o     o     o       |
/    ?     ?     ?     ??????????? ?     ?     ?     ?   |
|____?_____?_____?_____?_____?_____?_____?_____?_____?___'
  `-._________________________________________________.-'
"],
        &[r"
              w              w
             www            www
            w w w          w w w
           wwwwwww        wwwwwww         CC CC CC CC CC
              w              w            CC CC CC CC CC
              w              w

       K     K     K                 K     K     K
"],
    )
    .color('y')
    .weight(3)
    .biomes(OCEAN),
    Decor::new(
        "sunken-plane",
        &[r"
                                    _/|
                                  _/  |
                                _/    |
        _____________________.-'     _|
 \  _.-'  _   o  o  o  o  o   `-.___/ /
 -|-(_/  |_|  ___________________    /
 /  `-.__________________________`--'
         /_\          |__|
"],
        &[r"




              C  C  C  C  C


         kkk          kkkk
"],
    )
    .weight(3)
    .biomes(OCEAN),
    Decor::new(
        "cargo-container",
        &[r"
   ________________________
  |========================|
  |||||||||||||||||||||||||
  |||||||||||||||||||||||||
  |_______[]_______[]______|
 ________________________
|========================|
|||||||||||||||||||||||||
|||||||||||||||||||||||||
|_______[]_______[]_______|
"],
        &[r"
   wwwwwwwwwwwwwwwwwwwwwwww
  11111111111111111111111111
  1111111111111111111111111
  1111111111111111111111111
  1wwwwwwwkkwwwwwwwkkwwwwww1
 wwwwwwwwwwwwwwwwwwwwwwww
11111111111111111111111111
1111111111111111111111111
1111111111111111111111111
1wwwwwwwkkwwwwwwwkkwwwwwww1
"],
    )
    .tint("1:rbygc")
    .weight(4)
    .biomes(OCEAN),
    Decor::new(
        "lighthouse-ruin",
        &[r"
        _  ,_ _
       | \/ | \|
       |_  ?  _|
      /=========\
     |  [ ]      |
     /===========\
    |    [ ]     |
   /===============\
  |      [ ]       |
 /===================\
|____________________|
"],
        &[r"



       RRRRRRRRR
        K K
      RRRRRRRRRRR
         K K
    RRRRRRRRRRRRRRR
         K K
  RRRRRRRRRRRRRRRRRRR
"],
    )
    .weight(3)
    .biomes(OCEAN | REEF | ARCTIC),
    Decor::new(
        "rock-pile",
        &[r"
        _.--._
   _.-'/  .   `-._
 .'   /  '   .    `._
/ .  /_.-'  .  ' _.-' \
/__'_/_____/'--'_____/_\
"],
        &[r"
         K  K
    K     K     K
 K           K     K
  K    K    K     K
"],
    )
    .color('k')
    .weight(8)
    .biomes(KELP | REEF | OCEAN | FRESH),
    Decor::new(
        "rowboat-wreck",
        &[r"
                  /
                 /
 .------.______/___.-----.
  \  ?  `--.____.--' ?  /
   `-.______________.-'
"],
        &[r"

"],
    )
    .color('y')
    .weight(4)
    .biomes(KELP),
    Decor::new(
        "crab-trap",
        &[r"
   ,-------------.
  /  # # # # # # /|
 +-------------+ |
 | # # # # # # | |
 | #   __  # # |/
 +---(____)----+
"],
        &[r"

     y y y y y y

   y y y y y y
   y       y y
"],
    )
    .color('k')
    .weight(5)
    .biomes(KELP),
    Decor::new(
        "lobster-pot",
        &[r"
    _.-~~~-._
  .'| | | | |'.
 /  | | | | |  \
|===|=|=|=|=|===|
|  _|_|_|_|_|_  |
'-'-----------'-'
"],
        &[r"



kkkkkkkkkkkkkkkkk

kkkkkkkkkkkkkkkkk
"],
    )
    .color('y')
    .weight(5)
    .biomes(KELP),
    Decor::new(
        "rock-arch",
        &[r"
          _.----------._
       .-'  .  :  '  .  `-.
      /  '  ...    .  :    \
     /  :  /'         '\  . \
    /  .  /?????????????\   '\
   |  '  /???????????????\  : |
   |    |?????????????????|   |
  _|  . |?????????????????| ' |_
 /_|____/???????????????????\___\
"],
        &[r"
           K          K
       K    K        K    K
            KKK    K
                          K
       K


      K
"],
    )
    .color('k')
    .weight(3)
    .biomes(KELP | REEF),
    Decor::new(
        "cave-mouth",
        &[r"
         _.-----._
      .-'  . '  . `-.
    .'  :   .   '  : `.
   / .  _.-~~~~~-._  . \
  /  ' /           \ '  \
 / .  |             |  . \
|_'___|             |___'_|
"],
        &[r"
          K     K
      K    K    K   K
    K       K         K
     K   K       K   K

   K                   K
"],
    )
    .color('k')
    .weight(3)
    .biomes(KELP | DEEP),
    Decor::new(
        "black-smoker",
        &[
            r"
      ) ( )
     (  )  (
      ) (  )
       ( )
       |:|
      /: :\
     |:.':.|
    /.: ' :.\
   /' .:  .:'\
 _/_.:' '. ..:\_
",
            r"
       ( )
      )  ( (
     (  ) ( )
      ) (  )
       |:|
      /: :\
     |:.':.|
    /.: ' :.\
   /' .:  .:'\
 _/_.:' '. ..:\_
",
            r"
     ( )  )
      ) ( (
       ( )  )
      (  ) (
       |:|
      /: :\
     |:.':.|
    /.: ' :.\
   /' .:  .:'\
 _/_.:' '. ..:\_
",
        ],
        &[
            r"
      K K K
     K  K  K
      K K  K
       K K
        y
       y y
      y  y
      y   y
       y   y
     y       y
",
            r"
       K K
      K  K K
     K  K K K
      K K  K
        y
       y y
      y  y
      y   y
       y   y
     y       y
",
            r"
     K K  K
      K K K
       K K  K
      K  K K
        y
       y y
      y  y
      y   y
       y   y
     y       y
",
        ],
    )
    .color('k')
    .fps(0.05)
    .bubbles(7, 0)
    .weight(7)
    .biomes(DEEP),
    Decor::new(
        "whale-skeleton",
        &[r"
                  ,-/?/?/?/?/?/?/?/?/?/?/-.
                 /?/?/?/?/?/?/?/?/?/?/?/?/?\
    ___.---.__  (?(?(?(?(?(?(?(?(?(?(?(?(?(?)_
  .'  o       `-=o=o=o=o=o=o=o=o=o=o=o=o=o=o-o-o-.__
 (   __.-----._____________________________ ,o-o-o'
  `-'   `-.____.-'
"],
        &[r"



      W          W W W W W W W W W W W W W W W W
                                             W W W
"],
    )
    .weight(3)
    .biomes(DEEP),
    Decor::new(
        "sunken-submarine",
        &[r"
                   ,-.
                   | |__
             ______| |__|________
   __.------'  o    o    o    o   `-----.__
  (   ?     ?    ?    ?    ?    ?    ?    )>=
   `--.______________________________.--'
          /_\               /_\
"],
        &[r"
                   yyy
                   y yyy
             yyyyyyy yyyyyyyyyyyy
               Y    Y    Y    Y
"],
    )
    .color('k')
    .weight(3)
    .biomes(DEEP | OCEAN),
    Decor::new(
        "lost-rov",
        &[r"
      |
      |
  .---+---.__
 [| (O)(O) |_|-<
 [|=======|
   o     o
"],
        &[r"



 k   W  W
 k
"],
    )
    .color('y')
    .weight(4)
    .biomes(DEEP),
    Decor::new(
        "deep-cable",
        &[r"
                     ___
  _.-~-._      _.-~-[_o_]-._      _.-~-._
-'       `-.__.'              `-.-'       `-
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
     _.--._
  .-'  ,~' `-.__.--._
 /  ~'    ,~     `-. \
(___.-'~~'---._ ~' ___)
"],
        &[r"

        R
    R      R
       RR       R
"],
    )
    .color('k')
    .weight(5)
    .biomes(DEEP),
    Decor::new(
        "lava-vent",
        &[
            r"
         ( )
        (   )
         ) (


        /~~~\
       /     \
      /   .   \
    _/  :   :  \_
",
            r"

        ( ) )
         ( (
          ) )

        /~~~\
       /     \
      /   .   \
    _/  :   :  \_
",
            r"

          o
        ( ) (
         ) )  (

        /~~~\
       /     \
      /   .   \
    _/  :   :  \_
",
        ],
        &[
            r"
         K K
        K   K
         K K


         RRR
",
            r"

        K K K
         K K
          K K

         RRR
",
            r"

          K
        K K K
         K K  K

         RRR
",
        ],
    )
    .color('k')
    .fps(0.05)
    .bubbles(9, 3)
    .weight(5)
    .biomes(DEEP),
    Decor::new(
        "sunken-log",
        &[r"
    _.-._ ____________________________
  .'.---.'  '  .  '   '  .  ,     '   `\
 / | (o) |  .   '  ~  .   '  .  ' ~   \ |
 \ '.___.'. '  .  '   '   . ' .  _.-'`/
  `-.___.-'------------------------'
"],
        &[r"


                   g              g
"],
    )
    .color('y')
    .weight(7)
    .biomes(FRESH),
    Decor::new(
        "old-tire",
        &[r"
    .-----.
  .'  .-.  `.
 /   (   )   \
 |   (   )   |
 \    `-'    /
  `.       .'
    `-----'
"],
        &[r"

"],
    )
    .color('k')
    .weight(6)
    .biomes(FRESH),
    Decor::new(
        "shopping-cart",
        &[r"
 ,--.
  \  `.___________
   \   |#|#|#|#|#|
    \  |#|#|#|#|#|
     \_|_|_|_|_|_|
       (o)    (o)
"],
        &[r"


        c c c c c
        c c c c c
"],
    )
    .weight(4)
    .biomes(FRESH),
    Decor::new(
        "rusty-bicycle",
        &[r"
         __          _
        /__\_    ___/_|_
   .---.  \  `./`    `-.'`.
  / \|/ \  \  `--+---.  /   \
  |-(+)-|   `----(  )-'-(+)-|
  \ /|\ /            \ /|\ /
   `---'              `---'
"],
        &[r"

"],
    )
    .color('y')
    .weight(4)
    .biomes(FRESH),
    Decor::new(
        "river-boulders",
        &[r"
      _..._
   .-'     `-.     _.-._
  /  .   '    \ .-'  .  `-.
 /  '  .   .   Y   '    .  \
/__.___'__._____\_.___'_.__.\
"],
        &[r"
       KKK
   K         K      K K
     K          K    K    K
       K   K            K
   K      K       K     K  K
"],
    )
    .color('k')
    .weight(8)
    .biomes(FRESH),
    Decor::new(
        "sunken-canoe",
        &[r"
                                 __
    ___.-----------------.__.-'  `-.
 .-'  ?    ?     ?     ?     ?     `
 `-._________________________________.-'
"],
        &[r"

"],
    )
    .color('y')
    .weight(5)
    .biomes(FRESH),
    Decor::new(
        "sunken-car",
        &[r"
         .----------.
       .'| ???|????|'.
  ____/__|____|____|__\____
 /  (o)                  ,  \
|_____________________________|
   (O)                   (O)
"],
        &[r"



 y  yKy                  y  y
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
    K                     K
"],
    )
    .weight(4)
    .biomes(FRESH),
    Decor::new(
        "ice-rocks",
        &[r"
       _.--._
    .-'^^^^^`-._   _.-.
  .'^^^^^  ^^  `./^^^^`-.
 /  ' ^^  ' ^^ ^^| ' . ' \
/_.___'_.___'____\___.___'\
"],
        &[r"

       WWWWW
    WWWWW  WW     WWWW
      WW    WW WW
"],
    )
    .color('k')
    .weight(8)
    .biomes(ARCTIC),
    Decor::new(
        "ice-arch",
        &[r"
        _.-~~~~~~-._
     .-'############`-.
    /###/'        '\###\
   /##/'??????????'\### \
  /##/'????????????'\##  \
 |##|'??????????????'|##  |
_|##|'??????????????'|##__|_
"],
        &[r"

        WWWWWWWWWWWW
     WWW            WWW
    WW              WWW
   WW                WW
  WW                  WW
  WW                  WW
"],
    )
    .color('c')
    .weight(3)
    .biomes(ARCTIC),
    Decor::new(
        "expedition-ship",
        &[r"
              |           |           |
             /|\         /|\         /|\
            / | \       / | \       / | \
           /__|__\     /__|__\     /__|__\
              |           |           |
     |\       |           |           |      ___
     | \   ___|___________|___________|___  |[]_|
     |__\_/  o   o   o   o   o   o   o   \_|[]_|
  .-~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~-.
  \__?___?___?___?___?___?___?___?___?___?____/
   `-.______________________________________.-'
  ~^~^~^~^~^~^~^~^~^~^~^~^~^~^~^~^~^~^~^~^~^~^~
"],
        &[r"







             K   K   K   K   K   K   K
    WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW


  WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
"],
    )
    .color('y')
    .weight(2)
    .biomes(ARCTIC),
    Decor::new(
        "frozen-wreck",
        &[r"
           |\
           | \
    ___    |__\      ___
 .-'WWW'-._|__ `-.__.'WWW`-.
/WWWWW  ?   ?   ?   ?  WWWWW\
|WWW?  ?   ?  ?  ?   ? WWWWW|
 \WWW?___?___?___?____WWWW /
  `-VWWVWWVWWVWWVWWVWWVWW-'
    V  V V  V  V  V  V
"],
        &[r"



    WWW               WWW
 WWWWW                 WWWWW
 WWW                   WWWWW
  WWW                 WWWW
    CWWCWWCWWCWWCWWCWWCWW
    C  C C  C  C  C  C
"],
    )
    .color('y')
    .weight(3)
    .biomes(ARCTIC),
    Decor::new(
        "pirate-ship-ornament",
        &[r"
                 |>
        |        |        |
       /|\      /|\      /|\
      /_|_\    /_|_\    /_|_\
     /__|__\  /__|__\  /__|__\
        |        |        |
  ______|________|________|______
 /  o    o    o    o    o    o   \___
 \________________________________/ `-._
  `-.__________________________.-'
 .:'.:'.:'.:'.:'.:'.:'.:'.:'.:'.:'.:'.:'.
"],
        &[r"

        w        w        w
       www      www      www
      wwwww    wwwww    wwwww
     wwwwwww  wwwwwww  wwwwwww


    K    K    K    K    K    K


 YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
"],
    )
    .color('y')
    .weight(4)
    .biomes(TANK),
    Decor::new(
        "diver-with-chest",
        &[
            r"
     ____
    /    \
   | (  ) |         .-------.
   |_|__|_|        /=======/|
   /|====|\       |-------| |
  / |    | \      | (_)   |/
    |    |        |_______|
   _|_  _|_
  [___][___]
",
            r"
     ____
    /    \
   | (  ) |       .-------.
   |_|__|_|      /=======/
   /|====|\---._  '-------'
  / |    |    ` | .$*$*$*$.
    |    |       |-------|
   _|_  _|_      | (_)   |
  [___][___]     |_______|
",
            r"
     ____
    /    \
   | (  ) |       .-------.
   |_|__|_|      /=======/
   /|====|\---._  '-------'
  / |    |    ` | .$*$*$*$.
    |    |       |-------|
   _|_  _|_      | (_)   |
  [___][___]     |_______|
",
        ],
        &[
            r"

", r"

", r"

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
     o
    .
      O
   o   .
  .  O
    .o.
  (:::::)__
 (:.:.:.:)  `-._
",
            r"
      .
   o     o
     O
    .   .
  O  .
   .o.
  (:::::)__
 (:.:.:.:)  `-._
",
            r"
    .    O
      o
   .     .
     O  o
   o
    .O.
  (:::::)__
 (:.:.:.:)  `-._
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
 kkkkkkkkk  kkkk
",
            r"
      C
   C     C
     C
    C   C
  C  C
   CCC
  kkkkkkkkk
 kkkkkkkkk  kkkk
",
            r"
    C    C
      C
   C     C
     C  C
   C
    CCC
  kkkkkkkkk
 kkkkkkkkk  kkkk
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
    .-.
   (   )
   /|||\
  / ||| \
 (  ===  )
 |  ~~~  |
 |  ~~~  |
  \     /
   `---'
"],
        &[r"





    rrr
    rrr
"],
    )
    .color('y')
    .weight(5)
    .biomes(TANK),
    Decor::new(
        "tank-bridge",
        &[r"
          _.-~~~~~~-._
     _.-~'|-|-|-|-|-|-`~-._
   .'_____|_|_|_|_|_|_|____`.
  |_|?????????????????????|_|
  | |????.-~~~~~~~~-.?????| |
  |_|???/????????????\????|_|
 [___]?/??????????????\?[___]
"],
        &[r"

          R R R R R R
          R R R R R R R
  R R                     R R
  R R                     R R
  R R                     R R
"],
    )
    .color('r')
    .weight(4)
    .biomes(TANK),
    Decor::new(
        "tank-pagoda",
        &[r"
        _
       /_\
     _/___\_
    /_______\
      |[ ]|
   _/_______\_
  /___________\
    | [] [] |
 _/___________\_
/_______________\
   |[] [] []|
   |____##___|
"],
        &[r"




      wy yw


    w yy yy w


   wyy yy yyw
   wwwwwwwwwww
"],
    )
    .color('r')
    .weight(4)
    .biomes(TANK),
    Decor::new(
        "ceramic-castle",
        &[r"
   |>        |>        |>
  _|_       _|_       _|_
 |_|_|     |_|_|     |_|_|
 | o |_____| o |_____| o |
 |   |  _  |   |  _  |   |
 |___| / \ |___| / \ |___|
"],
        &[r"
    R         R         R


   K         K         K
"],
    )
    .weight(4)
    .biomes(TANK),
    Decor::new(
        "ruined-tower",
        &[r"
   _    _  _
  | |__| || |_
  |        ?  |
  |  []    ?  |
  |    ___    |
  |   |   |   |
 /|   |___| ? |\
/_|___________|_\
"],
        &[r"

"],
    )
    .color('k')
    .weight(4)
    .biomes(TANK | REEF | KELP),
    Decor::new(
        "sand-castle",
        &[r"
        |>
    _   |   _
   | |_|_|_| |
   |_  ___  _|
  _| |_|_|_| |_
 |_____ ___ ____|
"],
        &[r"
         R
"],
    )
    .color('y')
    .weight(3)
    .biomes(TANK),
    Decor::new(
        "wreck-with-flag",
        &[
            r"
                |~~~=>
                |
                |
     _          |
    / `-.___.-' |__
   /  ?    ?  ?    `-.__
  /  ?   ?   ?  ?   ?   `-.
  \__?___?___?___?___?____.-'
",
            r"
                |=~~~>
                |
                |
     _          |
    / `-.___.-' |__
   /  ?    ?  ?    `-.__
  /  ?   ?   ?  ?   ?   `-.
  \__?___?___?___?___?____.-'
",
            r"
                |~=~=>
                |
                |
     _          |
    / `-.___.-' |__
   /  ?    ?  ?    `-.__
  /  ?   ?   ?  ?   ?   `-.
  \__?___?___?___?___?____.-'
",
        ],
        &[
            r"
                 KKKkK
",
            r"
                 kKKKK
",
            r"
                 KkKkK
",
        ],
    )
    .color('y')
    .fps(0.08)
    .bubbles(12, 6)
    .weight(4)
    .biomes(OCEAN | REEF),
    Decor::new(
        "sunken-sword",
        &[r"
    ()
   -||-
    ||
    ||
    ||
 .:'||':.
"],
        &[r"
    yy
   yyyy



 yyyyyyyy
"],
    )
    .weight(3)
    .biomes(KELP | OCEAN | TANK),
];
