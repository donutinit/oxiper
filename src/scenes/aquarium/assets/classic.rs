//! asciiquarium's own eight fish, its seaweed and its castle, drawn again
//! for the world beyond classic (`biome = "classic"` keeps the originals in
//! `../art.rs`). Fish are drawn facing right with two frames of tail motion
//! and mirrored to face left. Masks: 1 body, 2 dorsal fin, 3 flippers,
//! 4 eye, 5 mouth, 6 tail, 7 gills.

use super::*;

/// asciiquarium's own eight fish, colored at random as there.
const fn classic(
    name: &'static str,
    art: &'static [&'static str],
    mask: &'static [&'static str],
    fps: f32,
) -> Species {
    Species::new(name, art, mask)
        .fps(fps)
        .biomes(TANK | REEF | KELP)
}

pub const CLASSIC: &[Species] = &[
    classic(
        "classic-1",
        &[
            r"
         ▗▄▖
      ▗▄▟██▙▄▖
╲    ▟██▓▓▒▒░█▙▖
◀▌▐██▓▒░ ▕   ●▐▶
╱    ▜██▓▒▒░░█▛▘
      ▝▀▜█▓▒▓▛▀▘
         ▝▀▘
",
            r"
         ▗▄▖
      ▗▄▟██▙▄▖
╱    ▟██▓▓▒▒░█▙▖
◀▌▐██▓▒░ ▕   ●▐▶
╲    ▜██▓▒▒░░█▛▘
      ▝▀▜█▓▒▓▛▀▘
         ▝▀▘
",
        ],
        &[
            r"
         222
      11111111
6    11111111111
66111111 7   415
6    11111111111
      1111111111
         333
",
            r"
         222
      11111111
6    11111111111
66111111 7   415
6    11111111111
      1111111111
         333
",
        ],
        0.3,
    ),
    classic(
        "classic-2",
        &[
            r"
   ▗▖
╲ ▗▟██▙▖
◀▐█▓▒ ▕●▸
╱ ▝▜██▛▘
   ▝▘
",
            r"
   ▗▖
╱ ▗▟██▙▖
◀▐█▓▒ ▕●▸
╲ ▝▜██▛▘
   ▝▘
",
        ],
        &[
            r"
   22
6 111111
61111 745
6 111111
   33
",
            r"
   22
6 111111
61111 745
6 111111
   33
",
        ],
        0.3,
    ),
    classic(
        "classic-3",
        &[
            r"
        ▗▟▙▖
╲▖   ▗▟▓▒▒▒▓▙▖
 ▜▙▗▟▓▒░▒░▒░▒▒▙▖
 ▟▛▝▜▒░▒░▒░▒░▒░●▶
╱▘  ▝▜▓▒▒▒▒▒▒▓▛▘
      ▝▀▀▜▙▄▟▛▘
",
            r"
        ▗▟▙▖
╱▘   ▗▟▓▒▒▒▓▙▖
 ▜▙▗▟▓▒░▒░▒░▒▒▙▖
 ▟▛▝▜▒░▒░▒░▒░▒░●▶
╲▖  ▝▜▓▒▒▒▒▒▒▓▛▘
      ▝▀▀▜▙▄▟▛▘
",
        ],
        &[
            r"
        2222
66   111111111
 661111111111111
 6611111111111145
66  111111111111
      133111111
",
            r"
        2222
66   111111111
 661111111111111
 6611111111111145
66  111111111111
      133111111
",
        ],
        0.35,
    ),
    classic(
        "classic-4",
        &[
            r"
 ▗▄▖
◀◣▟█▙●▸
  ▝▘
",
            r"
 ▗▄▖
◀◥▟█▙●▸
  ▝▘
",
        ],
        &[
            r"
 111
6611145
  33
",
            r"
 111
6611145
  33
",
        ],
        0.4,
    ),
    classic(
        "classic-5",
        &[
            r"
╲  ▗▄▄▖
◀▌▟██▓▕●▸
╱  ▝▀▀▘
",
            r"
   ▗▄▄▖
◁▌▟██▓▕●▸
   ▝▀▀▘
",
        ],
        &[
            r"
6  2222
661111745
6  1331
",
            r"
   2222
661111745
   1331
",
        ],
        0.3,
    ),
    classic(
        "classic-6",
        &[
            r"
    ▗▖
╲  ▟██▙
◀▌▟███▒▕●▸
╱  ▜██▛
    ▝▘
",
            r"
    ▗▖
   ▟██▙
◁▌▟███▒▕●▸
   ▜██▛
    ▝▘
",
        ],
        &[
            r"
    22
6  1111
6611111745
6  1111
    33
",
            r"
    22
   1111
6611111745
   1111
    33
",
        ],
        0.3,
    ),
    classic(
        "classic-7",
        &[
            r"
   ▗▖
◀▌▟█▒▕●▸
   ▝▘
",
            r"
   ▗▖
◁▌▟█▒▕●▸
   ▝▘
",
        ],
        &[
            r"
   22
66111745
   33
",
            r"
   22
66111745
   33
",
        ],
        0.4,
    ),
    classic(
        "classic-8",
        &[
            r"
  ▗▄▄▖
╲╱▟▓▓●▸
╱╲▝▀▀▘
",
            r"
  ▗▄▄▖
╱╲▟▓▓●▸
╲╱▝▀▀▘
",
        ],
        &[
            r"
  2222
6611145
661331
",
            r"
  2222
6611145
661331
",
        ],
        0.4,
    ),
];

pub const PLANTS: &[Plant] = &[Plant::stalk("seaweed", "╭╯", "╰╮", 3, 6)
    .fps(0.27)
    .weight(6)
    .traits(SHELTER)
    .biomes(TANK | REEF | KELP | OCEAN | FRESH)];

pub const DECOR: &[Decor] = &[Decor::new(
    "castle",
    &[
        r"
                ┃▀▀▀◣
                ┃
               ╱█╲
              ╱▓█▓╲
             ╱▓▓█▓▓╲
█▀█▀█▀█     ╱▓▓▓█▓▓▓╲     █▀█▀█▀█
█▓▓▓▓▓█▄▀▄▀▄▐▓▓▓█▓▓▓▌▄▀▄▀▄█▓▓▓▓▓█
█▓▓▮▓▓█▒▒▒▒▒▐▓▮▓█▓▮▓▌▒▒▒▒▒█▓▓▮▓▓█
█▓▓▓▓▓█▒▒▒▒▒▐▓▓▓█▓▓▓▌▒▒▒▒▒█▓▓▓▓▓█
█▓▓▓▓▓█▒▒▮▒▒▐▓▓▓▓▓▓▓▌▒▒▮▒▒█▓▓▓▓▓█
█▓▓▮▓▓█▒▒▒▒▒▐▓▟▀▀▀▙▓▌▒▒▒▒▒█▓▓▮▓▓█
█▓▓▓▓▓█▒▒▒▒▒▐▓▌╫╫╫▐▓▌▒▒▒▒▒█▓▓▓▓▓█
█▓▓▓▓▓█▒▒▒▒▒▐▓▌╫╫╫▐▓▌▒▒▒▒▒█▓▓▓▓▓█
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
",
        r"
                ┃▄▄▄◢
                ┃
               ╱█╲
              ╱▓█▓╲
             ╱▓▓█▓▓╲
█▀█▀█▀█     ╱▓▓▓█▓▓▓╲     █▀█▀█▀█
█▓▓▓▓▓█▄▀▄▀▄▐▓▓▓█▓▓▓▌▄▀▄▀▄█▓▓▓▓▓█
█▓▓▮▓▓█▒▒▒▒▒▐▓▮▓█▓▮▓▌▒▒▒▒▒█▓▓▮▓▓█
█▓▓▓▓▓█▒▒▒▒▒▐▓▓▓█▓▓▓▌▒▒▒▒▒█▓▓▓▓▓█
█▓▓▓▓▓█▒▒▮▒▒▐▓▓▓▓▓▓▓▌▒▒▮▒▒█▓▓▓▓▓█
█▓▓▮▓▓█▒▒▒▒▒▐▓▟▀▀▀▙▓▌▒▒▒▒▒█▓▓▮▓▓█
█▓▓▓▓▓█▒▒▒▒▒▐▓▌╫╫╫▐▓▌▒▒▒▒▒█▓▓▓▓▓█
█▓▓▓▓▓█▒▒▒▒▒▐▓▌╫╫╫▐▓▌▒▒▒▒▒█▓▓▓▓▓█
▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓
",
    ],
    &[
        r"
                WRRRR
                W
               rrr
              rrrrr
             rrrrrrr
            rrrrrrrrr

   Y          Y   Y          Y

         Y             Y
   Y          yyyyy          Y
               yyy
               yyy
",
        r"
                WRRRR
                W
               rrr
              rrrrr
             rrrrrrr
            rrrrrrrrr

   Y          Y   Y          Y

         Y             Y
   Y          yyyyy          Y
               yyy
               yyy
",
    ],
)
.fps(0.5)
.color('K')
.weight(7)
.traits(SHELTER)
.biomes(TANK)];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        use super::super::*;
        assert_species_drawn(super::CLASSIC);
        assert_plants_drawn(super::PLANTS);
        assert_decor_drawn(super::DECOR);
    }
}
