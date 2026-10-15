//! The sprites of asciiquarium 1.1, copied verbatim.
//!
//! Asciiquarium is Copyright (C) 2013 Kirk Baucom <kbaucom@schizoid.com>, most of
//! the art is by Joan Stark, and it is free software under the GNU General
//! Public License version 2 or (at your option) any later version.
//!
//! The strings are exactly what Perl makes of the original `q{}` literals, so
//! they keep their quirks: a leading newline (dropped when the shape is built),
//! `?` for explicitly transparent cells, and spaces that are opaque unless they
//! lead a line of an "auto transparent" sprite. Color masks use Term::Animation
//! letters (`r g y b m c w k`, uppercase = bold) and digits that are replaced
//! by random colors when a creature is spawned.

// Generated from asciiquarium.pl; edit with care, trailing spaces are significant.

/// The four rows of the surface, tiled across the screen.
pub const WATER: [&str; 4] = [
    r"~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~",
    r"^^^^ ^^^  ^^^   ^^^    ^^^^      ",
    r"^^^^      ^^^^     ^^^    ^^     ",
    r"^^      ^^^^      ^^^    ^^^^^^  ",
];

/// The castle in the bottom right corner.
pub const CASTLE: &str = r"
               T~~
               |
              /^\
             /   \
 _   _   _  /     \  _   _   _
[ ]_[ ]_[ ]/ _   _ \[ ]_[ ]_[ ]
|_=__-_ =_|_[ ]_[ ]_|_=-___-__|
 | _- =  | =_ = _    |= _=   |
 |= -[]  |- = _ =    |_-=_[] |
 | =_    |= - ___    | =_ =  |
 |=  []- |-  /| |\   |=_ =[] |
 |- =_   | =| | | |  |- = -  |
 |_______|__|_|_|_|__|_______|
";

pub const CASTLE_MASK: &str = r"
                RR

              yyy
             y   y
            y     y
           y       y



              yyy
             yy yy
            y y y y
            yyyyyyy
";

// Fish masks: 1 body, 2 dorsal fin, 3 flippers, 4 eye, 5 mouth, 6 tailfin, 7 gills.

pub const FISH_0R: &str = r#"
       \
     ...\..,
\  /'       \
 >=     (  ' >
/  \      / /
    `"'"'/''
"#;

pub const FISH_0R_MASK: &str = r"
       2
     1112111
6  11       1
 66     7  4 5
6  1      3 1
    11111311
";

pub const FISH_0L: &str = r#"
      /
  ,../...
 /       '\  /
< '  )     =<
 \ \      /  \
  `'\'"'"'
"#;

pub const FISH_0L_MASK: &str = r"
      2
  1112111
 1       11  6
5 4  7     66
 1 3      1  6
  11311111
";

pub const FISH_1R: &str = r"
    \
\ /--\
>=  (o>
/ \__/
    /
";

pub const FISH_1R_MASK: &str = r"
    2
6 1111
66  745
6 1111
    3
";

pub const FISH_1L: &str = r"
  /
 /--\ /
<o)  =<
 \__/ \
  \
";

pub const FISH_1L_MASK: &str = r"
  2
 1111 6
547  66
 1111 6
  3
";

pub const FISH_2R: &str = r"
       \:.
\;,   ,;\\\,,
  \\\;;:::::::o
  ///;;::::::::<
 /;` ``/////``
";

pub const FISH_2R_MASK: &str = r"
       222
666   1122211
  6661111111114
  66611111111115
 666 113333311
";

pub const FISH_2L: &str = r"
      .:/
   ,,///;,   ,;/
 o:::::::;;///
>::::::::;;\\\
  ''\\\\\'' ';\
";

pub const FISH_2L_MASK: &str = r"
      222
   1122211   666
 4111111111666
51111111111666
  113333311 666
";

pub const FISH_3R: &str = r"
  __
><_'>
   '
";

pub const FISH_3R_MASK: &str = r"
  11
61145
   3
";

pub const FISH_3L: &str = r"
 __
<'_><
 `
";

pub const FISH_3L_MASK: &str = r"
 11
54116
 3
";

pub const FISH_4R: &str = r"
   ..\,
>='   ('>
  '''/''
";

pub const FISH_4R_MASK: &str = r"
   1121
661   745
  111311
";

pub const FISH_4L: &str = r"
  ,/..
<')   `=<
 ``\```
";

pub const FISH_4L_MASK: &str = r"
  1211
547   166
 113111
";

pub const FISH_5R: &str = r"
   \
  / \
>=_('>
  \_/
   /
";

pub const FISH_5R_MASK: &str = r"
   2
  1 1
661745
  111
   3
";

pub const FISH_5L: &str = r"
  /
 / \
<')_=<
 \_/
  \
";

pub const FISH_5L_MASK: &str = r"
  2
 1 1
547166
 111
  3
";

pub const FISH_6R: &str = r"
  ,\
>=('>
  '/
";

pub const FISH_6R_MASK: &str = r"
  12
66745
  13
";

pub const FISH_6L: &str = r"
 /,
<')=<
 \`
";

pub const FISH_6L_MASK: &str = r"
 21
54766
 31
";

pub const FISH_7R: &str = r"
  __
\/ o\
/\__/
";

pub const FISH_7R_MASK: &str = r"
  11
61 41
61111
";

pub const FISH_7L: &str = r"
 __
/o \/
\__/\
";

pub const FISH_7L_MASK: &str = r"
 11
14 16
11116
";

/// Every fish as (shape, mask), alternating right- and left-facing like the original list.
pub const FISH: [(&str, &str); 16] = [
    (FISH_0R, FISH_0R_MASK),
    (FISH_0L, FISH_0L_MASK),
    (FISH_1R, FISH_1R_MASK),
    (FISH_1L, FISH_1L_MASK),
    (FISH_2R, FISH_2R_MASK),
    (FISH_2L, FISH_2L_MASK),
    (FISH_3R, FISH_3R_MASK),
    (FISH_3L, FISH_3L_MASK),
    (FISH_4R, FISH_4R_MASK),
    (FISH_4L, FISH_4L_MASK),
    (FISH_5R, FISH_5R_MASK),
    (FISH_5L, FISH_5L_MASK),
    (FISH_6R, FISH_6R_MASK),
    (FISH_6L, FISH_6L_MASK),
    (FISH_7R, FISH_7R_MASK),
    (FISH_7L, FISH_7L_MASK),
];

/// What is left of a fish the shark caught.
pub const SPLAT: [&str; 4] = [
    r"

   .
  ***
   '

",
    r#"

 ",*;`
 "*,**
 *"'~'

"#,
    r#"
  , ,
 " ","'
 *" *'"
  " ; .

"#,
    r#"
* ' , ' `
' ` * . '
 ' `' ",'
* ' " * .
" * ', '
"#,
];

/// Right- and left-facing.
pub const SHARK: [&str; 2] = [
    r"
                              __
                             ( `\
  ,??????????????????????????)   `\
;' `.????????????????????????(     `\__
 ;   `.?????????????__..---''          `~~~~-._
  `.   `.____...--''                       (b  `--._
    >                     _.-'      .((      ._     )
  .`.-`--...__         .-'     -.___.....-(|/|/|/|/'
 ;.'?????????`. ...----`.___.',,,_______......---'
 '???????????'-'
",
    r"
                     __
                    /' )
                  /'   (??????????????????????????,
              __/'     )????????????????????????.' `;
      _.-~~~~'          ``---..__?????????????.'   ;
 _.--'  b)                       ``--...____.'   .'
(     _.      )).      `-._                     <
 `\|\|\|\|)-.....___.-     `-.         __...--'-.'.
   `---......_______,,,`.___.'----... .'?????????`.;
                                     `-`???????????`
",
];

pub const SHARK_MASK: [&str; 2] = [
    r"





                                           cR
 
                                          cWWWWWWWW


",
    r"





        Rc

  WWWWWWWWc


",
];

/// The fishhook. The line above it is generated.
pub const HOOK: &str = r"
       o
      ||
      ||
/ \   ||
  \__//
  `--' 
";

/// The barb: the part of the hook that catches fish.
pub const HOOK_POINT: &str = r"
.
 
\
 
";

pub const SHIP: [&str; 2] = [
    r"
     |    |    |
    )_)  )_)  )_)
   )___))___))___)\
  )____)____)_____)\\
_____|____|____|____\\\__
\                   /
",
    r"
         |    |    |
        (_(  (_(  (_(
      /(___((___((___(
    //(_____(____(____(
__///____|____|____|_____
    \                   /
",
];

pub const SHIP_MASK: [&str; 2] = [
    r"
     y    y    y

                  w
                   ww
yyyyyyyyyyyyyyyyyyyywwwyy
y                   y
",
    r"
         y    y    y

      w
    ww
yywwwyyyyyyyyyyyyyyyyyyyy
    y                   y
",
];

pub const WHALE: [&str; 2] = [
    r"
        .-----:
      .'       `.
,????/       (o) \
\`._/          ,__)
",
    r"
    :-----.
  .'       `.
 / (o)       \????,
(__,          \_.'/
",
];

pub const WHALE_MASK: [&str; 2] = [
    r"
             C C
           CCCCCCC
           C  C  C
        BBBBBBB
      BB       BB
B    B       BWB B
BBBBB          BBBB
",
    r"
   C C
 CCCCCCC
 C  C  C
    BBBBBBB
  BB       BB
 B BWB       B    B
BBBB          BBBBB
",
];

/// Spout frames, aligned over the whale's blowhole when the frames are built.
pub const WATER_SPOUT: [&str; 7] = [
    r"


   :
",
    r"

   :
   :
",
    r"
  . .
  -:-
   :
",
    r"
  . .
 .-:-.
   :
",
    r"
  . .
'.-:-.`
'  :  '
",
    r"

 .- -.
;  :  ;
",
    r"


;     ;
",
];

/// Sea monster swimming right, four frames.
pub const MONSTER_R: [&str; 4] = [
    r"
                                                          ____
            __??????????????????????????????????????????/   o  \
          /    \????????_?????????????????????_???????/     ____ >
  _??????|  __  |?????/   \????????_????????/   \????|     |
 | \?????|  ||  |????|     |?????/   \?????|     |???|     |
",
    r"
                                                          ____
                                             __?????????/   o  \
             _?????????????????????_???????/    \?????/     ____ >
   _???????/   \????????_????????/   \????|  __  |???|     |
  | \?????|     |?????/   \?????|     |???|  ||  |???|     |
",
    r"
                                                          ____
                                  __????????????????????/   o  \
 _??????????????????????_???????/    \????????_???????/     ____ >
| \??????????_????????/   \????|  __  |?????/   \????|     |
 \ \???????/   \?????|     |???|  ||  |????|     |???|     |
",
    r"
                                                          ____
                       __???????????????????????????????/   o  \
  _??????????_???????/    \????????_??????????????????/     ____ >
 | \???????/   \????|  __  |?????/   \????????_??????|     |
  \ \?????|     |???|  ||  |????|     |?????/   \????|     |
",
];

/// Sea monster swimming left, four frames.
pub const MONSTER_L: [&str; 4] = [
    r"
    ____
  /  o   \??????????????????????????????????????????__
< ____     \???????_?????????????????????_????????/    \
      |     |????/   \????????_????????/   \?????|  __  |??????_
      |     |???|     |?????/   \?????|     |????|  ||  |?????/ |
",
    r"
    ____
  /  o   \?????????__
< ____     \?????/    \???????_?????????????????????_
      |     |???|  __  |????/   \????????_????????/   \???????_
      |     |???|  ||  |???|     |?????/   \?????|     |?????/ |
",
    r"
    ____
  /  o   \????????????????????__
< ____     \???????_????????/    \???????_??????????????????????_
      |     |????/   \?????|  __  |????/   \????????_??????????/ |
      |     |???|     |????|  ||  |???|     |?????/   \???????/ /
",
    r"
    ____
  /  o   \???????????????????????????????__
< ____     \??????????????????_????????/    \???????_??????????_
      |     |??????_????????/   \?????|  __  |????/   \???????/ |
      |     |????/   \?????|     |????|  ||  |???|     |?????/ /
",
];

pub const MONSTER_MASK: [&str; 2] = [
    r"

                                                            W



",
    r"

     W



",
];

pub const BIG_FISH: [&str; 2] = [
    r#"
 ______
`""-.  `````-----.....__
     `.  .      .       `-.
       :     .     .       `.
 ,     :   .    .          _ :
: `.   :                  (@) `._
 `. `..'     .     =`-.       .__)
   ;     .        =  ~  :     .-"
 .' .'`.   .    .  =.-'  `._ .'
: .'   :               .   .'
 '   .'  .    .     .   .-'
   .'____....----''.'=.'
   ""             .'.'
               ''"'`
"#,
    r#"
                           ______
          __.....-----'''''  .-""'
       .-'       .      .  .'
     .'       .     .     :
    : _          .    .   :     ,
 _.' (@)                  :   .' :
(__.       .-'=     .     `..' .'
 "-.     :  ~  =        .     ;
   `. _.'  `-.=  .    .   .'`. `.
     `.   .               :   `. :
       `-.   .     .    .  `.   `
          `.=`.``----....____`.
            `.`.             ""
              '`"``
"#,
];

pub const BIG_FISH_MASK: [&str; 2] = [
    r"
 111111
11111  11111111111111111
     11  2      2       111
       1     2     2       11
 1     1   2    2          1 1
1 11   1                  1W1 111
 11 1111     2     1111       1111
   1     2        1  1  1     111
 11 1111   2    2  1111  111 11
1 11   1               2   11
 1   11  2    2     2   111
   111111111111111111111
   11             1111
               11111
",
    r"
                           111111
          11111111111111111  11111
       111       2      2  11
     11       2     2     1
    1 1          2    2   1     1
 111 1W1                  1   11 1
1111       1111     2     1111 11
 111     1  1  1        2     1
   11 111  1111  2    2   1111 11
     11   2               1   11 1
       111   2     2    2  11   1
          111111111111111111111
            1111             11
              11111
",
];

pub const DUCKS_R: [&str; 3] = [
    r"
      _??????????_??????????_  
,____(')=??,____(')=??,____(')<
 \~~= ')????\~~= ')????\~~= ') 
",
    r"
      _??????????_??????????_  
,____(')=??,____(')<??,____(')=
 \~~= ')????\~~= ')????\~~= ') 
",
    r"
      _??????????_??????????_  
,____(')<??,____(')=??,____(')=
 \~~= ')????\~~= ')????\~~= ') 
",
];

pub const DUCKS_L: [&str; 3] = [
    r"
  _??????????_??????????_
>(')____,??=(')____,??=(')____,
 (` =~~/????(` =~~/????(` =~~/
",
    r"
  _??????????_??????????_
=(')____,??>(')____,??=(')____,
 (` =~~/????(` =~~/????(` =~~/
",
    r"
  _??????????_??????????_
=(')____,??=(')____,??>(')____,
 (` =~~/????(` =~~/????(` =~~/
",
];

pub const DUCKS_MASK: [&str; 2] = [
    r"
      g          g          g
wwwwwgcgy  wwwwwgcgy  wwwwwgcgy
 wwww Ww    wwww Ww    wwww Ww
",
    r"
  g          g          g
ygcgwwwww  ygcgwwwww  ygcgwwwww
 wW wwww    wW wwww    wW wwww
",
];

pub const DOLPHIN_R: [&str; 2] = [
    r"
        ,
      __)\_
(\_.-'    a`-.
(/~~````(/~^^`
",
    r"
        ,
(\__  __)\_
(/~.''    a`-.
    ````\)~^^`
",
];

pub const DOLPHIN_L: [&str; 2] = [
    r"
     ,
   _/(__
.-'a    `-._/)
'^^~\)''''~~\)
",
    r"
     ,
   _/(__  __/)
.-'a    ``.~\)
'^^~(/''''
",
];

pub const DOLPHIN_MASK: [&str; 2] = [
    r"


          W
",
    r"


   W
",
];

pub const SWAN_R: &str = r"
       ___
,_    / _,\
| \   \( \|
|  \_  \\
(_   \_) \
(\_   `   \
 \   -=~  /
";

pub const SWAN_L: &str = r"
 ___
/,_ \    _,
|/ )/   / |
  //  _/  |
 / ( /   _)
/   `   _/)
\  ~=-   /
";

pub const SWAN_MASK: [&str; 2] = [
    r"

         g
         yy
",
    r"

 g
yy
",
];
