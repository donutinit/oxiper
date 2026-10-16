//! Freshwater fish: tank favorites and the fish of rivers and lakes, continued.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "german-blue-ram",
        &[
            r"
   ▲▲▲▙▖
  ▗▟▓▓▓▙▖
◤▄█▓▒▒▒▒◉▌
◀█▒▒▒▒▒▒▛▘
◣▀▜▒▒▒▒▛▘
   ▝▀▀▘
",
            r"
   ▲▲▲▙▖
  ▗▟▓▓▓▙▖
◢▄█▓▒▒▒▒◉▌
◀█▒▒▒▒▒▒▛▘
◥▀▜▒▒▒▒▛▘
   ▝▀▀▘
",
        ],
        &[r"
   22222
  1111111
2211111141
2211111111
221111111
   2222
"],
    )
    .tint("1:yY 2:bB")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.25, 0.6)
    .weight(3)
    .traits(SHY)
    .home("log")
    .biomes(TANK | FRESH),
    Species::new(
        "electric-blue-acara",
        &[
            r"
  ◢▄▄▄▄▄▄▄▄▖
  ▗▟▓▓▓▓▓▓▓▓▓▙▖
◤▄█▓▒◇▒▒◇▒▒▒▒◉▌
◀█▒▒▒▒▒▒▒▒▒▒▒▛▘
◣▀▜▒░░░░░░░░▒▛▘
   ▝▀▀▀▀▀▀▀▘
",
            r"
  ◢▄▄▄▄▄▄▄▄▖
  ▗▟▓▓▓▓▓▓▓▓▓▙▖
◢▄█▓▒◇▒▒◇▒▒▒▒◉▌
◀█▒▒▒▒▒▒▒▒▒▒▒▛▘
◥▀▜▒░░░░░░░░▒▛▘
   ▝▀▀▀▀▀▀▀▘
",
        ],
        &[r"
  BBBBBBBBBB
  BBBBBBBBBBBBB
BBBBBcBBcBBBB4B
BBBBBBBBBBBBBBB
BBBBBBBBBBBBBBB
   BBBBBBBBB
"],
    )
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .traits(CURIOUS)
    .biomes(TANK | FRESH),
    Species::new(
        "flowerhorn",
        &[
            r"
           ▗▄██▙▖
         ▗▄█████▙▖
    ▗▄▟████████████▙▖
◤▄█▓▒▒█◆██◆██◆█████◉▌
◀█▒▒▒▒██████████████▌
◣▀▜▒▒▒▒██████████▛▘
     ▝▀▀▀▀▀▀▀▀▘
",
            r"
           ▗▄██▙▖
         ▗▄█████▙▖
    ▗▄▟████████████▙▖
◢▄█▓▒▒█◆██◆██◆█████◉▌
◀█▒▒▒▒██████████████▌
◥▀▜▒▒▒▒██████████▛▘
     ▝▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
           222222
         222222222
    11111122222222222
1111111K11K11K1111141
111111111111111111111
1111111111111111111
     1111111111
"],
    )
    .tint("1:rRmM 2:rRyY")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(3)
    .traits(PREDATOR | CURIOUS)
    .biomes(TANK | FRESH),
    Species::new(
        "tilapia",
        &[
            r"
    ▲▲▲▲▲▲▲▲▲▲▲▲
   ▗▟▓▒▓▒▓▒▓▒▓▒▓▒▙▖
◤▄█▓▒▓▒▓▒▓▒▓▒▓▒▓▒◉▌
 █▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▒▌
◣▀▜░░░░░░░░░░░░░░▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▘
",
            r"
    ▲▲▲▲▲▲▲▲▲▲▲▲
   ▗▟▓▒▓▒▓▒▓▒▓▒▓▒▙▖
◢▄█▓▒▓▒▓▒▓▒▓▒▓▒▓▒◉▌
◀█▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▒▌
◥▀▜░░░░░░░░░░░░░░▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
    222222222222
   1111111111111111
2211111111111111141
 211111111111111111
2211111111111111111
    222222222222
"],
    )
    .tint("1:wWgG 2:gG")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.3, 1.0)
    .weight(5)
    .traits(GRAZER)
    .biomes(FRESH),
    Species::new(
        "dwarf-gourami",
        &[
            r"
    ▗▄▄▄▖
  ▗▟▓▒▓▒▓▙▖
◤▄█▓▒▓▒▓▒◉▌
◀█▒▓▒▓▒▓▒▛▘
◣▀▜▓▒▓▒▓▛▘
    ▝▀▀▀▘
      │╱
      │
",
            r"
    ▗▄▄▄▖
  ▗▟▓▒▓▒▓▙▖
◢▄█▓▒▓▒▓▒◉▌
◀█▒▓▒▓▒▓▒▛▘
◥▀▜▓▒▓▒▓▛▘
    ▝▀▀▀▘
      ╲│
      │
",
        ],
        &[r"
    22222
  111111111
22111111141
22111111111
2211111111
    22222
      22
      2
"],
    )
    .tint("1:bB 2:rR")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(5)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "pearl-gourami",
        &[
            r"
    ▗▄▄▄▄▄▖
  ▗▟▓▓▓▓▓▓▙▖
◤▄█▓◦▓◦▓◦▓◉▌
◀█▓▓◦▓◦▓▓▓▛▘
◣▀▜▓▓▓▓▓▓▓▛▘
   ▝◥▀▀▀▀▀◤▘
    ╱╱│
",
            r"
    ▗▄▄▄▄▄▖
  ▗▟▓▓▓▓▓▓▙▖
◢▄█▓◦▓◦▓◦▓◉▌
◀█▓▓◦▓◦▓▓▓▛▘
◥▀▜▓▓▓▓▓▓▓▛▘
   ▝◥▀▀▀▀▀◤▘
    │╱╱
",
        ],
        &[r"
    1111111
  1111111111
1111W1W1W141
1111W1W11111
111111111111
   111111111
    111
"],
    )
    .tint("1:yYrR")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(4)
    .traits(SHY)
    .home("lily")
    .biomes(TANK | FRESH),
    Species::new(
        "kuhli-loach",
        &[
            r"
◢▄ ▄▄▄     ▄▄▄     ▄▄▄
◥▄▀   ▀▄▄▄▀   ▀▄▄▄▀   ▀◉▌
",
            r"
◢▄     ▄▄▄     ▄▄▄     ▄▄▄
◥▄▀▄▄▄▀   ▀▄▄▄▀   ▀▄▄▄▀◉▌
",
        ],
        &[r"
yyyKKKyyyyyKKKyyyyyKKK
yyyyyyKKKKKyyyKKKKKyyyy4y
"],
    )
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.25, 0.8)
    .school(2, 5)
    .weight(5)
    .traits(SHY | NOCTURNAL | SCAVENGER)
    .home("log")
    .biomes(TANK | FRESH),
    Species::new(
        "clown-loach",
        &[
            r"
    ▗▄▄▄▄▄▄▖
  ▗▟▓▓█▓▓█▓▓█▓▙▖
◤▄▓▓▓▓█▓▓█▓▓█▓◉▌
 █▒▒▒▒█▒▒█▒▒█▒▛▘⠒
◣▀▜▒▒▒█▒▒█▒▒█▒▘
    ▝▀▀▀▀▀▀▀▀▘
",
            r"
    ▗▄▄▄▄▄▄▖
  ▗▟▓▓█▓▓█▓▓█▓▙▖
◢▄▓▓▓▓█▓▓█▓▓█▓◉▌
◀█▒▒▒▒█▒▒█▒▒█▒▛▘⠐
◥▀▜▒▒▒█▒▒█▒▒█▒▘
    ▝▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
    11111111
  1111K11K11K111
111111K11K11K141
 11111K11K11K111w
111111K11K11K11
    1111111111
"],
    )
    .tint("1:yY")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .school(2, 4)
    .weight(5)
    .traits(SCAVENGER | NOCTURNAL)
    .home("log")
    .biomes(TANK | FRESH),
    Species::new(
        "silver-arowana",
        &[
            r"
          ▗▄▄▄▄▄▄▄▄▖
   ▗▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▙▖
◤▄█◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒◉▌▘
◀█◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒▛▀▀▘⠒
◣▀▜░░░░░░░░░░░░░░░░░░▀▘
    ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
          ▗▄▄▄▄▄▄▄▄▖
   ▗▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▙▖
◢▄█◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒◉▌▘
◀█◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒▛▀▀▘⠐
◥▀▜░░░░░░░░░░░░░░░░░░▀▘
    ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
          1111111111
   11111111111111111111
111W1W1W1W1W1W1W1W1W1411
11W1W1W1W1W1W1W1W1W1111w
111wwwwwwwwwwwwwwwwww11
    1111111111111111
"],
    )
    .tint("1:wW")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.4, 1.2)
    .weight(3)
    .traits(PREDATOR | CURIOUS)
    .biomes(TANK | FRESH),
    Species::new(
        "red-bellied-piranha",
        &[
            r"
      ▗▄▄▄▄▄▄▖
   ▗▟████████████▙▖
◤▄█▓▒███████████◉█▌▼
 █▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▛▲▲▜
◣▀▜▒▒▒▒▒▒▒▒▒▒▒▒▒▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▘
",
            r"
      ▗▄▄▄▄▄▄▖
   ▗▟████████████▙▖
◢▄█▓▒███████████◉█▌▼
◀█▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▛▲▲▜
◥▀▜▒▒▒▒▒▒▒▒▒▒▒▒▒▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
      11111111
   1111111111111111
111111111111111141W
 1RRRRRRRRRRRRRRRR1WW1
111RRRRRRRRRRRRR11
    RRRRRRRRRRRR
"],
    )
    .tint("1:wWK")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.4, 1.6)
    .school(1, 5)
    .weight(4)
    .traits(PREDATOR | SCAVENGER)
    .biomes(TANK | FRESH),
    Species::new(
        "pacu",
        &[
            r"
      ▗▄▄▄▄▖
    ▗▟▓▓▓▓▓▓▙▖
◤▄▟▓▒▒▒▒▒▒▒▒◉▌
 █▒▒▒▒▒▒▒▒▒▒▒▌
◣▀▜▒▒▒▒▒▒▒▒▒▛▘
    ▝▜▀▀▀▀▀▛▘
",
            r"
      ▗▄▄▄▄▖
    ▗▟▓▓▓▓▓▓▙▖
◢▄▟▓▒▒▒▒▒▒▒▒◉▌
◀█▒▒▒▒▒▒▒▒▒▒▒▌
◥▀▜▒▒▒▒▒▒▒▒▒▛▘
    ▝▜▀▀▀▀▀▛▘
",
        ],
        &[r"
      111111
    1111111111
11111111111141
 1222222222221
11122222222211
    222222222
"],
    )
    .tint("1:wWyY 2:yY")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.25, 0.9)
    .weight(3)
    .traits(GRAZER)
    .biomes(TANK | FRESH),
    Species::new(
        "rainbow-trout",
        &[
            r"
   ▗▖    ▗▄▄▄▖
◤▄▟▓∙▓∙▓▓∙▓∙▓▓▙▖
 █▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉▌
◣▀▜░░░░░░░░░░░░▛▘
    ▝▀▘  ▝▀▀▘
",
            r"
   ▗▖    ▗▄▄▄▖
◢▄▟▓∙▓∙▓▓∙▓∙▓▓▙▖
◀█▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉▌
◥▀▜░░░░░░░░░░░░▛▘
    ▝▀▘  ▝▀▀▘
",
        ],
        &[r"
   11    11111
1111K1K11K1K1111
 1RRRRRRRRRRRRRR41
111wwwwwwwwwwww11
    111  1111
"],
    )
    .tint("1:gGyY")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.7, 1.8)
    .weight(6)
    .biomes(FRESH),
    Species::new(
        "brown-trout",
        &[
            r"
   ▗▖    ▗▄▄▄▄▖
◤▄▟▓◦▓•▓◦▓•▓◦▓▓▙▖
 █◦▒•▒◦▒•▒◦▒▒▒▒◉▌
◣▀▜░░░░░░░░░░░░▛▘
    ▝▀▘  ▝▀▀▘
",
            r"
   ▗▖    ▗▄▄▄▄▖
◢▄▟▓◦▓•▓◦▓•▓◦▓▓▙▖
◀█◦▒•▒◦▒•▒◦▒▒▒▒◉▌
◥▀▜░░░░░░░░░░░░▛▘
    ▝▀▘  ▝▀▀▘
",
        ],
        &[r"
   11    111111
1111r1r1r1r1r111
 1r1r1r1r1r111141
111wwwwwwwwwwww11
    111  1111
"],
    )
    .tint("1:yY")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.5, 1.5)
    .weight(5)
    .traits(PREDATOR)
    .biomes(FRESH),
    Species::new(
        "atlantic-salmon",
        &[
            r"
   ▗▖     ▗▄▄▄▄▄▖
◤▄▟▓▓×▓▓×▓▓×▓▓▓▓▓▙▖
 █▒▒×▒▒×▒▒×▒▒▒▒▒▒▒◉▌
◣▀▜░░░░░░░░░░░░░░▛▘
    ▝▀▘  ▝▀▀▀▘
",
            r"
   ▗▖     ▗▄▄▄▄▄▖
◢▄▟▓▓×▓▓×▓▓×▓▓▓▓▓▙▖
◀█▒▒×▒▒×▒▒×▒▒▒▒▒▒▒◉▌
◥▀▜░░░░░░░░░░░░░░▛▘
    ▝▀▘  ▝▀▀▀▘
",
        ],
        &[r"
   11     1111111
11111K11K11K1111111
 111K11K11K111111141
111wwwwwwwwwwwwww11
    111  1111
"],
    )
    .tint("1:wW")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.8, 1.8)
    .weight(4)
    .biomes(FRESH | OCEAN | KELP),
    Species::new(
        "sockeye-salmon",
        &[
            r"
   ▗▖    ▗▄▄▄▄▄▖
◤▄▟▓▓▓▓▓▓▓▓▓▓▓▓▙▖
 █▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉█▀▜
◣▀▜▒▒▒▒▒▒▒▒▒▒▒▒▒▛▀▘
    ▝▀▘  ▝▀▀▀▘
",
            r"
   ▗▖    ▗▄▄▄▄▄▖
◢▄▟▓▓▓▓▓▓▓▓▓▓▓▓▙▖
◀█▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉█▀▜
◥▀▜▒▒▒▒▒▒▒▒▒▒▒▒▒▛▀▘
    ▝▀▘  ▝▀▀▀▘
",
        ],
        &[r"
   11    1111111
1111111111GGGGGGG
 1111111111GGGGG4GGG
111111111GGGGGGGGGG
    111  1111
"],
    )
    .tint("1:rR")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.8, 1.8)
    .weight(3)
    .biomes(FRESH | OCEAN | KELP),
    Species::new(
        "common-carp",
        &[
            r"
      ▗▄▄▄▄▄▄▄▄▄▄▖
   ▗▟▓◇▓◇▓◇▓◇▓◇▓◇▓◇▓▙▖
◤▄█▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒▒◉▌⠈
 █◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒▒▒▛▘⠒
◣▀▜░◇░◇░◇░◇░◇░◇░◇░░▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
      ▗▄▄▄▄▄▄▄▄▄▄▖
   ▗▟▓◇▓◇▓◇▓◇▓◇▓◇▓◇▓▙▖
◢▄█▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒▒◉▌⠁
◀█◇▒◇▒◇▒◇▒◇▒◇▒◇▒◇▒▒▒▛▘⠐
◥▀▜░◇░◇░◇░◇░◇░◇░◇░░▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
      111111111111
   11111y1y1y1y1y1y11
1111y1y1y1y1y1y1y1141w
 11y1y1y1y1y1y1y1y1111w
1111w1w1w1w1w1w1w1111
    1111111111111111
"],
    )
    .tint("1:yYwW")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .traits(SCAVENGER | GRAZER)
    .biomes(FRESH),
    Species::new(
        "channel-catfish",
        &[
            r"
        ▗▄▄▖
   ▗▟▓▓▓▓▓▓▓▓▓▓▓▙▄▄▖
◤▄█▒▒∙▒▒∙▒▒∙▒▒▒▒▒▒◉█▌─╌
 █░░░░░░░░░░░░░░░░▛▀▀▘╌
◣▀▜▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
        ▗▄▄▖
   ▗▟▓▓▓▓▓▓▓▓▓▓▓▙▄▄▖
◢▄█▒▒∙▒▒∙▒▒∙▒▒▒▒▒▒◉█▌╌─
◀█░░░░░░░░░░░░░░░░▛▀▀▘─
◥▀▜▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
        1111
   11111111111111111
11111K11K11K111111411ww
 1wwwwwwwwwwwwwwww1111w
111wwwwwwwwwwwwwww
"],
    )
    .tint("1:wWK")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.3, 0.9)
    .weight(5)
    .traits(SCAVENGER | NOCTURNAL)
    .home("log,boulder")
    .biomes(FRESH),
    Species::new(
        "northern-pike",
        &[
            r"
    ▗▄▄▄▄▖
◤▄▟▓◦▓◦▓◦▓◦▓◦▓◦▓◦▓◦▓◦▓◦▓▓▙▖
 █▒◦▒◦▒◦▒◦▒◦▒◦▒◦▒◦▒◦▒▒▒◉▌▀▀▀▀▀
◣▀▜░░░░░░░░░░░░░░░░░░░░▛▄▄▄▄▄▄
    ▝▀▀▘
",
            r"
    ▗▄▄▄▄▖
◢▄▟▓◦▓◦▓◦▓◦▓◦▓◦▓◦▓◦▓◦▓◦▓▓▙▖
◀█▒◦▒◦▒◦▒◦▒◦▒◦▒◦▒◦▒◦▒▒▒◉▌▀▀▀▀▀
◥▀▜░░░░░░░░░░░░░░░░░░░░▛▄▄▄▄▄▄
    ▝▀▀▘
",
        ],
        &[r"
    111111
1111y1y1y1y1y1y1y1y1y1111
 11y1y1y1y1y1y1y1y1y111411111
111wwwwwwwwwwwwwwwwwwww1111111
    1111
"],
    )
    .tint("1:gG")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.3, 2.0)
    .weight(3)
    .traits(PREDATOR)
    .biomes(FRESH),
    Species::new(
        "largemouth-bass",
        &[
            r"
   ▲▲▲▲▲▲▲ ▗▄▄▖
  ▗▟▓▓▓▓▓▓▓▓▓▓▙▖
◤▄█▓▓▓▓▓▓▓▓▓▓▓◉▌▀▀▀▀
 █▒▒▒▒▒▒▒▒▒▒▒▒▒▌
◣▀▜░░░░░░░░░░░░▛▄▄▄▄
    ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
   ▲▲▲▲▲▲▲ ▗▄▄▖
  ▗▟▓▓▓▓▓▓▓▓▓▓▙▖
◢▄█▓▓▓▓▓▓▓▓▓▓▓◉▌▀▀▀▀
◀█▒▒▒▒▒▒▒▒▒▒▒▒▒▌
◥▀▜░░░░░░░░░░░░▛▄▄▄▄
    ▝▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
   1111111 1111
  11111111111111
1111111111111141111
 1KKKKKKKKKKKKK1
111wwwwwwwwwwww11111
    wwwwwwwwwww
"],
    )
    .tint("1:gG")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.1)
    .weight(5)
    .traits(PREDATOR)
    .biomes(FRESH),
    Species::new(
        "bluegill",
        &[
            r"
    ▲▲▲▲▲▲▖▗▄▖
   ▗▟▓▓▓▓▓▓▓▓▙▖
◤▄▟▓▒▒▒▒▒▒▒█▓◉▌
 █▒▒▒▒▒▒▒▒▒█▓▓▌
◣▀▜▒▒▒▒▒▒▒▒▒▒▛▘
   ▝▜▀▀▀▀▀▀▛▘
",
            r"
    ▲▲▲▲▲▲▖▗▄▖
   ▗▟▓▓▓▓▓▓▓▓▙▖
◢▄▟▓▒▒▒▒▒▒▒█▓◉▌
◀█▒▒▒▒▒▒▒▒▒█▓▓▌
◥▀▜▒▒▒▒▒▒▒▒▒▒▛▘
   ▝▜▀▀▀▀▀▀▛▘
",
        ],
        &[r"
    1111111111
   111111111111
11111111111K141
 1RRRRRRRRRK111
111RRRRRRRRRR11
   RRRRRRRRRR
"],
    )
    .tint("1:gGcC")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .school(1, 3)
    .weight(6)
    .traits(CURIOUS)
    .home("lily")
    .biomes(FRESH),
    Species::new(
        "yellow-perch",
        &[
            r"
   ▲▲▲▲▲▲ ▗▄▄▄▖
◤▄▟▓▓█▓▓█▓▓█▓▙▖
 █▒▒▒█▒▒█▒▒█▒◉▌
◣▀▜░░█░░█░░█░▛▘
   ▝▀▀▀▀▀▀▀▀▘
",
            r"
   ▲▲▲▲▲▲ ▗▄▄▄▖
◢▄▟▓▓█▓▓█▓▓█▓▙▖
◀█▒▒▒█▒▒█▒▒█▒◉▌
◥▀▜░░█░░█░░█░▛▘
   ▝▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
   111111 11111
11111K11K11K111
 1111K11K11K141
11111K11K11K111
   rrrrrrrrrr
"],
    )
    .tint("1:yY")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .school(2, 6)
    .weight(5)
    .biomes(FRESH),
    Species::new(
        "walleye",
        &[
            r"
    ▲▲▲▲▲ ▗▄▄▄▖
◤▄▟▓▓▒▓▓▒▓▓▒▓▙▖
 █▒▒▒▒▒▒▒▒▒▒▒▒◉▌▀▘
◣▀▜░░░░░░░░░░░▛▘
    ▝▀▀▀▀▀▀▀▘
",
            r"
    ▲▲▲▲▲ ▗▄▄▄▖
◢▄▟▓▓▒▓▓▒▓▓▒▓▙▖
◀█▒▒▒▒▒▒▒▒▒▒▒▒◉▌▀▘
◥▀▜░░░░░░░░░░░▛▘
    ▝▀▀▀▀▀▀▀▘
",
        ],
        &[r"
    11111 11111
11111K11K11K111
 1111111111111411
111wwwwwwwwwww11
    11111111
"],
    )
    .tint("1:yY")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.3, 1.0)
    .weight(4)
    .traits(PREDATOR | NOCTURNAL)
    .biomes(FRESH),
    Species::new(
        "lake-sturgeon",
        &[
            r"
◢▄▄▖      ▗▄▄▄▖
 ▝▀▜▓◆▓◆▓◆▓◆▓◆▓◆▓◆▓▓▓▓▙▖
  ◢█▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉▙▄▃▂
   ◥░░░░░░░░░░░░░░░░░░▜▀▘⠒
     ▝▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
▗▄▄▖      ▗▄▄▄▖
◢▀▀▜▓◆▓◆▓◆▓◆▓◆▓◆▓◆▓▓▓▓▙▖
  ◢█▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉▙▄▃▂
   ◣░░░░░░░░░░░░░░░░░░▜▀▘⠐
     ▝▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
1111      11111
 1111W1W1W1W1W1W1W111111
  111111111111111111114111
   1wwwwwwwwwwwwwwwwww111w
     11111111111111
"],
    )
    .tint("1:wK")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.25, 0.7)
    .weight(2)
    .traits(SCAVENGER | DEFENDED)
    .biomes(FRESH),
    Species::new(
        "longnose-gar",
        &[
            r"
    ▗▄▄▄▖
◤▄▟▓▓∙▓▓∙▓▓∙▓▓∙▓▓∙▓▓▙▖
 █▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒▒◉▌▀▼▀▼▀▼▀
◣▀▜░░░░░░░░░░░░░░░░░░░▛▄▄▄▄▄▄▄
    ▝▀▀▘
",
            r"
    ▗▄▄▄▖
◢▄▟▓▓∙▓▓∙▓▓∙▓▓∙▓▓∙▓▓▙▖
◀█▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒▒◉▌▀▼▀▼▀▼▀
◥▀▜░░░░░░░░░░░░░░░░░░░▛▄▄▄▄▄▄▄
    ▝▀▀▘
",
        ],
        &[r"
    11111
11111K11K11K11K11K1111
 111K111K111K111K111141wwwwwww
111wwwwwwwwwwwwwwwwwww1wwwwwww
    1111
"],
    )
    .tint("1:gGyY")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.3, 1.2)
    .weight(3)
    .traits(PREDATOR | DEFENDED)
    .biomes(FRESH),
    Species::new(
        "paddlefish",
        &[
            r"
◢▄▄▖     ▗▄▄▄▄▖
 ▝▀▜▓▓▓▓▓▓▓▓▓▓▓▙▖???????▗▄▄▄▖
  ◢█▒▒▒▒▒▒▒▒▒▒▒◉▌━━━━━━━◖███◗
   ◥░░░░░░░░░░░▛▘???????▝▀▀▀▘
     ▝▀▀▀▀▀▀▀▀▘
",
            r"
▗▄▄▖     ▗▄▄▄▄▖
◢▀▀▜▓▓▓▓▓▓▓▓▓▓▓▙▖???????▗▄▄▄▖
  ◢█▒▒▒▒▒▒▒▒▒▒▒◉▌━━━━━━━◖███◗
   ◣░░░░░░░░░░░▛▘???????▝▀▀▀▘
     ▝▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
1111     111111
 1111111111111111       11111
  111111111111141wwwwwww11111
   1wwwwwwwwwww11       11111
     1111111111
"],
    )
    .tint("1:wK")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.3, 0.9)
    .weight(2)
    .traits(FILTER)
    .biomes(FRESH),
    Species::new(
        "elephantnose-fish",
        &[
            r"
   ▗▄▄▄▄▖
  ▗▟▓▓▓▓▓▓▙▖
◤▄█▓▒▒▒▒▒▒▒◉▌
 █▒▒▒▒▒▒▒▒▒▛▀▀▀▀╮
◣▀▜░░░░░░░░░▘???╵
    ▝▀▀▀▀▀▀▘
",
            r"
   ▗▄▄▄▄▖
  ▗▟▓▓▓▓▓▓▙▖
◢▄█▓▒▒▒▒▒▒▒◉▌
◀█▒▒▒▒▒▒▒▒▒▛▀▀▀▀╮
◥▀▜░░░░░░░░░▘???╵
    ▝▀▀▀▀▀▀▘
",
        ],
        &[r"
   111111
  1111111111
1111111111141
 11111111111yyyyy
111wwwwwwwww1   y
    11111111
"],
    )
    .tint("1:KwyK")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(3)
    .traits(SHY | NOCTURNAL)
    .home("log")
    .biomes(TANK | FRESH),
    Species::new(
        "glass-catfish",
        &[
            r"
   ╭──────────╮
◤│?╱╱╱╱╱╱╱╱?▒◉╲⠒
◣│?════════?▒▒╱⠒
   ╰──────────╯
",
            r"
   ╭──────────╮
◣│?╱╱╱╱╱╱╱╱?▒◉╲⠐
◤│?════════?▒▒╱⠐
   ╰──────────╯
",
        ],
        &[r"
   CCCCCCCCCCCC
CC cccccccc C4Cw
CC WWWWWWWW CCCw
   CCCCCCCCCCCC
"],
    )
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .school(4, 8)
    .weight(4)
    .traits(SHY)
    .home("vallisneria")
    .biomes(TANK | FRESH),
    Species::new(
        "archerfish",
        &[
            r"
                ⠁
      ▗▄▄▄▖????⠈
   ▗▟▓█▓█▓▓▙▖?⠂
◤▄█▒▒▒█▒█▒▒◉▌⠂
◀█▒▒▒▒█▒█▒▒▒▌
◣▀▜░░░░░░░░░▘
     ▝▀▀▀▀▘
",
            r"
                ⠐
      ▗▄▄▄▖????⠂
   ▗▟▓█▓█▓▓▙▖?⠄
◢▄█▒▒▒█▒█▒▒◉▌⠠
◀█▒▒▒▒█▒█▒▒▒▌
◥▀▜░░░░░░░░░▘
     ▝▀▀▀▀▘
",
        ],
        &[r"
                C
      11111    C
   111K1K1111 C
111111K1K1141C
111111K1K1111
1111111111111
     111111
"],
    )
    .tint("1:wW")
    .fps(0.3)
    .zone(Zone::Top)
    .swim(Swim::Hover)
    .speed(0.25, 0.8)
    .weight(3)
    .traits(PREDATOR)
    .biomes(TANK | FRESH),
    Species::new(
        "killifish",
        &[
            r"
    ▗▄▄▄▖
◤▄▟▓▒▒▒▒▒◉▌
◀█▒▒▒▒▒▒▒▛▘
◣▀▜░░░░░░▛▘
",
            r"
    ▗▄▄▄▖
◢▄▟▓▒▒▒▒▒◉▌
◀█▒▒▒▒▒▒▒▛▘
◥▀▜░░░░░░▛▘
",
        ],
        &[r"
    11111
66611111141
66111111111
66611111111
"],
    )
    .tint("1:bBrRyY 6:rRyYgG")
    .fps(0.4)
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.3, 1.0)
    .school(2, 5)
    .weight(4)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "bichir",
        &[
            r"
    ◢◣◢◣◢◣◢◣◢◣◢◣◢◣◢◣
◤▄▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▓◉▌
◣▀░░░░░░░░░░░░░░░░░░░░░░░░▛▀▘
                      ▝█▘
",
            r"
    ◢◣◢◣◢◣◢◣◢◣◢◣◢◣◢◣
◢▄▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▓◉▌
◥▀░░░░░░░░░░░░░░░░░░░░░░░░▛▀▘
                      ▗▄▖
",
        ],
        &[r"
    1111111111111111
1111111111111111111111111141
11yyyyyyyyyyyyyyyyyyyyyyyy111
                      111
"],
    )
    .tint("1:gKyw")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(3)
    .traits(PREDATOR | NOCTURNAL)
    .biomes(TANK | FRESH),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_species_drawn(super::SPECIES);
    }
}
