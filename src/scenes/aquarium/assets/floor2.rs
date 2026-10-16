//! Creatures of the sea floor: crabs, lobsters, starfish, snails, flatfish, continued.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "sea-pig",
        &[
            r"
 ▂▅▇▇▇▇▅▂
▟████████▙╮
▜▓▓▓▓▓▓▓▓▛●
 ╿╽ ╿╽ ╿╽
",
            r"
 ▃▆▇▇▇▇▆▃
▟████████▙╮
▜▓▓▓▓▓▓▓▓▛●
 ╽╿ ╽╿ ╽╿
",
        ],
        &[
            r"
 11111111
11111111111
11111111114
 11 11 11
",
            r"
 11111111
11111111111
11111111114
 11 11 11
",
        ],
    )
    .critter()
    .tint("1:mMwr")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.05)
    .fps(0.30)
    .weight(5)
    .biomes(DEEP)
    .traits(SCAVENGER),
    Species::new(
        "moon-snail",
        &[
            r"
 ╭────╮
╭╯╭──╮╰╮   │ │
│ │◉╭╯ │  ▗▟█●
╰╮╰─╯ ╭╯▂▄▟██▛
 ╰────╯▃▂▃▂▃▂▃
",
            r"
 ╭────╮
╭╯╭──╮╰╮   ╱ ╱
│ │◉╭╯ │  ▗▟█●
╰╮╰─╯ ╭╯▂▄▟██▛
 ╰────╯▂▃▂▃▂▃▂
",
        ],
        &[
            r"
 111111
11111111   3 3
1 1Y11 1  3334
11111 11333333
 1111113333333
",
            r"
 111111
11111111   3 3
1 1Y11 1  3334
11111 11333333
 1111113333333
",
        ],
    )
    .critter()
    .tint("1:wWyYrb 3:yYwr 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.30)
    .weight(4)
    .biomes(OCEAN | KELP | REEF)
    .traits(PREDATOR | SCAVENGER),
    Species::new(
        "queen-conch",
        &[
            r"
  ▲ ▲ ▲
 ╭┴─┴─┴╮
╭╯╭───╮╰╮  │ │
│ │ ◉╭╯ │ ▗▟█●
╰╮╰───╯╭╯▂▄▟██▛
 ╰─────╯▃▂▃▂▃▂▃
",
            r"
  ▲ ▲ ▲
 ╭┴─┴─┴╮
╭╯╭───╮╰╮  ╱ ╱
│ │ ◉╭╯ │ ▗▟█●
╰╮╰───╯╭╯▂▄▟██▛
 ╰─────╯▂▃▂▃▂▃▂
",
        ],
        &[
            r"
  1 1 1
 1111111
111111111  3 3
1 1 R11 1 3334
111111111333333
 11111113333333
",
            r"
  1 1 1
 1111111
111111111  3 3
1 1 R11 1 3334
111111111333333
 11111113333333
",
        ],
    )
    .critter()
    .tint("1:yYwr 3:yYwr 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.05)
    .fps(0.30)
    .weight(4)
    .biomes(REEF | OCEAN)
    .traits(GRAZER | DEFENDED),
    Species::new(
        "tiger-cowrie",
        &[
            r"
 ▂▅▇██▇▅▂
▟█◆▪◆▪◆█▙ ▗▟●
▜█▪◆▪◆▪█▛▄▟█▛
▝▀▀▀▀▀▀▀▘▃▂▃▂
",
            r"
 ▂▅▇██▇▅▂
▟█◆▪◆▪◆█▙ ▗▟●
▜█▪◆▪◆▪█▛▄▟█▛
▝▀▀▀▀▀▀▀▘▂▃▂▃
",
        ],
        &[
            r"
 22222222
221111122 334
2211111223333
2222222223333
",
            r"
 22222222
221111122 334
2211111223333
2222222223333
",
        ],
    )
    .critter()
    .tint("1:kKrb 2:yYwr 3:kKyw 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.05)
    .fps(0.30)
    .weight(4)
    .biomes(REEF)
    .traits(GRAZER | DEFENDED | NOCTURNAL)
    .home("rock,coral"),
    Species::new(
        "nerite-snail",
        &[
            r"
 ╭──╮
╭╯╭╮╰╮  │ │
╰╮╰╯╭╯ ▗▟█●
 ╰──╯▃▂▃▂▃▂
",
            r"
 ╭──╮
╭╯╭╮╰╮  ╱ ╱
╰╮╰╯╭╯ ▗▟█●
 ╰──╯▂▃▂▃▂▃
",
        ],
        &[
            r"
 1111
111111  3 3
111111 3334
 1111333333
",
            r"
 1111
111111  3 3
111111 3334
 1111333333
",
        ],
    )
    .critter()
    .tint("1:yYrm 3:wWyY 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.05)
    .fps(0.30)
    .school(1, 3)
    .weight(7)
    .biomes(TANK | FRESH | REEF)
    .traits(GRAZER | DEFENDED),
    Species::new(
        "apple-snail",
        &[
            r"
  ╭────╮
 ╭╯╭──╮╰╮
╭╯╭╯╭╮╰╮╰╮  │ │
│ ╰╮ ◉ ╭╯│ ▗▟█●
╰╮ ╰───╯╭╯▂▄▟██▛
 ╰───────╯▃▂▃▂▃▂
",
            r"
  ╭────╮
 ╭╯╭──╮╰╮
╭╯╭╯╭╮╰╮╰╮  ╱ ╱
│ ╰╮ ◉ ╭╯│ ▗▟█●
╰╮ ╰───╯╭╯▂▄▟██▛
 ╰───────╯▂▃▂▃▂▃
",
        ],
        &[
            r"
  111111
 11111111
1111111111  3 3
1 11 R 111 3334
11 1111111333333
 111111111333333
",
            r"
  111111
 11111111
1111111111  3 3
1 11 R 111 3334
11 1111111333333
 111111111333333
",
        ],
    )
    .critter()
    .tint("1:yYwr 3:yYwW 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.30)
    .weight(6)
    .biomes(FRESH | TANK)
    .traits(GRAZER),
    Species::new(
        "ramshorn-snail",
        &[
            r"
 ╭───╮
╭╯╭─╮╰╮   │ │
╰╮╰◉╯╭╯  ▗▟█●
 ╰───╯▃▂▃▂▃▂▃
",
            r"
 ╭───╮
╭╯╭─╮╰╮   ╱ ╱
╰╮╰◉╯╭╯  ▗▟█●
 ╰───╯▂▃▂▃▂▃▂
",
        ],
        &[
            r"
 11111
1111111   3 3
111R111  3334
 111113333333
",
            r"
 11111
1111111   3 3
111R111  3334
 111113333333
",
        ],
    )
    .critter()
    .tint("1:rRyY 3:wWyY 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.05)
    .fps(0.30)
    .school(1, 3)
    .weight(5)
    .biomes(FRESH | TANK)
    .traits(GRAZER),
    Species::new(
        "spanish-shawl",
        &[
            r"
  ╿ ╿ ╿ ╿    ╿ ╿
 ▟▓▒▓▒▓▒▓▒▓▒▙▟█●
▐▒▓▒▓▒▓▒▓▒▓▒▓█▛
 ▝▀▄▀▄▀▄▀▄▀▄▀▄▀▘
",
            r"
  ╽ ╽ ╽ ╽    ╿ ╿
 ▟▒▓▒▓▒▓▒▓▒▓▙▟█●
▐▓▒▓▒▓▒▓▒▓▒▓▒█▛
 ▝▄▀▄▀▄▀▄▀▄▀▄▀▄▘
",
        ],
        &[
            r"
  2 2 2 2    2 2
 111111111111114
111111111111111
 222222222222222
",
            r"
  2 2 2 2    2 2
 111111111111114
111111111111111
 222222222222222
",
        ],
    )
    .critter()
    .tint("1:bBmM 2:rRyY 4:yY")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.30)
    .weight(3)
    .biomes(KELP | OCEAN)
    .traits(STING | DEFENDED),
    Species::new(
        "sea-bunny",
        &[
            r"
     ▗▛ ▜▖
▟▒░▒▒░▒▒●▙
▜▒▒░▒▒░▒▒▛
 ▀▄▀▄▀▄▀▄
",
            r"
     ▗▛ ▜▖
▟▒░▒▒░▒▒●▙
▜▒▒░▒▒░▒▒▛
 ▄▀▄▀▄▀▄▀
",
            r"
     ▗▜ ▛▖
▟▒░▒▒░▒▒●▙
▜▒▒░▒▒░▒▒▛
 ▀▄▀▄▀▄▀▄
",
        ],
        &[
            r"
     22 22
1111111141
1111111111
 11111111
",
            r"
     22 22
1111111141
1111111111
 11111111
",
            r"
     22 22
1111111141
1111111111
 11111111
",
        ],
    )
    .critter()
    .tint("1:wWyw 2:kK 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.04)
    .fps(0.30)
    .weight(3)
    .biomes(OCEAN | REEF | KELP)
    .traits(GRAZER | DEFENDED),
    Species::new(
        "chiton",
        &[
            r"
 ▂▅▇▇▇▇▇▇▇▇▇▅▂
▟█▌▐█▌▐█▌▐█▌▐█▙
▝▀▄▀▄▀▄▀▄▀▄▀▄▀▘
",
            r"
 ▂▅▇▇▇▇▇▇▇▇▇▅▂
▟█▌▐█▌▐█▌▐█▌▐█▙
▝▄▀▄▀▄▀▄▀▄▀▄▀▄▘
",
        ],
        &[
            r"
 1111111111111
111111111111111
222222222222222
",
            r"
 1111111111111
111111111111111
222222222222222
",
        ],
    )
    .critter()
    .tint("1:kKrybg 2:wyk")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.01, 0.03)
    .fps(0.20)
    .weight(3)
    .biomes(KELP | OCEAN | REEF | ARCTIC)
    .traits(GRAZER | DEFENDED)
    .home("rock"),
    Species::new(
        "summer-flounder",
        &[
            r"
  ◢▂▄▆▇▇▇▇▇▇▇▇▆▄▂ ◉◉
◀▐▒░▒◦▒▒░▒▓▒░▒▒◦▒░▒▙▌
  ◥░▒▒◦▒░▒▓▒▒░◦▒▒░▒▛▘
   ▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄
",
            r"
  ◢▂▄▆▇▇▇▇▇▇▇▇▆▄▂ ◉◉
◁▐▒░▒◦▒▒░▒▓▒░▒▒◦▒░▒▙▌
  ◥░▒▒◦▒░▒▓▒▒░◦▒▒░▒▛▘
   ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
",
        ],
        &[
            r"
  yyyyyyyyyyyyyyy 44
yyyYykyyYykyYyykyYyyy
  yYyykyYykyyYkyyYyyy
   yyyyyyyyyyyyyyyy
",
            r"
  yyyyyyyyyyyyyyy 44
yyyYykyyYykyYyykyYyyy
  yYyykyYykyyYkyyYyyy
   yyyyyyyyyyyyyyyy
",
        ],
    )
    .tint("1:yykw 2:yrw 3:kwr 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.35)
    .fps(0.50)
    .weight(6)
    .biomes(OCEAN | KELP | REEF)
    .traits(PREDATOR | SHY),
    Species::new(
        "atlantic-halibut",
        &[
            r"
      ▂▃▅▆▇▇▇▇▇▇▇▇▇▇▆▅▃▂
  ◢▗▟▒▓▓▒░▒▓▒▓▓▒▓▓▒░▒▓▒▓▓▙●
◀▐▒▓▓▒░▒▓▒▓▓▒▓▓▒░▒▓▒▓▓▒▓▓▒░▌◗
  ◥▝▜▒▓▓▒░▒▓▒▓▓▒▓▓▒░▒▓▒▓▓▛▘
      ▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄
",
            r"
      ▂▃▅▆▇▇▇▇▇▇▇▇▇▇▆▅▃▂
  ◢▗▟▒▓▓▒░▒▓▒▓▓▒▓▓▒░▒▓▒▓▓▙●
◁▐▒▓▓▒░▒▓▒▓▓▒▓▓▒░▒▓▒▓▓▒▓▓▒░▌◗
  ◥▝▜▒▓▓▒░▒▓▒▓▓▒▓▓▒░▒▓▒▓▓▛▘
      ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
",
        ],
        &[
            r"
      yyyyyyyyyyyyyyyyyy
  yyyyyyyYyyyyyyyyyYyyyyyy4
yyyyyyYyyyyyyyyyYyyyyyyyyyYyy
  yyyyyyyYyyyyyyyyyYyyyyyyy
      yyyyyyyyyyyyyyyyyy
",
            r"
      yyyyyyyyyyyyyyyyyy
  yyyyyyyYyyyyyyyyyYyyyyyy4
yyyyyyYyyyyyyyyyYyyyyyyyyyYyy
  yyyyyyyYyyyyyyyyyYyyyyyyy
      yyyyyyyyyyyyyyyyyy
",
        ],
    )
    .tint("4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.08, 0.25)
    .fps(0.40)
    .weight(3)
    .biomes(ARCTIC | OCEAN | KELP)
    .traits(PREDATOR),
    Species::new(
        "common-sole",
        &[
            r"
  ▂▅▇▇▇▇▇▇▅▂
◀▐▒░▒▒░▒▒░▒▒▙●
  ▀▄▀▄▀▄▀▄▀▄
",
            r"
  ▂▅▇▇▇▇▇▇▅▂
◁▐▒░▒▒░▒▒░▒▒▙●
  ▄▀▄▀▄▀▄▀▄▀
",
        ],
        &[
            r"
  yyyyyyyyyy
yyywyywyywyyy4
  yyyyyyyyyy
",
            r"
  yyyyyyyyyy
yyywyywyywyyy4
  yyyyyyyyyy
",
        ],
    )
    .tint("1:ykw 2:ykw 3:kwy 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.30)
    .fps(0.50)
    .weight(5)
    .biomes(KELP | OCEAN | ARCTIC)
    .traits(SHY | NOCTURNAL),
    Species::new(
        "peacock-flounder",
        &[
            r"
  ◢▂▄▆▇▇▇▇▇▇▇▇▆▄▂ ◉◉
◀▐▒◎▒░▒○▒◎▒░▒○▒◎▒░▒▙▌
  ◥░▒○▒◎▒░▒▒○▒◎▒░▒▒▛▘
   ▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄
",
            r"
  ◢▂▄▆▇▇▇▇▇▇▇▇▆▄▂ ◉◉
◁▐▒◎▒░▒○▒◎▒░▒○▒◎▒░▒▙▌
  ◥░▒○▒◎▒░▒▒○▒◎▒░▒▒▛▘
   ▄▀▄▀▄▀▄▀▄▀▄▀▄▀▄▀
",
        ],
        &[
            r"
  bbbbbbbbbbbbbbb 44
bbbCbBbcbCbBbcbCbBbbb
  bBbcbCbBbbcbCbBbbbb
   bbbbbbbbbbbbbbbb
",
            r"
  bbbbbbbbbbbbbbb 44
bbbCbBbcbCbBbcbCbBbbb
  bBbcbCbBbbcbCbBbbbb
   bbbbbbbbbbbbbbbb
",
        ],
    )
    .tint("4:yY")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.30)
    .fps(0.50)
    .weight(3)
    .biomes(REEF)
    .traits(SHY | PREDATOR),
    Species::new(
        "stargazer",
        &[
            r"
            ▗●●▖
   ▗▄▄▄▟▓▒▓▒▓▒▓▙▄▀▔
◀▐▓▒▓◦▓▒▓▒◦▓▒▓▒▓◦▒▌
",
            r"
            ▗◉◉▖
   ▗▄▄▄▟▓▒▓▒▓▒▓▙▄▀▔
◁▐▓▒▓◦▓▒▓▒◦▓▒▓▒▓◦▒▌
",
        ],
        &[
            r"
            1441
   11111111111111WW
11111y1111y11111y11
",
            r"
            1441
   11111111111111WW
11111y1111y11111y11
",
        ],
    )
    .tint("1:kykw")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.05, 0.15)
    .fps(0.40)
    .weight(3)
    .biomes(REEF | OCEAN | KELP)
    .traits(PREDATOR | SHY | NOCTURNAL),
    Species::new(
        "bristlenose-pleco",
        &[
            r"
     ▗▟▛▜▙▖
 ◢▃▟▒▓▒▓▒▓▒▓▙▄▖
◀▐▓▒▓▒▓▒▓▒▓▒▓▒█●⠒
 ◥▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
     ▗▙▜▛▟▖
 ◢▃▟▒▓▒▓▒▓▒▓▙▄▖
◁▐▓▒▓▒▓▒▓▒▓▒▓▒█●⠒
 ◥▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
     222222
 11111111111111
11131313131313143
 11111111111111
",
            r"
     222222
 11111111111111
11131313131313143
 11111111111111
",
        ],
    )
    .tint("1:kykb 2:kyw 3:wyyY")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.05, 0.15)
    .fps(0.40)
    .weight(6)
    .biomes(TANK | FRESH)
    .traits(GRAZER | NOCTURNAL | SHY)
    .home("rock,wreck"),
    Species::new(
        "bronze-corydoras",
        &[
            r"
 ◢  ▗▄▆▆▆▃▖
◀▌▟▓▒▓▒▓▒▓▙●⠒
 ◥  ▝▀▀▀▀▀▘
",
            r"
    ▗▄▆▆▆▃▖
◁▌▟▓▒▓▒▓▒▓▙●⠒
    ▝▀▀▀▀▀▘
",
        ],
        &[
            r"
 2  1111111
2211111111142
 2  1111111
",
            r"
    1111111
2211111111142
    1111111
",
        ],
    )
    .tint("1:yYyr 2:wy 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.30)
    .fps(0.50)
    .school(3, 6)
    .weight(7)
    .biomes(TANK | FRESH)
    .traits(SCAVENGER | SHY)
    .home("rock"),
    Species::new(
        "red-lipped-batfish",
        &[
            r"
    ▗▄▆▆▆▆▄▖
 ◢▃▟▒▒▒▒▒●▒▙▖
◀▐▓▓▒▒▒▒▒▒▓▓▙◖▓◗
  ╱▌╲  ╱▐╲
",
            r"
    ▗▄▆▆▆▆▄▖
 ◢▃▟▒▒▒▒▒●▒▙▖
◁▐▓▓▒▒▒▒▒▒▓▓▙◖▓◗
  ╲▌╱  ╲▐╱
",
        ],
        &[
            r"
    11111111
 111111114111
1111111111111RRR
  111  111
",
            r"
    11111111
 111111114111
1111111111111RRR
  111  111
",
        ],
    )
    .tint("1:wyyk 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.03, 0.10)
    .fps(0.35)
    .weight(2)
    .biomes(REEF)
    .traits(PREDATOR | SHY),
    Species::new(
        "sea-robin",
        &[
            r"
     ▗▲▲▲▖
◀▐▓▓▓▒▒▒▒▒▓●▙
 ◥▀▜▒▒▒▒▒▒▒▒▛▀
  ╿╽╿╽ ╿╽╿╽
",
            r"
     ▗▲▲▲▖
◁▐▓▓▓▒▒▒▒▒▓●▙
 ◥▀▜▒▒▒▒▒▒▒▒▛▀
  ╽╿╽╿ ╽╿╽╿
",
        ],
        &[
            r"
     11111
1111111111111
 1122222222221
  2222 2222
",
            r"
     11111
1111111111111
 1122222222221
  2222 2222
",
        ],
    )
    .tint("1:rRyk 2:bBgc 4:wW")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.06, 0.20)
    .fps(0.40)
    .weight(4)
    .biomes(KELP | OCEAN)
    .traits(SCAVENGER),
    Species::new(
        "hellbender",
        &[
            r"
              ▂▄▆▇▇▇▇▆▄▂
◀▄▄▃▃▄▄▟▓▒▓▒▓▒▓▒▓▒▓▒▓▒▙●▌
◥▀▀▀▀▀▀▜▒▓▒▓▒▓▒▓▒▓▒▓▒▓▛▀
   ╱▙▖           ╱▙▖
",
            r"
              ▂▄▆▇▇▇▇▆▄▂
◀▃▃▄▄▃▃▟▓▒▓▒▓▒▓▒▓▒▓▒▓▒▙●▌
◥▀▀▀▀▀▀▜▒▓▒▓▒▓▒▓▒▓▒▓▒▓▛▀
   ▗▙▖           ▗▙▖
",
        ],
        &[
            r"
              1111111111
2222222112121212121212141
222222212121212121212111
   111           111
",
            r"
              1111111111
2222222112121212121212141
222222212121212121212111
   111           111
",
        ],
    )
    .tint("1:yykw 2:yky 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.30)
    .weight(3)
    .biomes(FRESH)
    .traits(PREDATOR | NOCTURNAL | SHY)
    .home("rock"),
    Species::new(
        "neon-goby",
        &[
            r"
  ▂▅▄
◀▟▓▓▓●
",
            r"
  ▂▅▄
◁▟▓▓▓●
",
        ],
        &[
            r"
  111
111114
",
            r"
  111
111114
",
        ],
    )
    .tint("1:cCbC")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.10, 0.30)
    .fps(0.50)
    .school(1, 3)
    .weight(4)
    .biomes(REEF | TANK)
    .traits(SHY)
    .home("coral,rock"),
    Species::new(
        "tripod-fish",
        &[
            r"
    ▗▄▄▄▖
≈≈≈◀▐▓▓▟●
     ╱│╲
    ╱ │ ╲
   ╱  │  ╲
",
            r"
    ▗▄▄▄▖
∼∼∼◁▐▓▓▟●
     ╱│╲
    ╱ │ ╲
   ╱  │  ╲
",
        ],
        &[
            r"
    11111
222211114
     222
    2 2 2
   2  2  2
",
            r"
    11111
222211114
     222
    2 2 2
   2  2  2
",
        ],
    )
    .tint("1:kwKw 2:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.01, 0.03)
    .fps(0.30)
    .weight(2)
    .biomes(DEEP)
    .traits(FILTER),
    Species::new(
        "garden-eel",
        &[
            r"
 ▗▟●
 ▐█
▐█
 ▐█
▐█
 ▐█
",
            r"
▗▟●
▐█
 ▐█
▐█
 ▐█
 ▐█
",
        ],
        &[
            r"
 114
 11
11
 11
11
 11
",
            r"
114
11
 11
11
 11
 11
",
        ],
    )
    .tint("1:wyyk 2:kK 4:kK")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.01, 0.03)
    .fps(0.25)
    .school(3, 6)
    .weight(5)
    .biomes(REEF | OCEAN)
    .traits(FILTER | SHY),
    Species::new(
        "giant-isopod",
        &[
            r"
   ▂▅▇▇▇▇▇▇▇▇▅▂  ╱
 ▟█▌▐█▌▐█▌▐█▌▐█▙▟▌●
▐▓▒▓▒▓▒▓▒▓▒▓▒▓▒█▛
 ▝╿▀╿▀╿▀╿▀╿▀╿▀╿▀▘
",
            r"
   ▂▅▇▇▇▇▇▇▇▇▅▂  ╱
 ▟█▌▐█▌▐█▌▐█▌▐█▙▟▌●
▐▓▒▓▒▓▒▓▒▓▒▓▒▓▒█▛
 ▝▀╿▀╿▀╿▀╿▀╿▀╿▀╿▘
",
        ],
        &[
            r"
   111111111111  1
 111111111111111114
11111111111111111
 1111111111111111
",
            r"
   111111111111  1
 111111111111111114
11111111111111111
 1111111111111111
",
        ],
    )
    .critter()
    .tint("1:wWyk")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.06)
    .fps(0.30)
    .weight(4)
    .biomes(DEEP)
    .traits(SCAVENGER)
    .home("whale,rock"),
    Species::new(
        "sea-spider",
        &[
            r"
   ╱╲      ╱╲
  ╱  ╲▟●▙▶╱  ╲
 ╱   ╱   ╲    ╲
╱   ╱     ╲    ╲
",
            r"
   ╱╲      ╱╲
  ╱  ╲▟●▙▶╱  ╲
 ╱    ╲ ╱     ╲
╱      ╳       ╲
",
        ],
        &[
            r"
   11      11
  1  114111  1
 1   1   1    1
1   1     1    1
",
            r"
   11      11
  1  114111  1
 1    1 1     1
1      1       1
",
        ],
    )
    .critter()
    .tint("1:wWyr")
    .zone(Zone::Bottom)
    .swim(Swim::Crawl)
    .speed(0.02, 0.07)
    .fps(0.35)
    .weight(3)
    .biomes(ARCTIC | DEEP | KELP)
    .traits(SHY)
    .home("rock"),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_species_drawn(super::SPECIES);
    }
}
