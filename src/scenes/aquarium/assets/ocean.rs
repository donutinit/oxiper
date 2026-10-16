//! Open-ocean and coastal fish: tunas, billfish, mackerel, sardines, kelp-forest fish.
//! Drawn in Unicode, facing right; the left-facing version is mirrored.

use super::*;

pub const SPECIES: &[Species] = &[
    Species::new(
        "bluefin-tuna",
        &[
            r"
       ◢▙
◣    ▂▄▟▓▓▓▙▄▂
 ◥▙▾▾▟▓▓▓▓▓▓▓▓▙▃
 ◢▛▴▴▜▒▒▒▒▒▒▒◉▒▒▶
◤    ▀▜░░░░░░░▛▀▘
",
            r"
       ◢▙
     ▂▄▟▓▓▓▙▄▂
◥█▙▾▾▟▓▓▓▓▓▓▓▓▙▃
◢█▛▴▴▜▒▒▒▒▒▒▒◉▒▒▶
 ◥   ▀▜░░░░░░░▛▀▘
",
        ],
        &[
            r"
       66
6    222222222
 667722222222222
 6677111111114111
6    wwwwwwwwwwww
",
            r"
       66
     222222222
6667722222222222
66677111111114111
 6   wwwwwwwwwwww
",
        ],
    )
    .tint("1:bB 2:bB 6:bB 7:yY")
    .fps(0.4)
    .zone(Zone::Middle)
    .speed(2.0, 3.5)
    .weight(5)
    .biomes(OCEAN)
    .traits(PREDATOR),
    Species::new(
        "yellowfin-tuna",
        &[
            r"
      ▁▂▃◢▙▃▂
◣     ▂▄▟▓▓▓▓▓▙▄▂
 ◥▙▾▾▾▟▓▓▓▓▓▓▓▓▓▙▃
 ◢▛▴▴▴▜▒▒▒▒▒▒▒▒◉▒▒▶
◤     ▀▜░░░░░░░░▛▀▘
",
            r"
      ▁▂▃◢▙▃▂
      ▂▄▟▓▓▓▓▓▙▄▂
◥█▙▾▾▾▟▓▓▓▓▓▓▓▓▓▙▃
◢█▛▴▴▴▜▒▒▒▒▒▒▒▒◉▒▒▶
 ◥    ▀▜░░░░░░░░▛▀▘
",
        ],
        &[
            r"
      6666666
6     22222222222
 66666222222222222
 666661YYYYYYYY4YY1
6     wwwwwwwwwwwww
",
            r"
      6666666
      22222222222
666666222222222222
6666661YYYYYYYY4YY1
 6    wwwwwwwwwwwww
",
        ],
    )
    .tint("1:bB 2:bB 6:yY")
    .fps(0.4)
    .zone(Zone::Middle)
    .speed(2.0, 3.5)
    .weight(5)
    .biomes(OCEAN)
    .traits(PREDATOR),
    Species::new(
        "skipjack-tuna",
        &[
            r"
      ◢▙
◣    ▂▄▟▓▓▓▙▄▂
 ◥▙▾▟▓▒▒▒▒▒▒◉▒▶
◤    ▀▜┈┈┈┈┈┈▛▀
",
            r"
      ◢▙
     ▂▄▟▓▓▓▙▄▂
◥█▙▾▟▓▒▒▒▒▒▒◉▒▶
 ◥   ▀▜┈┈┈┈┈┈▛▀
",
        ],
        &[
            r"
      66
6    222222222
 66712111111411
6    ww777777ww
",
            r"
      66
     222222222
666712111111411
 6   ww777777ww
",
        ],
    )
    .tint("1:bB 2:bB 6:bB 7:kK")
    .fps(0.4)
    .zone(Zone::Middle)
    .speed(1.8, 3.2)
    .school(3, 6)
    .weight(5)
    .biomes(OCEAN),
    Species::new(
        "albacore",
        &[
            r"
         ◢▙▖
◣     ▂▄▟▓▓▓▓▙▄▂
 ◥▙▄▟▓▓▓▓▓▓▓▓▓▓▙▃
 ◢▛▀▜▒▒▒▒▒▒▒▒▒▒◉▒▶
◤     ▀▜░░░░░░░▛▀▘
           ◥▙▃▂▁
",
            r"
         ◢▙▖
      ▂▄▟▓▓▓▓▙▄▂
◥█▙▄▟▓▓▓▓▓▓▓▓▓▓▙▃
◢█▛▀▜▒▒▒▒▒▒▒▒▒▒◉▒▶
 ◥    ▀▜░░░░░░░▛▀▘
          ◥▙▃▂▁
",
        ],
        &[
            r"
         666
6     2222222222
 6662222222222222
 66611111111111411
6     wwwwwwwwwwww
           66666
",
            r"
         666
      2222222222
66662222222222222
666611111111111411
 6    wwwwwwwwwwww
          66666
",
        ],
    )
    .tint("1:bB 2:bB 6:yY")
    .fps(0.4)
    .zone(Zone::Middle)
    .speed(2.0, 3.2)
    .weight(4)
    .biomes(OCEAN),
    Species::new(
        "blue-marlin",
        &[
            r"
◢▙       ▁▂▃▄▅▆▇█▆▄
 ▝▙   ▂▄▟▓▓▓▓▓▓▓▓▓▙▄▃
  ▐█▄▟▒▒▒▒▒▒▒▒▒▒◉▒▒━━━━━━━▸
 ▗▛  ▀▜░░░░░░░░░░░▛▀
◥▛        ◥▙▂
",
            r"
 ◢▙      ▁▂▃▄▅▆▇█▆▄
  ▝▙  ▂▄▟▓▓▓▓▓▓▓▓▓▙▄▃
  ▐█▄▟▒▒▒▒▒▒▒▒▒▒◉▒▒━━━━━━━▸
  ▗▛ ▀▜░░░░░░░░░░░▛▀
 ◥▛       ◥▙▂
",
        ],
        &[
            r"
66       6666666666
 66   222222222222222
  66111111111111411WWWWWWWW
 66  wwwwwwwwwwwwwww
66        666
",
            r"
 66      6666666666
  66  222222222222222
  66111111111111411WWWWWWWW
  66 wwwwwwwwwwwwwww
 66       666
",
        ],
    )
    .tint("1:bB 2:bB 6:bB")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(2.5, 4.0)
    .weight(3)
    .biomes(OCEAN)
    .traits(PREDATOR),
    Species::new(
        "striped-marlin",
        &[
            r"
◢▙    ▁▂▃▅▆▇▇▇▇▇▇▇▆▅▃
 ▝▙   ▂▄▟▓▓▓▓▓▓▓▓▓▙▄▃
  ▐█▄▟▒┆▒┆▒┆▒┆▒┆▒◉▒▒━━━━▸
 ▗▛  ▀▜░░░░░░░░░░░▛▀
◥▛        ◥▙▂
",
            r"
 ◢▙   ▁▂▃▅▆▇▇▇▇▇▇▇▆▅▃
  ▝▙  ▂▄▟▓▓▓▓▓▓▓▓▓▙▄▃
  ▐█▄▟▒┆▒┆▒┆▒┆▒┆▒◉▒▒━━━━▸
  ▗▛ ▀▜░░░░░░░░░░░▛▀
 ◥▛       ◥▙▂
",
        ],
        &[
            r"
66    666666666666666
 66   222222222222222
  661117171717171411WWWWW
 66  wwwwwwwwwwwwwww
66        666
",
            r"
 66   666666666666666
  66  222222222222222
  661117171717171411WWWWW
  66 wwwwwwwwwwwwwww
 66       666
",
        ],
    )
    .tint("1:bB 2:bB 6:bB 7:cC")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(2.5, 4.0)
    .weight(3)
    .biomes(OCEAN)
    .traits(PREDATOR),
    Species::new(
        "swordfish",
        &[
            r"
◢▙       ◢▙▃
 ▝▙   ▂▄▟▓▓▓▓▓▓▓▓▙▄▂
  ▐█▄▟▒▒▒▒▒▒▒▒▒▒▒◉▒▒━━━━━━━━━▸
 ▗▛▀▜░░░░░░░░░░░░▛▀▘
◥▛   ▝▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
 ◢▙      ◢▙▃
  ▝▙  ▂▄▟▓▓▓▓▓▓▓▓▙▄▂
  ▐█▄▟▒▒▒▒▒▒▒▒▒▒▒◉▒▒━━━━━━━━━▸
  ▗▛▀▜░░░░░░░░░░░▛▀▘
 ◥▛  ▝▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
66       666
 66   22222222222222
  6611111111111114112222222222
 666wwwwwwwwwwwwwwww
66   wwwwwwwwwwwww
",
            r"
 66      666
  66  22222222222222
  6611111111111114112222222222
  66wwwwwwwwwwwwwwww
 66  wwwwwwwwwwwww
",
        ],
    )
    .tint("1:bB 2:kK 6:kK")
    .fps(0.4)
    .zone(Zone::Middle)
    .speed(2.2, 3.6)
    .weight(3)
    .biomes(OCEAN)
    .traits(PREDATOR | NOCTURNAL),
    Species::new(
        "sailfish",
        &[
            r"
         ▗▟▒∙▒∙▒∙▒∙▒∙▙▖
◢▙   ▗▟▓▓▓▓▓▓▓▓▓▓▓▓▙▄▃
 ▝▙▃▄▟▒▒▒▒▒▒▒▒▒▒▒◉▒▒━━━━━━▸
 ▗▛▀▜░░░░░░░░░░░░░░▛▀
◥▛   ▝▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
         ▗▟▒∙▒∙▒∙▒∙▒∙▙▖
 ◢▙  ▗▟▓▓▓▓▓▓▓▓▓▓▓▓▙▄▃
  ▝▙▄▟▒▒▒▒▒▒▒▒▒▒▒◉▒▒━━━━━━▸
  ▗▛▀▜░░░░░░░░░░░░░▛▀
 ◥▛   ▝▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
         222W2W2W2W2W22
66   11111111111111111
 6666111111111111411WWWWWWW
 6666wwwwwwwwwwwwwwww
66   wwwwwwwwwwwwww
",
            r"
         222W2W2W2W2W22
 66  11111111111111111
  666111111111111411WWWWWWW
  666wwwwwwwwwwwwwwww
 66   wwwwwwwwwwwww
",
        ],
    )
    .tint("1:bB 2:cC 6:bB")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(2.8, 4.0)
    .weight(3)
    .biomes(OCEAN)
    .traits(PREDATOR),
    Species::new(
        "mahi-mahi",
        &[
            r"
    ▁▂▃▄▄▄▄▄▄▄▄▄▄▅▆▇▜
◣▖ ▗▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▐
 ▝█▟▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉▒█▐
◤▘ ▝▜░░░░░░░░░░░░░░▛▐
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
    ▁▂▃▄▄▄▄▄▄▄▄▄▄▅▆▇▜
 ◢▙▗▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▐
 ▝█▟▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉▒█▐
 ◤▙▝▜░░░░░░░░░░░░░░▛▐
   ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
    77777777777777777
66 2222222222222222222
 661111111111111114111
66 wwwwwwwwwwwwwwwwww
   wwwwwwwwwwwwwww
",
            r"
    77777777777777777
 662222222222222222222
 661111111111111114111
 66wwwwwwwwwwwwwwwwww
   wwwwwwwwwwwwwww
",
        ],
    )
    .tint("1:yY 2:gG 6:yY 7:gG")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(1.5, 3.0)
    .weight(5)
    .biomes(OCEAN)
    .traits(CURIOUS),
    Species::new(
        "wahoo",
        &[
            r"
     ▁▁▂▂▂▂▂▂▂▂▂▂▃▃
◣▖  ▂▄▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▙▄▃
 ▝█▄▟▒╎▒╎▒╎▒╎▒╎▒╎▒╎▒╎▒╎◉▒▒━▸
◤▘  ▀▜░░░░░░░░░░░░░░░░▛▀▀▘
",
            r"
     ▁▁▂▂▂▂▂▂▂▂▂▂▃▃
 ◢▙ ▂▄▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▙▄▃
 ▗█▄▟▒╎▒╎▒╎▒╎▒╎▒╎▒╎▒╎▒╎◉▒▒━▸
 ◥▙ ▀▜░░░░░░░░░░░░░░░░▛▀▀▘
",
        ],
        &[
            r"
     66666666666666
66  2222222222222222222222
 6661171717171717171717411WW
66  wwwwwwwwwwwwwwwwwwwwww
",
            r"
     66666666666666
 66 2222222222222222222222
 6661171717171717171717411WW
 66 wwwwwwwwwwwwwwwwwwwwww
",
        ],
    )
    .tint("1:bB 2:bB 6:bB 7:cC")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(2.5, 4.0)
    .weight(3)
    .biomes(OCEAN | REEF)
    .traits(PREDATOR),
    Species::new(
        "king-mackerel",
        &[
            r"
       ▁▃▅▃
◢▙   ▂▄▟▓▓▓▓▓▓▓▓▓▓▙▄▂
 ▝█▄▟▒≈▒≈▒≈▒≈▒≈▒◉▒▒▶
◥▛  ▀▜░░░░░░░░░░░░▛▀▘
",
            r"
       ▁▃▅▃
 ◢▙  ▂▄▟▓▓▓▓▓▓▓▓▓▓▙▄▂
 ▗█▄▟▒≈▒≈▒≈▒≈▒≈▒◉▒▒▶
 ◥▛ ▀▜░░░░░░░░░░░░▛▀▘
",
        ],
        &[
            r"
       6666
66   2222222222222222
 6661171717171714111
66  wwwwwwwwwwwwwwwww
",
            r"
       6666
 66  2222222222222222
 6661171717171714111
 66 wwwwwwwwwwwwwwwww
",
        ],
    )
    .tint("1:cC 2:bB 6:bB 7:bB")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(2.0, 3.4)
    .weight(4)
    .biomes(OCEAN),
    Species::new(
        "atlantic-mackerel",
        &[
            r"
     ▂▄▂  ▃▄
◣▖ ▗▟╲╱╲╱╲╱╲╱▙▄▂
 ▝█▟▒▒▒▒▒▒▒▒▒◉▒▶
◤▘ ▝▜░░░░░░░░▛▀▘
",
            r"
     ▂▄▂  ▃▄
 ◢▙▗▟╲╱╲╱╲╱╲╱▙▄▂
 ▗█▟▒▒▒▒▒▒▒▒▒◉▒▶
 ◥▙▝▜░░░░░░░░▛▀▘
",
        ],
        &[
            r"
     222  22
66 2277777777222
 661111111111411
66 wwwwwwwwwwwww
",
            r"
     222  22
 662277777777222
 661111111111411
 66wwwwwwwwwwwww
",
        ],
    )
    .tint("1:cCbB 2:kKbB 6:bB 7:gGbB")
    .zone(Zone::Upper)
    .fps(0.3)
    .speed(1.6, 3.0)
    .school(3, 7)
    .weight(7)
    .biomes(OCEAN | KELP | ARCTIC),
    Species::new(
        "pacific-sardine",
        &[
            r"
   ▁▃▃▂
◣▟▓◦▓◦▓▓◉▙▶
◥▜░░░░░░░▛▀
",
            r"
   ▁▃▃▂
▝▟▓◦▓◦▓▓◉▙▶
▗▜░░░░░░░▛▀
",
        ],
        &[
            r"
   6666
66676766461
6wwwwwwwwww
",
            r"
   6666
66676766461
6wwwwwwwwww
",
        ],
    )
    .tint("1:wWcC 6:bBcC 7:kK")
    .zone(Zone::Upper)
    .fps(0.4)
    .speed(1.0, 2.2)
    .school(6, 12)
    .weight(8)
    .biomes(OCEAN | KELP)
    .traits(FILTER),
    Species::new(
        "anchovy",
        &[
            r"
    ▂▃▂
◣▟▓▓▓▓▓▓▓◉▙▀▶
◥▜▒━━━━━━━▛▀▘
",
            r"
    ▂▃▂
▝▟▓▓▓▓▓▓▓◉▙▀▶
▗▜▒━━━━━━━▛▀▘
",
        ],
        &[
            r"
    666
6666666664661
6w1WWWWWWWwww
",
            r"
    666
6666666664661
6w1WWWWWWWwww
",
        ],
    )
    .tint("1:wWcC 6:cC")
    .zone(Zone::Any)
    .fps(0.4)
    .speed(0.8, 1.8)
    .school(8, 12)
    .weight(7)
    .biomes(OCEAN | KELP | REEF)
    .traits(FILTER),
    Species::new(
        "atlantic-herring",
        &[
            r"
      ▁▃▄▃
◣▖ ▗▟▓▓▓▓▓▓▓▓▓▓▙▄
 ▝█▟▒∙▒∙▒∙▒∙▒∙▒◉▒▐
◤▘ ▝▜░░░░░░░░░░▛▘
",
            r"
      ▁▃▄▃
 ◢▙▗▟▓▓▓▓▓▓▓▓▓▓▙▄
 ▗█▟▒∙▒∙▒∙▒∙▒∙▒◉▒▐
 ◥▙▝▜░░░░░░░░░░▛▘
",
        ],
        &[
            r"
      6666
66 66666666666666
 6611W1W1W1W1W1411
66 wwwwwwwwwwwwww
",
            r"
      6666
 6666666666666666
 6611W1W1W1W1W1411
 66wwwwwwwwwwwwww
",
        ],
    )
    .tint("1:wWbB 6:bBcC")
    .zone(Zone::Upper)
    .fps(0.4)
    .speed(1.0, 2.4)
    .school(6, 12)
    .weight(8)
    .biomes(OCEAN | ARCTIC | KELP)
    .traits(FILTER),
    Species::new(
        "menhaden",
        &[
            r"
     ▁▃▅▃▁
◣▖  ▗▟▓▓▓▓▓▓▓▓▓▙▖
 ▝█▟▒▒▒▒▒▒▒▒▒●▒▒◉▒▐
◤▘ ▝▜░░░░░░░░░░░░▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▘
",
            r"
     ▁▃▅▃▁
 ◢▙ ▗▟▓▓▓▓▓▓▓▓▓▙▖
 ▗█▟▒▒▒▒▒▒▒▒▒●▒▒◉▒▐
 ◥▙▝▜░░░░░░░░░░░░▛▘
    ▝▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
     66666
66  6666666666666
 661111111111711411
66 wwwwwwwwwwwwwwww
    wwwwwwwwwwww
",
            r"
     66666
 66 6666666666666
 661111111111711411
 66wwwwwwwwwwwwwwww
    wwwwwwwwwwww
",
        ],
    )
    .tint("1:wWcC 6:bB 7:kK")
    .zone(Zone::Middle)
    .fps(0.4)
    .speed(0.8, 1.8)
    .school(5, 10)
    .weight(6)
    .biomes(OCEAN | KELP)
    .traits(FILTER),
    Species::new(
        "flying-fish",
        &[
            r"
  ◢▒▒▒▒▒▒◣
   ◥▒▒▒▒▒▒▒◣
◣▖  ▗▟▓▓▓▓▓▓▓▙▄▂
 ▝█▟▒▒▒▒▒▒▒◉▒▒▶
◢▌ ▝▜░░░░░░░░▛▀
◥▙
",
            r"


◣▖  ▗▟▓▓▓▓▓▓▓▙▄▂
 ▝█▟▒▒▒▒▒▒▒◉▒▒▶
◢▌ ▝▜░░░░░░░░▛▀
◥▙   ◥▒▒▒▒▒▒▒◣
",
        ],
        &[
            r"
  33333333
   333333333
66  666666666666
 66611111114116
66 66wwwwwwww66
66
",
            r"


66  666666666666
 66611111114116
66 66wwwwwwww66
66   333333333
",
        ],
    )
    .tint("1:bBcC 3:cCwW 6:bB")
    .zone(Zone::Top)
    .swim(Swim::Dart)
    .fps(0.4)
    .speed(1.5, 3.2)
    .school(2, 5)
    .weight(4)
    .biomes(OCEAN | REEF),
    Species::new(
        "ocean-sunfish",
        &[
            r"
   ▗▙
   ▐█▙
   ▐██▙▂
  ▗▟████▙▄▃▂▁
 ▐▒▓▓▓▓▓▓▓▓▓▓▓▙▖
▐▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▙▌
▐▒▒▒▒▒▒▒▒▒▒▒▒▒▒◉▒▐▶
 ▐░░░░░░░░░░░░░▛▘
  ▝▜████▛▀▀▀▀▘
   ▐██▛▘
",
            r"
    ▗▙
   ▗█▙
   ▐██▙▂
  ▗▟████▙▄▃▂▁
 ▐░▓▓▓▓▓▓▓▓▓▓▓▙▖
▐░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▙▌
▐░▒▒▒▒▒▒▒▒▒▒▒▒▒◉▒▐▶
 ▐░░░░░░░░░░░░░▛▘
  ▝▜████▛▀▀▀▀▘
    ▐█▛▘
",
        ],
        &[
            r"
   77
   777
   77777
  77777777777
 121111111111111
122111111111111111
2222222222222224222
 wwwwwwwwwwwwwwww
  777777777777
   77777
",
            r"
    77
   777
   77777
  77777777777
 1w1111111111111
1w2111111111111111
2w22222222222224222
 wwwwwwwwwwwwwwww
  777777777777
    7777
",
        ],
    )
    .tint("1:wWcC 2:wW 7:bB")
    .zone(Zone::Middle)
    .swim(Swim::Drift)
    .fps(0.3)
    .speed(0.25, 0.6)
    .weight(2)
    .biomes(OCEAN | KELP)
    .traits(CURIOUS),
    Species::new(
        "opah",
        &[
            r"
     ▁▃▅▅▃▁
   ▗▟▓▓▓▓▓▓▓▓▙▖
◣▖▟▓▓◦▓▓◦▓▓◦▓▓▙▖
 ▝█▓◦▓▓◦▓▓◦▓▓▓◉▓▐
◤▘▜▓▓◦▓▓◦▓▓◦▓▓▛▘
   ▝▜▒▒▒▒▒▒▒▒▛▘
     ▝▀▀▀▀▀▀▘
",
            r"
     ▁▃▅▅▃▁
   ▗▟▓▓▓▓▓▓▓▓▙▖
◢▙▟▓▓◦▓▓◦▓▓◦▓▓▙▖
 ▝█▓◦▓▓◦▓▓◦▓▓▓◉▓▐
◥▛▜▓▓◦▓▓◦▓▓◦▓▓▛▘
   ▝▜▒▒▒▒▒▒▒▒▛▘
     ▝▀▀▀▀▀▀▘
",
        ],
        &[
            r"
     333333
   112222222211
3312282282282211
 3128228228222421
3312282282282211
   111111111111
     11111111
",
            r"
     333333
   112222222211
3312282282282211
 3128228228222421
3312282282282211
   111111111111
     11111111
",
        ],
    )
    .tint("1:bBcC 3:rR 6:rR 8:wW")
    .zone(Zone::Lower)
    .swim(Swim::Hover)
    .fps(0.3)
    .speed(0.3, 0.9)
    .weight(2)
    .biomes(OCEAN | DEEP),
    Species::new(
        "great-barracuda",
        &[
            r"
   ▁▃    ▂▄▅▃
◣▖   ▂▄▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▙▄▃
 ▝█▄▟▒▒▓▒▒▓▒▒▓▒▒▓▒▒▒▒◉▒▒━━▷
 ◤▘ ▀▜░░░░░░░░░░░░░░░░▛▀▴▴▴▴
",
            r"
   ▁▃    ▂▄▅▃
 ◢▙  ▂▄▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▙▄▃
 ▗█▄▟▒▒▓▒▒▓▒▒▓▒▒▓▒▒▒▒◉▒▒━━▷
 ◥▛ ▀▜░░░░░░░░░░░░░░░░▛▀▴▴▴▴
",
        ],
        &[
            r"
   66    6666
66   222222222222222222222
 66611171171171171111411kkk
 66 wwwwwwwwwwwwwwwwwwwwWWWW
",
            r"
   66    6666
 66  222222222222222222222
 66611171171171171111411kkk
 66 wwwwwwwwwwwwwwwwwwwwWWWW
",
        ],
    )
    .tint("1:cCwW 2:kK 6:kK 7:kK")
    .zone(Zone::Upper)
    .swim(Swim::Dart)
    .fps(0.4)
    .speed(0.8, 3.6)
    .weight(4)
    .biomes(REEF | OCEAN)
    .traits(PREDATOR | CURIOUS),
    Species::new(
        "atlantic-bonito",
        &[
            r"
         ◢▙▖
◣▖   ▂▄▟▓╲▓╲▓╲▓╲▓▙▄▃
 ▝█▄▟▒▒▒▒▒▒▒▒▒▒▒▒◉▒▶
◤▘  ▀▜░░░░░░░░░░░▛▀▘
",
            r"
         ◢▙▖
 ◢▙  ▂▄▟▓╲▓╲▓╲▓╲▓▙▄▃
 ▗█▄▟▒▒▒▒▒▒▒▒▒▒▒▒◉▒▶
 ◥▙ ▀▜░░░░░░░░░░░▛▀▘
",
        ],
        &[
            r"
         666
66   222272727272222
 6661111111111111411
66  wwwwwwwwwwwwwwww
",
            r"
         666
 66  222272727272222
 6661111111111111411
 66 wwwwwwwwwwwwwwww
",
        ],
    )
    .tint("1:bBcC 2:bB 6:bB 7:kK")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(2.0, 3.4)
    .school(3, 6)
    .weight(5)
    .biomes(OCEAN),
    Species::new(
        "greater-amberjack",
        &[
            r"
        ▁▃▅▅▃▁
◣▖   ▗▟▓▓▓▓▓▓▓▓▓▙█▖
 ▝█▄▟▒▒▒▒▒▒▒▒▒▒▒▒█◉▒▐
 ▗█▀▜▒▒▒━━━━━━━━━▒▒▐
◤▘  ▝▜░░░░░░░░░░░░▛▘
      ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
        ▁▃▅▅▃▁
 ◢▙  ▗▟▓▓▓▓▓▓▓▓▓▙█▖
 ▐█▄▟▒▒▒▒▒▒▒▒▒▒▒▒█◉▒▐
 ▗█▀▜▒▒▒━━━━━━━━━▒▒▐
 ◥▙ ▝▜░░░░░░░░░░░░▛▘
      ▝▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
        666666
66   11111111111171
 66611111111111117411
 6661111YYYYYYYYY111
66  wwwwwwwwwwwwwwww
      wwwwwwwwwww
",
            r"
        666666
 66  11111111111171
 66611111111111117411
 6661111YYYYYYYYY111
 66 wwwwwwwwwwwwwwww
      wwwwwwwwwww
",
        ],
    )
    .tint("1:cCbB 2:yY 6:yY 7:kK")
    .fps(0.4)
    .zone(Zone::Middle)
    .speed(1.2, 2.8)
    .school(1, 3)
    .weight(4)
    .biomes(OCEAN | REEF)
    .traits(CURIOUS),
    Species::new(
        "yellowtail",
        &[
            r"
       ▁▃▃
◣▖  ▂▄▟▓▓▓▓▓▓▓▓▓▙▄
 ▝█▟▒▒━━━━━━━━━━◉▒▶
◤▘ ▝▜░░░░░░░░░░░▛▀▘
",
            r"
       ▁▃▃
▐█  ▂▄▟▓▓▓▓▓▓▓▓▓▙▄
 ▐█▟▒▒━━━━━━━━━━◉▒▶
▐▌ ▝▜░░░░░░░░░░░▛▀▘
",
        ],
        &[
            r"
       777
66  22222222222222
 661117777777777411
66 wwwwwwwwwwwwwwww
",
            r"
       777
66  22222222222222
 661117777777777411
66 wwwwwwwwwwwwwwww
",
        ],
    )
    .tint("1:bBcC 2:bB 6:yY 7:yY")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(1.5, 3.2)
    .school(2, 5)
    .weight(5)
    .biomes(OCEAN | KELP),
    Species::new(
        "cobia",
        &[
            r"
     ▁▂▂▂▂▂▂▂▂▂▂▂▃
 ▗▄▖ ▂▄▟▓▓▓▓▓▓▓▓▓▓▙▄▂
▐█▌▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓◉▒▀▶
 ▝▀▘ ▀▜░░░░░░░░░░░░▛▀▘
",
            r"
     ▁▂▂▂▂▂▂▂▂▂▂▂▃
▗▟█▙ ▂▄▟▓▓▓▓▓▓▓▓▓▓▙▄▂
 ▐█▟▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓◉▒▀▶
▝▜█▛ ▀▜░░░░░░░░░░░░▛▀▘
",
        ],
        &[
            r"
     6666666666666
 666 2222222222222222
666677777777777777774111
 666 wwwwwwwwwwwwwwwww
",
            r"
     6666666666666
6666 2222222222222222
 666777777777777777774111
6666 wwwwwwwwwwwwwwwww
",
        ],
    )
    .tint("1:yY 2:yY 6:yY 7:kK")
    .fps(0.4)
    .zone(Zone::Middle)
    .speed(1.0, 2.4)
    .weight(3)
    .biomes(OCEAN | REEF)
    .traits(PREDATOR | CURIOUS),
    Species::new(
        "tarpon",
        &[
            r"
         ◢█▙▃▂▁
◣▖   ▂▄▟▓▓▓▓▓▓▓▓▓▙▄
 ▝█▄▟◇◆◇◆◇◆◇◆◇◆◇◆◇◉▒▙
 ▗█▀▜◆◇◆◇◆◇◆◇◆◇◆◇▒▒▐▶
◤▘   ▝▜░░░░░░░░░░▛▘
",
            r"
         ◢█▙▃▂▁
 ◢▙  ▂▄▟▓▓▓▓▓▓▓▓▓▙▄
 ▐█▄▟◇◆◇◆◇◆◇◆◇◆◇◆◇◉▒▙
 ▗█▀▜◆◇◆◇◆◇◆◇◆◇◆◇▒▒▐▶
 ◥▙  ▝▜░░░░░░░░░░▛▘
",
        ],
        &[
            r"
         666666
66   22222222222222
 66611212121212121411
 66612121212121211111
66   wwwwwwwwwwwwww
",
            r"
         666666
 66  22222222222222
 66611212121212121411
 66612121212121211111
 66  wwwwwwwwwwwwww
",
        ],
    )
    .tint("1:wWcC 2:wW 6:wWcC")
    .fps(0.4)
    .zone(Zone::Upper)
    .speed(1.0, 2.6)
    .weight(3)
    .biomes(OCEAN | REEF | FRESH),
    Species::new(
        "bonefish",
        &[
            r"
      ▂▄▃
◣▖ ▗▟▓▓▓▓▓▓▓▓▓▙▄
 ▝█▟▒┈▒┈▒┈▒┈▒┈◉▒▀▀▷
◤▘ ▝▜░░░░░░░░░░▛▘
",
            r"
      ▂▄▃
 ◢▙▗▟▓▓▓▓▓▓▓▓▓▙▄
 ▗█▟▒┈▒┈▒┈▒┈▒┈◉▒▀▀▷
 ◥▙▝▜░░░░░░░░░░▛▘
",
        ],
        &[
            r"
      222
66 2222222222222
 661121212121241111
66 wwwwwwwwwwwwww
",
            r"
      222
 662222222222222
 661121212121241111
 66wwwwwwwwwwwwww
",
        ],
    )
    .tint("1:wWcC 2:cC 6:cCwW")
    .fps(0.4)
    .zone(Zone::Lower)
    .speed(1.0, 2.6)
    .school(1, 3)
    .weight(4)
    .biomes(REEF | OCEAN)
    .traits(SCAVENGER),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_species_drawn(super::SPECIES);
    }
}
