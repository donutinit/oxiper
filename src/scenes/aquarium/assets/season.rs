//! Things that only come in their season (see `Season`).

use super::*;

pub const SPECIES: &[Species] = &[
    // A fish with a sugar-skull face, flowers in its eye sockets.
    Species::new(
        "calavera-fish",
        &[
            r"
    .---.
|\ /(O O)\
| >| *v* |>
|/ \ === /
    '---'
",
            r"
    .---.
\\ /(O O)\
) >| *v* |>
//\ \ === /
    '---'
",
        ],
        &[r"
    wwwww
11 ww2 2ww
1 1w 3k3 w1
11 w yyy w
    wwwww
"],
    )
    .tint("1:rRmMcC 2:yY 3:rRmMbB")
    .fps(0.3)
    .school(1, 2)
    .weight(8)
    .biomes(TANK | REEF | FRESH | KELP)
    .season(MUERTOS),
    Species::new(
        "alebrije-fish",
        &[
            r"
 \\   .~~~~.
=<<  /*^*^*^\
=<< ( o  ::: )>
=<<  \*v*v*v/
 //   '~~~~'
",
            r"
 //   .~~~~.
=<<  /*^*^*^\
=<< ( o  ::: )>
=<<  \*v*v*v/
 \\   '~~~~'
",
        ],
        &[r"
 11   222222
111  23535352
111 2 4  555 21
111  23535352
 11   222222
"],
    )
    .tint("1:rRmMyY 2:mMrRbB 3:yYgGcC 5:cCbBwW")
    .fps(0.4)
    .weight(8)
    .biomes(TANK | REEF | OCEAN | FRESH)
    .season(MUERTOS),
    // Butterfly-winged alebrije.
    Species::new(
        "alebrije-butterflyfish",
        &[
            r"
    /\  /\
 ~, /**\/**\ ,~
~~<(( ::::  4>>
 ~' \**/\**/ '~
    \/  \/
",
            r"
    ||  ||
 -, /**\/**\ ,-
--<(( ::::  4>>
 -' \**/\**/ '-
    ||  ||
",
        ],
        &[r"
    11  11
 22 1  11  1 22
222  55555  44
 22 1  11  1 22
    11  11
"],
    )
    .tint("1:rRmMyY 2:cCbBgG 5:yYwW")
    .fps(0.35)
    .weight(7)
    .biomes(REEF | OCEAN | TANK)
    .season(MUERTOS),
    // Round, spiny, every color at once.
    Species::new(
        "alebrije-puffer",
        &[
            r"
    \|/ \|/
  ,-'*@*'*@*-.
 /*  ~ ~ ~ ~  *\
<|*  o  .--.   |}>
 \*  ~ ~ ~ ~  */
  `-.*@*'*@*,-'
    /|\ /|\
",
            r"
    /|\ /|\
  ,-'*@*'*@*-.
 /*  ~ ~ ~ ~  *\
<|*  o  .--.   |}>
 \*  ~ ~ ~ ~  */
  `-.*@*'*@*,-'
    \|/ \|/
",
        ],
        &[r"
    222 222
  222131213122
 21  3 3 3 3  12
521  4  1111   255
 21  3 3 3 3  12
  2221312131222
    222 222
"],
    )
    .tint("1:rRyY 2:mMcCgG 3:yYrRbB 5:rRmM")
    .fps(0.3)
    .speed(0.25, 1.2)
    .weight(6)
    .biomes(TANK | REEF | FRESH)
    .season(MUERTOS),
    // A salamander in carnival colors.
    Species::new(
        "alebrije-axolotl",
        &[
            r"
          \\|/
  .~~~~~~~.(o   )
~~{ *  *  *  ~ v)
  /\  /\ .-/\ /\
",
            r"
          //|/
  .~~~~~~~.(o   )
~~{ *  *  *  ~ v)
  /\  /\ .-\/ \/
",
        ],
        &[r"
          5555
  11111111154   5
111 3  2  3  1 k5
  11  11 1111 11
"],
    )
    .tint("1:mMrR 2:yYcC 3:gGbB 5:rRmM")
    .swim(Swim::Wave)
    .zone(Zone::Lower)
    .speed(0.2, 0.8)
    .fps(0.25)
    .weight(7)
    .biomes(FRESH | TANK | KELP)
    .critter()
    .season(MUERTOS),
    // A jellyfish in a flowered catrina hat.
    Species::new(
        "catrina-jellyfish",
        &[
            r"
   ,*@*,
.-(_*o*_)-.
  .-'''-.
 ( o   o )
 (   v   )
  `-.-.-'
   ) ) (
  ( ( ) )
",
            r"
   ,*@*,
.-(_*o*_)-.
  .-'''-.
 ( o   o )
 (   v   )
  `-.-.-'
   ( ( )
  ) ) ( (
",
        ],
        &[r"
   1212
kkk222kkk
  WWWWW
 W k   k W
 W   k   W
  WWWWWWW
   c c c
  c c c c
"],
    )
    .tint("1:rRmM 2:yY")
    .swim(Swim::Drift)
    .zone(Zone::Upper)
    .speed(0.15, 0.5)
    .fps(0.2)
    .weight(7)
    .biomes(TANK | REEF | OCEAN | KELP)
    .critter()
    .traits(STING)
    .season(MUERTOS),
    // A crab with a little skull on its shell.
    Species::new(
        "calaverita-crab",
        &[
            r"
 \\ .----. //
(o)/ o  o \(o)
 ( (  vv  ) )
 /\ \_||_/ /\
/  \ ^^^^ /  \
",
            r"
 // .----. \\
(o)/ o  o \(o)
 ( (  vv  ) )
 /\ \_||_/ /\
\  / ^^^^ \  /
",
        ],
        &[r"
 rr wwwwww rr
rRrr k  k rrRr
 r r  kk  r r
 rr wwyyww rr
r  r YYYY r  r
"],
    )
    .swim(Swim::Crawl)
    .zone(Zone::Bottom)
    .speed(0.2, 0.6)
    .fps(0.4)
    .weight(7)
    .biomes(TANK | REEF | KELP | FRESH | OCEAN)
    .critter()
    .traits(SCAVENGER | DEFENDED)
    .season(MUERTOS),
    // A goldfish whose tail is a marigold in bloom; they come in little schools.
    Species::new(
        "pez-cempasuchil",
        &[
            r"
 .@@.  ,--.
(@@@@)<  ~ o>
 `@@'  `--'
",
            r"
 ,@@,  ,--.
{@@@@}<  ~ o>
 '@@`  `--'
",
        ],
        &[r"
 yYYy  1111
yYRRYy1  3 41
 yYYy  1111
"],
    )
    .tint("1:yYrR 3:rRmM")
    .swim(Swim::Wave)
    .speed(0.4, 1.4)
    .fps(0.5)
    .school(3, 7)
    .weight(8)
    .biomes(TANK | FRESH | REEF | KELP | OCEAN)
    .season(MUERTOS),
    // A seahorse with a crest of flames, painted all over.
    Species::new(
        "alebrije-seahorse",
        &[
            r"
  /\/\
 (  * o)>=-
<`)@ * )
 (  *  (
< )* @ )
 (  @ (
<  )* )
  (_,-'
",
            r"
  /\/\
 (  * o)>=-
~`)@ * )
 (  *  (
~ )* @ )
 (  @ (
~  )* )
  (_,-'
",
        ],
        &[r"
  3333
 1  2 41666
3115 2 1
 1  2  1
3 12 5 1
 1  5 1
3  12 1
  11111
"],
    )
    .tint("1:mMrRbB 2:yYcC 3:cCgGyY 5:rRmM 6:yY")
    .swim(Swim::Hover)
    .speed(0.05, 0.25)
    .fps(0.25)
    .weight(6)
    .biomes(REEF | TANK | KELP)
    .critter()
    .traits(SHY)
    .home("cempasuchil,terciopelo")
    .season(MUERTOS),
    // An octopus with a sugar-skull head: flowers on its brow, icing
    // around its eyes, tentacles in every color.
    Species::new(
        "calavera-octopus",
        &[
            r"
     .-'''''-.
   .'@ .-.-. @'.
  /  (*)   (*)  \
  |  '-' ^ '-'  |
   \  \vvvvv/  /
    `-.:___:.-'
   ,~/ /|  |\ \~,
  (~ (~ (  ) ~) ~)
",
            r"
     .-'''''-.
   .'@ .-.-. @'.
  /  (*)   (*)  \
  |  '-' ^ '-'  |
   \  \vvvvv/  /
    `-.:___:.-'
   ,~\ \|  |/ /~,
  ~) ~) )  ( (~ (~
",
        ],
        &[
            r"
     WWWWWWWWW
   WW3 22222 3WW
  W  535   535  W
  W  555 R 555  W
   W  WWWWWWW  W
    WWW2WWW2WWW
   121 11  11 121
  12 12 1  1 21 21
",
            r"
     WWWWWWWWW
   WW3 22222 3WW
  W  535   535  W
  W  555 R 555  W
   W  WWWWWWW  W
    WWW2WWW2WWW
   121 11  11 121
  21 21 1  1 12 12
",
        ],
    )
    .left(
        &[
            r"
     .-'''''-.
   .'@ .-.-. @'.
  /  (*)   (*)  \
  |  '-' ^ '-'  |
   \  \vvvvv/  /
    `-.:___:.-'
   ,~/ /|  |\ \~,
  (~ (~ (  ) ~) ~)
",
            r"
     .-'''''-.
   .'@ .-.-. @'.
  /  (*)   (*)  \
  |  '-' ^ '-'  |
   \  \vvvvv/  /
    `-.:___:.-'
   ,~\ \|  |/ /~,
  ~) ~) )  ( (~ (~
",
        ],
        &[
            r"
     WWWWWWWWW
   WW3 22222 3WW
  W  535   535  W
  W  555 R 555  W
   W  WWWWWWW  W
    WWW2WWW2WWW
   121 11  11 121
  12 12 1  1 21 21
",
            r"
     WWWWWWWWW
   WW3 22222 3WW
  W  535   535  W
  W  555 R 555  W
   W  WWWWWWW  W
    WWW2WWW2WWW
   121 11  11 121
  21 21 1  1 12 12
",
        ],
    )
    .tint("1:mMcCgG 2:cCmMyY 3:yYrR 5:mMbBrR")
    .swim(Swim::Drift)
    .speed(0.1, 0.4)
    .fps(0.25)
    .weight(6)
    .biomes(REEF | OCEAN | KELP | TANK | ARCTIC | DEEP)
    .critter()
    .traits(INK | CURIOUS)
    .season(MUERTOS),
    // A sea turtle whose shell is carved and painted like an alebrije.
    Species::new(
        "alebrije-turtle",
        &[
            r"
      _.-'''''-._
    .'*<>*@*<>*'.    _,--.
 ,-/<>*@*<>*@*<>\'-._/ o  )
(_,'-.__________.-' `-.__.'
   `-.\ \        `.\ \
      `-'          `-'
",
            r"
      _.-'''''-._
    .'*<>*@*<>*'.    _,--.
 ,-/<>*@*<>*@*<>\'-._/ o  )
(_,'-.__________.-' `-.__.'
    `-.\ \      `-.\ \
       `-'         `-'
",
        ],
        &[
            r"
      11111111111
    1123325233211    66666
 111332523325233166666 4  6
1111111111111111111 6666666
   6666 6        666 6
      666          666
",
            r"
      11111111111
    1123325233211    66666
 111332523325233166666 4  6
1111111111111111111 6666666
    6666 6      6666 6
       666         666
",
        ],
    )
    .tint("1:mMrR 2:yYcC 3:cCgGbB 5:rRmMyY 6:gGyYcC")
    .swim(Swim::Wave)
    .speed(0.3, 0.8)
    .fps(0.2)
    .weight(5)
    .biomes(REEF | OCEAN | TANK | KELP)
    .critter()
    .traits(DEFENDED | CURIOUS)
    .season(MUERTOS),
    // A manta ray painted with dots and diamonds, wings beating.
    Species::new(
        "alebrije-ray",
        &[
            r"
            _.-~-._
       _.-'*  @  *'-._
   ,.-'*  <> * <> *  *'-._
<=-=-=-=-=-=-=-=-=-=-=-=-=(o)>
   `'-.*  <> * <> *  *.-'
       `'-._*  @  *_.-'
            `'-~-'
",
            r"
            ?
       _.-'''''''-._
   ,.-'*  <> * <> *  *'-._
<=-=-=-=-=-=-=-=-=-=-=-=-=(o)>
   `'-.*  <> * <> *  *.-'
       `'-._._._._.-'
            ?
",
        ],
        &[
            r"
            1115111
       11112  5  21111
   11112  33 2 33 2  21111
131313131313131313131313131411
   11112  33 2 33 2  2111
       111112  5  21111
            111511
",
            r"

       1111111111111
   11112  33 2 33 2  21111
131313131313131313131313131411
   11112  33 2 33 2  2111
       11111111111111

",
        ],
    )
    .tint("1:mMbBcC 2:yYrR 3:cCgG 5:rRmMyY")
    .swim(Swim::Wave)
    .zone(Zone::Middle)
    .speed(0.4, 1.2)
    .fps(0.25)
    .weight(5)
    .biomes(REEF | OCEAN | KELP | ARCTIC | DEEP)
    .traits(FILTER)
    .season(MUERTOS),
];

pub const PLANTS: &[Plant] = &[
    // Marigolds: the flower of the dead.
    Plant::sprite(
        "cempasuchil",
        &[
            r"
 ,@,
(@@@)  ,@,
 `@'  (@@@)
  |    `@'
 \|/    |
  |  \  |
  |_  \ |/
   \|  |
",
            r"
  ,@,
 (@@@)  ,@,
  `@'  (@@@)
  |     `@'
 \|/    |
  |  \  |
  |_  \ |/
   \|  |
",
        ],
        &[
            r"
 yYy
yYYYy  yYy
 yYy  yYYYy
  g    yYy
 ggg    g
  g  g  g
  gg  g gg
   gg  g
",
            r"
  yYy
 yYYYy  yYy
  yYy  yYYYy
  g     yYy
 ggg    g
  g  g  g
  gg  g gg
   gg  g
",
        ],
    )
    .fps(0.2)
    .weight(8)
    .biomes(TANK | FRESH | KELP | REEF | OCEAN)
    .season(MUERTOS),
    // Cockscomb, velvet-red ruffles.
    Plant::sprite(
        "terciopelo",
        &[
            r"
  ,#%#%,
 (%#%#%#)
  `\|||/'
    |||
   \ | /
    \|/
     |
",
            r"
   ,#%#%,
  (%#%#%#)
   `\|||/'
    |||
   \ | /
    \|/
     |
",
        ],
        &[
            r"
  mMrMmm
 mMrMrMmm
  mgggggm
    ggg
   g g g
    ggg
     g
",
            r"
   mMrMmm
  mMrMrMmm
   mgggggm
    ggg
   g g g
    ggg
     g
",
        ],
    )
    .fps(0.2)
    .weight(7)
    .biomes(TANK | FRESH | KELP | REEF)
    .season(MUERTOS),
    // Baby's breath, a white cloud on thin stems.
    Plant::sprite(
        "nube",
        &[
            r"
 o  .  o .
 .\ | /  o
  o\|/ o/ .
 .  | .\|  o
    |/  |/
    |   |
",
            r"
 .  o  . o
 o\ | /  .
  .\|/ ./ o
 o  | o\|  .
    |\  |\
    |   |
",
        ],
        &[r"
 W  w  W w
 wg g g  W
  Wggg Wg w
 w  g wgg  W
    gg  gg
    g   g
"],
    )
    .fps(0.25)
    .weight(6)
    .biomes(TANK | FRESH | KELP)
    .season(MUERTOS),
];

pub const DECOR: &[Decor] = &[
    Decor::new(
        "ofrenda",
        &[
            r"
o----------------------------------------------o
?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|
?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/
????????????????????.======.
????????????????????| .--. |
?????????????????)??| (oo) |??)
?????????????????|??| '--' |??|
?????????,-.??)??#??'======'??#??)
?????????(o?o)|?.--------------.?|.-^-.
???.@.????vvv?#?|/\/\/\/\/\/\/\|?#(=+=)???.@.
??(@@@)?.------------------------------.?(@@@)
???`@'??|<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>|??`@'
.==============================================.
|/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\|
'=============================================='
",
            r"
o----------------------------------------------o
?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|
?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/
????????????????????.======.
????????????????????| .--. |
?????????????????|??| (oo) |??|
?????????????????|??| '--' |??|
?????????,-.??|??#??'======'??#??|
?????????(o?o)|?.--------------.?|.-^-.
???.@.????vvv?#?|/\/\/\/\/\/\/\|?#(=+=)???.@.
??(@@@)?.------------------------------.?(@@@)
???`@'??|<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>|??`@'
.==============================================.
|/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\|
'=============================================='
",
            r"
o----------------------------------------------o
?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|?|:v:|
?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/?\^v^/
????????????????????.======.
????????????????????| .--. |
?????????????????(??| (oo) |??(
?????????????????|??| '--' |??|
?????????,-.??(??#??'======'??#??(
?????????(o?o)|?.--------------.?|.-^-.
???.@.????vvv?#?|/\/\/\/\/\/\/\|?#(=+=)???.@.
??(@@@)?.------------------------------.?(@@@)
???`@'??|<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>|??`@'
.==============================================.
|/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\<>/\|
'=============================================='
",
        ],
        &[r"
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
 rrMrr GGcGG yyRyy MMYMM ccBcc RRrRR YYGYY BByBB
 rrMrr GGcGG yyRyy MMYMM ccBcc RRrRR YYGYY BByBB
                    yyyyyyyy
                    y wwww y
                 Y  y wkkw y  Y
                 W  y wwww y  W
         www  Y  r  yyyyyyyy  r  Y
         wM MwW yyyyyyyyyyyyyyyy WyYyYy
   yYy    wkw r yrrMMGGccYYBBrry ryYYYy   yYy
  yYYYy yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy yYYYy
   gyg  yrrMMYYGGccBBrrMMYYGGccBBrrMMYYy  gyg
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
yMMrrYYGGccBBMMrrYYGGccBBMMrrYYGGccBBMMrrYYGGccy
yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
"],
    )
    .fps(0.35)
    .weight(7)
    .biomes(TANK | REEF | FRESH | KELP | OCEAN | ARCTIC)
    .traits(SHELTER | GLOW)
    .season(MUERTOS),
    Decor::new(
        "calavera-de-azucar",
        &[r"
?????*@*@*@*@*@*
??,-'~~~~~~~~~~~`-,
?/??.---.???.---.??\
|???(*@*)???(*@*)???|
|?*?`---'?^?`---'?*?|
?\???????/_\???????/
???\??|vvvvvvv|??/
??????`-.___.-'
???????'-----'
"],
        &[r"
     12121212121
  WWWWWmWWmWWmWWWWW
 W  WWWWW   WWWWW  W
W   W121W   W121W   W
W 3 WWWWW k WWWWW 3 W
 W       WkW       W
   W  kyyyyyyyk  W
      WWWWWWWWW
       WWWWWWW
"],
    )
    .tint("1:rRmMbB 2:yYcC 3:mMgG")
    .weight(6)
    .biomes(TANK | REEF | FRESH | KELP | OCEAN)
    .traits(SHELTER)
    .season(MUERTOS),
    Decor::new(
        "veladoras",
        &[
            r"

?????????)
?|???????|???????(
?|???(???#???|???|???)
?#???|???#???|???#???|
?#???#???#???#???#???#
?#???#???#???#???#???#
(______________________)
",
            r"

?????????|
?(???????|???????)
?|???)???#???(???|???|
?#???|???#???|???#???|
?#???#???#???#???#???#
?#???#???#???#???#???#
(______________________)
",
            r"

?????????(
?)???????|???????|
?|???|???#???)???|???(
?#???|???#???|???#???|
?#???#???#???#???#???#
?#???#???#???#???#???#
(______________________)
",
        ],
        &[r"

         Y
 Y       W       Y
 W   Y   M   Y   W   Y
 R   W   M   W   G   W
 R   Y   M   C   G   B
 R   Y   M   C   G   B
yyyyyyyyyyyyyyyyyyyyyyyy
"],
    )
    .fps(0.35)
    .weight(7)
    .biomes(TANK | REEF | FRESH | KELP | OCEAN | ARCTIC)
    .traits(GLOW)
    .season(MUERTOS),
    Decor::new(
        "pan-de-muerto",
        &[r"
?????o
???,-'-'-.
?,'/|||\_`.
(=|=====|=)
?`-.___.-'

"],
        &[r"
     W
   YYYYYYY
 yyWWWWyyyy
yWWyyyyyWWy
 yyyyyyyyy

"],
    )
    .weight(6)
    .biomes(TANK | REEF | FRESH | KELP)
    .season(MUERTOS),
    Decor::new(
        "papel-picado",
        &[r"
*????????????????????????????????????????*
|___\????????????????????????????????____|
|????______\??????????????????______/????|
|??|:v:|?|:v:|_______________/???|:v:|???|
|??\^v^/?\^v^/?|:v:|?|:v:|?|:v:|?\^v^/???|
|??????????????\^v^/?\^v^/?\^v^/?????????|
|????????????????????????????????????????|
|????????????????????????????????????????|
|????????????????????????????????????????|
"],
        &[r"
Y                                        Y
ywwww                                wwwwy
y    wwwwwww                  wwwwwww    y
y  rrMrr GGcGGwwwwwwwwwwwwwwww   RRrRR   y
y  rrMrr GGcGG yyRyy MMYMM ccBcc RRrRR   y
y              yyRyy MMYMM ccBcc         y
y                                        y
y                                        y
y                                        y
"],
    )
    .weight(5)
    .biomes(TANK | REEF | FRESH | KELP | OCEAN)
    .season(MUERTOS),
    // A small altar for small screens: an arch of marigolds over a
    // portrait, two candles, pan de muerto, a sugar skull, water.
    Decor::new(
        "ofrendita",
        &[
            r"
   .@*@*@*@*@*@*@*@*@*@*@*.
   @'      .-----.       '@
   *   )   |  o  |    (   *
   @   #   | /|\ |    #   @
   *   #   '-----'    #   *
.============================.
|.@.  (=^=)   {oo}   [_]  .@.|
|<>/\<>/\<>/\<>/\<>/\<>/\<>/\|
'============================'
",
            r"
   .@*@*@*@*@*@*@*@*@*@*@*.
   @'      .-----.       '@
   *   |   |  o  |    |   *
   @   #   | /|\ |    #   @
   *   #   '-----'    #   *
.============================.
|.@.  (=^=)   {oo}   [_]  .@.|
|<>/\<>/\<>/\<>/\<>/\<>/\<>/\|
'============================'
",
            r"
   .@*@*@*@*@*@*@*@*@*@*@*.
   @'      .-----.       '@
   *   (   |  o  |    )   *
   @   #   | /|\ |    #   @
   *   #   '-----'    #   *
.============================.
|.@.  (=^=)   {oo}   [_]  .@.|
|<>/\<>/\<>/\<>/\<>/\<>/\<>/\|
'============================'
",
        ],
        &[r"
   yYRYRYRYRYRYRYRYRYRYRYRy
   Yy      yyyyyyy       yY
   R   Y   y  w  y    Y   R
   Y   W   y www y    W   Y
   R   W   yyyyyyy    W   R
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WYRY  yYYYy   WMMW   CCC  YRYW
WMMYYCCGGRRBBMMYYCCGGRRBBMMYYW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
"],
    )
    .fps(0.35)
    .weight(6)
    .biomes(TANK | REEF | FRESH | KELP | OCEAN | ARCTIC | DEEP)
    .traits(SHELTER | GLOW)
    .season(MUERTOS),
    // A clay sahumerio with glowing copal, its smoke curling up.
    Decor::new(
        "copal",
        &[
            r"
    (
     )  (
    (  )
  ._____.
  \*~*~*/
   `---'
    |_|
   /___\
",
            r"
     )
    (  )
     )  (
  ._____.
  \*~*~*/
   `---'
    |_|
   /___\
",
            r"
    )  (
   (  )
    (
  ._____.
  \*~*~*/
   `---'
    |_|
   /___\
",
        ],
        &[
            r"
    w
     w  w
    w  w
  rrrrrrr
  rRYRYRr
   rrrrr
    rrr
   rrrrr
",
            r"
     w
    w  w
     w  w
  rrrrrrr
  rRYRYRr
   rrrrr
    rrr
   rrrrr
",
            r"
    w  w
   w  w
    w
  rrrrrrr
  rRYRYRr
   rrrrr
    rrr
   rrrrr
",
        ],
    )
    .fps(0.2)
    .weight(6)
    .biomes(TANK | REEF | FRESH | KELP | OCEAN | ARCTIC | DEEP)
    .traits(GLOW)
    .season(MUERTOS),
    // A grave dressed for the vigil: its cross, a blanket of marigolds,
    // candles at either side.
    Decor::new(
        "tumba",
        &[
            r"
          _|_
    )     |     (
    #  .@*@|@*@.  #
    # (@*@*@*@*@) #
 .@.#(*@*@*@*@*@*)#.@.
(@@@)'-----------'(@@@)
",
            r"
          _|_
    |     |     |
    #  .@*@|@*@.  #
    # (@*@*@*@*@) #
 .@.#(*@*@*@*@*@*)#.@.
(@@@)'-----------'(@@@)
",
            r"
          _|_
    (     |     )
    #  .@*@|@*@.  #
    # (@*@*@*@*@) #
 .@.#(*@*@*@*@*@*)#.@.
(@@@)'-----------'(@@@)
",
        ],
        &[r"
          wwy
    Y     Y     Y
    W  yYRYwYRYy  W
    W yYRYRYRYRYy W
 yYyWyRYRYRYRYRYRyWyYy
yYYYyyyyyyyyyyyyyyyYYYy
"],
    )
    .fps(0.35)
    .weight(5)
    .biomes(TANK | REEF | FRESH | KELP | OCEAN | ARCTIC | DEEP)
    .traits(GLOW)
    .season(MUERTOS),
];

pub const VISITORS: &[Visitor] = &[
    Visitor::new(
        "trajinera",
        Lane::Boat,
        &[
            r"
???|?????????,------------------------,
???|????????/?     DIA DE MUERTOS     ?\
???|??o?????(@*@*@*@*@*@*@*@*@*@*@*@*@*)
???|-/|\????|????o?????o??????o????????|
???_.?|?????|???/|\???/|\????/|\???????|??????_.
???|?\________________________________________/
",
            r"
???|?????????,------------------------,
???|????????/?     DIA DE MUERTOS     ?\
???|-?o?????(@*@*@*@*@*@*@*@*@*@*@*@*@*)
???|?/|\????|????o?????o??????o????????|
???_.?|?????|???/|\???/|\????/|\???????|??????_.
???|?\________________________________________/
",
        ],
        &[
            r"
   y         MMMMMMMMMMMMMMMMMMMMMMMMMM
   y        r YYYYYYYYYYYYYYYYYYYYYYYY r
   y  Y     gRYMYYYCYRYMYYYCYRYMYYYCYRYg
   yyrrr    y    Y     M      C        y
   yy r     y   rrr   ggg    bbb       y      yy
   y yrrrYYYYGGGGMMMMccccBBBBrrrrYYYYGGGGMMMMcy
",
            r"
   y         MMMMMMMMMMMMMMMMMMMMMMMMMM
   y        r YYYYYYYYYYYYYYYYYYYYYYYY r
   yy Y     gRYMYYYCYRYMYYYCYRYMYYYCYRYg
   y rrr    y    Y     M      C        y
   yy r     y   rrr   ggg    bbb       y      yy
   y yrrrYYYYGGGGMMMMccccBBBBrrrrYYYYGGGGMMMMcy
",
        ],
    )
    .left(
        &[
            r"
?????????,------------------------,?????????|
????????/?     DIA DE MUERTOS     ?\????????|
????????(*@*@*@*@*@*@*@*@*@*@*@*@*@)?????o??|
????????|????????o??????o?????o????|????/|\-|
._??????|???????/|\????/|\???/|\???|?????|?._
?\________________________________________/?|
",
            r"
?????????,------------------------,?????????|
????????/?     DIA DE MUERTOS     ?\????????|
????????(*@*@*@*@*@*@*@*@*@*@*@*@*@)?????o?-|
????????|????????o??????o?????o????|????/|\?|
._??????|???????/|\????/|\???/|\???|?????|?._
?\________________________________________/?|
",
        ],
        &[
            r"
         MMMMMMMMMMMMMMMMMMMMMMMMMM         y
        r YYYYYYYYYYYYYYYYYYYYYYYY r        y
        gYRYCYYYMYRYCYYYMYRYCYYYMYRg     Y  y
        y        C      M     Y    y    rrryy
yy      y       bbb    ggg   rrr   y     r yy
 ycMMMMGGGGYYYYrrrrBBBBccccMMMMGGGGYYYYrrry y
",
            r"
         MMMMMMMMMMMMMMMMMMMMMMMMMM         y
        r YYYYYYYYYYYYYYYYYYYYYYYY r        y
        gYRYCYYYMYRYCYYYMYRYCYYYMYRg     Y yy
        y        C      M     Y    y    rrr y
yy      y       bbb    ggg   rrr   y     r yy
 ycMMMMGGGGYYYYrrrrBBBBccccMMMMGGGGYYYYrrry y
",
        ],
    )
    .speed(0.5, 0.8)
    .fps(0.15)
    .weight(5)
    .biomes(FRESH | TANK | KELP | REEF | OCEAN)
    .season(MUERTOS),
    Visitor::new(
        "monarch-butterflies",
        Lane::Sky,
        &[
            r"
?????????????\?/
?????????????)V(?????????????????\?/
??,\?/,??????/?\??????,\?/,??????)V(????,\?/,
?(?@V@?)?????????????(?@V@?)?????/?\???(?@V@?)
??`/?\'???????????????`/?\'?????????????`/?\'
",
            r"
????????????,\?/,
???????????(?@V@?)??????????????,\?/,
???\?/??????`/?\'??????\?/?????(?@V@?)???\?/
???)V(?????????????????)V(??????`/?\'????)V(
???/?\?????????????????/?\???????????????/?\
",
        ],
        &[
            r"
             y y
             yWy                 y y
  yy yy      y y      yy yy      yWy    yy yy
 y YwY y             y YwY y     y y   y YwY y
  yy yy               yy yy             yy yy
",
            r"
            yy yy
           y YwY y              yy yy
   y y      yy yy      y y     y YwY y   y y
   yWy                 yWy      yy yy    yWy
   y y                 y y               y y
",
        ],
    )
    .speed(0.4, 0.7)
    .fps(0.3)
    .weight(6)
    .biomes(REEF | OCEAN | KELP | FRESH | TANK | ARCTIC)
    .season(MUERTOS),
    Visitor::new(
        "skeleton-canoe",
        Lane::Boat,
        &[
            r"
?????_,^^,_
???<========>
?????(o?o)
????/|HHH|\-\
'????/_?_\???\????'
<_.-.-.-.-.-.-.-._>
",
            r"
?????_,^^,_
???<========>
?????(o?o)
????/|HHH|\\
'????/_?_\??\?????'
<_.-.-.-.-.-.-.-._>
",
        ],
        &[
            r"
     YYYYYY
   rrrrrrrrrr
     Wk kW
    WWWWWWWyy
y    WW WW   y    y
yrYYGGMMccBBrrYYGGy
",
            r"
     YYYYYY
   rrrrrrrrrr
     Wk kW
    WWWWWWWy
y    WW WW  y     y
yrYYGGMMccBBrrYYGGy
",
        ],
    )
    .speed(0.4, 0.7)
    .fps(0.15)
    .weight(4)
    .biomes(FRESH | TANK | KELP | REEF | OCEAN)
    .season(MUERTOS),
    Visitor::new(
        "marigold-raft",
        Lane::Float,
        &[
            r"
??????????????)
????.@.??.@.??|????????.@.??.@.
??(@@@)(@@@)??.---.??(@@@)(@@@)
(@*@*@*@*@*@*@*@*@*@*@*@*@*@*)
?`-.@.@.@.@.@.@.@.@.@.@.@.@.-'

",
            r"
??????????????|
????.@.??.@.??|????????.@.??.@.
??(@@@)(@@@)??.---.??(@@@)(@@@)
(@*@*@*@*@*@*@*@*@*@*@*@*@*@*)
?`-.@.@.@.@.@.@.@.@.@.@.@.@.-'

",
            r"
??????????????(
????.@.??.@.??|????????.@.??.@.
??(@@@)(@@@)??.---.??(@@@)(@@@)
(@*@*@*@*@*@*@*@*@*@*@*@*@*@*)
?`-.@.@.@.@.@.@.@.@.@.@.@.@.-'

",
        ],
        &[r"
              Y
    yYy  yYy  W        yYy  yYy
  yYYYyyYYYy  yyyyy  yYYYyyYYYy
YRYMYYYCYRYMYYYCYRYMYYYCYRYMYY
 yggYgCgRgMgYgCgRgMgYgCgRgMggy

"],
    )
    .speed(0.25, 0.4)
    .fps(0.35)
    .weight(4)
    .biomes(FRESH | TANK | KELP | REEF | OCEAN)
    .traits(GLOW)
    .season(MUERTOS),
    // The xoloitzcuintle, who guides the souls across the river to
    // Mictlán, swims by with one of them riding on its back.
    Visitor::new(
        "xolo",
        Lane::Float,
        &[
            r"
                 /\  /\
       .-.      /  \/  \__
      (o o)    |    o     `>
       )=(_____|  .____.-'
  ~~~~(_________@*@*@*/~~~~~
    ~~~  ~~~~~ ~~~ ~~~ ~~~
",
            r"
                 /\  /\
       .-.      /  \/  \__
      (o o)    |    o     `>
       )=(_____|  .____.-'
  ~~~~(_________@*@*@*/~~~~~
   ~~~ ~~~~~ ~~~ ~~~ ~~~ ~
",
        ],
        &[
            r"
                 KK  KK
       WWW      K  KK  KKK
      Wk kW    K    W     KK
       WWWWKKKKK  KKKKKKKK
  ccccKKKKKKKKKKYRYRYRKccccc
    ccc  ccccc ccc ccc ccc
",
            r"
                 KK  KK
       WWW      K  KK  KKK
      Wk kW    K    W     KK
       WWWWKKKKK  KKKKKKKK
  ccccKKKKKKKKKKYRYRYRKccccc
   ccc ccccc ccc ccc ccc c
",
        ],
    )
    .speed(0.2, 0.35)
    .fps(0.2)
    .weight(5)
    .biomes(FRESH | TANK | KELP | REEF | OCEAN)
    .season(MUERTOS),
    // La Catrina rows by in her great feathered hat.
    Visitor::new(
        "catrina-boat",
        Lane::Boat,
        &[
            r"
             ,~@*@~,
        _.-~(*@*@*@*)~-._
       (____.-------.____)
               (o.o)
          \   _/)=(\_
  \________\_/_______\______/
",
            r"
             ,~@*@~,
        _.-~(*@*@*@*)~-._
       (____.-------.____)
               (o.o)
              _/)=(\_
  \______\___/_______\______/
",
        ],
        &[
            r"
             MMYRYMM
        MMMMMRYRYRYRMMMMM
       MMMMMMMMMMMMMMMMMMM
               WKKKW
          y   mmWWWmm
  yyyyyyyyyyyyyyyyyyyyyyyyyyy
",
            r"
             MMYRYMM
        MMMMMRYRYRYRMMMMM
       MMMMMMMMMMMMMMMMMMM
               WKKKW
              mmWWWmm
  yyyyyyyyyyyyyyyyyyyyyyyyyyy
",
        ],
    )
    .speed(0.3, 0.5)
    .fps(0.15)
    .weight(5)
    .biomes(FRESH | TANK | KELP | REEF | OCEAN)
    .season(MUERTOS),
];
