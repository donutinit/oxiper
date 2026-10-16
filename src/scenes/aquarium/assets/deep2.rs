//! Deep-sea and polar fish, continued.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "ribbon-sawtail-fish",
        &[
            r"
 ▁▂▃▂▁▂▃▂▁▂▃▂▁▂▃▂▁▂▃
◀◀▓▒░▒▓▒░▒▓▒░▒▓▒░▒●▶
 ▽▾▽▾▽▾▽▾▽▾▽▾▽▾▽▾▽▾▽
",
            r"
 ▃▂▁▂▃▂▁▂▃▂▁▂▃▂▁▂▃▂▁
◁◀▒░▒▓▒░▒▓▒░▒▓▒░▒▓●▶
 ▾▽▾▽▾▽▾▽▾▽▾▽▾▽▾▽▾▽▾
",
        ],
        &[r"
 6666666666666666666
66111111111111111141
 WWWWWWWWWWWWWWWWWWW
"],
    )
    .tint("4:wWyY 1:bm 6:bm")
    .color('b')
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.2, 0.6)
    .fps(0.3)
    .weight(5)
    .biomes(DEEP)
    .traits(PREDATOR | GLOW),
    Species::new(
        "slickhead",
        &[
            r"
         ▗▄▄▄▄▄▄▄▖
◢◣▂▃▅▇▓▒▒▒▒▒▒▒▒▒●▜▖
◥◤▔▀▀▀▀▀▀░░░░░░░░▛▘
         ▝▀▀▀▀▀▀▀▘
",
            r"
         ▗▄▄▄▄▄▄▄▖
▗◣▂▃▅▇▓▒▒▒▒▒▒▒▒▒●▜▖
▝◤▔▀▀▀▀▀▀░░░░░░░░▛▘
         ▝▀▀▀▀▀▀▀▘
",
        ],
        &[r"
         111111111
6611111111111111411
661111111WWWWWWWW11
         111111111
"],
    )
    .tint("4:wWyY 1:Kb 6:Kb")
    .color('K')
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.2, 0.5)
    .fps(0.3)
    .weight(6)
    .biomes(DEEP)
    .traits(PREDATOR),
    Species::new(
        "bigscale",
        &[
            r"
    ▗▄▄▄▄▄▖
◢◣▟▓◆◇◆◇◆◇●▙
◥◤▜▓◇◆◇◆◇◆▒▛
    ▝▀▀▀▀▀▘
",
            r"
    ▗▄▄▄▄▄▖
▗◣▟▓◆◇◆◇◆◇●▙
▝◤▜▓◇◆◇◆◇◆▒▛
    ▝▀▀▀▀▀▘
",
        ],
        &[r"
    1111111
66111W1W1W41
6611W1W1W111
    1111111
"],
    )
    .tint("4:wWyY 1:rm 6:rm")
    .color('r')
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.2, 0.6)
    .fps(0.3)
    .weight(6)
    .biomes(DEEP)
    .traits(FILTER),
    Species::new(
        "pacific-blackdragon",
        &[
            r"
            ▗▄▄▄▄▄▄▄▄▖
◢◣▂▃▄▅▆▇▓▓▒▒▒▒▒▒▒▒▒▒●▜▙
◥◤▔▀▀▀▀▀▀▜▒◦▒◦▒◦▒◦▒◦▛▼▼
                    │
                    ╰✦
",
            r"
            ▗▄▄▄▄▄▄▄▄▖
▗◣▂▃▄▅▆▇▓▓▒▒▒▒▒▒▒▒▒▒●▜▙
▝◤▔▀▀▀▀▀▀▜◦▒◦▒◦▒◦▒◦▒▛▼▼
                    │
                    ╯✦
",
        ],
        &[r"
            1111111111
66111111111111111111411
66111111111C1C1C1C1C1WW
                    g
                    gY
"],
    )
    .tint("4:wWyY 1:Kb 6:Kb")
    .color('K')
    .zone(Zone::Lower)
    .swim(Swim::Dart)
    .speed(0.2, 0.6)
    .fps(0.25)
    .weight(5)
    .biomes(DEEP)
    .traits(PREDATOR | GLOW),
    Species::new(
        "threadfin-dragonfish",
        &[
            r"
 ∼∼∼∼∼∼∼▖  ▗▄▄▄▄▄▖
≈≈≈≈≈≈◢▓▓▓▒▒▒▒▒▒▜▖
∼∼∼∼∼∼◥▓▒◦▒◦▒◦▒●▐▌
 ≈≈≈≈≈≈▀▀▀▀▀▀▀▀▀▀▘
        │    ╲
       ╰│     ╲
",
            r"
 ≈≈≈≈≈≈≈▖  ▗▄▄▄▄▄▖
∼∼∼∼∼∼◢▓▓▓▒▒▒▒▒▒▜▖
≈≈≈≈≈≈◥▓▒◦▒◦▒◦▒●▐▌
∼∼∼∼∼∼∼▀▀▀▀▀▀▀▀▀▀▘
        ╱    │
       ╱      │
",
        ],
        &[r"
 mmmmmmmm  1111111
mmmmmm111111111111
mmmmmm111C1C1C1411
 mmmmmmm1111111111
        m    m
       mm     m
"],
    )
    .tint("4:wWyY 1:mb 6:mb")
    .color('m')
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.15, 0.45)
    .fps(0.2)
    .weight(3)
    .biomes(DEEP)
    .traits(PREDATOR | GLOW),
    Species::new(
        "hagfish",
        &[
            r"
   ⠐    ⠂   ⠠    ⠐   ⠂
◀◀▓▒░▒▓▒░▒▓▒░▒▓▒░▒▓▒░▒▓●⠶
  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
     ⠂    ⠐   ⠠    ⠂  ⠐
",
            r"
   ⠂    ⠠   ⠐    ⠂   ⠠
◁◀░▒▓▒░▒▓▒░▒▓▒░▒▓▒░▒▓▒░●⠶
  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
     ⠠    ⠂   ⠐    ⠠  ⠂
",
        ],
        &[r"
   C    C   C    C   C
661111111111111111111114W
  1111111111111111111111
     C    C   C    C  C
"],
    )
    .tint("1:mMrR 6:mb")
    .color('m')
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.1, 0.3)
    .fps(0.25)
    .weight(5)
    .biomes(DEEP)
    .traits(SCAVENGER | SHY)
    .home("whale"),
    Species::new(
        "giant-tube-eye",
        &[
            r"
              ◉◉
              ▐▌
◢◣▀▓▒▒▒▒▒▒▒▒▒▒▒▓█▜▖
◥◤▄▓░░░░░░░░░░░▓█▙▘
",
            r"
              ◎◎
              ▐▌
▗◣▀▓▒▒▒▒▒▒▒▒▒▒▒▓█▜▖
▝◤▄▓░░░░░░░░░░░▓█▙▘
",
        ],
        &[r"
              44
              11
6611111111111111111
6611WWWWWWWWWWW1111
"],
    )
    .tint("4:GgwW 1:Kb 6:Kb")
    .color('K')
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.1, 0.35)
    .fps(0.2)
    .weight(3)
    .biomes(DEEP)
    .traits(PREDATOR | GLOW),
    Species::new(
        "snipe-eel",
        &[
            r"
  ⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉
◀◀━━━━━━━━━━━━━━━━━━●━━━━━╸
  ⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠤⠤╰╌╌╌╌
",
            r"
  ⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒⠒
◁◀━━━━━━━━━━━━━━━━━━●━━━━━╸
  ⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠤⠤╰╌╌╌╌
",
        ],
        &[r"
  11111111111111111111
661111111111111111114111111
  1111111111111111111111111
"],
    )
    .tint("4:wWyY 1:Kb 6:Kb")
    .color('K')
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.15, 0.4)
    .fps(0.3)
    .weight(4)
    .biomes(DEEP)
    .traits(PREDATOR),
    Species::new(
        "daggertooth",
        &[
            r"
         ▗▄▄▄▄▄▄▖
◢◣▂▃▅▇▓▒▒▒▒▒▒▒▒●▜▙
◥◤▔▀▀▀▀▀▜▒▒▒▒▒▒▼▼▼
         ▝▀▀▀▀▀▲▲▲
",
            r"
         ▗▄▄▄▄▄▄▖
▗◣▂▃▅▇▓▒▒▒▒▒▒▒▒●▜▙
▝◤▔▀▀▀▀▀▜▒▒▒▒▒▒▼▼▼
         ▝▀▀▀▀▀▲▲▲
",
        ],
        &[r"
         11111111
661111111111111411
661111111111111WWW
         111111WWW
"],
    )
    .tint("4:wWyY 1:Kb 6:Kb")
    .color('K')
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.2, 0.7)
    .fps(0.3)
    .weight(5)
    .biomes(DEEP)
    .traits(PREDATOR),
    Species::new(
        "arctic-char",
        &[
            r"
      ▂▅▇▅▂
◢◣▂▅▇▓▒∙▒∙▒∙▒▒●▜▖
◥◤▔▀▓▓▓▓▓▓▓▓▓▓▓▛▘
         ◥◤
",
            r"
      ▂▅▇▅▂
▗◣▂▅▇▓▒∙▒∙▒∙▒▒●▜▖
▝◤▔▀▓▓▓▓▓▓▓▓▓▓▓▛▘
         ◥◤
",
        ],
        &[r"
      66666
6611111W1W1W11411
6611RRRRRRRRRRR11
         RR
"],
    )
    .tint("1:cbwW 4:wW 6:cb")
    .color('c')
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.5, 1.4)
    .fps(0.3)
    .school(1, 3)
    .weight(7)
    .biomes(ARCTIC | FRESH)
    .traits(PREDATOR),
    Species::new(
        "polar-cod",
        &[
            r"
  ▂▅▂▂▅▂
◢◣▇▓▒▒▒●▜▖
◥◤▔▀░░░░▛▘
",
            r"
  ▂▅▂▂▅▂
▗◣▇▓▒▒▒●▜▖
▝◤▔▀░░░░▛▘
",
        ],
        &[r"
  666666
6611111411
6611WWWW11
"],
    )
    .tint("1:bcwW 4:wW 6:cb")
    .color('c')
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.6, 1.5)
    .fps(0.4)
    .school(4, 9)
    .weight(8)
    .biomes(ARCTIC)
    .traits(FILTER),
    Species::new(
        "crocodile-icefish",
        &[
            r"
     ▲▴▲▴▲▴
◢◣▒░░░░░░░░░░●▀▀▀▀▀▀▜
◥◤▔░░░░░░░░░░░▄▄▄▄▄▄▛
",
            r"
     ▲▴▲▴▲▴
▗◣▒░░░░░░░░░░●▀▀▀▀▀▀▜
▝◤▔░░░░░░░░░░░▄▄▄▄▄▄▛
",
        ],
        &[r"
     WWWWWW
661111111111141111111
661WWWWWWWWWWW1111111
"],
    )
    .tint("4:rRyY 1:Ww 6:Ww")
    .color('W')
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.1, 0.35)
    .fps(0.2)
    .weight(3)
    .biomes(ARCTIC)
    .traits(PREDATOR),
    Species::new(
        "lumpsucker",
        &[
            r"
    ▗▄▆▆▆▆▆▄▖
◢◣▟█▓◍▒◍▒▒▒●▙
◥◤█▓▒◍▒◍▒◍▒▒█
   ▝▙▓▒◍▒◍▒▒▟
    ▝▀▀◎▀▀▀▀▘
",
            r"
    ▗▄▆▆▆▆▆▄▖
▗◣▟█▓◍▒◍▒▒▒●▙
▝◤█▓▒◍▒◍▒◍▒▒█
   ▝▙▓▒◍▒◍▒▒▟
    ▝▀▀◎▀▀▀▀▘
",
        ],
        &[r"
    111111111
66111W1W11141
66111W1W1W111
   1111W1W111
    111W11111
"],
    )
    .tint("1:gcbm 4:yY 6:gc")
    .color('g')
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.1, 0.4)
    .fps(0.2)
    .weight(5)
    .biomes(ARCTIC)
    .traits(SHY | DEFENDED)
    .home("rock"),
    Species::new(
        "atlantic-wolffish",
        &[
            r"
  ▂▃▂▃▂▃▂▃▂▃▂▃▂▗▄▄▄▄▄▄▄▖
◢▓▒▒░▓▒▒░▓▒▒░▓▒▟█▓▒▒▒●▜▙
◥▓▒▒░▓▒▒░▓▒▒░▓▒▜█▓▒▒▒▒▒▛▼▼
               ▝▀▀▀▀▀▀▀▲▲▲
",
            r"
  ▃▂▃▂▃▂▃▂▃▂▃▂▃▗▄▄▄▄▄▄▄▖
◣░▓▒▒░▓▒▒░▓▒▒░▒▟█▓▒▒▒●▜▙
◤░▓▒▒░▓▒▒░▓▒▒░▒▜█▓▒▒▒▒▒▛▼▼
               ▝▀▀▀▀▀▀▀▲▲▲
",
        ],
        &[r"
  6666666666666111111111
611111111111111111111411
611111111111111111111111WW
               11111111WWW
"],
    )
    .tint("1:bwkK 4:yY 6:bm")
    .color('b')
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.2, 0.5)
    .fps(0.25)
    .weight(4)
    .biomes(ARCTIC)
    .traits(PREDATOR | DEFENDED)
    .home("rock"),
    Species::new(
        "capelin",
        &[
            r"
   ▃▅▃
◢◣▅▓▒●▜
◥◤▔▀░░▛
",
            r"
   ▃▅▃
▗◣▅▓▒●▜
▝◤▔▀░░▛
",
        ],
        &[r"
   666
6611141
6611WW1
"],
    )
    .tint("1:cwWC 4:wW 6:Cc")
    .color('C')
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.7, 1.8)
    .fps(0.4)
    .school(6, 12)
    .weight(9)
    .biomes(ARCTIC)
    .traits(FILTER),
    Species::new(
        "shorthorn-sculpin",
        &[
            r"
         ╱╲╱╲╱
     ▗▄▄▄▄▄▄▄▖
◢◣▟▓▒▒▓▒▒▒▒▒▒●▜▖
◥◤▜▓▒░▒░▒▒▒▒▒▒▛▘
       ◥▇▇▇▇▇▇◣
",
            r"
         ╲╱╲╱╲
     ▗▄▄▄▄▄▄▄▖
▗◣▟▓▒▒▓▒▒▒▒▒▒●▜▖
▝◤▜▓▒░▒░▒▒▒▒▒▒▛▘
       ◥▆▆▆▆▆▆◣
",
        ],
        &[r"
         11111
     111111111
6611111111111411
661111WWWWWWWW11
       66666666
"],
    )
    .tint("1:ygrb 4:yY 6:yg")
    .color('y')
    .zone(Zone::Lower)
    .swim(Swim::Dart)
    .speed(0.1, 0.5)
    .fps(0.25)
    .weight(5)
    .biomes(ARCTIC)
    .traits(SHY | PREDATOR)
    .home("rock"),
    Species::new(
        "antarctic-toothfish",
        &[
            r"
        ▂▄▆▆▄▂
     ▗▄▄▄▄▄▄▄▄▄▄▄▖
◢◣▟█▓▒◦▒▒◦▒▒◦▒▒◦▒●▜▖
◥◤▜█▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▛▼
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▲▲
",
            r"
        ▂▄▆▆▄▂
     ▗▄▄▄▄▄▄▄▄▄▄▄▖
▗◣▟█▓▒◦▒▒◦▒▒◦▒▒◦▒●▜▖
▝◤▜█▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▛▼
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▲▲
",
        ],
        &[r"
        666666
     1111111111111
661111W11W11W11W1411
66111111111111111111W
   111111111111111WW
"],
    )
    .tint("1:bwKk 4:wW 6:wW")
    .color('w')
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.2, 0.6)
    .fps(0.25)
    .weight(3)
    .biomes(ARCTIC)
    .traits(PREDATOR),
    Species::new(
        "arctic-cod",
        &[
            r"
   ▂▅▂▂▅▂▂▅▂
◢◣▃▅▇▓▒▒▒▒●▜▖
◥◤▔▀▒░░░░░░▛▘
           ╵
",
            r"
   ▂▅▂▂▅▂▂▅▂
▗◣▃▅▇▓▒▒▒▒●▜▖
▝◤▔▀▒░░░░░░▛▘
           ╷
",
        ],
        &[r"
   666666666
6611111111411
66111WWWWWW11
           W
"],
    )
    .tint("1:bcwy 4:wW 6:bm")
    .color('b')
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.5, 1.3)
    .fps(0.4)
    .school(5, 10)
    .weight(7)
    .biomes(ARCTIC)
    .traits(FILTER),
    Species::new(
        "eelpout",
        &[
            r"
 ▁▁▁▂▂▂▃▃▃▄▄▄▅▅▅▅▅▅▅▅▅▅▅
◀◀▓▒░▒▓▒░▒▓▒░▒▓▒░▒▓▒░▒▓●▜▖
 ▔▔▔▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▛▀▘
",
            r"
 ▁▁▁▂▂▂▃▃▃▄▄▄▅▅▅▅▅▅▅▅▅▅▅
◁◀░▒▓▒░▒▓▒░▒▓▒░▒▓▒░▒▓▒░●▜▖
 ▔▔▔▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▛▀▘
",
        ],
        &[r"
 66666666666666666666666
66111111111111111111111411
 WWWWWWWWWWWWWWWWWWWWWW111
"],
    )
    .tint("1:ywcg 4:wW 6:yg")
    .color('y')
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.15, 0.4)
    .fps(0.3)
    .weight(5)
    .biomes(ARCTIC)
    .traits(SHY)
    .home("rock"),
    Species::new(
        "sea-raven",
        &[
            r"
    ╲╱╲╱╲╱╲╱
    ▗▄▄▄▄▄▄▄▄▖
◢◣▟▓▒▒▒▒▒▒▒▒●▜▖
◥◤▜▓▒░▒░▒░▒▒▒▛▘
  ▝▀▀▀▀▀▀▀▀▀▀▘
",
            r"
    ╱╲╱╲╱╲╱╲
    ▗▄▄▄▄▄▄▄▄▖
▗◣▟▓▒▒▒▒▒▒▒▒●▜▖
▝◤▜▓▒░▒░▒░▒▒▒▛▘
  ▝▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
    66666666
    1111111111
661111111111411
66111W1W1W11111
  111111111111
"],
    )
    .tint("1:rmyR 4:yY 6:rm")
    .color('r')
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.1, 0.4)
    .fps(0.25)
    .weight(4)
    .biomes(ARCTIC)
    .traits(PREDATOR | SHY)
    .home("rock"),
    Species::new(
        "sea-lamprey",
        &[
            r"
 ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
◀◀▒▒∙∙▒▒∙∙▒▒∙∙▒▒∙∙▒▒∙∙▓●◎
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
 ▗▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
◁◀▒▒∙∙▒▒∙∙▒▒∙∙▒▒∙∙▒▒∙∙▓●◎
 ▝▄▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
 11111111111111111111111
6611WW11WW11WW11WW11WW14W
 11111111111111111111111
"],
    )
    .tint("1:kKyg 6:Kb")
    .color('K')
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.2, 0.5)
    .fps(0.3)
    .weight(4)
    .biomes(ARCTIC | FRESH)
    .traits(PREDATOR),
    Species::new(
        "antarctic-silverfish",
        &[
            r"
   ▗▄▄▄▄▖
◢◣█▓▒◇▒●▜▖
◥◤▀▒░◇░▒▛▘
   ▝▀▀▀▀▘
",
            r"
   ▗▄▄▄▄▖
▗◣█▓▒◇▒●▜▖
▝◤▀▒░◇░▒▛▘
   ▝▀▀▀▀▘
",
        ],
        &[r"
   111111
66111W1411
6611WWW111
   111111
"],
    )
    .tint("1:cwWC 4:wW 6:Cc")
    .color('C')
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.5, 1.2)
    .fps(0.4)
    .school(5, 10)
    .weight(7)
    .biomes(ARCTIC)
    .traits(FILTER),
    Species::new(
        "emerald-rockcod",
        &[
            r"
    ▗▄▖▗▄▖
◢◣▟█▓▒▒▒▒▒▒▒●▜▖
◥◤▜█▓▒░▒░▒░▒▒▛▘
   ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
    ▗▄▖▗▄▖
▗◣▟█▓▒▒▒▒▒▒▒●▜▖
▝◤▜█▓▒░▒░▒░▒▒▛▘
   ▝▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
    666666
661111111111411
661111W1W1W1111
   11111111111
"],
    )
    .tint("1:gGcy 4:yY 6:gc")
    .color('g')
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .speed(0.1, 0.4)
    .fps(0.25)
    .weight(5)
    .biomes(ARCTIC)
    .traits(PREDATOR | SHY)
    .home("rock"),
    Species::new(
        "arctic-snailfish",
        &[
            r"
      ░▒▒▒▒▒▒░
⠒⠲⠶░░▒▒▒▒▒▒●▒░▒
⠤⠴⠶░░▒▒▒▒▒▒▒▒▒░
      ░░◎░░░░░
",
            r"
      ░▒▒▒▒▒▒░
⠤⠴⠶░░▒▒▒▒▒▒●▒░▒
⠒⠲⠶░░▒▒▒▒▒▒▒▒▒░
      ░░◎░░░░░
",
        ],
        &[r"
      11111111
666111111114111
666111111111111
      11W11111
"],
    )
    .tint("1:mMwW 4:wW 6:MM")
    .color('M')
    .zone(Zone::Lower)
    .swim(Swim::Drift)
    .speed(0.1, 0.25)
    .fps(0.25)
    .weight(4)
    .biomes(ARCTIC)
    .traits(SHY | SCAVENGER)
    .home("rock"),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_species_drawn(super::SPECIES);
    }
}
