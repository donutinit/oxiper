use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "common-goldfish",
        &[r"
   .--.
\ /    '.
 X  ( )  o)
/ \ '-._.'
"],
        &[r"
   1111
111111111
 1111111141
1111111111
"],
    )
    .tint("1:yYrR")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .weight(8)
    .biomes(TANK | FRESH),
    Species::new(
        "comet-goldfish",
        &[r"
 \\\\  _.-.
=====<( )  o)
 ////  '-._.'
"],
        &[r"
 1111111111
1111111111141
 111111111111
"],
    )
    .tint("1:yYrR")
    .zone(Zone::Upper)
    .swim(Swim::Wave)
    .speed(0.6, 1.5)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "fantail-goldfish",
        &[
            r"
\\   .--.
=\\/    '.
=//\ ( ) o)
//  '-._.'
",
            r"
//   .--.
=//\    '.
=\\/ ( ) o)
\\  '-._.'
",
        ],
        &[r"
111111111
1111111111
11111111141
1111111111
"],
    )
    .tint("1:yYrRwW")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.8)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "oranda-goldfish",
        &[r"
        .%%%.
   .--.'%%%%%\
\ /    '-.  o)
 X  ( )  '-.'
/ \ '-.__.-'
"],
        &[r"
        12221
   11111222221
11111111111141
 111111111111
111111111111
"],
    )
    .tint("1:wWyY 2:rR")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "ranchu",
        &[r"
   .-%%%%.
 .'%%%%%%'\_
<  ( )  o )
 '.__.__.'
"],
        &[r"
   1122221
 11222222111
11111111411
 111111111
"],
    )
    .tint("1:yYrRwW 2:rR")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(3)
    .biomes(TANK | FRESH),
    Species::new(
        "bubble-eye-goldfish",
        &[r"
  .--._
 /     '.
<  ( )  o)
 '.____(O)
"],
        &[r"
  11111
 11111111
1111111141
 111111222
"],
    )
    .tint("1:yYrRwW 2:wWcC")
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.6)
    .weight(3)
    .biomes(TANK | FRESH),
    Species::new(
        "black-moor",
        &[
            r"
\\    .--.
=\\/     '.
=//\ ( )  (o)
//  '-.__.'
",
            r"
//    .--.
=//\     '.
=\\/ ( )  (o)
\\  '-.__.'
",
        ],
        &[r"
1111111111
11111111111
1111111111141
11111111111
"],
    )
    .tint("1:K")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "kohaku-koi",
        &[r"
\\\    _.--~~--.__
 \\\ .'          '-.
==<<(   ~~   ~~    o)~
 /// '.     ~~  _.-'
///     '-.___.-'
"],
        &[r"
111111111112211111
 1111111111111111111
1111111122111221111412
 1111111111122111111
11111111111111111
"],
    )
    .tint("1:wW 2:rR")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "showa-koi",
        &[r"
\\\    _.--%%--.__
 \\\ .'  ~~      '-.
==<<(  %%  ~~~~   o)~
 /// '.      %%  _.-'
///     '-.___.-'
"],
        &[r"
11111111111KK11111
 1111111122111111111
1111111KK112222111412
 111111111111KK111111
11111111111111111
"],
    )
    .tint("1:wW 2:rR")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "veiltail-betta",
        &[
            r"
        _.--.
 \\\\\.'     '.
))))))   ( o )>
 /////'. ___.'
   \\\\\\
",
            r"
        _.--.
 /////.'     '.
((((((   ( o )>
 \\\\\'. ___.'
   //////
",
        ],
        &[r"
        22222
 22222111111111
222222111114111
 2222211111111
   222222
"],
    )
    .tint("1:rRbBmM 2:rRbBmM")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "halfmoon-betta",
        &[
            r"
   \ | /    .-.
 \  \|/  .-'   '.
((( ))) (  o   )>
 /  /|\  '-.__.'
   / | \
",
            r"
   | | |    .-.
 \  \|/  .-'   '.
((( ))) (  o   )>
 /  /|\  '-.__.'
   | | |
",
        ],
        &[r"
   222211111111
 2222211111111111
22222221111411111
 222221111111111
   22221
"],
    )
    .tint("1:rRbBmM 2:rRbBmM")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "guppy",
        &[r"
\\\\  _.-.
))))=( ~ o)
////  '-'
"],
        &[r"
6666111111
66666611141
666611111
"],
    )
    .tint("1:yYcCmM 6:rRbBgG")
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.4, 1.2)
    .school(3, 8)
    .weight(8)
    .biomes(TANK | FRESH),
    Species::new(
        "endlers-livebearer",
        &[r"
><=~o)
"],
        &[r"
66CG4
"],
    )
    .tint("6:rRyY")
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.4, 1.3)
    .school(3, 8)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "molly",
        &[r"
    .~~~~~.
\\ /  ___  '.
>>(  (___)  o)
// '-.___.-'
"],
        &[r"
    2222211
1111111111111
11111111111141
111111111111
"],
    )
    .tint("1:KyYwW 2:KyYwW")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "platy",
        &[r"
\\ .--.
>>( . o)
//'--'
"],
        &[r"
1111111
11111141
111111
"],
    )
    .tint("1:rRyYbB")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.35, 1.0)
    .school(2, 6)
    .weight(7)
    .biomes(TANK | FRESH),
    Species::new(
        "swordtail",
        &[r"
  \\ .--.
  >>(  .o)
===//'--'
"],
        &[r"
  6661111
  66611141
666666611
"],
    )
    .tint("1:rRyYgG 6:rRKyY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.4, 1.2)
    .school(1, 4)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "neon-tetra",
        &[r"
><==o)
"],
        &[r"
rr1R4
"],
    )
    .tint("1:cCbB")
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.5, 1.4)
    .school(5, 12)
    .weight(8)
    .biomes(TANK | FRESH),
    Species::new(
        "cardinal-tetra",
        &[r"
><===o)
"],
        &[r"
rrCRR4
"],
    )
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.5, 1.4)
    .school(5, 12)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "black-skirt-tetra",
        &[r"
 _.--.
>< ||o)
>'.||.'
"],
        &[r"
 11111
1KKK141
11KK111
"],
    )
    .tint("1:wWyY")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.4, 1.1)
    .school(4, 9)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "rummy-nose-tetra",
        &[r"
>|=(==o)
"],
        &[r"
wK11114R
"],
    )
    .tint("1:wy")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.5, 1.2)
    .school(6, 12)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "zebra-danio",
        &[r"
><=-=-=-o>
"],
        &[r"
66BwBwBw4
"],
    )
    .tint("6:wW")
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.8, 2.0)
    .school(5, 12)
    .weight(8)
    .biomes(TANK | FRESH),
    Species::new(
        "harlequin-rasbora",
        &[r"
\\ .--.
>>( ###o)
// '--'
"],
        &[r"
6661111
66611KK41
1111111
"],
    )
    .tint("1:yYrR 6:rR")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.4, 1.0)
    .school(5, 10)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "cherry-barb",
        &[r"
\\ .--.
>>( __o)
// '--'
"],
        &[r"
1111111
11111141
1111111
"],
    )
    .tint("1:rR")
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.4, 1.1)
    .school(3, 8)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "tiger-barb",
        &[r"
 _.|-.
>(||| o)
 '.|.'
"],
        &[r"
 11K11
11KKK141
 11K11
"],
    )
    .tint("1:yY")
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.5, 1.6)
    .school(4, 9)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "white-cloud-minnow",
        &[r"
><==-o>
"],
        &[r"
66wwW4
"],
    )
    .tint("6:rR")
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.4, 1.1)
    .school(5, 12)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "freshwater-angelfish",
        &[r"
        /|
       / |
      /  |
\\\\  /   '.
=<<=(  |  o)>
////  \  .'
       \ |
        \|
         '
"],
        &[r"
        1K
       11K
      111K
111111111111
1111111K11411
11111111111
       11K
        1K
         1
"],
    )
    .tint("1:wWyY")
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .school(1, 3)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "discus",
        &[r"
    _.---._
  .'  ~ ~  '.
\/  ~  ~  ~  \
>=  ~ ~  ~ ~  o)
/\  ~  ~  ~  /
  '.  ~ ~  .'
    '-._.-'
"],
        &[r"
    1111111
  11112121111
11112112112111
1111212112121141
11112112112111
  11112121111
    1111111
"],
    )
    .tint("1:rRyYcC 2:bBmMgG")
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.6)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "oscar",
        &[r"
    _.------._
\\\\'  ~  ~   '-.
 )))  ~  ~ ~ ~  o )
 ////.  ~ ~   _.-'
     '-.____.-'
"],
        &[r"
    1111111111
11111112112111111
 111112112121211411
 11111112121111111
     1111111111
"],
    )
    .tint("1:Kw 2:rRyY")
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.25, 0.9)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "convict-cichlid",
        &[r"
\\ .---.
>>(|||| o)
// '---'
"],
        &[r"
11111111
111KKKK141
11111111
"],
    )
    .tint("1:wWcC")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "electric-yellow-cichlid",
        &[r"
 .====.
\\(     o)
//'-----'
"],
        &[r"
 YKKKKY
YYYYYYYY4Y
YYYYYYYYY
"],
    )
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .school(1, 3)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "german-blue-ram",
        &[r"
   /|/|
 .'    '.
>=(    o )
  '.__.'
"],
        &[r"
   2222
 11111111
1111111411
  111111
"],
    )
    .tint("1:yY 2:bB")
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.25, 0.6)
    .weight(3)
    .biomes(TANK | FRESH),
    Species::new(
        "electric-blue-acara",
        &[r"
   ____
\\/ ~ ~ '.
>=(  ~ o )
//'-.__.'
"],
        &[r"
   BBBB
BBBBcBcBBB
BBBBBcB4BB
BBBBBBBBB
"],
    )
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "flowerhorn",
        &[r"
         .%%%.
  _.---.'%%%%%\
\\'  ~  ~     o)
>=(  ~  ~ ~   >
//'.__    _.-'
     '--'
"],
        &[r"
         22222
  1111111222222
11111K11K1111141
11111K11K1K1111
11111111111111
     1111
"],
    )
    .tint("1:rRmM 2:rRyY")
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(3)
    .biomes(TANK | FRESH),
    Species::new(
        "tilapia",
        &[r"
      .~~~~~~.
\\  .-'      '-.
>>=(          o )
//  '-.____.-'
"],
        &[r"
      22222211
1111111111111111
11111111111111411
11111111111111
"],
    )
    .tint("1:wWgG 2:gG")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.3, 1.0)
    .weight(5)
    .biomes(FRESH),
    Species::new(
        "dwarf-gourami",
        &[r"
 _.--.
\\(~~~o)
//'--'
   \|
"],
        &[r"
 11111
11122241
111111
   11
"],
    )
    .tint("1:bB 2:rR")
    .zone(Zone::Upper)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "pearl-gourami",
        &[r"
  _.---.
\\( .:.: o)
//'-----'
  \\\
"],
        &[r"
  1W111W
1111WWWW141
111111111
  111
"],
    )
    .tint("1:yYrR")
    .zone(Zone::Upper)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "kuhli-loach",
        &[
            r"
~=-=-=-=-=-=-=o)
",
            r"
~-=-=-=-=-=-=-o)
",
        ],
        &[r"
yKyKyKyKyKyKyK4y
"],
    )
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.25, 0.8)
    .school(2, 5)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "clown-loach",
        &[r"
 _.--.__
\\( ||| |o)
//'-----'
"],
        &[r"
 1111111
1111KKK1K41
111111111
"],
    )
    .tint("1:yY")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .school(2, 4)
    .weight(5)
    .biomes(TANK | FRESH),
    Species::new(
        "silver-arowana",
        &[r"
        __.------~~~--.__
\\\\ _.-'' ) ) ) )  ''-._
=<<=(  ) ) ) ) ) )  o  _.-'/
//// '-._  ) ) )  _..-' /_.'
         '--.______.-'  ~~
"],
        &[r"
        111111111www11111
11111111111W1W1W1W1111111
1111111W1W1W1W1W1W1141111111
11111111111W1W1W111111111111
         111111111111111ww
"],
    )
    .tint("1:wW")
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.4, 1.2)
    .weight(3)
    .biomes(TANK | FRESH),
    Species::new(
        "red-bellied-piranha",
        &[r"
      _.-~-._
\\\ .-'  ::  '-.
=<<(     ::   o )
/// '-.____.-'VvVv
"],
        &[r"
      1111111
111111111KK11111
111111111KK111411
RRRRRRRRRRRRRWWWW1
"],
    )
    .tint("1:wWK")
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.4, 1.6)
    .school(1, 5)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "pacu",
        &[r"
       _.--~~--._
\\\\ .-'  ) ) )  '-.
=<<=(  ) ) ) ) )   o )
//// '-.  ) ) ) _.-'
       '-.____.-'
"],
        &[r"
       1111111111
11111111112121211111
1111111212121212111412
11111111112121211111
       1111111111
"],
    )
    .tint("1:wWyY 2:yY")
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.25, 0.9)
    .weight(3)
    .biomes(TANK | FRESH),
    Species::new(
        "rainbow-trout",
        &[r"
       _.-~-._
\\  _.'. : . : '-.
>>=(===========  o)
//  '-._ :  : _.-'
"],
        &[r"
       1111111
111111111K111K1111
11R1RRRRRRRRRRR1141
111111111K11K11111
"],
    )
    .tint("1:gGyY")
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.7, 1.8)
    .weight(6)
    .biomes(FRESH),
    Species::new(
        "brown-trout",
        &[r"
       _.--~-._
\\  .'o . o . '.__
>>=(  o  o . o  o )
//  '-.___ __.-'
"],
        &[r"
       11111111
111111r111r1111111
111111r11r111r11411
1111111111111111
"],
    )
    .tint("1:yY")
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.5, 1.5)
    .weight(5)
    .biomes(FRESH),
    Species::new(
        "atlantic-salmon",
        &[r"
        _.--~~~-.__
\\\  _.-' x  x  x '-._
=<<=(  x  x  x  x    o )
///  '-._  x  x _.-'
       '-.______.-'
"],
        &[r"
        11111111111
1111111111K11K11K11111
1111111K11K11K11K1111411
11111111111K11K11111
       111111111111
"],
    )
    .tint("1:wW")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.8, 1.8)
    .weight(4)
    .biomes(FRESH | OCEAN | KELP),
    Species::new(
        "sockeye-salmon",
        &[r"
        _.--~~~-.__
\\\  _.-'         '-._
=<<=(              o \
///  '-._    _  _.-'<
       '-.__\/_.-'
"],
        &[r"
        11111111111
111111111111111111GGGG
1111111111111111GGG4GG
1111111111111111GGGGG
       11111111111
"],
    )
    .tint("1:rR")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.8, 1.8)
    .weight(3)
    .biomes(FRESH | OCEAN | KELP),
    Species::new(
        "common-carp",
        &[r"
       _.------~~~~--._
\\\\ .-' (_)(_)(_)(_)(_)'-._
=<<<(  (_)(_)(_)(_)(_)   o  )~
////'-. (_)(_)(_)(_)  _.-'~
       '-._________.-'
"],
        &[r"
       y11111111111111y
111111111yyyyyyyyyyyyyyy111y
1111y11yyyyyyyyyyyyyyy111411y1
11111111yyyyyyyyyyyy11y1111
       111yyyyyyyyy111
"],
    )
    .tint("1:yYwW")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .biomes(FRESH),
    Species::new(
        "channel-catfish",
        &[r"
        _.--.
\\\\ .-'  : : '-._
=<<<(  : :  :   o )--~
////'-._______.-'=~~
"],
        &[r"
        11ww1
111111w111K1K11w11
w111111K1K11K111411www
11111w111111111w1www
"],
    )
    .tint("1:wWK")
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.3, 0.9)
    .weight(5)
    .biomes(FRESH),
    Species::new(
        "northern-pike",
        &[r"
                   __
\\\    .-~~--.._ _..-'''-.__
=<<==(  : : : : : : :  o \==-'
////  '-.__ : : : __..--'--'
"],
        &[r"
                   11
1111111111111111111111111111
11111111y1y1y1y1y1y1y114111111
111111111111y1y1y11111111111
"],
    )
    .tint("1:gG")
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.3, 2.0)
    .weight(3)
    .biomes(FRESH),
    Species::new(
        "largemouth-bass",
        &[r"
      .-~~~~~~-._
\\ .-'  /\/\/\ '-._
>>=(  - - - - -  o \
// '-._________  _.-'
"],
        &[r"
      1K111111K11
1111K11111111111K11
111111K1K1K1K1K11411
1111K11111111111111K1
"],
    )
    .tint("1:gG")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.1)
    .weight(5)
    .biomes(FRESH),
    Species::new(
        "bluegill",
        &[r"
     _.-~~-._
\\ .-'  ||  '-.
>>=(  || ||  o )
// '-._____.-'
"],
        &[r"
     11111111
11111111bb11111
111111bb1bb11411
11RRRRRRRRRR11
"],
    )
    .tint("1:gGcC")
    .zone(Zone::Upper)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .school(1, 3)
    .weight(6)
    .biomes(FRESH),
    Species::new(
        "yellow-perch",
        &[r"
     .--.  .~~.
\\ .-'||'--'||'-.
>>=(  ||  ||   o )
// '-.__.-'--.-'
"],
        &[r"
     1111111rr1
111111KK1111KK111
111111KK11KK111411
1111111111111111
"],
    )
    .tint("1:yY")
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .school(2, 6)
    .weight(5)
    .biomes(FRESH),
    Species::new(
        "walleye",
        &[r"
      _.-~-.  _.-.
\\ .-'  :  '-'  : '-.__
>>=( : : : : : : (o)  )
// '-.____________..-'
"],
        &[r"
      111111111111
11111111K1111111K111111
11111K1K1K1K1K1K1141111
1111111111111111111111
"],
    )
    .tint("1:yY")
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.3, 1.0)
    .weight(4)
    .biomes(FRESH),
    Species::new(
        "lake-sturgeon",
        &[r"
             ___..---.._
\\\\      _.-'^ ^ ^ ^ ^ '-._
===<<<<(  ^ ^ ^ ^ ^  o \___.
///      '-.._ ~ ~ ~ _..-'  ~~
"],
        &[r"
             11111111111
11111111111111w1w1w1w1w11111
1111111111w1w1w1w1w114111111
111111111111111w1w1w11111111ww
"],
    )
    .tint("1:wK")
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.25, 0.7)
    .weight(2)
    .biomes(FRESH),
    Species::new(
        "longnose-gar",
        &[r"
  .------------------.
\\)  : : : : : : : : '._
>>=(  : : : : : : o )=^=^=^=-'
"],
        &[r"
  11111111111111111111
11111K1K1K1K1K1K1K1K1111
111111K1K1K1K1K1K1411111111111
"],
    )
    .tint("1:gGyY")
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.3, 1.2)
    .weight(3)
    .biomes(FRESH),
    Species::new(
        "paddlefish",
        &[r"
       _.---._
\\\  _.-'      '-._________
===<(   . .  .  o ______  '.
////  '-.__    _.-'       )
           ''-''  '------'
"],
        &[r"
       1111111
111111111111111111111111111
1111111111111111411111111111
111111111111111111111111111
           111111111111111
"],
    )
    .tint("1:wK")
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.3, 0.9)
    .weight(2)
    .biomes(FRESH),
    Species::new(
        "elephantnose-fish",
        &[r"
 _.--.
\\(  o )_
//'-.__) '.
       '--'
"],
        &[r"
 11111
111114111
11111111111
       1111
"],
    )
    .tint("1:KwyK")
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(3)
    .biomes(TANK | FRESH),
    Species::new(
        "glass-catfish",
        &[r"
  .-----.
>< )=|=|=( o)
  '-----'
"],
        &[r"
  CCCCCCC
CCCCcccccCC4C
  CCCCCCC
"],
    )
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .school(4, 8)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "archerfish",
        &[r"
                .
              '
         _.-~'.
\\  .-'| | |  o)
>=<(  | | |   ~_)
//  '-.____.-'
"],
        &[r"
                1
              1
         111111
1111111K1K1K1141
111111K1K1K111111
11111111111111
"],
    )
    .tint("1:wW")
    .zone(Zone::Top)
    .swim(Swim::Hover)
    .speed(0.25, 0.8)
    .weight(3)
    .biomes(TANK | FRESH),
    Species::new(
        "killifish",
        &[r"
   .~~.
(((=(  o)>
   '--'
"],
        &[r"
   1111
6661111411
   1111
"],
    )
    .tint("1:bBrRyY 6:rRyYgG")
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.3, 1.0)
    .school(2, 5)
    .weight(4)
    .biomes(TANK | FRESH),
    Species::new(
        "bichir",
        &[r"
      ^ ^ ^ ^ ^ ^ ^ ^
~=-=-=-=-=-=-=-=-=-=-.o )
             \\\ '--'
"],
        &[r"
      111111111111111
1111111111111111111111411
             11111111
"],
    )
    .tint("1:gKyw")
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(3)
    .biomes(TANK | FRESH),
];
