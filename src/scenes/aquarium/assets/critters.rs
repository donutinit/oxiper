//! Swimmers that are not fish: jellyfish, cephalopods, seahorses, turtles, mammals.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "moon-jellyfish",
        &[
            r"
 .-'''''-.
(  o o o  )
 '-.....-'
  ) ( ) (
 ( ) ( ) )
",
            r"
 .-'''''-.
(  o o o  )
 '-.....-'
  ( ) ( )
   ) ( ) (
",
            r"
  .-'''-.
.'  o o  '.
'-.......-'
  ) ( ) (
 ( ) ( ) )
",
        ],
        &[
            r"
 111111111
1  W W W  1
 111111111
  1 1 1 1
 1 1 1 1 1
",
            r"
 111111111
1  W W W  1
 111111111
  1 1 1 1
   1 1 1 1
",
            r"
  1111111
11  W W  11
11111111111
  1 1 1 1
 1 1 1 1 1
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
    .-'''''''-.
  .'  .     .  '.
 /  .   '''   .  \
(__.-'''''''''-.__)
 ; ) ; ( ; ) ; ( ;
  ( ; ) ; ( ; ) ;
  ; ( ; ) ; ( ; (
   ) ; (  ; ) ;
   ; )   ( ;  ;
    (     ;  )
",
            r"
    .-'''''''-.
  .'  .     .  '.
 /  .   '''   .  \
(__.-'''''''''-.__)
 ( ; ) ; ( ; ) ; )
  ; ( ; ) ; ( ; (
  ( ; ) ; ( ; ) ;
   ; ) (  ; ( ;
   ) ;   ; (  )
    ;     (  ;
",
        ],
        &[r"
    11111111111
  11  1     1  11
 1  1   111   1  1
1111111111111111111
 2 2 2 2 2 2 2 2 2
  2 2 2 2 2 2 2 2
  2 2 2 2 2 2 2 2
   2 2 2  2 2 2
   2 2   2 2  2
    2     2  2
"],
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
  .---.
 /     \
|  o o  |
 \_____/
  | | |
  | | |
  ; | ;
  | ; |
  ; | ;
",
            r"
  .---.
 /     \
|  o o  |
 \_____/
  | | |
  | | |
  | ; |
  ; | ;
  | ; |
",
        ],
        &[r"
  11111
 1     1
1  k k  1
 1111111
  1 1 1
  1 1 1
  1 1 1
  1 1 1
  1 1 1
"],
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
       ,.
    _.' '-.__
 .-'  ~ ~    '-.
(_.-'''''''''-._)
  ~ ) ( ) ( ) ~
    ( ) ) ( (
     ) ( ( )
     ( ) )
      ) (
",
            r"
       ,.
    _.' '-.__
 .-'  ~ ~    '-.
(_.-'''''''''-._)
  ~ ( ) ( ) ( ~
    ) ( ( ) )
     ( ) ) (
     ) ( (
      ( )
",
        ],
        &[r"
       11
    111 11111
 111  1 1    111
11111111111111111
  2 2 2 2 2 2 2
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
 .--.
(::::)
(;;;;)
(::::)
(;;;;)
 '--'
  ))
  ((
   )
",
            r"
 .--.
(;;;;)
(::::)
(;;;;)
(::::)
 '--'
  ((
  ))
  (
",
        ],
        &[
            r"
 WWWW
WRRRRW
WYYYYW
WGGGGW
WCCCCW
 WWWW
  cc
  cc
   c
",
            r"
 WWWW
WRRRRW
WYYYYW
WGGGGW
WCCCCW
 WWWW
  cc
  cc
  c
",
        ],
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
   ,.-'''-.,
 .' .  :  . '.
(  :  .:.  :  )
 '-._.---._.-'
   )  ( )  (
   (  )  ( |
         |
         )
",
            r"
   ,.-'''-.,
 .' .  :  . '.
(  :  .:.  :  )
 '-._.---._.-'
   (  ) (  )
   )  (  | (
         )
         |
",
        ],
        &[r"
   111111111
 11 1  R  1 11
1  R  1R1  R  1
 1111111111111
   1  1 1  1
   1  1  1 1
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
~{~}~{~}~{~}~{~}~
 ~{~}~{~}~{~}~{~
  '-.._____.._.-'
",
            r"
{~}~{~}~{~}~{~}~{
 }~{~}~{~}~{~}~{
  '-.._____.._.-'
",
        ],
        &[r"
11111111111111111
 111111111111111
  111111111111111
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
   _.-'''-._
 .'   .-.   '.
(    ( O )    )
 '.   '-'   .'
   '-._.-._.'
    ) ( ) ( )
    ( ) ) ( (
     ) ( ( )
",
            r"
   _.-'''-._
 .'   .-.   '.
(    ( O )    )
 '.   '-'   .'
   '-._.-._.'
    ( ) ( ) (
    ) ( ( ) )
     ( ) ) (
",
        ],
        &[r"
   111111111
 11   111   11
1    1 Y 1    1
 11   111   11
   1111111111
    2 2 2 2 2
    2 2 2 2 2
     2 2 2 2
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
  ,.       ,.       ,.     .-.
~(oo)~~~~(OO)~~~~(oo)~~~~(   )
  '`       '`       '`     '-'
  ,.       ,.       ,.
",
            r"
  ,.       ,.       ,.     .-.
~(oo)~~~~(OO)~~~~(oo)~~~~(   )
  `'       `'       `'     '-'
   ,'       ,'       ,'
",
        ],
        &[
            r"
  11       11       11     111
11WW111111WW111111WW111111   1
  11       11       11     111
  11       11       11
",
            r"
  11       11       11     111
11WW111111WW111111WW111111   1
  11       11       11     111
   11       11       11
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
 .--. .--. .--. .--. .--.
(    ):    ):    ):    ):    )
 '--' '--' '--' '--' '--'
",
            r"
 .--. .--. .--. .--. .--.
(    );    );    );    );    )
 '--' '--' '--' '--' '--'
",
        ],
        &[r"
 1111 1111 1111 1111 1111
1    1W    1W    1W    1W    1
 1111 1111 1111 1111 1111
"],
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
      .-'''-.
    .'  . .  '.
 ,-/  .  (o)  \
(  (  `~._    _)
 `~,'--.  `--'
  `~ ,~' ~,
 ,~'~, `~'~,
(~ ~ ')  (~ ~'
",
            r"
      .-'''-.
    .'  . .  '.
 ,-/  .  (o)  \
(  (  `~._    _)
 `-.~-'  `--'
   ,~~, ,~~'
  (~ ~ ~ ~ ')
   `~'`~'`~'
",
        ],
        &[
            r"
      1111111
    11  1 1  11
 111  1  1W1  1
1  1  1111    11
 1111111  1111
  11 111 11
 11111 11111
11 1 11  11 11
",
            r"
      1111111
    11  1 1  11
 111  1  1W1  1
1  1  1111    11
 111111  1111
   1111 1111
  11 1 1 1 11
   111111111
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
   .---.
 .' o o '.
(  '(_)'  )
 '-.___.-'
 ,/|/ \|\.
(_/ )(  \_)
",
            r"
   .---.
 .' o o '.
(  '(_)'  )
 '-.___.-'
  //|\ |\\
 (_) )( (_)
",
        ],
        &[
            r"
   yyyyy
 yB W W By
y  ByyyB  y
 ByyyyyyyB
 yyyy yyyy
yyy yy  yyy
",
            r"
   yyyyy
 yB W W By
y  ByyyB  y
 ByyyyyyyB
  yyyy yyy
 yyy yy yyy
",
        ],
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
 ,-.  .---.  ,-.
(   )' o o '(   )
 '-'(  ,^,  )'-'
    '-.___.-'
     /|/|\|\
     ' ' ' '
",
            r"
  ,-. .---. ,-.
 (   )' o o '(   )
 '-' (  ,^,  )'-'
    '-.___.-'
     /|/|\|\
     ' ' ' '
",
            r"
 ,. .---. .,
(  )' o o '(  )
 '. (  ,^,  ) .'
    '-.___.-'
     /|/|\|\
     ' ' ' '
",
        ],
        &[
            r"
 111  11111  111
1   11 4 4 11   1
 1111  111  1111
    111111111
     1111111
     1 1 1 1
",
            r"
  111 11111 111
 1   11 4 4 11   1
 111 1  111  1111
    111111111
     1111111
     1 1 1 1
",
            r"
 11 11111 11
1  11 4 4 11  1
 11 1  111  1 11
    111111111
     1111111
     1 1 1 1
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
     _.--._
   .'      '._
 ,'   O       '-.__
(     '       __.'>
 `.__ ___..-''
    ) ) ) )
",
            r"
     _.--._
   .'      '._
 ,'   O       '-.__
(     '       __.'>
 `.___ __..-''
    ( ( ( (
",
        ],
        &[
            r"
     111111
   11      111
 11   B       11111
1     1       11111
 1111 11111111
    1 1 1 1
",
            r"
     111111
   11      111
 11   B       11111
1     1       11111
 11111 1111111
    1 1 1 1
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
  ,.---.
<(  O   )=-~~>
  '`---'
",
            r"
  ,.---.
<(  O   )==~~>
  '`---'
",
            r"
  ,.---.
<(  O   )-~~~>
  '`---'
",
        ],
        &[r"
  112221
11  k   122222
  112221
"],
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
 .-.
<(O )=~>
 '-'
",
            r"
 .-.
<(O )-~>
 '-'
",
        ],
        &[r"
 BBB
BCW BBBB
 BBB
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
   _.-----._
 .'  ~ . ~  '._
(  . ~  ~ .    O)=-~
 '-..~~~~~~..-'-'
",
            r"
   _.-----._
 .'  . ~ .  '._
(  ~ .  . ~    O)=-~
 '-..~~~~~~..-'-'
",
        ],
        &[
            r"
   111111111
 11  2 1 2  111
1  1 2  2 1    41212
 1111222222111111
",
            r"
   111111111
 11  1 2 1  111
1  2 1  1 2    41212
 1111222222111111
",
        ],
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
   ,-''-.
 ,' ,--. '.
/ ,' (@) `. \___
| | ( _ ) | | o )==
\ `.  _  ,' /'~~'~~
 `. `---' .'   ~~
   `-...-'
",
            r"
   ,-''-.
 ,' ,--. '.
/ ,' (@) `. \___
| | ( _ ) | | o )==
\ `.  _  ,' /'~'~~~
 `. `---' .'  ~ ~
   `-...-'
",
        ],
        &[
            r"
   111111
 11 1111 11
1 11 1Y1 11 1111
1 1 1 1 1 1 1 k 122
1 11  1  11 1122122
 11 11111 11   22
   1111111
",
            r"
   111111
 11 1111 11
1 11 1Y1 11 1111
1 1 1 1 1 1 1 k 122
1 11  1  11 1121222
 11 11111 11  2 2
   1111111
",
        ],
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
   ,.
  (  o)>-
 ~`)  )
  (  (
~ )  )
  ( (
   ) )
  (_,'
",
            r"
   ,.
  (  o)>-
  `)  )
 ~(  (
  )  )
~ ( (
   ( (
  (_,'
",
        ],
        &[
            r"
   11
  1  4111
 111  1
  1  1
1 1  1
  1 1
   1 1
  1111
",
            r"
   11
  1  4111
  11  1
 11  1
  1  1
1 1 1
   1 1
  1111
",
        ],
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
    ~   ~~  ~    ~
 ~~ ,\~~/~ ~/~~. ,'
(  (~~~~~~~~~~~~~~ o>-
 `~ \~ ~/ ~\~~ ~ '
   ~   ~~~  ~
",
            r"
   ~   ~~  ~    ~
 ~~ ,/~~\~ ~\~~. ,'
(  (~~~~~~~~~~~~~~ o>-
 `~ /~ ~\ ~/~~ ~ '
    ~  ~~~  ~
",
        ],
        &[
            r"
    2   22  2    2
 22 222222 22222 22
1  122222222222222 412
 22 22 22 2222 2 2
   2   222  2
",
            r"
   2   22  2    2
 22 222222 22222 22
1  122222222222222 412
 22 22 22 2222 2 2
    2  222  2
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
    ,~  ~,  ~
 ~,-'~~'~~'~,~~. ,
(  '~~~~~~~~~~~ o>-
 `-.~ `~' ~`-~'
",
            r"
   ~,  ~,   ~
 ,~-'~~'~~'~~,~. ,
(  '~~~~~~~~~~~ o>-
 `-~.`~' ~ `-.~'
",
        ],
        &[
            r"
    22  22  2
 222222222222222 2
1  222222222222 412
 2222 222 22222
",
            r"
   22  22   2
 222222222222222 2
1  222222222222 412
 2222222 2 22222
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
     ,--.__
 .--'  o   )~~
(  ,-._.-.'
 `-'/ /\ \
",
            r"
     ,--.__
 .--'  o   )~~
(  ,-._.-.'
 `-' \ \/ /
",
        ],
        &[
            r"
     111111
 1111  4   1WW
1  11111111
 1111 11 1
",
            r"
     111111
 1111  4   1WW
1  11111111
 111 1 11 1
",
        ],
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
-=o>
", r"
~=o>
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
   \|/
 ,-'''-.
(  (@)  )>
 '-.,.-'
",
            r"
 ,-'''-.
(  (@)  )>
 '-.,.-'
   /|\
",
        ],
        &[
            r"
   www
 wwwwwww
w  wRw  wW
 wwwwwww
",
            r"
 wwwwwww
w  wRw  wW
 wwwwwww
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
\ /
(@)~
",
            r"
(@)~
/ \
",
        ],
        &[
            r"
1 1
1W1W
",
            r"
1W1W
1 1
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
   ,~'~~'~,
 ,~~(~~~~~)~~, .
(~~~~~~~~~~~~~~~o)
 `~~(~~~~~)~~'
   `~,~~,~'
",
            r"
   ,~~'~'~,
 ,~~(~~~~~)~~, .
(~~~~~~~~~~~~~~~o)
 `~~(~~~~~)~~'
   `~~,~~,'
",
        ],
        &[r"
   11111111
 111Y11111Y111 Y
Y111111111111111WY
 111Y11111Y111
   11111111
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
    Species::new(
        "hawksbill-turtle",
        &[
            r"
     _.-'''''-._
   .'/\/\/\/\/\'.     ,__
 ,'/\/\/\/\/\/\/'-.__/ o \>
(_  `-._______.-'  `-'`--'
  `-.\ \.'      `.\ \'
     `-'          `-'
",
            r"
     _.-'''''-._
   .'/\/\/\/\/\'.     ,__
 ,'/\/\/\/\/\/\/'-.__/ o \>
(_  `-._______.-'  `-'`--'
   ,'/ /      ,'/ /
  '-'        '-'
",
        ],
        &[
            r"
     11111111111
   11yyyyyyyyyy11     111
 11yyyyyyyyyyyyy11111y 4 y2
21  1111111111111  1111111
  111y y11      11y y1
     111          111
",
            r"
     11111111111
   11yyyyyyyyyy11     111
 11yyyyyyyyyyyyy11111y 4 y2
21  1111111111111  1111111
   11y y      11y y
  111        111
",
        ],
    )
    .critter()
    .traits(DEFENDED | GRAZER)
    .tint("1:yYrR 2:yYgG 4:W")
    .color('y')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.3, 0.7)
    .fps(0.2)
    .weight(4)
    .biomes(REEF | OCEAN | TANK),
    Species::new(
        "red-eared-slider",
        &[
            r"
   _.-'''-._
 .'/\/\/\/\'.     ,-.
(  \/\/\/\/  )___/ o_)=
 '-.______.-'`-.___.'
    '-'  '-'
",
            r"
   _.-'''-._
 .'/\/\/\/\'.     ,-.
(  \/\/\/\/  )___/ o_)=
 '-.______.-'`-.___.'
   '-'    '-'
",
        ],
        &[
            r"
   111111111
 11yyyyyyyy11     111
1  yyyyyyyy  1111y 411R
 11111111111111111111
    111  111
",
            r"
   111111111
 11yyyyyyyy11     111
1  yyyyyyyy  1111y 411R
 11111111111111111111
   111    111
",
        ],
    )
    .critter()
    .traits(DEFENDED | GRAZER | SCAVENGER)
    .home("log,lily")
    .tint("1:gGyY 4:W")
    .color('g')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.2, 0.6)
    .fps(0.25)
    .weight(7)
    .biomes(FRESH | TANK),
    Species::new(
        "banded-sea-krait",
        &[
            r"
.-=-.     .-=-.     .-=-
     '-=-'     '-=-'    (o>~
",
            r"
     .-=-.     .-=-.
'-=-'     '-=-'     '-=-(o>~
",
        ],
        &[
            r"
wwbww     wwbww     wwbw
     wwbww     wwbww    wWwR
",
            r"
     wwbww     wwbww
wwbww     wwbww     wwbwwWwR
",
        ],
    )
    .critter()
    .traits(STING | PREDATOR | NOCTURNAL)
    .color('w')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.2, 0.7)
    .fps(0.3)
    .weight(3)
    .biomes(REEF | OCEAN),
    Species::new(
        "harbor-seal",
        &[
            r#"
                _.--._
          _.---'' o   `.
     _.--''  .  .  .   `>=
  ,-'  . .   .   .   _.-'=
 /  .   .  .   .  .-'
(__ _.-----..____.-' \ \_
  \_`-.__  `--.   ) )`-'
      `-'`--'  `-'
"#,
            r#"
                _.--._
          _.---'' o   `.
     _.--''  .  .  .   `>=
  ,-'  . .   .   .   _.-'=
 /  .   .  .   .  .-'
(__ _.-----..____.-'
  \_`-.__  `--.  \ \_\
      `-'`-.__`-.-'`-'
"#,
        ],
        &[
            r#"
                1k11k1    
          1k11111 4   1k  
     1k1111  k  k  k   11W
  111  k k   k   k   1k11W
 1  k   k  k   k  k11     
111 1k11111kk1111k11 1 11 
  1111k11  111k   1 1111  
      1111111  111        
"#,
            r#"
                1k11k1    
          1k11111 4   1k  
     1k1111  k  k  k   11W
  111  k k   k   k   1k11W
 1  k   k  k   k  k11     
111 1k11111kk1111k11      
  1111k11  111k  1 111    
      11111k1111k11111    
"#,
        ],
    )
    .critter()
    .traits(PREDATOR | CURIOUS)
    .tint("1:wyKc 4:W")
    .color('w')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.4, 1.2)
    .fps(0.3)
    .weight(6)
    .biomes(ARCTIC | KELP | OCEAN),
    Species::new(
        "california-sea-lion",
        &[
            r#"
                     _.-.
                    / o `.
                   |  __  >==
                   (  `-' /
               _.--'`.   (
          _.--' .   . `-._)
 .-.__.--'  .   .    .  /
(   ___.--.____.-'`-.  / \
 `-'    `--.   ) )  `-'\ \
            `-'`'
"#,
            r#"
                     _.-.
                    / o `.
                   |  __  >==
                   (  `-' /
               _.--'`.   (
          _.--' .   . `-._)
 .-.__.--'  .   .    .  /
(   ___.--.____.-'`-.  /
 `-'    `--.  \ \ )  `-'
            `-'`-'
"#,
        ],
        &[
            r#"
                     1k1k    
                    1 4 1k   
                   1  11  1WW
                   1  111 1  
               1k1111k   1   
          1k111 k   k 11k11  
 k1k11k111  k   k    k  1    
1   111k11k1111k1111k  1 1   
 111    111k   1 1  1111 1   
            11111            
"#,
            r#"
                     1k1k    
                    1 4 1k   
                   1  11  1WW
                   1  111 1  
               1k1111k   1   
          1k111 k   k 11k11  
 k1k11k111  k   k    k  1    
1   111k11k1111k1111k  1     
 111    111k  1 1 1  111     
            111111           
"#,
        ],
    )
    .critter()
    .traits(PREDATOR | CURIOUS)
    .tint("1:ykwy 4:W")
    .color('y')
    .swim(Swim::Dart)
    .zone(Zone::Any)
    .speed(0.5, 1.8)
    .fps(0.3)
    .weight(5)
    .biomes(OCEAN | KELP | REEF),
    Species::new(
        "emperor-penguin",
        &[
            r#"
                  ,-"'"-.
            _.--'#### o  `.
      _.--'############  `>>
  _.-'################:::.-'
==-.#################::::'
  `-._................-'
      `-.........._.-'
         `--'-'-\ \
"#,
            r#"
                  ,-"'"-.
            _.--'#### o  `.
      _.--'############  `>>
  _.-'################:::.-'
==-.#################::::'
  `-._................-'
      `-.........._.-'
         `--'-'-/ /
"#,
        ],
        &[
            r#"
                  kkkkkkk   
            kkkkkkkkk 4  kk 
      kkkkkkkkkkkkkkkkk  kyy
  kkkkkkkkkkkkkkkkkkkkYYYkkk
yykkkkkkkkkkkkkkkkkkkYYYYk  
  kkkkWWWWWWWWWWWWWWWkkk    
      kkkWWWWWWWWWkkkk      
         kkkkkkkk k         
"#,
            r#"
                  kkkkkkk   
            kkkkkkkkk 4  kk 
      kkkkkkkkkkkkkkkkk  kyy
  kkkkkkkkkkkkkkkkkkkkYYYkkk
yykkkkkkkkkkkkkkkkkkkYYYYk  
  kkkkWWWWWWWWWWWWWWWkkk    
      kkkWWWWWWWWWkkkk      
         kkkkkkkk k         
"#,
        ],
    )
    .critter()
    .traits(PREDATOR)
    .color('K')
    .swim(Swim::Dart)
    .zone(Zone::Any)
    .speed(0.6, 2.0)
    .fps(0.4)
    .school(1, 4)
    .weight(5)
    .biomes(ARCTIC),
    Species::new(
        "sea-otter",
        &[
            r#"
                   .-""-.
                .-'  o   `.
    _.---._   .'  .-.  `>=
 .-'  . .  `-/   (   )  _.'
(  ___   .  . `-._`-' .'
 `-'  `--.______.--'`-'
 ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~
"#,
            r#"
                   .-""-.
                .-'  o   `.
    _.---._   .'  .-.  `>=
 .-'  . .  `-/   (   )  _.'
(  ___   .  . `-._`-' .'
 `-'  `--.______.--'`-'
  ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~ ~
"#,
        ],
        &[
            r#"
                   k1111k  
                k11  4   1k
    1k111k1   k1  k1k  11W 
 k11  k k  111   1   1  1k1
1  111   k  k 11k1111 k1   
 111  111k111111k111111    
 c c c c c c c c c c c c c 
"#,
            r#"
                   k1111k  
                k11  4   1k
    1k111k1   k1  k1k  11W 
 k11  k k  111   1   1  1k1
1  111   k  k 11k1111 k1   
 111  111k111111k111111    
  c c c c c c c c c c c c c
"#,
        ],
    )
    .critter()
    .traits(PREDATOR | CURIOUS)
    .home("kelp")
    .tint("1:yykw 4:kW")
    .color('y')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.2, 0.7)
    .fps(0.3)
    .weight(5)
    .biomes(KELP | OCEAN),
    Species::new(
        "manatee",
        &[
            r#"
                     _.--._
               __.--'  o  `-.
     _..---'''''            `.
 .-._.'   .   ..   .  ___  )>
/   `   .   .    .  (   )_.'
(  _  `-.__ .   . _.-.`-'
 `-' `-.   ``---''` .  \ \
     `-.__)  `-.__.'   `-'
"#,
            r#"
                     _.--._
               __.--'  o  `-.
     _..---'''''            `.
 .-._.'   .   ..   .  ___  )>
/   `   .   .    .  (   )_.'
(  _  `-.__ .   . _.-.`-'
 `-' `-.   ``---''` .
     `-.__)  `-.__.'`-.`-.
"#,
        ],
        &[
            r#"
                     1k11k1   
               11k111  4  11k 
     1kk11111111            1k
 k1k1k1   k   kk   k  111  11 
1   1   k   k    k  1   11k1  
1  1  11k11 k   k 1k1k111     
 111 11k   11111111 k  1 1    
     11k111  11k11k1   111    
"#,
            r#"
                     1k11k1   
               11k111  4  11k 
     1kk11111111            1k
 k1k1k1   k   kk   k  111  11 
1   1   k   k    k  1   11k1  
1  1  11k11 k   k 1k1k111     
 111 11k   11111111 k         
     11k111  11k11k111k11k    
"#,
        ],
    )
    .critter()
    .traits(GRAZER | CURIOUS)
    .tint("1:wKyc 4:kW")
    .color('w')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.15, 0.4)
    .fps(0.15)
    .weight(4)
    .biomes(FRESH | REEF | KELP),
    Species::new(
        "dugong",
        &[
            r#"
                  _.--._
           __..--'  o   `-.
   _..---''''             `.
\\ .-'  .   .   .   ___  `.
 ))   .   .   .   (   ) _.'>:
// `-.__  .   .  _.-'`-'_.'
   `--..______..--' `-. \
        `-.__      _.-`.__)
"#,
            r#"
                  _.--._
           __..--'  o   `-.
   _..---''''             `.
// .-'  .   .   .   ___  `.
 ))   .   .   .   (   ) _.'>:
\\ `-.__  .   .  _.-'`-'_.'
   `--..______..--' `-.
        `-.__      _.-`.__)
"#,
        ],
        &[
            r#"
                  1k11k1     
           11kk111  4   11k  
   1kk1111111             1k 
11 k11  k   k   k   111  1k  
 11   k   k   k   1   1 1k111
11 11k11  k   k  1k111111k1  
   111kk111111kk111 11k 1    
        11k11      1k11k111  
"#,
            r#"
                  1k11k1     
           11kk111  4   11k  
   1kk1111111             1k 
11 k11  k   k   k   111  1k  
 11   k   k   k   1   1 1k111
11 11k11  k   k  1k111111k1  
   111kk111111kk111 11k      
        11k11      1k11k111  
"#,
        ],
    )
    .critter()
    .traits(GRAZER)
    .tint("1:wKyc 4:kW")
    .color('w')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.15, 0.4)
    .fps(0.15)
    .weight(3)
    .biomes(REEF | OCEAN),
    Species::new(
        "harbour-porpoise",
        &[
            r#"
             /\
            /  \
       _.--'    `--.__
 _..--'            o  `-.
<-.______                 `>
  `-.__  `--..____..--'--''
"#,
            r#"
             /\
            /  \
       _.--'    `--.__
 _..--'            o  `-.
<-.______                 `>
  `-.__  `--..____..--'--''
"#,
        ],
        &[
            r#"
             11             
            1  1            
       1k111    111k11      
 1kk111            4  11k   
11k111111                 11
  11k11  111kk1111kk1111111 
"#,
            r#"
             11             
            1  1            
       1k111    111k11      
 1kk111            4  11k   
11k111111                 11
  11k11  111kk1111kk1111111 
"#,
        ],
    )
    .critter()
    .traits(PREDATOR | CURIOUS)
    .tint("1:wKbc 4:W")
    .color('w')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.5, 1.4)
    .fps(0.3)
    .school(1, 3)
    .weight(5)
    .biomes(OCEAN | KELP | ARCTIC),
    Species::new(
        "beluga-calf",
        &[
            r#"
                   _..-._
               _.-'      `-.
        __..-''   o        `.
  _..--''                ,-'>
\\ `--..___       _..---'
 `\_  \  `-._  _.-'
   `-' `-.__.-'
"#,
            r#"
                   _..-._
               _.-'      `-.
        __..-''   o        `.
  _..--''                ,-'>
// `--..___       _..---'
 /'_  \  `-._  _.-'
 `-'`-.__.-'
"#,
        ],
        &[
            r#"
                   111111    
               1111      111 
        1111111   4        11
  1111111                1111
11 11111111       1111111    
 111  1  1111  1111          
   111 11111111              
"#,
            r#"
                   111111    
               1111      111 
        1111111   4        11
  1111111                1111
11 11111111       1111111    
 111  1  1111  1111          
 11111111111                 
"#,
        ],
    )
    .critter()
    .traits(CURIOUS)
    .color('W')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.3, 0.9)
    .fps(0.25)
    .school(1, 2)
    .weight(4)
    .biomes(ARCTIC),
    Species::new(
        "walrus",
        &[
            r#"
                _.--._
          _.---'  o   `-.
      _.-'    ,.,.,,    `.
   .-'   .   (,,,,,,,)    |
 .'    .  .    ||   ||    |
(   .       .  ||   ||_.-'
 `-._   .  _.-'`---'
    `-.___.-'   \ \
       `-'-.__   `-'
"#,
            r#"
                _.--._
          _.---'  o   `-.
      _.-'    ,.,.,,    `.
   .-'   .   (,,,,,,,)    |
 .'    .  .    ||   ||    |
(   .       .  ||   ||_.-'
 `-._   .  _.-'`---'
    `-.___.-'    \ \
       `-'-.__  `-'`-'
"#,
        ],
        &[
            r#"
                1k11k1     
          1k1111  4   11k  
      1k11    ykykyy    1k 
   k11   k   1yyyyyyy1    W
 k1    k  k    WW   WW    W
1   k       k  WW   WW1k11 
 11k1   k  1k1111111       
    11k111k11   1 1        
       1111k11   111       
"#,
            r#"
                1k11k1     
          1k1111  4   11k  
      1k11    ykykyy    1k 
   k11   k   1yyyyyyy1    W
 k1    k  k    WW   WW    W
1   k       k  WW   WW1k11 
 11k1   k  1k1111111       
    11k111k11    1 1       
       1111k11  111111     
"#,
        ],
    )
    .critter()
    .traits(DEFENDED | CURIOUS)
    .tint("1:yrwy 4:kW")
    .color('y')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.2, 0.6)
    .fps(0.2)
    .weight(4)
    .biomes(ARCTIC),
    Species::new(
        "axolotl",
        &[
            r"
       \\|//
      ,-\|/-.
 __.-'`--'--`.__
(~~~~~~~~~~~ o  ')
 `-~'-.-'-.-'`--'
    '-'  '-'
",
            r"
       //|\\
      ,-/|\-.
 __.-'`--'--`.__
(~~~~~~~~~~~ o  ')
 `-~'-.-'-.-'`--'
     '-' '-'
",
        ],
        &[
            r"
       RRRRR
      1RRRRR1
 111111111111111
111111111111 k  11
 1111111111111111
    111  111
",
            r"
       RRRRR
      1RRRRR1
 111111111111111
111111111111 k  11
 1111111111111111
     111 111
",
        ],
    )
    .critter()
    .traits(PREDATOR | NOCTURNAL)
    .home("hornwort,vallisneria,cabomba")
    .tint("1:wWmMyY")
    .color('w')
    .swim(Swim::Wave)
    .zone(Zone::Lower)
    .speed(0.1, 0.4)
    .fps(0.25)
    .weight(6)
    .biomes(FRESH | TANK),
    Species::new(
        "frog",
        &[
            r"
       _ _
      (o o)
 _..-'-----'-._
'---._______.--'
  /_/ `-'  `-'
",
            r"
       _ _
      (o o)
  _.-'-----'-._
 /'-._______.--'
/_/  `-'  `-'
",
        ],
        &[
            r"
       1 1
      14 41
 11111111111111
1111111111111111
  111 111  111
",
            r"
       1 1
      14 41
  1111111111111
 111111111111111
111  111  111
",
        ],
    )
    .critter()
    .traits(PREDATOR | SHY)
    .home("lily,pondweed")
    .tint("1:gGyg 4:kW")
    .color('g')
    .swim(Swim::Dart)
    .zone(Zone::Any)
    .speed(0.1, 0.6)
    .fps(0.3)
    .weight(6)
    .biomes(FRESH | TANK),
    Species::new(
        "river-otter",
        &[
            r#"
                   _.-.
         __.------'o  `.
  _..---'  . . . .   ,-'>=
.'  . .  .   .   .  /
\_ .  .  .  .  __.-'\ \
  `-.___..---''   )  ) )
     /  /    `-.-'`-'-'
    (_.'`-._ ~~~
        `--'  ~
"#,
            r#"
                   _.-.
         __.------'o  `.
  _..---'  . . . .   ,-'>=
.'  . .  .   .   .  /
\_ .  .  .  .  __.-'
  `-.___..---''  \ \
     /  /    `-.-'`-'
    (_.'`-.__  ~~~
        `--'     ~
"#,
        ],
        &[
            r#"
                   1k1k   
         11k11111114  1k  
  1kk1111  k k k k   1111W
k1  k k  k   k   k  1     
11 k  k  k  k  11k111 1   
  11k111kk11111   1  1 1  
     1  1    11k1111111   
    11k111k1 ccc          
        1111  c           
"#,
            r#"
                   1k1k   
         11k11111114  1k  
  1kk1111  k k k k   1111W
k1  k k  k   k   k  1     
11 k  k  k  k  11k11      
  11k111kk11111  1 1      
     1  1    11k11111     
    11k111k11  ccc        
        1111     c        
"#,
        ],
    )
    .critter()
    .traits(PREDATOR | CURIOUS)
    .tint("1:yyrw 4:kW")
    .color('y')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.4, 1.2)
    .fps(0.3)
    .weight(5)
    .biomes(FRESH),
    Species::new(
        "platypus",
        &[
            r#"
                    _.--.
         _.---.___.'  o `.__
  _.--.--'  . .  .    _____)
 .'   .  . . . . .    `----'
(  ~ ~ ~ ~ ~ ~ ~ ~ ~ _.-' \
 `-.__.-._.-'''-.__.-' `-'`-'
       `-.__.-'
"#,
            r#"
                    _.--.
         _.---.___.'  o `.__
  _.--.--'  . .  .    _____)
 .'   .  . . . . .    `----'
(  ~ ~ ~ ~ ~ ~ ~ ~ ~ _.-'
 `-.__.-._.-'''-.__.-' \ \
       `-.__.-'
"#,
        ],
        &[
            r#"
                    1k11k    
         1k111k111k1  4 1k11 
  1k11k111  k k  k    111111 
 k1   k  k k k k k    111111 
1  k k k k k k k k k 1k11 1  
 11k11k1k1k11111k11k11 111111
       11k11k11              
"#,
            r#"
                    1k11k    
         1k111k111k1  4 1k11 
  1k11k111  k k  k    111111 
 k1   k  k k k k k    111111 
1  k k k k k k k k k 1k11    
 11k11k1k1k11111k11k11 1 1   
       11k11k11              
"#,
        ],
    )
    .critter()
    .traits(PREDATOR | NOCTURNAL | SHY)
    .home("log")
    .tint("1:yykw 4:kW")
    .color('y')
    .swim(Swim::Wave)
    .zone(Zone::Lower)
    .speed(0.2, 0.6)
    .fps(0.3)
    .weight(4)
    .biomes(FRESH),
    Species::new(
        "diving-beetle",
        &[
            r"
  _.---._
 /       \=o>
(_________)
  /' |' \
",
            r"
  _.---._
 /       \=o>
(_________)
  \, |, /
",
        ],
        &[r"
  kkkkkkk
 k       kyWk
kkkkkkkkkkk
  kk kk k
"],
    )
    .critter()
    .traits(PREDATOR)
    .color('y')
    .swim(Swim::Dart)
    .zone(Zone::Any)
    .speed(0.2, 0.8)
    .fps(0.35)
    .weight(4)
    .biomes(FRESH | TANK),
    Species::new(
        "pygmy-seahorse",
        &[
            r"
 ,.
(o)>
 ))
 (,
",
            r"
 ,.
(o)>
 ))
 ,)
",
        ],
        &[r"
 11
1W11
 11
 11
"],
    )
    .critter()
    .traits(SHY)
    .home("sea-whip,coral")
    .tint("1:mMyYwr")
    .color('m')
    .swim(Swim::Hover)
    .zone(Zone::Lower)
    .speed(0.03, 0.12)
    .fps(0.2)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "sea-nettle",
        &[
            r"
   .-'''''-.
 .' | | | | '.
(__|_|_|_|_|__)
  ) ( ) ( ) (
 ( ) ( ) ( ) )
  ; ) ; ( ; )
   ( ;  ) ;
    ;   (
",
            r"
   .-'''''-.
 .' | | | | '.
(__|_|_|_|_|__)
  ( ) ( ) ( )
 ) ( ) ( ) ( (
  ) ; ) ; ( ;
   ; (  ; )
    (   ;
",
        ],
        &[r"
   111111111
 11 W W W W 11
111W1W1W1W1W111
  2 2 2 2 2 2
 2 2 2 2 2 2 2
  2 2 2 2 2 2
   2 2  2 2
    2   2
"],
    )
    .critter()
    .traits(STING)
    .tint("1:yYrRmM 2:yYrR")
    .color('y')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.04, 0.12)
    .fps(0.15)
    .weight(5)
    .biomes(KELP | OCEAN | TANK),
    Species::new(
        "crystal-jelly",
        &[
            r"
   _.---._
 .'  . .  '.
(___.---.___)
   '-._.-'
    ) ( )
    ( ) (
",
            r"
   _.---._
 .'  . .  '.
(___.---.___)
   '-._.-'
    ( ) (
    ) ( )
",
        ],
        &[r"
   1C111C1
 C1  C C  1C
1111C111C1111
   11C1C11
    1 1 1
    1 1 1
"],
    )
    .critter()
    .traits(GLOW)
    .tint("1:cwCW")
    .color('c')
    .swim(Swim::Drift)
    .zone(Zone::Any)
    .speed(0.04, 0.12)
    .fps(0.18)
    .weight(5)
    .biomes(OCEAN | KELP | ARCTIC),
    Species::new(
        "tadpole",
        &[
            r"
~-~(o)
",
            r"
-~-(o)
",
        ],
        &[r"
1111W1
"],
    )
    .critter()
    .traits(SHY)
    .home("lily,pondweed,hornwort")
    .tint("1:kKgy")
    .color('k')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.1, 0.4)
    .fps(0.5)
    .school(4, 10)
    .weight(5)
    .biomes(FRESH | TANK),
    Species::new(
        "newt",
        &[
            r"
     ,-.
 _.-'`o )>
(__~~~~'-'
  /\  /\
",
            r"
     ,-.
 _.-'`o )>
(__~~~~'-'
  \/  \/
",
        ],
        &[r"
     111
 11111W 11
111RRRR111
  11  11
"],
    )
    .critter()
    .traits(DEFENDED | SHY)
    .home("log,hornwort")
    .tint("1:gyGY")
    .color('g')
    .swim(Swim::Wave)
    .zone(Zone::Lower)
    .speed(0.1, 0.35)
    .fps(0.3)
    .weight(4)
    .biomes(FRESH | TANK),
    Species::new(
        "beaver",
        &[
            r#"
               _.-.
         _..--' o `.
     _.-'  .   .   `>
  .-'  . .   .   .  |
 /  .   .  .   .  _.'
(  .  .    . _..-'(
 `-.__   _.-'  ^^ `-'
######`-'
########\
 ######/
"#,
            r#"
               _.-.
         _..--' o `.
     _.-'  .   .   `>
  .-'  . .   .   .  |
 /  .   .  .   .  _.'
(  .  .    . _..-'(
 `-.__   _.-'  ^^ `-'
######`-'
 #######\
  #####/
"#,
        ],
        &[
            r#"
               1k1k  
         1kk111 4 1k 
     1k11  k   k   11
  k11  k k   k   k  1
 1  k   k  k   k  1k1
1  k  k    k 1kk111  
 11k11   1k11  WW 111
kkkkkk111            
kkkkkkkk1            
 kkkkkk1             
"#,
            r#"
               1k1k  
         1kk111 4 1k 
     1k11  k   k   11
  k11  k k   k   k  1
 1  k   k  k   k  1k1
1  k  k    k 1kk111  
 11k11   1k11  WW 111
kkkkkk111            
 kkkkkkk1            
  kkkkk1             
"#,
        ],
    )
    .critter()
    .traits(GRAZER | CURIOUS)
    .home("log")
    .tint("1:yyrw 4:W")
    .color('y')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.3, 0.8)
    .fps(0.3)
    .weight(4)
    .biomes(FRESH),
    Species::new(
        "adelie-penguin",
        &[
            r#"
         _.-"-.
     _.-'### o `.
  _.-'########  `>>
==-.#########.--.'
  `-..........-'
    `-...._.-'
      `-'-\ \
"#,
            r#"
         _.-"-.
     _.-'### o `.
  _.-'########  `>>
==-.#########.--.'
  `-..........-'
    `-...._.-'
      `-'-/ /
"#,
        ],
        &[
            r#"
         kkkkkk    
     kkkkkkk 4 kk  
  kkkkkkkkkkkk  kyy
yykkkkkkkkkkkkkkkk 
  kkkWWWWWWWWkkk   
    kkkWWWkkkk     
      kkkkk k      
"#,
            r#"
         kkkkkk    
     kkkkkkk 4 kk  
  kkkkkkkkkkkk  kyy
yykkkkkkkkkkkkkkkk 
  kkkWWWWWWWWkkk   
    kkkWWWkkkk     
      kkkkk k      
"#,
        ],
    )
    .critter()
    .traits(PREDATOR)
    .color('K')
    .swim(Swim::Dart)
    .zone(Zone::Any)
    .speed(0.6, 2.0)
    .fps(0.4)
    .school(3, 7)
    .weight(5)
    .biomes(ARCTIC),
    Species::new(
        "green-sea-turtle",
        &[
            r"
      _.-'''''-._
    .'\_/ \_/ \_/'.    _,--.
 ,-/ \_/ \_/ \_/ \_'-._/ o  )
(_,'-.____________.-' `-.__.'
   `-.\ \        `.\ \
      `-'          `-'
",
            r"
      _.-'''''-._
    .'\_/ \_/ \_/'.    _,--.
 ,-/ \_/ \_/ \_/ \_'-._/ o  )
(_,'-.____________.-' `-.__.'
    `-.\ \      `-.\ \
       `-'         `-'
",
            r"
      _.-'''''-._
    .'\_/ \_/ \_/'.    _,--.
 ,-/ \_/ \_/ \_/ \_'-._/ o  )
(_,'-.____________.-' `-.__.'
     `-.\ \      `-.\ \
        `-'         `-'
",
        ],
        &[
            r"
      11111111111
    11g1g g1g g1g11    11111
 11g g1g g1g g1g g11111g 4  2
211111111111111111111 1111111
   111g g        11g g
      111          111
",
            r"
      11111111111
    11g1g g1g g1g11    11111
 11g g1g g1g g1g g11111g 4  2
211111111111111111111 1111111
    111g g      111g g
       111         111
",
            r"
      11111111111
    11g1g g1g g1g11    11111
 11g g1g g1g g1g g11111g 4  2
211111111111111111111 1111111
     111g g      111g g
        111         111
",
        ],
    )
    .critter()
    .traits(GRAZER | DEFENDED | CURIOUS)
    .tint("1:gGyY 2:gGyY 4:W")
    .color('g')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.3, 0.8)
    .fps(0.2)
    .weight(5)
    .biomes(REEF | OCEAN | TANK),
    Species::new(
        "leatherback-turtle",
        &[
            r"
      _.-----._
  _.-'| | | | |'-._    ,-.
 ,'   | | | | |   '-.__.' o )
(_  __|_|_|_|_|__  _,--'-.__.'
  `-.__          __.-' \  \
       `--....--'     `--'
",
            r"
      _.-----._
  _.-'| | | | |'-._    ,-.
 ,'   | | | | |   '-.__.' o )
(_  __|_|_|_|_|__  _,--'-.__.'
  `-.__          __.-' ,' ,'
       `--....--'   '-'
",
        ],
        &[
            r"
      111111111
  11112 2 2 2 21111    111
 11   2 2 2 2 2   1111111 4 1
11  1121212121211  11111111111
  11111          11111 1  1
       1111111111     1111
",
            r"
      111111111
  11112 2 2 2 21111    111
 11   2 2 2 2 2   1111111 4 1
11  1121212121211  11111111111
  11111          11111 11 11
       1111111111   111
",
        ],
    )
    .critter()
    .traits(DEFENDED)
    .tint("1:kwbk 2:wWcb 4:W")
    .color('w')
    .swim(Swim::Wave)
    .zone(Zone::Any)
    .speed(0.3, 0.8)
    .fps(0.15)
    .weight(2)
    .biomes(OCEAN),
];
