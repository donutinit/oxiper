//! Swimmers that are not fish: jellyfish, cephalopods, seahorses, turtles, mammals, continued.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "hawksbill-turtle",
        &[
            r"
      ▗▄▄▄▄▄▄▄▄▄▄▖
   ▗▟█╱╲╱╲╱╲╱╲╱╲█▙▖   ▗▄▄▖
 ▗▟██╲╱╲╱╲╱╲╱╲╱╲╱██▙▄▄▟█●▜▶
▐█╱╲╱╲╱╲╱╲╱╲╱╲█▛▀▀▀▀▘
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
  ◥██▙          ◥██▙
    ▝▀           ▝▀▀
",
            r"
      ▗▄▄▄▄▄▄▄▄▄▄▖
   ▗▟█╱╲╱╲╱╲╱╲╱╲█▙▖   ▗▄▄▖
 ▗▟██╲╱╲╱╲╱╲╱╲╱╲╱██▙▄▄▟█●▜▶
▐█╱╲╱╲╱╲╱╲╱╲╱╲█▛▀▀▀▀▘
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
 ◢██◤          ◢██◤
  ▗▄           ▗▄
",
            r"
      ▗▄▄▄▄▄▄▄▄▄▄▖
   ▗▟█╱╲╱╲╱╲╱╲╱╲█▙▖   ▗▄▄▖
 ▗▟██╲╱╲╱╲╱╲╱╲╱╲╱██▙▄▄▟█●▜▶
▐█╱╲╱╲╱╲╱╲╱╲╱╲█▛▀▀▀▀▘
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
◢██◤         ◢██◤
 ▗            ▗
",
        ],
        &[
            r"
      111111111111
   111yyyyyyyyyy111   1111
 1111yyyyyyyyyyyy1111111412
11yyyyyyyyyyyy11YYYYY
 111111111111111
  1111          1111
    11           111
",
            r"
      111111111111
   111yyyyyyyyyy111   1111
 1111yyyyyyyyyyyy1111111412
11yyyyyyyyyyyy11YYYYY
 111111111111111
 1111          1111
  11           11
",
            r"
      111111111111
   111yyyyyyyyyy111   1111
 1111yyyyyyyyyyyy1111111412
11yyyyyyyyyyyy11YYYYY
 111111111111111
1111         1111
 1            1
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
   ▄▄▄▄▄▄▄
 ▄▟▓▒▓▒▓▒▓▙▄     ▄▄▄
▐▓▒▓▒▓▒▓▒▓▒▓▙▄▄▄▟█●▐▌
▝▀▀▀▀▀▀▀▀▀▀▀▀▜▛▀▀▀▀▘
   ◥█◣  ◢█◤
",
            r"
   ▄▄▄▄▄▄▄
 ▄▟▓▒▓▒▓▒▓▙▄     ▄▄▄
▐▓▒▓▒▓▒▓▒▓▒▓▙▄▄▄▟█●▐▌
▝▀▀▀▀▀▀▀▀▀▀▀▀▜▛▀▀▀▀▘
  ◢█◤     ◢█◤
",
        ],
        &[
            r"
   1111111
 111y1y1y111     111
11y1y1y1y1y111111141R
11111111111111111111
   111  111
",
            r"
   1111111
 111y1y1y111     111
11y1y1y1y1y111111141R
11111111111111111111
  111     111
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
 ▗▄▄▖      ▗▄▄▖      ▗▄▄▖
▟▌▒▐▙▄▄▄▄▄▟▌▒▐▙▄▄▄▄▄▟▌▒▐▙▄▄◖●▶
",
            r"
      ▗▄▄▖      ▗▄▄▖
▄▄▄▄▄▟▌▒▐▙▄▄▄▄▄▟▌▒▐▙▄▄▄▄▄▄▄◖●▶
",
        ],
        &[
            r"
 wwww      wwww      wwww
wwbwwwwwwwwwbwwwwwwwwwbwwwwwWR
",
            r"
      wwww      wwww
wwwwwwwbwwwwwwwwwbwwwwwwwwwwWR
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
            r"
                    ▗▄▄▄▄▖
                  ▗▟██████▙▖
        ▗▄▄▄▄▄▄▄▄▄▟███●████▙▖
     ▗▟█▪███████▪██████▪██▀◗⠒⠤
◢█▙██▪█████▪████████▪█████▛▘
◥█▙██▪████████████████████▛▀
    ▝▀██████████████████▀▀▘
                   ▝█◣
",
            r"
                    ▗▄▄▄▄▖
                  ▗▟██████▙▖
        ▗▄▄▄▄▄▄▄▄▄▟███●████▙▖
     ▗▟█▪███████▪██████▪██▀◗⠒⠤
◥█▙██▪█████▪████████▪█████▛▘
◢█▙██▪████████████████████▛▀
    ▝▀██████████████████▀▀▘
                  ◢██◤
",
        ],
        &[
            r"
                    111111
                  1111111111
        111111111111114111111
     111K1111111K111111K11WkWW
11111K11111K11111111K1111111
11111K111WWWWWWWWWWWWW111111
    11111111111111111111111
                   111
",
            r"
                    111111
                  1111111111
        111111111111114111111
     111K1111111K111111K11WkWW
11111K11111K11111111K1111111
11111K111WWWWWWWWWWWWW111111
    11111111111111111111111
                  1111
",
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
            r"
                    ▗▄▄▖
                  ▗▟████▙▖
                 ▟█▌●████▀◗⠒⠤
                  ▜████▛▘
                  ▟████▛
               ▗▟█████▛
        ▗▄▄▄▟███████████▙
◢█▙████████████████████▛▘
◥█▙▝▀████████████████████▀▀
                   ◥██▙
",
            r"
                    ▗▄▄▖
                  ▗▟████▙▖
                 ▟█▌●████▀◗⠒⠤
                  ▜████▛▘
                  ▟████▛
               ▗▟█████▛
        ▗▄▄▄▟███████████▙
◥█▙████████████████████▛▘
◢█▙▝▀████████████████████▀▀
                  ◢██◤
",
        ],
        &[
            r"
                    1111
                  11111111
                 11k41111YkWW
                  1111111
                  111111
               11111111
        11111111111111111
11111111111YYYYYYY1111111
111111111111111111111111111
                   1111
",
            r"
                    1111
                  11111111
                 11k41111YkWW
                  1111111
                  111111
               11111111
        11111111111111111
11111111111YYYYYYY1111111
111111111111111111111111111
                  1111
",
        ],
    )
    .critter()
    .traits(PREDATOR | CURIOUS)
    .tint("1:yrKy 4:W")
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
            r"
                 ▗▄▄▄▖
               ▗▟█████▙▖
    ▗▄▄▄▄▄▄▄▄▄▄▟██●██▙▀▀▶
 ▗▟█████████████████▀▘
◢█▙▄████████████████▄▟▘
 ▝▙▄      ◥███◣
          ▝▀▀
",
            r"
                 ▗▄▄▄▖
               ▗▟█████▙▖
    ▗▄▄▄▄▄▄▄▄▄▄▟██●██▙▀▀▶
 ▗▟█████████████████▀▘
◢█▙▄████████████████▄▟▘
 ▀▜▄      ◢███◤
          ▗▄▄
",
        ],
        &[
            r"


                  4   YYy
               YYYYYWW
    WWWWWWWWWWWWWWWW


",
            r"


                  4   YYy
               YYYYYWW
    WWWWWWWWWWWWWWWW


",
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
            r"
                    ▗▄▄▄▄▖
        ▗▟▀◆▀▙▖    ▗▟█●███▙▖
  ◢██▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟████▛▀◗⠒⠤
 ▝▜███████████████████▛▘
≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼
",
            r"
                    ▗▄▄▄▄▖
        ▗▟▀◆▀▙▖    ▗▟█●███▙▖
  ◢██▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟████▛▀◗⠒⠤
 ▝▜███████████████████▛▘
∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈∼≈
",
        ],
        &[
            r"
                    111111
        YYYcYYY    111411111
  111YYYYYYYYYYYYYY111111WkWW
 11111111111111111111111
cccccccccccccccccccccccccccccc
",
            r"
                    111111
        YYYcYYY    111411111
  111YYYYYYYYYYYYYY111111WkWW
 11111111111111111111111
cccccccccccccccccccccccccccccc
",
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
            r"
              ▗▄▄▄▄▄▄▖
         ▗▟███████████▙▖
     ▗▟██████▪█████▪██●██▙▖
   ▗▟█████▪██████▪██████▛██⠒⠤
◢████████▪███████████████▛
◥███████████████████████▛
   ▝▀█████████████████▀▀▘
                 ◥██▙
",
            r"
              ▗▄▄▄▄▄▄▖
         ▗▟███████████▙▖
     ▗▟██████▪█████▪██●██▙▖
   ▗▟█████▪██████▪██████▛██⠒⠤
◥████████▪███████████████▛
◢███████████████████████▛
   ▝▀█████████████████▀▀▘
                ◢██◤
",
        ],
        &[
            r"
              11111111
         111111111111111
     11111111K11111K1141111
   1111111K111111K1111111WWWW
111111111K111WWWWWWWWWW111
1111111WWWWWWWWWWWWWW1111
   1111111111111111111111
                 1111
",
            r"
              11111111
         111111111111111
     11111111K11111K1141111
   1111111K111111K1111111WWWW
111111111K111WWWWWWWWWW111
1111111WWWWWWWWWWWWWW1111
   1111111111111111111111
                1111
",
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
            r"
               ▗▄▄▄▄▄▖
◥◣        ▗▟████████▙▖
 ▜█     ▗▟█████▪███●██▙▖
 ▄▄▄▄▗▟██████▪█████████▛▀◗⠒
◢◤ ▝▙█████▪███████████▛▘
    ▝▀██████████████▀▀▘
               ◥█▙
",
            r"
               ▗▄▄▄▄▄▖
◢◣        ▗▟████████▙▖
 ▜█     ▗▟█████▪███●██▙▖
 ▄▄▄▄▗▟██████▪█████████▛▀◗⠒
◥◤ ▝▙█████▪███████████▛▘
    ▝▀██████████████▀▀▘
              ◢█◤
",
        ],
        &[
            r"
               1111111
11        111111111111
 11     1111111K11141111
 111111111111K1111111111WkW
11 1111111K1111WWWWWWW11
    1111111111111111111
               111
",
            r"
               1111111
11        111111111111
 11     1111111K11141111
 111111111111K1111111111WkW
11 1111111K1111WWWWWWW11
    1111111111111111111
              111
",
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
            r"
              ◢◣
             ◢██◣
        ▄▄▄▄▟▓▓▓▓▓▓▙▄▄▄▄
◢█▙▄▄▄▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓●▓▓▀▶
 ◥██▛░░░░░░░░░░░░░░░░░░░░▀▀▘
 ◢█▜▄▄▄▄▄▄▄▄▄▄▄▄▙▄▄▄▟▀▘
",
            r"
              ◢◣
             ◢██◣
        ▄▄▄▄▟▓▓▓▓▓▓▙▄▄▄▄
 ◥█▙▄▄▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓●▓▓▀▶
 ◢██▛░░░░░░░░░░░░░░░░░░░░▀▀▘
◥█▜▄▄▄▄▄▄▄▄▄▄▄▄▄▙▄▄▄▟▀▘
",
        ],
        &[
            r"
              11
             1111
        1111111111111111
111111111111111111111114111k
 1111WWWWWWWWWWWWWWWWWWWW111
 1111111111111111111111
",
            r"
              11
             1111
        1111111111111111
 11111111111111111111114111k
 1111WWWWWWWWWWWWWWWWWWWW111
11111111111111111111111
",
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
            r"
                     ▄▄▄▄
                  ▄▟▓▓▓▓▓▙▄
          ▄▄▄▄▄▟▓▓▓▓▓▓▓▓▓▓▙
  ▄▄▄▄▟▓▓▓▓▓▓▓▓▓▓▓▓●▓▓▓▓▓▓◗
◢█▙▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓‿▓▓▓▀
 ◥██▀▀▀▀▜▙▓▓▓▓▓▓▓▓▓▓▓▓▛▀
  ◥◤       ▝▀▀▙▖▗▟▀▘
",
            r"
                     ▄▄▄▄
                  ▄▟▓▓▓▓▓▙▄
          ▄▄▄▄▄▟▓▓▓▓▓▓▓▓▓▓▙
  ▄▄▄▄▟▓▓▓▓▓▓▓▓▓▓▓▓●▓▓▓▓▓▓◗
◥█▙▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓‿▓▓▓▀
 ◢██▀▀▀▀▜▙▓▓▓▓▓▓▓▓▓▓▓▓▛▀
  ◢◣       ▝▀▀▙▖ ▗▟▀▘
",
        ],
        &[
            r"



                   4      k
                      k


",
            r"



                   4      k
                      k


",
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
            r"
                 ▗▄▄▄▄▄▄▖
             ▗▟████████▙▖
          ▗▟█▪███●█▪███▙▖
       ▗▟████▪███▐███████▌
   ▗▟█████████████▪██┃█┃▌
 ▟███████████▪███████┃▀┃
▐████████████████████╹█╹
◢█▙█████████████████▀▀▘
  ▝▀▀▀▀▀▀▀    ◥██◣
",
            r"
                 ▗▄▄▄▄▄▄▖
             ▗▟████████▙▖
          ▗▟█▪███●█▪███▙▖
       ▗▟████▪███▐███████▌
   ▗▟█████████████▪██┃█┃▌
 ▟███████████▪███████┃▀┃
▐████████████████████╹█╹
◥█▙█████████████████▀▀▘
  ▝▀▀▀▀▀▀▀   ◢██◤
",
        ],
        &[
            r"
                 11111111
             111111111111
          111K11141K11111
       111111K1111YYYYYYY1
   111111111111111K11W1W1
 111111111111K1111111W1W
1111111WWWWWWWWWWWWWWW1W
11111111111111111111111
  11111111    1111
",
            r"
                 11111111
             111111111111
          111K11141K11111
       111111K1111YYYYYYY1
   111111111111111K11W1W1
 111111111111K1111111W1W
1111111WWWWWWWWWWWWWWW1W
11111111111111111111111
  11111111   1111
",
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
               ╲╲│╱╱
             ▗▄▄▄▟▓▓▓▙▄▖
  ▄▄▄▄▄▄▄▄▟▓▓▓▓▓▓●▓▓▓◗
▗▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▀
 ▝▀▀▀▀▜▙▄▄▄▄▄▄▄▛▀▀▀▘
     ▝▙▖   ▗▟▘
",
            r"
               ╱╱│╲╲
             ▗▄▄▄▟▓▓▓▙▄▖
  ▄▄▄▄▄▄▄▄▟▓▓▓▓▓▓●▓▓▓◗
▗▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▀
 ▝▀▀▀▀▜▙▄▄▄▄▄▄▄▛▀▀▀▘
      ▝▙▖  ▗▟▘
",
        ],
        &[
            r"
               RRRRR
             11111111111
  111111111111111k1111
111111111111111111111
 1111111111111111111
     111   111
",
            r"
               RRRRR
             11111111111
  111111111111111k1111
111111111111111111111
 1111111111111111111
      111  111
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
   ▗▄▖    ▗▄▖
  ▟◉█▙▄▄▟█◉▙
 ▟▓▓▓▓▓▓▓▓▓▓▓▙
 ▜▓▓▓▀▀▀▀▀▓▓▓▛
◢█▛░░░░░░░░░▜█◣
◥◣ ▝▀▙   ▟▀▘ ◢◤
",
            r"
   ▗▄▖    ▗▄▖
  ▟◉█▙▄▄▟█◉▙
 ▟▓▓▓▓▓▓▓▓▓▓▓▙
 ▜▓▓▓▀▀▀▀▀▓▓▓▛
◢█▛░░░░░░░░░▜█◣
  ◥◣▝▀▙▟▀▘◢◤
",
        ],
        &[
            r"
   111    111
  1411111141
 1111111111111
 1111kkkkk1111
111yyyyyyyyy111
11 1k1   1k1 11
",
            r"
   111    111
  1411111141
 1111111111111
 1111kkkkk1111
111yyyyyyyyy111
  111k11k111
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
            r"
                    ▗▄▄▄▄▖
                  ▗▟█●████▙▖
      ▗▄▄▄▄▄▄▄▄▄▄▄▟█████▛▀◗⠒⠤
▄▄▗▟████████████████████▛
 ▝▀▀▀▙████████████▀▀▘
                 ▗▟
",
            r"
                    ▗▄▄▄▄▖
                  ▗▟█●████▙▖
      ▗▄▄▄▄▄▄▄▄▄▄▄▟█████▛▀◗⠒⠤
◢▀▗▟████████████████████▛
 ◥▀▀▀▙████████████▀▀▘
                ▝▙
",
        ],
        &[
            r"
                    111111
                  1114111111
      1111111111111111111WkWW
11111111111111WWWWWWWW111
 11111111111111111111
                 11
",
            r"
                    111111
                  1114111111
      1111111111111111111WkWW
11111111111111WWWWWWWW111
 11111111111111111111
                11
",
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
            r"
                  ▗▄▄▄▄▖
         ▗▄▄▄▄▄▟██████▙▖
    ▗▟█████▪████▪███●██▙▄▄∙▄▖
  ▗▟█████▪███████▪█████▛▀▀▀▀▘
 ▟██████████████████████▛▘
▐█████▌▜▙██████████████▀▀
▝▀▀▀▀▀▘        ◥█◣
",
            r"
                  ▗▄▄▄▄▖
         ▗▄▄▄▄▄▟██████▙▖
    ▗▟█████▪████▪███●██▙▄▄∙▄▖
  ▗▟█████▪███████▪█████▛▀▀▀▀▘
 ▟██████████████████████▛▘
▐█████▌▜▙██████████████▀▀
 ▝▀▀▀▀▘        ◢█◤
",
        ],
        &[
            r"
                  111111
         111111111111111
    1111111K1111K1114111kkKkk
  1111111K1111111K111111kkkkk
 111111111WWWWWWWWWWWW1111
kkkkkkk111111111111111111
kkkkkkk        111
",
            r"
                  111111
         111111111111111
    1111111K1111K1114111kkKkk
  1111111K1111111K111111kkkkk
 111111111WWWWWWWWWWWW1111
kkkkkkk111111111111111111
               111
",
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
   ▗▄▄▄▄▄▄▄▖
 ▗▟▓▒▒▒▒▒▒▒▓█▙▄
▐▓▓▒▓▓▓▓▓▓▓▓▓█●▶
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
  ╲╲  ╲╲  ╲╲
",
            r"
   ▗▄▄▄▄▄▄▄▖
 ▗▟▓▒▒▒▒▒▒▒▓█▙▄
▐▓▓▒▓▓▓▓▓▓▓▓▓█●▶
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
  ╱╱  ╱╱  ╱╱
",
        ],
        &[
            r"
   KKKKKKKKK
 KKKyyyyyyyKKKK
KKKyKKKKKKKKKKWy
 KKKKKKKKKKKKKKK
  ww  ww  ww
",
            r"
   KKKKKKKKK
 KKKyyyyyyyKKKK
KKKyKKKKKKKKKKWy
 KKKKKKKKKKKKKKK
  ww  ww  ww
",
        ],
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
 ▗▄▖
▗▟◉▜▶
 ▐◆▌
 ▐█▘
 ▝◖
",
            r"
 ▗▄▖
▗▟◉▜▶
 ▐◆▌
 ▐█▘
  ◗▘
",
        ],
        &[
            r"
 111
11W1y
 1y1
 111
 11
",
            r"
 111
11W1y
 1y1
 111
  11
",
        ],
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
   ▗▄▄▄▄▄▄▄▖
 ▗▟▓▓▒▓▓▒▓▓▓▙▖
▟▓▓▒▓▓▓▓▒▓▓▓▓▓▙
▜▀▀▀▀▀▀▀▀▀▀▀▀▀▛
 ┃ ╎ ┃ ╎ ┃ ╎ ┃
 ╎ ┃ ╎ ┃ ╎ ┃ ╎
 ┃ ╎ ┃ ╎ ┃ ╎ ┃
  ╎ ┃  ╎ ┃ ╎
   ┃   ╎
",
            r"
   ▗▄▄▄▄▄▄▄▖
 ▗▟▓▓▒▓▓▒▓▓▓▙▖
▟▓▓▒▓▓▓▓▒▓▓▓▓▓▙
▜▀▀▀▀▀▀▀▀▀▀▀▀▀▛
 ╎ ┃ ╎ ┃ ╎ ┃ ╎
 ┃ ╎ ┃ ╎ ┃ ╎ ┃
 ╎ ┃ ╎ ┃ ╎ ┃ ╎
  ┃ ╎  ┃ ╎ ┃
   ╎   ┃
",
        ],
        &[
            r"
   111111111
 1111W11W11111
111W1111W111111
111111111111111
 2 2 2 2 2 2 2
 2 2 2 2 2 2 2
 2 2 2 2 2 2 2
  2 2  2 2 2
   2   2
",
            r"
   111111111
 1111W11W11111
111W1111W111111
111111111111111
 2 2 2 2 2 2 2
 2 2 2 2 2 2 2
 2 2 2 2 2 2 2
  2 2  2 2 2
   2   2
",
        ],
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
   ▗▄▄▄▄▄▖
 ▗▟░░∙░∙░░▙▖
▟░░∙░░░░∙░░▙
▜▀▀▀▀▀▀▀▀▀▀▀▛
  ╎ ┃ ╎ ┃ ╎
  ┃ ╎ ┃ ╎ ┃
   ╎ ┃ ╎ ┃
",
            r"
   ▗▄▄▄▄▄▖
 ▗▟░░∙░∙░░▙▖
▟░░∙░░░░∙░░▙
▜▀▀▀▀▀▀▀▀▀▀▀▛
  ┃ ╎ ┃ ╎ ┃
  ╎ ┃ ╎ ┃ ╎
   ┃ ╎ ┃ ╎
",
        ],
        &[
            r"
   1111111
 1111C1C1111
111C1111C111
1CCCCCCCCCCC1
  1 1 1 1 1
  1 1 1 1 1
   1 1 1 1
",
            r"
   1111111
 1111C1C1111
111C1111C111
1CCCCCCCCCCC1
  1 1 1 1 1
  1 1 1 1 1
   1 1 1 1
",
        ],
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
   ╭─╮   ▗▄▄▖
───╯ ╰──▟█●█▙
",
            r"
╭─╮  ╭─╮ ▗▄▄▖
╯ ╰──╯ ╰▟█●█▙
",
        ],
        &[
            r"
   111   1111
1111111111W11
",
            r"
111  111 1111
1111111111W11
",
        ],
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
       ▗▄▄▄▖
 ▗▄▄▄▄▟▓▓▓▓●▓◗
▟▓▓▓▓▓▓▓▓▓▓▓▓▀
◥▙▒▒▒▒▒▒▒▒▒▒▛▘
  ╱╲   ╱╲
",
            r"
       ▗▄▄▄▖
 ▗▄▄▄▄▟▓▓▓▓●▓◗
▟▓▓▓▓▓▓▓▓▓▓▓▓▀
◥▙▒▒▒▒▒▒▒▒▒▒▛▘
  ╲╱   ╲╱
",
        ],
        &[
            r"
       11111
 1111111111W11
11111111111111
11RRRRRRRRRR11
  11   11
",
            r"
       11111
 1111111111W11
11111111111111
11RRRRRRRRRR11
  11   11
",
        ],
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
            r"
               ▗▄▄▄▄▄▖
         ▗▄▄▄▄▟██████▙▖
     ▗▟██████▪████●███◗⠒⠤
   ▗▟█████▪██████████▙▀
  ▟██████████████████▌
  ▜██████████████████▛
◢╱╲╱╲╱▜▙██████████▀▀▘▝▙
▐╱╲╱╲╱╲╱▌
◥╲╱╲╱╲╱◤
 ▀▀▀▀▀▀
",
            r"
               ▗▄▄▄▄▄▖
         ▗▄▄▄▄▟██████▙▖
     ▗▟██████▪████●███◗⠒⠤
   ▗▟█████▪██████████▙▀
  ▟██████████████████▌
  ▜██████████████████▛
◢╲╱╲╱╲▜▙██████████▀▀▘▗▟
▐╲╱╲╱╲╱╲▌
◥╱╲╱╲╱╲◤
 ▀▀▀▀▀▀
",
        ],
        &[
            r"
               1111111
         11111111111111
     11111111K11114111kWW
   1111111K11111111111W
  11111111111111111111
  111111WWWWWWWWW11111
kkkkkk11111111111111111
kkkkkkkkk
kkkkkkkk
 kkkkkk
",
            r"
               1111111
         11111111111111
     11111111K11114111kWW
   1111111K11111111111W
  11111111111111111111
  111111WWWWWWWWW11111
kkkkkk11111111111111111
kkkkkkkkk
kkkkkkkk
 kkkkkk
",
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
            r"
          ▗▄▄▖
        ▗▟████▙▖
 ▗▄▄▄▄▄▟█◖●◗█▙▀▶
▗▟█████████████▀▘
◢█▙▄███████████▄▟▘
 ▝▙▄     ◥███◣
         ▝▀▀
",
            r"
          ▗▄▄▖
        ▗▟████▙▖
 ▗▄▄▄▄▄▟█◖●◗█▙▀▶
▗▟█████████████▀▘
◢█▙▄███████████▄▟▘
 ▀▜▄     ◢███◤
         ▗▄▄
",
        ],
        &[
            r"


         WkW  Rr
               WW
    WWWWWWWWWWW


",
            r"


         WkW  Rr
               WW
    WWWWWWWWWWW


",
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
      ▗▄▄▄▄▄▄▄▄▄▄▄▄▖
   ▗▟█◢◣◢◣◢◣◢◣◢◣█▙▖   ▗▄▄▖
 ▗▟██◥◤◥◤◥◤◥◤◥◤◥◤██▙▄▄▟█●█▌
▐█◢◣◢◣◢◣◢◣◢◣◢◣█▛▀▀▀▀▘
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
 ◥███▙        ◥███▙
   ▝▀▀            ▝▀▀
",
            r"
      ▗▄▄▄▄▄▄▄▄▄▄▄▄▖
   ▗▟█◢◣◢◣◢◣◢◣◢◣█▙▖   ▗▄▄▖
 ▗▟██◥◤◥◤◥◤◥◤◥◤◥◤██▙▄▄▟█●█▌
▐█◢◣◢◣◢◣◢◣◢◣◢◣█▛▀▀▀▀▘
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
 ◢███◤        ◢███◤
  ▗▄▄           ▗▄▄
",
            r"
      ▗▄▄▄▄▄▄▄▄▄▄▄▄▖
   ▗▟█◢◣◢◣◢◣◢◣◢◣█▙▖   ▗▄▄▖
 ▗▟██◥◤◥◤◥◤◥◤◥◤◥◤██▙▄▄▟█●█▌
▐█◢◣◢◣◢◣◢◣◢◣◢◣█▛▀▀▀▀▘
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
◢███◤       ◢███◤
 ▗▄           ▗▄
",
        ],
        &[
            r"
      11111111111111
   111gGgGgGgGgG111   1111
 1111gGgGgGgGgGgG1111111411
11gGgGgGgGgGgG11YYYYY
 111111111111111
 11111        11111
   111            111
",
            r"
      11111111111111
   111gGgGgGgGgG111   1111
 1111gGgGgGgGgGgG1111111411
11gGgGgGgGgGgG11YYYYY
 111111111111111
 11111        11111
  111           111
",
            r"
      11111111111111
   111gGgGgGgGgG111   1111
 1111gGgGgGgGgGgG1111111411
11gGgGgGgGgGgG11YYYYY
 111111111111111
11111       11111
 11           11
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
      ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
  ▗▟█┃█┃█┃█┃█┃█┃█┃██▙▖   ▗▄▄▖
▗▟██┃█┃█┃█┃█┃█┃█┃███▙▄▄▟█●▜▌
▝▜█┃█┃█┃█┃█┃█┃█┃██▛▀▀▀▘
  ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
    ◥█▙       ◥████▙
      ▝▀         ▝▀▀▀
",
            r"
      ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
  ▗▟█┃█┃█┃█┃█┃█┃█┃██▙▖   ▗▄▄▖
▗▟██┃█┃█┃█┃█┃█┃█┃███▙▄▄▟█●▜▌
▝▜█┃█┃█┃█┃█┃█┃█┃██▛▀▀▀▘
  ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
   ◢█◤       ◢████◤
    ▗▄        ▗▄▄▄
",
        ],
        &[
            r"
      1111111111111111
  111w1w1w1w1w1w1w1111   1111
1111w1w1w1w1w1w1w11111111412
111w1w1w1w1w1w1w111WWWW
  111111111111111
    111       111111
      11         1111
",
            r"
      1111111111111111
  111w1w1w1w1w1w1w1111   1111
1111w1w1w1w1w1w1w11111111412
111w1w1w1w1w1w1w111WWWW
  111111111111111
   111       111111
    11        1111
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

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_species_drawn(super::SPECIES);
    }
}
