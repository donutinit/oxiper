//! Freshwater fish: tank favorites and the fish of rivers and lakes.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "common-goldfish",
        &[
            r"
  ◢▀▀▀▀▀◣
╲▗▟▓▓▓▓▓▙▖
›▐▒▒▒▒▒▒●▌
╱▝▜░░░░░▛▘
",
            r"
  ◢▀▀▀▀▀◣
─▗▟▓▓▓▓▓▙▖
›▐▒▒▒▒▒▒●▌
─▝▜░░░░░▛▘
",
        ],
        &[r"
  1111111
1111111111
1111111141
1111111111
"],
    )
    .tint("1:yYrR")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .weight(8)
    .traits(SCAVENGER | CURIOUS)
    .biomes(TANK | FRESH),
    Species::new(
        "comet-goldfish",
        &[
            r"
╲╲╲╲▗▄▄▄▄▄▄▄▖
━━━━▐▓▒▒▒▒▒●▌
╱╱╱╱▝░░░░░░░▘
",
            r"
─╲╲╲▗▄▄▄▄▄▄▄▖
━━━╸▐▓▒▒▒▒▒●▌
─╱╱╱▝░░░░░░░▘
",
        ],
        &[r"
1111111111111
1111111111141
1111111111111
"],
    )
    .tint("1:yYrR")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Wave)
    .speed(0.6, 1.5)
    .weight(6)
    .traits(SCAVENGER | CURIOUS)
    .biomes(TANK | FRESH),
    Species::new(
        "fantail-goldfish",
        &[
            r"
╲╲◣  ▗▄▄▄▄▖
 ╲◥▗▟▓▓▓▓▓▙▖
──◀▐▒▒▒▒▒▒●▌
 ╱◢▝▜░░░░░▛▘
╱╱◥  ▝▀▀▀▀▘
",
            r"
─╲◣  ▗▄▄▄▄▖
 ╌◥▗▟▓▓▓▓▓▙▖
──◀▐▒▒▒▒▒▒●▌
 ╌◢▝▜░░░░░▛▘
─╱◥  ▝▀▀▀▀▘
",
        ],
        &[r"
111  111111
 11111111111
111111111141
 11111111111
111  111111
"],
    )
    .tint("1:yYrRwW")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.8)
    .weight(6)
    .traits(SCAVENGER)
    .biomes(TANK | FRESH),
    Species::new(
        "oranda-goldfish",
        &[
            r"
     ⣠⣾⣿⣿⣷⣄
╲▗▟▓▓▓⣿⣿⣿⣿▙▖
›▐▒▒▒▒▒⣿⣿⣿●▌
╱▝▜░░░░░░░▛▘
",
            r"
     ⣠⣾⣿⣿⣷⣄
─▗▟▓▓▓⣿⣿⣿⣿▙▖
›▐▒▒▒▒▒⣿⣿⣿●▌
─▝▜░░░░░░░▛▘
",
        ],
        &[r"
     222222
111111222211
111111122241
111111111111
"],
    )
    .tint("1:wWyY 2:rR")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(5)
    .traits(SCAVENGER)
    .biomes(TANK | FRESH),
    Species::new(
        "ranchu",
        &[
            r"
   ▗▄⣤⣶⣶⣤▄▖
╲▗▟▓⣿⣿⣿⣿▓▙▖
›▐▒▒⣿⣿⣿⣿▒●▌
╱▝▜░░░░░░▛▘
  ▝▀▀▀▀▀▀▘
",
            r"
   ▗▄⣤⣶⣶⣤▄▖
─▗▟▓⣿⣿⣿⣿▓▙▖
›▐▒▒⣿⣿⣿⣿▒●▌
─▝▜░░░░░░▛▘
  ▝▀▀▀▀▀▀▘
",
        ],
        &[r"
   11222211
11112222111
11112222141
11111111111
  11111111
"],
    )
    .tint("1:yYrRwW 2:rR")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(3)
    .traits(SCAVENGER)
    .biomes(TANK | FRESH),
    Species::new(
        "bubble-eye-goldfish",
        &[
            r"
   ▗▄▄▄▄▄▄▄▖
╲◣▗▟▓▓▓▓▓▓▙▖
══◀▐▒▒▒▒▒▒●▌
╱◥▝▜░░░░░░▛◯
          ○
",
            r"
   ▗▄▄▄▄▄▄▄▖
─◣▗▟▓▓▓▓▓▓▙▖
══◀▐▒▒▒▒▒▒●▌
─◥▝▜░░░░░░▛◯
          ○
",
        ],
        &[r"
   111111111
111111111111
111111111141
111111111112
          2
"],
    )
    .tint("1:yYrRwW 2:wWcC")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.6)
    .weight(3)
    .traits(SCAVENGER)
    .biomes(TANK | FRESH),
    Species::new(
        "black-moor",
        &[
            r"
◢▒▓◣ ▗▄▄▄▄▖
▒▓▓▓▟▓▓▓▓▓▙▖
▒▒▒▒▐▓▒▒▒▒◖◉◗
▒▓▓▓▜▓▓▓▓▓▛▘
◥▒▓◤ ▝▀▀▀▀▘
",
            r"
◣▒▓◢ ▗▄▄▄▄▖
▒▓▓▓▟▓▓▓▓▓▙▖
▒▒▒▒▐▓▒▒▒▒◖◉◗
▒▓▓▓▜▓▓▓▓▓▛▘
◤▒▓◥ ▝▀▀▀▀▘
",
        ],
        &[r"
1111 111111
111111111111
1111111111141
111111111111
1111 111111
"],
    )
    .tint("1:K")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.25, 0.7)
    .weight(4)
    .traits(SCAVENGER)
    .biomes(TANK | FRESH),
    Species::new(
        "kohaku-koi",
        &[
            r"
  ╲╲  ◢▀▀▀▀▀▀▀▀◣
═╲◣▗▟▓▓▓▓▓▓▓▓▓▓▙▖
═══◀▐▓▒▒▒▒▒▒▒▒▒●▌⠒
═╱◥▝▜░░░░░░░░░░▛▘
  ╱╱ ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
  ╱╱  ◢▀▀▀▀▀▀▀▀◣
═╱◥▗▟▓▓▓▓▓▓▓▓▓▓▙▖
═══◀▐▓▒▒▒▒▒▒▒▒▒●▌⠒
═╲◣▝▜░░░░░░░░░░▛▘
  ╲╲ ▝▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
  11  1122221111
11111112222211111
111111222111222411
11111111222221111
  11 11111111111
"],
    )
    .tint("1:wW 2:rR")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .traits(SCAVENGER | CURIOUS)
    .biomes(TANK | FRESH),
    Species::new(
        "showa-koi",
        &[
            r"
  ╲╲  ◢▀▀▀▀▀▀▀▀◣
═╲◣▗▟▓▓▓▓▓▓▓▓▓▓▙▖
═══◀▐▓▒▒▒▒▒▒▒▒▒●▌⠒
═╱◥▝▜░░░░░░░░░░▛▘
  ╱╱ ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
  ╱╱  ◢▀▀▀▀▀▀▀▀◣
═╱◥▗▟▓▓▓▓▓▓▓▓▓▓▙▖
═══◀▐▓▒▒▒▒▒▒▒▒▒●▌⠒
═╲◣▝▜░░░░░░░░░░▛▘
  ╲╲ ▝▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
  11  11KK112211
111112211KK112211
111111KK1122211411
1111111KK11222111
  11 11KK1122111
"],
    )
    .tint("1:wW 2:rR")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(4)
    .traits(SCAVENGER | CURIOUS)
    .biomes(TANK | FRESH),
    Species::new(
        "veiltail-betta",
        &[
            r"
     ╱╱╱◢▄▄▄▄◣
╲╲╲╲╲╲▗▟█████▙▖
══════◀▐▓▒▒▒▒●▌
╱╱╱╱╱╱▝▜█████▛▘
  ╱╱╱╱╱╱◥▀▀▀▀◤
",
            r"
     ╲╲╲◢▄▄▄▄◣
╱╱╱╱╱╱▗▟█████▙▖
══════◀▐▓▒▒▒▒●▌
╲╲╲╲╲╲▝▜█████▛▘
  ╲╲╲╲╲╲◥▀▀▀▀◤
",
        ],
        &[r"
     222222222
222222111111111
222222211111141
222222111111111
  222222222222
"],
    )
    .tint("1:rRbBmM 2:rRbBmM")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(5)
    .traits(PUFF)
    .alt(
        &[r"
  ╲  ╲   ╱╱◢▄▄▄▄◣
╲╲╲╲╲╲╲╲╲▗▟█████▙▖
═════════◀▐▓▒▒▒▒●▌
╱╱╱╱╱╱╱╱╱▝▜█████▛▘
  ╱╱╱╱╱╱╱╱╱◥▀▀▀▀◤
"],
        &[r"
  2  2   22222222
222222222111111111
222222222211111141
222222222111111111
  222222222222222
"],
    )
    .home("lily")
    .biomes(TANK | FRESH),
    Species::new(
        "halfmoon-betta",
        &[
            r"
  ╲ │ ╱    ◢▄▄▄◣
 ╲ ╲│╱ ╱▗▟█████▙▖
═══════◀▐▓▒▒▒▒▒●▌
 ╱ ╱│╲ ╲▝▜█████▛▘
  ╱ │ ╲    ◥▀▀▀◤
",
            r"
 ╲  ╎  ╱   ◢▄▄▄◣
 ╲ ╲╎╱ ╱▗▟█████▙▖
═══════◀▐▓▒▒▒▒▒●▌
 ╱ ╱╎╲ ╲▝▜█████▛▘
 ╱  ╎  ╲   ◥▀▀▀◤
",
        ],
        &[
            r"
  2 2 2    22222
 2 222 2111111111
22222222111111141
 2 222 2111111111
  2 2 2    22222
",
            r"
 2  2  2   22222
 2 222 2111111111
22222222111111141
 2 222 2111111111
 2  2  2   22222
",
        ],
    )
    .tint("1:rRbBmM 2:rRbBmM")
    .fps(0.3)
    .zone(Zone::Upper)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .weight(5)
    .traits(PUFF)
    .alt(
        &[r"
╲ ╲  │  ╱ ╱  ◢▄▄▄▄◣
 ╲ ╲ ╲│╱ ╱▗▟█████▙▖
═════════◀▐▓▒▒▒▒▒●▌
 ╱ ╱ ╱│╲ ╲▝▜█████▛▘
╱ ╱  │  ╲ ╲  ◥▀▀▀▀◤
"],
        &[r"
2 2  2  2 2  222222
 2 2 222 2111111111
2222222222111111141
 2 2 222 2111111111
2 2  2  2 2  222222
"],
    )
    .home("lily")
    .biomes(TANK | FRESH),
    Species::new(
        "guppy",
        &[
            r"
◢▒▓▓▙▗▄▄▖
▒▓▒▒▒▐▓●▌
◥▒▓▓▛▝▀▀▘
",
            r"
◣▒▓▓▙▗▄▄▖
▓▒▒▒▒▐▓●▌
◤▒▓▓▛▝▀▀▘
",
        ],
        &[r"
666661111
666661141
666661111
"],
    )
    .tint("1:yYcCmM 6:rRbBgG")
    .fps(0.4)
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.4, 1.2)
    .school(3, 8)
    .weight(8)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "endlers-livebearer",
        &[
            r"
◢▓▙▄▄▖
◥▒▛▐●▌
",
            r"
◣▓▙▄▄▖
◤▒▛▐●▌
",
        ],
        &[r"
666CCG
666C4G
"],
    )
    .tint("6:rRyY")
    .fps(0.4)
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.4, 1.3)
    .school(3, 8)
    .weight(5)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "molly",
        &[
            r"
    ╭┬┬┬┬┬┬╮
◥◣▗▟██████▙▖
═◀▐▓▒▒▒▒▒▒●▌
◢◤▝▜░░░░░░▛▘
",
            r"
    ╭┬┬┬┬┬┬╮
─◣▗▟██████▙▖
═◀▐▓▒▒▒▒▒▒●▌
─◥▝▜░░░░░░▛▘
",
        ],
        &[r"
    22222222
111111111111
111111111141
111111111111
"],
    )
    .tint("1:KyYwW 2:KyYwW")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .weight(5)
    .traits(GRAZER | SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "platy",
        &[
            r"
 ▗▄▄▄▄▄▄▖
◥▟▓▓▓▓▓▙▖
›▐▒▒▒▒▒●▌
◢▜░░░░░▛▘
 ▝▀▀▀▀▀▀▘
",
            r"
 ▗▄▄▄▄▄▄▖
─▟▓▓▓▓▓▙▖
›▐▒▒▒▒▒●▌
─▜░░░░░▛▘
 ▝▀▀▀▀▀▀▘
",
        ],
        &[r"
 11111111
111111111
111111141
111111111
 11111111
"],
    )
    .tint("1:rRyYbB")
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.35, 1.0)
    .school(2, 6)
    .weight(7)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "swordtail",
        &[
            r"
       ▗▄▄▄▄▖
     ╲◀▐▒▒▒●▌
╺━━━━━◣▝░░░░▘
",
            r"
       ▗▄▄▄▄▖
     ╱◀▐▒▒▒●▌
╶╌╌╌╌╌◣▝░░░░▘
",
        ],
        &[r"
       111111
     66111141
6666666111111
"],
    )
    .tint("1:rRyYgG 6:rRKyY")
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.4, 1.2)
    .school(1, 4)
    .weight(5)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "neon-tetra",
        &[
            r"
 ▗▄▄▄▄▖
◀▐━━━●▌
 ▝▀▀▀▀▘
",
            r"
 ▗▄▄▄▄▖
◁▐━━━●▌
 ▝▀▀▀▀▘
",
        ],
        &[r"
 bbbbbb
r111141
 rrrrrr
"],
    )
    .tint("1:cCbB")
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.5, 1.4)
    .school(5, 12)
    .weight(8)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "cardinal-tetra",
        &[
            r"
 ▗▄▄▄▄▄▖
◀▐━━━━●▌
 ▝▀▀▀▀▀▘
",
            r"
 ▗▄▄▄▄▄▖
◁▐━━━━●▌
 ▝▀▀▀▀▀▘
",
        ],
        &[r"
 bbbbbbb
rCCCCC4C
 RRRRRRR
"],
    )
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.5, 1.4)
    .school(5, 12)
    .weight(6)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "black-skirt-tetra",
        &[
            r"
  ▗▄▄▄▄▖
◀▐▓▒▒▒●▌
 ◢▓▓▓▓▓◣
  ◥▓▓▓◤
",
            r"
  ▗▄▄▄▄▖
◁▐▓▒▒▒●▌
 ◢▓▓▓▓▓◣
  ◥▓▓▓◤
",
        ],
        &[r"
  111111
11KK1141
 KKKKKKK
  KKKKK
"],
    )
    .tint("1:wWyY")
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.4, 1.1)
    .school(4, 9)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "rummy-nose-tetra",
        &[
            r"
◥▗▄▄▄▄▄▖
◀▐▓▒▒▒●▌
◢▝▀▀▀▀▀▘
",
            r"
◤▗▄▄▄▄▄▖
◁▐▓▒▒▒●▌
◣▝▀▀▀▀▀▘
",
        ],
        &[r"
w111111R
K111114R
w111111R
"],
    )
    .tint("1:wy")
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.5, 1.2)
    .school(6, 12)
    .weight(6)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "zebra-danio",
        &[
            r"
◥▗▄▄▄▄▄▖
◀▐▬▬▬▬●▌
◢▝▀▀▀▀▀▘
",
            r"
◤▗▄▄▄▄▄▖
◁▐▬▬▬▬●▌
◣▝▀▀▀▀▀▘
",
        ],
        &[r"
wBBBBBBB
wwBwBw4w
wBBBBBBB
"],
    )
    .fps(0.4)
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .speed(0.8, 2.0)
    .school(5, 12)
    .weight(8)
    .biomes(TANK | FRESH),
    Species::new(
        "harlequin-rasbora",
        &[
            r"
◥▗▄▄▄▄▄▄▖
◀▐▒◢███●▌
◢▝▀◥███◤▘
",
            r"
◤▗▄▄▄▄▄▄▖
◁▐▒◢███●▌
◣▝▀◥███◤▘
",
        ],
        &[r"
611111111
611KKKK41
611KKKKK1
"],
    )
    .tint("1:yYrR 6:rR")
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Cruise)
    .speed(0.4, 1.0)
    .school(5, 10)
    .weight(6)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "cherry-barb",
        &[
            r"
 ▗▄▄▄▄▄▄▖
◥▗▟▓▓▓▓▙▖
◀▐▓▬▬▒▒●▌
◢▝▜░░░░▛▘
",
            r"
 ▗▄▄▄▄▄▄▖
◤▗▟▓▓▓▓▙▖
◁▐▓▬▬▒▒●▌
◣▝▜░░░░▛▘
",
        ],
        &[r"
 11111111
111111111
111KK1141
111111111
"],
    )
    .tint("1:rR")
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.4, 1.1)
    .school(3, 8)
    .weight(6)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "tiger-barb",
        &[
            r"
   ◢▀▀▀▀◣
◥▗▟▓▓▓▓▙▖
◀▐▓▓▓▓▓●▌
◢▝▜▓▓▓▓▛▘
   ◥▀▀▀▀◤
",
            r"
   ◢▀▀▀▀◣
◤▗▟▓▓▓▓▙▖
◁▐▓▓▓▓▓●▌
◣▝▜▓▓▓▓▛▘
   ◥▀▀▀▀◤
",
        ],
        &[r"
   K1K111
111K1K111
111K1K141
111K1K111
   K1K111
"],
    )
    .tint("1:yY")
    .fps(0.4)
    .zone(Zone::Middle)
    .swim(Swim::Dart)
    .speed(0.5, 1.6)
    .school(4, 9)
    .weight(6)
    .biomes(TANK | FRESH),
    Species::new(
        "white-cloud-minnow",
        &[
            r"
◥▐▬▬▬▬●▌
◢▝▀▀▀▀▀▘
",
            r"
◤▐▬▬▬▬●▌
◣▝▀▀▀▀▀▘
",
        ],
        &[r"
6wbbbb4W
6wwwwwwW
"],
    )
    .tint("6:rR")
    .fps(0.4)
    .zone(Zone::Upper)
    .swim(Swim::Cruise)
    .speed(0.4, 1.1)
    .school(5, 12)
    .weight(5)
    .traits(SHY)
    .biomes(TANK | FRESH),
    Species::new(
        "freshwater-angelfish",
        &[
            r"
╲        ◢◣
 ╲      ◢██◣
  ╲    ◢████◣
   ╲  ◢██████◣
═◀▐▓▓▓▓▓▓▒▒▒●▌
   ╱  ◥██████◤
  ╱    ◥████◤
 ╱      ◥██◤
╱        ◥◤
",
            r"
╲        ◢◣
 ╲      ◢██◣
  ╲    ◢████◣
   ╲  ◢██████◣
─◀▐▓▓▓▓▓▓▒▒▒●▌
   ╱  ◥██████◤
  ╱    ◥████◤
 ╱      ◥██◤
╱        ◥◤
",
        ],
        &[r"
1        11
 1      K11K
  1    1K11K1
   1  11K11K11
11111111K11K41
   1  11K11K11
  1    1K11K1
 1      K11K
1        11
"],
    )
    .tint("1:wWyY")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.7)
    .school(1, 3)
    .weight(6)
    .traits(PREDATOR)
    .home("amazon,vallisneria")
    .biomes(TANK | FRESH),
    Species::new(
        "discus",
        &[
            r"
      ▗▄▄▄▄▄▄▖
    ▗▟▓≈∼≈∼≈∼▓▙▖
 ◥◣▐▒≈∼≈∼≈∼≈∼≈∼▒▌
══◀▐▒∼≈∼≈∼≈∼≈∼≈●▌
 ◢◤▐▒≈∼≈∼≈∼≈∼≈∼▒▌
    ▝▜▓≈∼≈∼≈∼▓▛▘
      ▝▀▀▀▀▀▀▘
",
            r"
      ▗▄▄▄▄▄▄▖
    ▗▟▓≈∼≈∼≈∼▓▙▖
 ╲◣▐▒≈∼≈∼≈∼≈∼≈∼▒▌
──◀▐▒∼≈∼≈∼≈∼≈∼≈●▌
 ╱◥▐▒≈∼≈∼≈∼≈∼≈∼▒▌
    ▝▜▓≈∼≈∼≈∼▓▛▘
      ▝▀▀▀▀▀▀▘
",
        ],
        &[r"
      11111111
    111222222111
 1111222222222211
11111222222222241
 1111222222222211
    111222222111
      11111111
"],
    )
    .tint("1:rRyYcC 2:bBmMgG")
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Hover)
    .speed(0.25, 0.6)
    .weight(4)
    .traits(SHY)
    .home("log")
    .biomes(TANK | FRESH),
    Species::new(
        "oscar",
        &[
            r"
    ◢▀▀▀▀▀▀▀▀▀◣
╲◣ ▗▟▓▓▓▓▓▓▓▓▓▙▖
═◀▐◉▒▓▒▓▒▒▓▒▒▓▒●◗
╱◥ ▝▜▓▓▓▓▓▓▓▓▓▛▘
    ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
    ◢▀▀▀▀▀▀▀▀▀◣
─◣ ▗▟▓▓▓▓▓▓▓▓▓▙▖
═◀▐◉▒▓▒▓▒▒▓▒▒▓▒●◗
─◥ ▝▜▓▓▓▓▓▓▓▓▓▛▘
    ▝▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[r"
    11111111111
11 1122112211211
11121121121121141
11 1112211221111
    11221122111
"],
    )
    .tint("1:Kw 2:rRyY")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Cruise)
    .speed(0.25, 0.9)
    .weight(4)
    .traits(PREDATOR | CURIOUS)
    .biomes(TANK | FRESH),
    Species::new(
        "convict-cichlid",
        &[
            r"
  ◢▀▀▀▀▀◣
╲◣▟▓▓▓▓▓▙▖
═◀▐▓▓▓▓▓●▌
╱◥▜░░░░░▛▘
",
            r"
  ◢▀▀▀▀▀◣
─◣▟▓▓▓▓▓▙▖
═◀▐▓▓▓▓▓●▌
─◥▜░░░░░▛▘
",
        ],
        &[r"
  1K1K111
111K1K1111
111K1K1141
111K1K1111
"],
    )
    .tint("1:wWcC")
    .fps(0.3)
    .zone(Zone::Lower)
    .swim(Swim::Wave)
    .speed(0.3, 0.9)
    .weight(5)
    .home("rock,log")
    .biomes(TANK | FRESH),
    Species::new(
        "electric-yellow-cichlid",
        &[
            r"
   ◢▀▀▀▀▀▀◣
╲◣▗▟▓▓▓▓▓▓▙▖
══◀▐▒▒▒▒▒▒●▌
╱◥▝▜░░░░░░▛▘
   ◥▀▀▀▀▀▀◤
",
            r"
   ◢▀▀▀▀▀▀◣
─◣▗▟▓▓▓▓▓▓▙▖
──◀▐▒▒▒▒▒▒●▌
─◥▝▜░░░░░░▛▘
   ◥▀▀▀▀▀▀◤
",
        ],
        &[r"
   KKKKKKKK
YYYYYYYYYYYY
YYYYYYYYYY4Y
YYYYYYYYYYYY
   KKKKKKKK
"],
    )
    .fps(0.3)
    .zone(Zone::Middle)
    .swim(Swim::Wave)
    .speed(0.3, 1.0)
    .school(1, 3)
    .weight(5)
    .traits(GRAZER)
    .home("rock")
    .biomes(TANK | FRESH),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_species_drawn(super::SPECIES);
    }
}
