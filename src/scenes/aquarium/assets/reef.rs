//! Coral reef fish.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "ocellaris-clownfish",
        &[r"
   ,--.
\ /|)(|`.
 >=|)(|o>
/ \|)(|.'
   `--'
"],
        &[r"
   1111
6 6177111
 66177145
6 6177111
   1111
"],
    )
    .tint("1:yR 7:wW 6:yR")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .weight(8)
    .biomes(REEF | TANK),
    Species::new(
        "maroon-clownfish",
        &[
            r"
    _.--,
\__/ |)(\`.
 >==-|)(  o>
/--\ |)(/.'
    `--'
",
            r"
    _.--,
\_/  |)(\`.
 >--=|)(  o>
/-/  |)(/.'
    `--'
",
        ],
        &[r"
    11111
6666 771111
 6666771  45
6666 771111
    1111
"],
    )
    .tint("1:rR 7:wW 6:rR")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.25, 0.8)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "tomato-clownfish",
        &[r"
   .--.
 \/ )|`.
>=<  |  o>
 /\ )|.'
   `--'
"],
        &[r"
   1111
 66 7711
666  7  45
 66 7711
   1111
"],
    )
    .tint("1:rR 7:wW 6:rR")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "blue-tang",
        &[
            r"
     ,--.
\   /  ,'`.
 >=<  ((  ) o>
/   \  `-._.'
",
            r"
     ,--.
.   /  ,'`.
 >=<  ((  ) o>
'   \  `-._.'
",
        ],
        &[r"
     2222
6   2  2222
 666  71  1 45
6   1  111111
"],
    )
    .tint("1:bB 2:bB 6:yY 7:K")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.4, 1.2)
    .weight(8)
    .biomes(REEF | TANK),
    Species::new(
        "yellow-tang",
        &[r"
     ,-.
\   /   `.
 >=<    / o>
/   \ `-'`
"],
        &[r"
     222
6   2   22
 666    1 45
6   1 1111
"],
    )
    .tint("1:yY 2:yY 6:yY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.4, 1.3)
    .weight(8)
    .biomes(REEF | TANK),
    Species::new(
        "powder-blue-tang",
        &[r"
     _.--._
\   /      `.
 >=<  | |    o>
/   \  `----'
"],
        &[r"
     222222
6   2      22
 666  7 7    45
6   1  111111
"],
    )
    .tint("1:bB 2:yY 6:wW 7:wW")
    .zone(Zone::Upper)
    .swim(Swim::Wave)
    .speed(0.4, 1.1)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "naso-tang",
        &[r"
         ,
    _.--/'._
\  /       `-.
 >=<   ~~     o>
/  \  `-.__.-'`
"],
        &[r"
         2
    22222222
6  1       111
 666   11     45
6  1  1111111
"],
    )
    .tint("1:cC 2:yY 6:yY")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.4, 1.0)
    .weight(4)
    .biomes(REEF),
    Species::new(
        "achilles-tang",
        &[r"
    ,---.
\  /     `.
 >=<  (  ) o>
/  \ `---'
"],
        &[r"
    22222
6  1     11
 666  8  1 45
6  1 11111
"],
    )
    .tint("1:K 2:rR 6:rR 8:rR")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.4, 1.1)
    .weight(3)
    .biomes(REEF),
    Species::new(
        "convict-tang",
        &[r"
   ,---.
\ /|||||`.
 >|||||| o>
/ \||||'`
"],
        &[r"
   22222
6 6777771
 6777777 45
6 6777711
"],
    )
    .tint("1:wW 7:K 2:K 6:wW")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.4, 1.1)
    .weight(4)
    .biomes(REEF),
    Species::new(
        "moorish-idol",
        &[
            r"
      ,
 `-._/|
     / |__
\  ,'  |  `-.
 >=  ) | |   `o>
/  `.  | |  .-'
     `-|_|-'
",
            r"
      ,
 '-._/|
     / |__
\  ,'  |  `-.
 >-  ) | |   `o>
/  `.  | |  .-'
     `-|_|-'
",
        ],
        &[r"
      2
 222222
     2 222
6  11  K  111
 66  1 K 8   145
6  11  K 8  111
     11K1811
"],
    )
    .tint("1:wW 2:wW 6:K 8:yY")
    .fps(0.25)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.8)
    .weight(4)
    .biomes(REEF),
    Species::new(
        "copperband-butterflyfish",
        &[r"
     ,--.
\  ,'|| |`.
 >=  || | o`-
/  `.|| |.'
     `--'
"],
        &[r"
     2222
6  11771711
 66  77171455
6  11771711
     1111
"],
    )
    .tint("1:wW 2:yY 5:yR 6:yY 7:yR")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "raccoon-butterflyfish",
        &[r"
   ,-.__
\ /  ||`.
 >=  ||o>
/ \  ||.'
   `-'`
"],
        &[r"
   22211
6 6  KK11
 66  KK45
6 6  KK11
   1111
"],
    )
    .tint("1:yY 2:wW 6:K")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "longnose-butterflyfish",
        &[r"
    ,--.
\  /    `-.__
 >=<  ()     o=-
/  \    _.-'
    `--'
"],
        &[r"
    2222
6  1    11111
 666  77     455
6  1    1111
    1111
"],
    )
    .tint("1:yY 2:yY 6:yY 7:K")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(REEF),
    Species::new(
        "threadfin-butterflyfish",
        &[r"
   _
 ,'`-._,.
\ /  |  |`.
 >=  |  |o>
/ \  |  |.'
    `-.-'
"],
        &[r"
   2
 22222222
6 6  K  K11
 66  K  K45
6 6  K  K11
    11111
"],
    )
    .tint("1:yY 2:wW 6:yY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "emperor-angelfish",
        &[r"
        ,--.
      ,'====`.
\    /'======`.
 >=<  ====== o>
/    \'======.'
      `.====.-'
        `--'
"],
        &[r"
        2222
      11777711
6    1177777711
 666  777777 45
6    177777711
      117777111
        1111
"],
    )
    .tint("1:bB 2:bB 6:yY 7:yY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "queen-angelfish",
        &[r"
      ,-.__
 ~-._/ (o)`.
\    /::::::`.
 >=<  :::::: o>
/    \::::::.'
      `-.__.-'
"],
        &[r"
      22222
 22222 cCc11
6    177777711
 666  777777 45
6    177777711
      11111111
"],
    )
    .tint("1:bB 2:bB 6:yY 7:yY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "french-angelfish",
        &[r"
   ,---.
\ /.:.:.`.
 >=:.:.:. o>
/ \.:.:.'.'
   `---'
"],
        &[r"
   22222
6 6777771
 66777771 45
6 6777771
   22222
"],
    )
    .tint("1:K 2:yY 6:K 7:yY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.8)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "flame-angelfish",
        &[r"
   ,---.
\ /|  | `.
 >=|  |  o>
/ \|  | .'
   `---'
"],
        &[r"
   22222
6 6K  K 11
 66K  K  45
6 6K  K 11
   22222
"],
    )
    .tint("1:rR 2:bB 6:yR")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "regal-angelfish",
        &[r"
    ,--.
\  /'|:|`.
 >=< |:|  o>
/  \.|:|.'
    `--'
"],
        &[r"
    2222
6  1178711
 666 878  45
6  1178711
    1111
"],
    )
    .tint("1:yY 2:bB 6:yY 7:bB 8:wW")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(3)
    .biomes(REEF),
    Species::new(
        "potters-angelfish",
        &[r"
   .-.
\_/:::\
 >=:::o>
/-\::/'
"],
        &[r"
   222
6661112
 6611145
6661122
"],
    )
    .tint("1:yR 2:bB 6:bB")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.4, 1.2)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "red-lionfish",
        &[r"
       \ \ | / /
      \ \ \|/ / /
   _.-'\_\_|_/_/'-._
\ /   ===||===||===  `-.
 >=<   ~~~~~~~~~~~~    o>
/ \   ===||===||===  .-'
   `-._ / /|\ \ _.-'
       / / | \ \
"],
        &[r"
       222222222
      22222222222
   33333333333333333
616111111771117711111111
 666111111111111111111145
616111111771117711111111
   33333333333333333
       333333333
"],
    )
    .tint("1:rR 2:wW 3:rR 6:rR 7:wW")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.2, 0.6)
    .weight(3)
    .biomes(REEF | TANK),
    Species::new(
        "clown-triggerfish",
        &[r"
     ,---.
\   / : : `.
 >=<  :  :  o>
/   \ : : .'
     `---'
"],
        &[r"
     22222
611111717111
 6661171171145
611111717111
     11111
"],
    )
    .tint("1:K 2:yR 5:yY 6:yY 7:wW")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "picasso-triggerfish",
        &[r"
    ,-~-.
\  /=-~=-`.
 >=<  ~-=~  o>
/  \=~-=~-.'
    `-~-'
"],
        &[r"
    22222
61117897811
 6661198791145
611179879811
    22222
"],
    )
    .tint("1:yY 6:bB 7:gGc 8:yYr 9:bBm")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "stoplight-parrotfish",
        &[r"
    ,-----.
\  /.:.:.:.\
 >=<:.:.:.: o)]
/  \.:.:.:.'
    `-----'
"],
        &[r"
    1111111
611111111111
 66681111111455
611111111111
    1111111
"],
    )
    .tint("1:gGc 6:gGc 8:yY 5:wW")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(6)
    .biomes(REEF | TANK),
    Species::new(
        "bumphead-parrotfish",
        &[r"
              .---.
          _.-'     `.
\       ,'  : : :    `.
 \  _.-'  : : : : :    |
  >=<  : : : : : : :  o|)
 /  `-.__: : : : : :     |
/        `-.___: : _.--'
              `-.-'
"],
        &[r"
              11111
          11111111111
61111111111171717111111
 61111111171717171711115
  66611717171717171711455
 6111111171717171717111115
611111111111111717111111
              11111
"],
    )
    .tint("1:bBg 6:gG 7:cC 5:wW")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.6)
    .weight(2)
    .biomes(REEF),
    Species::new(
        "bluestreak-cleaner-wrasse",
        &[r"
   ____
><~~~~~~o>
   `--'
"],
        &[r"
   1111
66BBBBBB45
   1111
"],
    )
    .tint("1:wW 6:wW 5:bB")
    .zone(Zone::Lower)
    .swim(Swim::Dart)
    .speed(0.3, 1.0)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "bird-wrasse",
        &[r"
   ,------.
\ /  ~ ~ ~ `-._
 >=   ~ ~ ~    o=->
/ \  ~ ~ ~ _.-'
   `------'
"],
        &[r"
   11111111
616117171711111
 661117171711114115
616117171711111
   11111111
"],
    )
    .tint("1:gGc 6:yG 7:bB 5:wW")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.4, 1.1)
    .weight(4)
    .biomes(REEF),
    Species::new(
        "humphead-wrasse",
        &[r"
               .--.
           _.-'    `-.
\        ,'  ~  ~  ~  `.
 \  _.-'  ~  ~  ~  ~    )
  >=<  ~  ~  ~  ~  ~  o D=
 /  `-.__ ~  ~  ~  ~    )
/        `-.__ ~  ~ _.-'
               `-.-'
"],
        &[r"
               1111
           11111111111
611111111111171171171111
 611111111711711711711115
  666117117117117117114155
 611111111711711711711115
611111111111111711711111
               11111
"],
    )
    .tint("1:bBc 6:bB 7:gG 5:mM")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.2, 0.6)
    .weight(2)
    .biomes(REEF),
    Species::new(
        "six-line-wrasse",
        &[r"
   ,-----.
><=-=-=-=o>
   `-----'
"],
        &[r"
   1111111
66717171745
   1111111
"],
    )
    .tint("1:bBc 7:yR 6:bB")
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.4, 1.3)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "mandarin-dragonet",
        &[r"
   ,~-~-.
\_/~@~@~@~o>
   `~-~-'
"],
        &[r"
   179791
666787878745
   179791
"],
    )
    .tint("1:bB 7:yR 8:gG 9:rR 6:bB")
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.15, 0.5)
    .weight(3)
    .biomes(REEF | TANK),
    Species::new(
        "royal-gramma",
        &[
            r"
    ,---.
\  /  :  `.
 >=<   :  o>
/  \  :  .'
    `---'
",
            r"
    ,---.
 \_/  :  `.
 >=<   :  o>
 /_\  :  .'
    `---'
",
        ],
        &[
            r"
    11777
61111777777
 66611177745
61111777777
    11777
",
            r"
    11777
 6611777777
 66611177745
 6611777777
    11777
",
        ],
    )
    .tint("1:yY 7:mM 6:yY")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .biomes(REEF | TANK)
    .fps(0.25),
    Species::new(
        "yellow-boxfish",
        &[r"
  .---------.
 /  o  o  o |
={  o  o  o O-
 \  o  o  o |
  `---------'
"],
        &[r"
  11111111111
 111711711711
66117117117141
 111711711711
  11111111111
"],
    )
    .tint("1:yY 7:bBK 6:yY")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.2, 0.7)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "longhorn-cowfish",
        &[r"
  .---------.,/
 /  o  o  o |/'
={  o  o  o O-'
 \  o  o  o |
  `---------'
"],
        &[r"
  1111111111111
 11171171171111
661171171171411
 111711711711
  11111111111
"],
    )
    .tint("1:wWb 7:bB 6:wW")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.2, 0.7)
    .weight(3)
    .biomes(REEF | TANK),
    Species::new(
        "porcupinefish",
        &[r"
     \ | / | /
   \_.-'`'-._/
 \ /  .  .  . \
>=<   .  .  O  )
 / \  .  .  . /
   /'-.___.-'\
     / | \ | \
"],
        &[r"
     222222222
   22222222222
 61611711711711
6661117117114115
 61611711711711
   22222222222
     222222222
"],
    )
    .tint("1:yYw 2:yY 6:yY 7:wW")
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.15, 0.5)
    .weight(4)
    .biomes(REEF | TANK | OCEAN),
    Species::new(
        "dogface-pufferfish",
        &[r"
    ,----.
\  / . .  `-.
 >=<  . .  )o_>
/  \ . .  .-'
    `----'
"],
        &[r"
    111117
6111171711117
 6661171711K4KK
6111171711711
    111111
"],
    )
    .tint("1:yY 7:K 6:yY")
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.2, 0.6)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "firefish-goby",
        &[r"
      /
   ,-'--.
><=~~~~~o>
   `----'
"],
        &[r"
      1
   777111
6677711145
   777111
"],
    )
    .tint("1:wW 7:rR 6:rR 5:wW")
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.2, 0.7)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "yellow-watchman-goby",
        &[r"
   _.--._
><<=====o)
   `----'
"],
        &[r"
   111111
6611111145
   111111
"],
    )
    .tint("1:yY 7:bB")
    .zone(Zone::Lower)
    .swim(Swim::Dart)
    .speed(0.2, 0.8)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "banggai-cardinalfish",
        &[r"
   ,|`-.
\ /|:|:|`.
 >=|:|:|o)
/ \|:|:|.'
   `-.-'
"],
        &[r"
   22222
6168787811
 668787845
6168787811
   11111
"],
    )
    .tint("1:wW 8:K 7:wW 6:wW 2:wW")
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.2, 0.7)
    .school(5, 10)
    .weight(6)
    .biomes(REEF | TANK),
    Species::new(
        "foxface-rabbitfish",
        &[r"
    _.--._
\  /      `-.__
 >=<  ~~~~~~o=>
/  \      _.-'
    `----'
"],
        &[r"
    111111
611111111111111
 666111111114WW
61111111111111
    111111
"],
    )
    .tint("1:yY 6:yY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "harlequin-sweetlips",
        &[r"
    ,----._
\  /' * * * `-.
 >=<  * * *  o))
/  \' * * * _.-'
    `-.__.-'
"],
        &[r"
    2222222
611111717171111
 666117171711455
6111117171711111
    11111111
"],
    )
    .tint("1:wW 7:K 5:yY 6:yY 3:yY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.2, 0.6)
    .weight(3)
    .biomes(REEF),
    Species::new(
        "flame-hawkfish",
        &[r"
   ,--._
\ /+++++`-.
 >=+++++o>
/ \+++++.-'
   `----'
"],
        &[r"
   22222
61677777111
 667777745
61677777111
   111111
"],
    )
    .tint("1:rR 7:yY 6:rR 2:rR")
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.2, 0.7)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "squirrelfish",
        &[r"
     ,-/|/|-.
\   /--=-=-=-\
 >=<   =-=-=-=(O)
/   \--=-=-=-/
     `-------'
"],
        &[r"
     22222222
61111117171711
 6661117171717W4W
61111117171711
     111111111
"],
    )
    .tint("1:rR 6:rR 2:rR 7:wW")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.2, 0.7)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "lyretail-anthias",
        &[r"
\\  ,-.
 >=<(  o>
//  `-'
"],
        &[r"
6611222
 66611145
6611111
"],
    )
    .tint("1:rRm 2:mM 6:yR")
    .zone(Zone::Upper)
    .swim(Swim::Wave)
    .speed(0.4, 1.2)
    .school(5, 12)
    .weight(7)
    .biomes(REEF | TANK),
    Species::new(
        "coral-grouper",
        &[r"
        _.-----._
      ,'  .  .  .`.__
\    /  .  .  .  .   `-.
 >==<  .  .  .  .  .  o  )
/    \  .  .  .  .  .  _.-'
      `.  .  .  .  .__.-'
        `-._______.-'
"],
        &[r"
        171111171
      111171171171711
611111117117117117111117
 6666117117117117117114115
611111117117117117117111711
      1711711711711711711
        1171111111711
"],
    )
    .tint("1:rR 7:wW 6:rR 5:rR")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.2, 0.5)
    .weight(3)
    .biomes(REEF | TANK),
    Species::new(
        "bicolor-blenny",
        &[r"
   ,-----.'
><=======o>
   `-----'
"],
        &[r"
   77111111
66777711145
   7711111
"],
    )
    .tint("1:K 7:yR 6:yR")
    .zone(Zone::Lower)
    .swim(Swim::Dart)
    .speed(0.2, 0.8)
    .weight(5)
    .biomes(REEF | TANK),
    Species::new(
        "orchid-dottyback",
        &[r"
   ,---._
\_/=====-o>
/_\`-----'
"],
        &[r"
   111111
66677777145
6661111111
"],
    )
    .tint("1:mM 7:yY 6:mM")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(REEF | TANK),
    Species::new(
        "blue-devil-damselfish",
        &[r"
   ,--.
\ /    `.
 >=<   o>
/ \    .'
   `--'
"],
        &[r"
   2222
616111111
 66611145
111111111
   1111
"],
    )
    .tint("1:bB 6:bB 2:bB")
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.3, 1.1)
    .weight(7)
    .biomes(REEF | TANK),
    Species::new(
        "domino-damsel",
        &[r"
   ,---.
\ /  *  `.
 >=<   * o>
/ \  *  .'
   `---'
"],
        &[r"
   11111
61611W1111
 666111W145
61611W1111
   11111
"],
    )
    .tint("1:K 6:K")
    .zone(Zone::Lower)
    .swim(Swim::Dart)
    .speed(0.3, 1.1)
    .weight(6)
    .biomes(REEF | TANK),
    Species::new(
        "sergeant-major",
        &[r"
    ,---.
\  /|.|.|`.
 >=<|.|.| o>
/  \|.|.|.'
    `---'
"],
        &[r"
    22222
6111K1K1K11
 666K1K1K145
6111K1K1K11
    11111
"],
    )
    .tint("1:yY 6:yY 2:yY")
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.3, 1.1)
    .school(1, 3)
    .weight(7)
    .biomes(REEF | TANK),
    Species::new(
        "green-chromis",
        &[
            r"
  ,-.
><==:o>
  `-'
",
            r"
  ,-.
)(==:o>
  `-'
",
        ],
        &[r"
  111
6611745
  111
"],
    )
    .tint("1:gG 7:cC 6:gG")
    .zone(Zone::Upper)
    .swim(Swim::Wave)
    .speed(0.4, 1.3)
    .school(6, 12)
    .weight(8)
    .biomes(REEF | TANK)
    .fps(0.4),
];
