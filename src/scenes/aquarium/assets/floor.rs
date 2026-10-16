//! Creatures of the sea floor: crabs, lobsters, starfish, snails, flatfish.
//! Drawn by hand in Unicode; the legs and claws alternate between frames.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "red-rock-crab",
        &[
            r"
▗▟▖      ▗▟▖
▝▜▙▄●██●▄▟▛▘
 ▐▓██████▓▌
 ╱╱╱╱  ╲╲╲╲
",
            r"
▟▙▖      ▗▟▙
▝▜▙▄●██●▄▟▛▘
 ▐▓██████▓▌
 ╲╱╱╲  ╱╲╲╱
",
        ],
        &[r"
111      111
111141141111
 1111111111
 1111  1111
"],
    )
    .critter()
    .tint("1:rRy 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.35)
    .fps(0.40)
    .weight(7)
    .biomes(KELP | OCEAN | REEF)
    .traits(SCAVENGER | DEFENDED | SHY)
    .home("rock,cave,trap,pot"),
    Species::new(
        "blue-crab",
        &[
            r"
▗▟▙        ▟▙▖
▝▜█▙▄▄▄▄▄▄▟█▛▘
  ▐█●████●█▌
  ▝▀██████▀▘
  ╱ ╱ ╱  ╲ ╲ ╲
",
            r"
▟▙▖        ▗▟▙
▝▜█▙▄▄▄▄▄▄▟█▛▘
  ▐█●████●█▌
  ▝▀██████▀▘
  ╲ ╱ ╲  ╱ ╲ ╱
",
        ],
        &[r"
222        222
11111111111111
  1141111411
  1111111111
  1 1 1  1 1 1
"],
    )
    .critter()
    .tint("1:bBcB 2:rRyY 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.12, 0.40)
    .fps(0.45)
    .weight(6)
    .biomes(OCEAN | KELP)
    .traits(SCAVENGER | PREDATOR),
    Species::new(
        "hermit-crab",
        &[
            r"
  ▗▄▄▖      ● ●
 ▟▓▒◜▒▓▙▄▟▀█▀▙▖
▐▓▒▒▒◝▓▓▛▜██▛▘╲
 ▝▀▀▀▀▀▘ ╱╱ ╲╲
",
            r"
  ▗▄▄▖      ● ●
 ▟▓▒◜▒▓▙▄▟▀█▀▙▖
▐▓▒▒▒◝▓▓▛▜██▛▘╱
 ▝▀▀▀▀▀▘ ╲╱ ╱╲
",
        ],
        &[r"
  1111      4 4
 11111113333333
111111111333333
 1111111 33 33
"],
    )
    .critter()
    .tint("1:yYwr 3:rRyY 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.05, 0.20)
    .fps(0.40)
    .school(1, 4)
    .weight(7)
    .biomes(REEF | KELP | TANK | OCEAN)
    .traits(SCAVENGER | SHY)
    .home("rock"),
    Species::new(
        "fiddler-crab",
        &[
            r"
 │ │   ▗▄▄▖
 ● ●  ▟██▛▘
▐████▙▟█▛▘
▝▀████▛▀▀
 ╱╱ ╲╲
",
            r"
 │ │   ▗▟▙▖
 ● ●  ▟██▛▘
▐████▙▟█▛▘
▝▀████▛▀▀
 ╲╱ ╱╲
",
        ],
        &[r"
 1 1   2222
 4 4  22222
1111112222
111111122
 11 11
"],
    )
    .critter()
    .tint("1:yYrR 2:rRyY 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.08, 0.30)
    .fps(0.45)
    .weight(6)
    .biomes(REEF | KELP | TANK)
    .traits(SCAVENGER | SHY)
    .home("rock"),
    Species::new(
        "decorator-crab",
        &[
            r"
  ╲│╱ ✿ ╲│╱
▗▟▒▓●░░●▓▒▙▖
▐▓▒░❋▒▒◆░▒▓▌
 ▝▀▜████▛▀▘
╱╱ ╱╱  ╲╲ ╲╲
",
            r"
  ╲│╱ ✿ ╲│╱
▗▟▒▓●░░●▓▒▙▖
▐▓▒░❋▒▒◆░▒▓▌
 ▝▀▜████▛▀▘
╲╱ ╲╱  ╱╲ ╱╲
",
        ],
        &[r"
  222 3 222
111142241111
111231152111
 1111111111
11 11  11 11
"],
    )
    .critter()
    .tint("1:yYr 2:gG 3:mM 4:wW 5:rbc")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.06, 0.20)
    .fps(0.40)
    .weight(3)
    .biomes(REEF | KELP | OCEAN)
    .traits(SCAVENGER | SHY)
    .home("rock,coral"),
    Species::new(
        "japanese-spider-crab",
        &[
            r"
 ╱╲                ╱╲
╱  ╲ ▗▄▄▄▄▄▄▄▄▄▄▖ ╱  ╲
    ╲▟▓▓▓▓▓▓▓▓▓▓▙╱
    ╱▐▓▒●░▒▒░●▒▓▌╲
   ╱  ▝▀▜████▛▀▘  ╲
  ╱  ╱ ╱      ╲ ╲  ╲
 ╱  ╱ ╱        ╲ ╲  ╲
",
            r"
 ╱╲                ╱╲
╱  ╲ ▗▄▄▄▄▄▄▄▄▄▄▖ ╱  ╲
    ╲▟▓▓▓▓▓▓▓▓▓▓▙╱
    ╱▐▓▒●░▒▒░●▒▓▌╲
   ╱  ▝▀▜████▛▀▘  ╲
  ╱ ╱   ╱    ╲   ╲ ╲
 ╱ ╱   ╱      ╲   ╲ ╲
",
        ],
        &[r"
 22                22
1  1 111111111111 1  1
    11111111111111
    11114111141111
   1  1111111111  1
  1  1 1      1 1  1
 1  1 1        1 1  1
"],
    )
    .critter()
    .tint("1:rRyw 2:yYw 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.04, 0.12)
    .fps(0.30)
    .weight(2)
    .biomes(DEEP | KELP | OCEAN)
    .traits(SCAVENGER | DEFENDED)
    .home("rock,whale"),
    Species::new(
        "red-king-crab",
        &[
            r"
▗▙▖ ▗▄▄▄▄▄▄▖ ▗▟▖
 ▜█▙▟▓●▒▒●▓▙▟█▛
  ▝█▓▒▒▒▒▒▒▓█▘
 ╱╱ ▝▀████▀▘ ╲╲
╱ ╱ ╱      ╲ ╲ ╲
",
            r"
▟▙▖ ▗▄▄▄▄▄▄▖ ▗▟▙
 ▜█▙▟▓●▒▒●▓▙▟█▛
  ▝█▓▒▒▒▒▒▒▓█▘
 ╲╲ ▝▀████▀▘ ╱╱
╲ ╱ ╲      ╱ ╲ ╱
",
        ],
        &[r"
222 11111111 222
 11111411411111
  111111111111
 11 11111111 11
1 1 1      1 1 1
"],
    )
    .critter()
    .tint("1:rRyR 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.06, 0.20)
    .fps(0.35)
    .weight(6)
    .biomes(ARCTIC | KELP)
    .traits(SCAVENGER | DEFENDED)
    .home("rock"),
    Species::new(
        "snow-crab",
        &[
            r"
  ╱╲  ▗▄▄▄▄▖  ╱╲
 ╱  ╲▟▓●▓▓●▓▙╱  ╲
╱ ╱ ╱▝█▓▒▒▓█▘╲ ╲ ╲
 ╱ ╱  ▝▀▀▀▀▘  ╲ ╲
",
            r"
  ╱╲  ▗▄▄▄▄▖  ╱╲
 ╱  ╲▟▓●▓▓●▓▙╱  ╲
╱ ╱ ╱▝█▓▒▒▓█▘╲ ╲ ╲
╱ ╱    ▝▀▀▀▀▘   ╲ ╲
",
        ],
        &[r"
  11  111111  11
 1  1114114111  1
1 1 1111111111 1 1
 1 1  111111  1 1
"],
    )
    .critter()
    .tint("1:wWyY 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.06, 0.22)
    .fps(0.40)
    .weight(5)
    .biomes(ARCTIC | KELP)
    .traits(SCAVENGER)
    .home("rock"),
    Species::new(
        "yeti-crab",
        &[
            r"
 ▗▟▒▙      ▗▟▒▙
 ▜▒░▒▙▄▄▄▄▟▒░▒▛
  ▝▀▜██▐▌██▛▀▘
   ▐▒░▒▒▒▒░▒▌
  ╱╎╎╎╎╎╎╎╎╲
 ╱ ┆ ┆ ┆ ┆ ┆ ╲
",
            r"
 ▗▟▒▙      ▗▟▒▙
 ▜▒░▒▙▄▄▄▄▟▒░▒▛
  ▝▀▜██▐▌██▛▀▘
   ▐▒░▒▒▒▒░▒▌
  ╲╎╎╎╎╎╎╎╎╱
╱  ┆ ┆ ┆ ┆ ┆  ╲
",
        ],
        &[r"
 wwWw      wwWw
 wWWWwwwwwwWWWw
  wwwwwwwwwwww
   wWWWWWWWWw
  wWWWWWWWWw
 w W W W W W w
"],
    )
    .critter()
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.04, 0.14)
    .fps(0.30)
    .weight(3)
    .biomes(DEEP)
    .traits(SCAVENGER | FILTER)
    .home("vent"),
    Species::new(
        "sally-lightfoot-crab",
        &[
            r"
▗▟▖     ▗▟▖
 ▝▜▙●▄▄●▟▛▘
  ▐▓████▓▌
 ╱╱╱    ╲╲╲
",
            r"
▟▙▖     ▗▟▙
 ▝▜▙●▄▄●▟▛▘
  ▐▓████▓▌
 ╲╱╱    ╲╲╱
",
        ],
        &[r"
111     111
 1114114111
  11222211
 111    111
"],
    )
    .critter()
    .tint("1:rRmR 2:bBcy 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.15, 0.40)
    .fps(0.50)
    .weight(4)
    .biomes(REEF | OCEAN)
    .traits(SCAVENGER | GRAZER | SHY)
    .home("rock"),
    Species::new(
        "horseshoe-crab",
        &[
            r"
       ▗▄▄▄▄▄▖
     ▗▟▓▓▒●▒▒▒▙▖
◀━━━▟▓▓▓▒▒▒▒▒▒▒▜▌
    ▝▀▜▓▓▒▒▒▒▒▒▛▀▘
      ╱╱ ╱╱ ╱╱
",
            r"
       ▗▄▄▄▄▄▖
     ▗▟▓▓▒●▒▒▒▙▖
◀━━━▟▓▓▓▒▒▒▒▒▒▒▜▌
    ▝▀▜▓▓▒▒▒▒▒▒▛▀▘
      ╲╱ ╲╱ ╲╱
",
        ],
        &[r"
       1111111
     11111411111
22221111111111111
    11111111111111
      11 11 11
"],
    )
    .critter()
    .tint("1:yrkw 2:kyr 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.04, 0.14)
    .fps(0.30)
    .weight(4)
    .biomes(OCEAN | KELP | REEF)
    .traits(SCAVENGER | DEFENDED),
    Species::new(
        "american-lobster",
        &[
            r"
                  ╭──────────
 ▗▄▄▖     ▗▄▄▄▄▄▖╭╯  ▗▄▄▄▖
▐████▙▄▄▄▟▓▓▓▓▓▓●▙▄▄▄▟███▙
 ▝▜██▛▀▀▀▜▓▒▒▒▒▒▓▛▘  ▝▀▀▀▘
          ╱╱╱╱╲╲╲╲
",
            r"
                  ╭──────────
 ▗▄▄▖     ▗▄▄▄▄▄▖╭╯  ▗▄▄▄▖
▐████▙▄▄▄▟▓▓▓▓▓▓●▙▄▄▄▟███▙
 ▝▜██▛▀▀▀▜▓▒▒▒▒▒▓▛▘  ▝▀▀▀▘
          ╲╱╲╱╲╱╲╱
",
        ],
        &[r"
                  33333333333
 1111     111111133  22222
11111111111111114122222222
 111111111111111111  22222
          11111111
"],
    )
    .critter()
    .tint("1:rRyb 2:rRyw 3:wW 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.22)
    .fps(0.35)
    .weight(5)
    .biomes(KELP | OCEAN | ARCTIC)
    .traits(SCAVENGER | PREDATOR | NOCTURNAL)
    .home("rock,cave,trap,pot"),
    Species::new(
        "spiny-lobster",
        &[
            r"
                  ╭──────
                ╭─╯
 ▗▄▖   ▗▄▄▄▄▄▄▄▄▖
▐███▙▄▟▓▓▒▒▒▒▒▒▓●▙━━━
 ▝▜█▛▀▀▀▜▓▒▒▒▒▒▓▛▘
       ╱╱╱╱ ╲╲╲
",
            r"
                  ╭──────
                ╭─╯
 ▗▄▖   ▗▄▄▄▄▄▄▄▄▖
▐███▙▄▟▓▓▒▒▒▒▒▒▓●▙━━━
 ▝▜█▛▀▀▀▜▓▒▒▒▒▒▓▛▘
       ╲╱╱╲ ╱╲╲
",
        ],
        &[r"
                  333333
                333
 111   1111111111
111111111222222141333
 11111111111111111
       1111 111
"],
    )
    .critter()
    .tint("1:rRyy 2:yYr 3:rR 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.25)
    .fps(0.35)
    .weight(4)
    .biomes(REEF | OCEAN | KELP)
    .traits(SCAVENGER | NOCTURNAL | DEFENDED)
    .home("rock,cave,trap,pot"),
    Species::new(
        "slipper-lobster",
        &[
            r"
    ▗▄▄▄▄▄▄▖
 ▗▟▓▓▒▒▒▒●▒▙◢█◣
▐██▓▒▒▒▒▒▒▒▒▛▀▘
 ▝▀▜█████▛▀▘
  ╱╱╱╱╱╱
",
            r"
    ▗▄▄▄▄▄▄▖
 ▗▟▓▓▒▒▒▒●▒▙◢█◣
▐██▓▒▒▒▒▒▒▒▒▛▀▘
 ▝▀▜█████▛▀▘
  ╲╱╲╱╲╱
",
        ],
        &[r"
    11111111
 11111111411111
111111111111111
 11111111111
  111111
"],
    )
    .critter()
    .tint("1:rRyR 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.08, 0.18)
    .fps(0.35)
    .weight(3)
    .biomes(REEF | OCEAN)
    .traits(SCAVENGER | NOCTURNAL | SHY)
    .home("rock,cave"),
    Species::new(
        "crayfish",
        &[
            r"
                ╭─────
 ▗▄▖     ▗▄▄▄▄▖╭╯ ▗▟▙▖
▐█▙▄▄▄▄▟▓▓▓▓▓●▙▄▄▄▟██▛▘
 ▝▜▛▀▀▀▜▓▒▒▒▒▒▛▘   ▝▀▀▘
       ╱╱╱╱╲╲╲╲
",
            r"
                ╭─────
 ▗▄▖     ▗▄▄▄▄▖╭╯ ▗▟▙▖
▐█▙▄▄▄▄▟▓▓▓▓▓●▙▄▄▄▟██▛▘
 ▝▜▛▀▀▀▜▓▒▒▒▒▒▛▘   ▝▀▀▘
       ╲╱╲╱╲╱╲╱
",
        ],
        &[r"
                333333
 111     11111133 1111
11111111111114111111111
 111111111111111   1111
       11111111
"],
    )
    .critter()
    .tint("1:rRrbk 3:wW 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.30)
    .fps(0.40)
    .weight(7)
    .biomes(FRESH)
    .traits(SCAVENGER | NOCTURNAL | SHY)
    .home("rock"),
    Species::new(
        "amano-shrimp",
        &[
            r"
   ▗▄▄▄▄▖
◢▟▒▒░▒▒▒●▙╾─
 ▝▀▀▀▀▀▀▀▀▘
  ╵╷╵╷╵╷
",
            r"
   ▗▄▄▄▄▖
◢▟▒▒░▒▒▒●▙╾─
 ▝▀▀▀▀▀▀▀▀▘
  ╷╵╷╵╷╵
",
        ],
        &[r"
   111111
111111114111
 1111111111
  111111
"],
    )
    .critter()
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.08, 0.25)
    .fps(0.50)
    .weight(5)
    .biomes(TANK | FRESH)
    .traits(GRAZER | SHY)
    .home("seaweed,rock"),
    Species::new(
        "pistol-shrimp",
        &[
            r"
         ● ●     ▗▄▄▖
 ▗▄▄▄▄▄▄▄▄▟▓▙▄▄▄▄▟██▛▘
◢▓▒▒▒▒▒▒▒▒▒▒▓▛▘   ▝▀▀▘
 ▝▀▀▀▀▀▀▀▀▀▀▘
  ╱ ╱ ╱ ╱ ╱
",
            r"
         ● ●     ▗▄▄▖
 ▗▄▄▄▄▄▄▄▄▟▓▙▄▄▄▄▟██▛▘
◢▓▒▒▒▒▒▒▒▒▒▒▓▛▘   ▝▀▀▘
 ▝▀▀▀▀▀▀▀▀▀▀▘
  ╲ ╲ ╲ ╲ ╲
",
        ],
        &[r"
         4 4     2222
 111111111111111122222
11111111111111   2222
 111111111111
  1 1 1 1 1
"],
    )
    .critter()
    .tint("1:rRyy 2:yYrw 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.06, 0.20)
    .fps(0.40)
    .weight(3)
    .biomes(REEF | KELP)
    .traits(SHY)
    .home("rock"),
    Species::new(
        "mantis-shrimp",
        &[
            r"
           ◉ ◉
▗▄▄▄▄▄▄▄▄▄▄▟▓▙▖
◢▓▒▓▒▓▒▓▒▓▒▓▓▛▀◥
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
  ╱ ╱ ╱ ╱ ╱ ╱
",
            r"
           ◉ ◉
▗▄▄▄▄▄▄▄▄▄▄▟▓▙▖
◢▓▒▓▒▓▒▓▒▓▒▓▓▛▀◥
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
  ╲ ╲ ╲ ╲ ╲ ╲
",
        ],
        &[r"
           4 4
111111111111111
1121212121211122
 11111111111111
  1 1 1 1 1 1
"],
    )
    .critter()
    .tint("1:gGrbmc 2:rRyY 4:yY")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.30)
    .fps(0.50)
    .weight(3)
    .biomes(REEF)
    .traits(PREDATOR | SHY)
    .home("rock,coral"),
    Species::new(
        "sea-star",
        &[
            r"
     ▟▙
    ▟██▙
▄▄▄▄▟██▙▄▄▄▄
 ▝▀▜█◆◆█▛▀▘
   ▟█▛▜█▙
  ▟█▘  ▝█▙
 ▝▀▘    ▝▀▘
",
            r"
     ▟▙
    ▟██▙
▄▄▄▄▟██▙▄▄▄▄
 ▝▀▜█◆◆█▛▀▘
   ▟█▛▜█▙
  ▟█▘  ▝█▙
▝▀▘      ▝▀▘
",
        ],
        &[r"
     11
    1111
111111111111
 1111221111
   111111
  111  111
 111    111
"],
    )
    .critter()
    .tint("1:rRyYmb 2:wWyY")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.20)
    .weight(8)
    .biomes(REEF | OCEAN | KELP | ARCTIC | TANK)
    .traits(SCAVENGER | PREDATOR),
    Species::new(
        "knobby-sea-star",
        &[
            r"
     ▟▙
    ▟◆◆▙
●▄▄▄▟██▙▄▄▄●
 ▝▜█◇◆◆◇█▛▘
   ▟█▛▜█▙
  ▟█▘  ▝█▙
 ◆▀▘    ▝▀◆
",
            r"
     ▟▙
    ▟◆◆▙
●▄▄▄▟██▙▄▄▄●
 ▝▜█◇◆◆◇█▛▘
   ▟█▛▜█▙
  ▟█▘  ▝█▙
◆▀▘      ▝▀◆
",
        ],
        &[
            r"
     11
    1221
211111111112
 1112222111
   111111
  111  111
 211    112
",
            r"
     11
    1221
211111111112
 1112222111
   111111
  111  111
211      112
",
        ],
    )
    .critter()
    .tint("1:rRyYmM 2:wWyY")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.05)
    .fps(0.20)
    .weight(5)
    .biomes(REEF | KELP | OCEAN)
    .traits(SCAVENGER),
    Species::new(
        "crown-of-thorns",
        &[
            r"
  ╲│╱  ╲│╱
 ╲╲▟▓▓▓▓▙╱╱
─┼▐●▒▓▓▒●▌┼─
 ╱╱▜▓▓▓▓▛╲╲
  ╱│╲  ╱│╲
",
            r"
  ╲│╲  ╱│╱
 ╲╲▟▓▓▓▓▙╱╱
─┼▐●▒▓▓▒●▌┼─
 ╱╱▜▓▓▓▓▛╲╲
  ╲│╱  ╲│╱
",
        ],
        &[r"
  222  222
 2211111122
221411114122
 2211111122
  222  222
"],
    )
    .critter()
    .tint("1:mMrR 2:yYrR")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.20)
    .weight(3)
    .biomes(REEF)
    .traits(STING | DEFENDED | GRAZER),
    Species::new(
        "brittle-star",
        &[
            r"
╭╯    ╷    ╰╮
╰╮  ▗▟█▙▖  ╭╯
 ╰──▐█◉█▌──╯
 ╭╯ ▝▜█▛▘ ╰╮
╭╯         ╰╮
",
            r"
╰╮    ╷    ╭╯
╭╯  ▗▟█▙▖  ╰╮
 ╭──▐█◉█▌──╮
 ╰╮ ▝▜█▛▘ ╭╯
╰╮         ╭╯
",
        ],
        &[r"
11    1    11
11  11111  11
 11111211111
 11 11111 11
11         11
"],
    )
    .critter()
    .tint("1:rRyYmc 2:wWyY")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.03, 0.08)
    .fps(0.30)
    .weight(6)
    .biomes(REEF | OCEAN | KELP | DEEP | ARCTIC)
    .traits(SCAVENGER | SHY)
    .home("rock"),
    Species::new(
        "basket-star",
        &[
            r"
╲│╱╲╱  ╲╱╲│╱
 ╲╲│╱╱╲╲│╱╱
─╲╲▟▓▓▓▓▙╱╱─
 ╱╱▜▓◉◉▓▛╲╲
╱╱│╲╲  ╱╱│╲╲
",
            r"
╱│╲╱╲  ╱╲╱│╲
 ╱╲│╲╲╱╱│╱╲
─╲╲▟▓▓▓▓▙╱╱─
 ╱╱▜▓◉◉▓▛╲╲
╲╱│╱╲  ╲╱│╱╲
",
        ],
        &[r"
11111  11111
 1111111111
111111111111
 1111221111
11111  11111
"],
    )
    .critter()
    .tint("1:wWyYrR 2:yY")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.05)
    .fps(0.25)
    .weight(4)
    .biomes(ARCTIC | DEEP | KELP)
    .traits(FILTER | NOCTURNAL)
    .home("coral"),
    Species::new(
        "purple-sea-urchin",
        &[
            r"
 ╲  ╲││╱  ╱
─╲▗▟████▙▖╱─
──█▓▓▓▓▓▓█──
─╱▝▜████▛▘╲─
 ╱  ╱││╲  ╲
",
            r"
 ╱  ╱││╲  ╲
─╲▗▟████▙▖╱─
──█▓▓▓▓▓▓█──
─╱▝▜████▛▘╲─
 ╲  ╲││╱  ╱
",
        ],
        &[r"
 1  1111  1
112222222211
112222222211
112222222211
 1  1111  1
"],
    )
    .critter()
    .tint("1:mMbr 2:mMrk")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.20)
    .school(1, 3)
    .weight(7)
    .biomes(KELP | OCEAN | ARCTIC | REEF)
    .traits(GRAZER | DEFENDED),
    Species::new(
        "pencil-urchin",
        &[
            r"
 ╿ ╿ ╿ ╿ ╿
╱┃╽┃╿┃╽┃╿┃╲
 ▟▓▓▓▓▓▓▓▙
 ▝▀▀▀▀▀▀▀▘
",
            r"
 ┃ ┃ ┃ ┃ ┃
╱┃╽┃╿┃╽┃╿┃╲
 ▟▓▓▓▓▓▓▓▙
 ▝▀▀▀▀▀▀▀▘
",
        ],
        &[r"
 1 1 1 1 1
11111111111
 222222222
 222222222
"],
    )
    .critter()
    .tint("1:rRyYmw 2:kKwy")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.20)
    .weight(4)
    .biomes(REEF)
    .traits(GRAZER | DEFENDED | NOCTURNAL)
    .home("rock,coral"),
    Species::new(
        "sea-cucumber",
        &[
            r"
    ╻   ╻   ╻   ╻
  ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
◢▓▒▓◆▓▒▓▒◆▓▒▓▒◆▓▒▓▙❋
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
  ╵╷╵╷╵╷╵╷╵╷╵╷╵╷
",
            r"
    ╻   ╻   ╻   ╻
  ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
◢▓▒▓◆▓▒▓▒◆▓▒▓▒◆▓▒▓▙✱
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
  ╷╵╷╵╷╵╷╵╷╵╷╵╷╵
",
        ],
        &[r"
    2   2   2   2
  1111111111111111
11112111121111211113
 11111111111111111
  11111111111111
"],
    )
    .critter()
    .tint("1:kyrm 2:yYwr 3:yYrw")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.05)
    .fps(0.30)
    .weight(6)
    .biomes(OCEAN | KELP | REEF | TANK)
    .traits(SCAVENGER),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_species_drawn(super::SPECIES);
    }
}
