//! Seaweed, kelp, grasses, anemones, corals and sponges.

use super::*;

pub const PLANTS: &[Plant] = &[
    Plant::stalk("sea-whip", "╲", " ╱", 4, 10)
        .color('r')
        .fps(0.25)
        .weight(6)
        .biomes(REEF),
    Plant::stalk("grape-algae", "●", " ◍", 3, 8)
        .color('g')
        .fps(0.2)
        .weight(4)
        .biomes(REEF | TANK),
    Plant::stalk("giant-kelp-stipe", "╰╮", "╭╯", 8, 22)
        .color('y')
        .fps(0.2)
        .weight(7)
        .biomes(KELP),
    Plant::stalk("eelgrass", "│", " ╎", 3, 8)
        .color('g')
        .fps(0.3)
        .weight(8)
        .biomes(FRESH | OCEAN),
    Plant::stalk("vallisneria", "╲│", " │╱", 4, 12)
        .color('G')
        .fps(0.3)
        .weight(7)
        .biomes(FRESH | TANK),
    Plant::stalk("sargassum-vine", "╰◦", " ◦╯", 4, 12)
        .color('y')
        .fps(0.25)
        .weight(7)
        .biomes(OCEAN)
        .traits(SHELTER),
    Plant::stalk("red-ribbon-algae", "▚", " ▞", 3, 9)
        .color('r')
        .fps(0.25)
        .weight(5)
        .biomes(KELP | ARCTIC | OCEAN),
    Plant::stalk("hornwort", "╲│╱", "╱│╲", 3, 8)
        .color('g')
        .fps(0.25)
        .weight(6)
        .biomes(FRESH)
        .traits(SHELTER),
    Plant::stalk("cabomba", "╯│╰", "╮│╭", 4, 10)
        .color('g')
        .fps(0.3)
        .weight(6)
        .biomes(TANK | FRESH)
        .traits(SHELTER),
    Plant::stalk("rotala", "╴│", " │╶", 4, 11)
        .color('R')
        .fps(0.25)
        .weight(5)
        .biomes(TANK),
    Plant::stalk("ludwigia", "◂│", " │▸", 4, 10)
        .color('r')
        .fps(0.25)
        .weight(5)
        .biomes(TANK | FRESH),
    Plant::stalk("bamboo-coral-stalk", "═╬═", " ║ ", 4, 14)
        .color('W')
        .fps(0.1)
        .weight(5)
        .biomes(DEEP),
    Plant::stalk("wire-coral", "∽", " ∼", 5, 18)
        .color('K')
        .fps(0.1)
        .weight(4)
        .biomes(DEEP),
    Plant::stalk("ribbon-kelp", "▐", " ▌", 5, 14)
        .color('y')
        .fps(0.2)
        .weight(6)
        .biomes(KELP | ARCTIC),
    Plant::stalk("water-milfoil", "╲┼╱", " │ ", 3, 9)
        .color('g')
        .fps(0.25)
        .weight(5)
        .biomes(FRESH)
        .traits(SHELTER),
    Plant::stalk("chara", "✱", " ✲", 2, 6)
        .color('G')
        .fps(0.2)
        .weight(4)
        .biomes(FRESH),
    Plant::stalk("turtle-grass", "┃", " ╽", 2, 6)
        .color('g')
        .fps(0.3)
        .weight(7)
        .biomes(REEF | OCEAN),
    Plant::stalk("sea-lettuce-frill", "⌒", " ‿", 2, 5)
        .color('G')
        .fps(0.3)
        .weight(4)
        .biomes(KELP | ARCTIC),
    Plant::sprite(
        "brain-coral",
        &[
            r"
     ▄▟▓▒▓▙▄
  ▗▟▓▒▓▓▒▒▓▒▓▙▖
 ▐▓▒▒▓▓▒▓▒▒▓▓▒▒▌
  ▝▜▒▓▒▒▓▓▒▓▒▛▘
   ▀▀▀▀▀▀▀▀▀▀▀
",
            r"
     ▄▟▒▓▒▙▄
  ▗▟▒▓▒▓▓▒▒▓░▙▖
 ▐▒▓▓▒▒▓▒▓▓▒▒▓▓▌
  ▝▜▓▒▓▓▒▒▓▒▓▛▘
   ▀▀▀▀▀▀▀▀▀▀▀
",
        ],
        &[r"
     1111111
  1111111111111
 111111111111111
  1111111111111
   11111111111
"],
    )
    .tint("1:yYgGmM")
    .color('g')
    .fps(0.15)
    .weight(6)
    .biomes(REEF),
    Plant::sprite(
        "staghorn-coral",
        &[
            r"
  ╲    ╷   ╱
   ╲  ╱┃  ╱
    ╲╱ ┃ ╱
     ╲╱┃╱
      ╲┃╱
       ┃
      ▟█▙
",
            r"
  ╷    ╵   ╷
   │  ╱┃  │
    ╲╱ ┃ ╱
     ╲╱┃╱
      ╲┃╱
       ┃
      ▟█▙
",
        ],
        &[r"
  1111111111
   11111111
    111111
     1111
      111
       1
      111
"],
    )
    .tint("1:yYmMwW")
    .color('g')
    .fps(0.25)
    .weight(6)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "elkhorn-coral",
        &[
            r"
 ╱╲  ╱╲   ╱╲  ╱╲
╱▒▒╲╱▒▒╲ ╱▒▒╲╱▒▒╲
▐▓▒▓▒▓▒▙█▟▓▒▓▒▓▒▌
 ▀▀▜▓▒▓▒█▒▓▒▓▛▀▀
     ▀▜▒█▒▛▀
       ▐█▌
       ▐█▌
      ▟███▙
",
            r"
 │╲  ╱│   │╲  ╱│
╱▒▒╲╱▒▒╲ ╱▒▒╲╱▒▒╲
▐▓▒▓▒▓▒▙█▟▓▒▓▒▓▒▌
 ▀▀▜▓▒▓▒█▒▓▒▓▛▀▀
     ▀▜▒█▒▛▀
       ▐█▌
       ▐█▌
      ▟███▙
",
        ],
        &[r"
 111111111111111
11111111111111111
11111111111111111
 111111111111111
     1111111
       111
       111
      11111
"],
    )
    .tint("1:yYrR")
    .color('g')
    .fps(0.2)
    .weight(4)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "sea-fan",
        &[
            r"
    ╭──────╮
  ╭─┼┬┬┬┬┬┬┼─╮
  ├┼┼┼┼┼┼┼┼┼┼┤
  ╰┬┼┼┼┼┼┼┼┼┬╯
    ╰┬┼┼┼┼┬╯
      ╲││╱
       ││
      ▟██▙
",
            r"
     ╭──────╮
   ╭─┼┬┬┬┬┬┬┼─╮
   ├┼┼┼┼┼┼┼┼┼┼┤
   ╰┬┼┼┼┼┼┼┼┼┬╯
     ╰┬┼┼┼┼┬╯
      ╲││╱
       ││
      ▟██▙
",
        ],
        &[
            r"
    11111111
  111111111111
  111111111111
  111111111111
    11111111
      1111
       11
      1111
",
            r"
     11111111
   111111111111
   111111111111
   111111111111
     11111111
      1111
       11
      1111
",
        ],
    )
    .tint("1:rRmMyY")
    .color('g')
    .fps(0.15)
    .weight(6)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "mushroom-coral",
        &[
            r"
   ▄▄▟██▙▄▄
 ▗▟▓▒▓▒▒▓▒▓▙▖
  ▝▜▒░▒░▒░▛▘
     ▐▓▓▌
    ▗▟▓▓▙▖
",
            r"
   ▄▄▟██▙▄▄
 ▗▟▒▓▒▓▓▒▓▒▙▖
  ▝▜░▒░▒░▒▛▘
     ▐▓▓▌
    ▗▟▓▓▙▖
",
        ],
        &[r"
   11111111
 111111111111
  1111111111
     1111
    111111
"],
    )
    .tint("1:gGmMcC")
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(REEF),
    Plant::sprite(
        "bubble-coral",
        &[
            r"
  ◌○◌ ○◌
 ○◌◉○◌◉○◌
  ◌○◌○◌○
   ╰││╯
",
            r"
  ○◌○ ◌○
 ◌○◉◌○◉◌○
  ○◌○◌○◌
   ╰││╯
",
        ],
        &[r"
  111111
 11111111
  111111
   1111
"],
    )
    .tint("1:wWyYcC")
    .color('g')
    .fps(0.15)
    .weight(4)
    .biomes(REEF),
    Plant::sprite(
        "pillar-coral",
        &[
            r"
 ◦◦  ◦◦
 ██  ██  ◦◦
 ██▟▙██▗▖██
 ▓▒▓▒▓▒▓▒▓▒
",
            r"
 ∙∙  ∙∙
 ██  ██  ∙∙
 ██▟▙██▗▖██
 ▒▓▒▓▒▓▒▓▒▓
",
        ],
        &[r"
 111111
 1111111111
 1111111111
 1111111111
"],
    )
    .tint("1:yYwW")
    .color('g')
    .fps(0.2)
    .weight(4)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "table-coral",
        &[
            r"
  ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
 ▐▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▌
  ▝▀▀▀▀▀▜▌▐▛▀▀▀▀▀▘
        ▐██▌
        ▐██▌
      ▗▟████▙▖
",
            r"
  ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
 ▐▒▓▒▓▒▓▒▓▒▓▒▓▒▓▒▓▌
  ▝▀▀▀▀▀▜▌▐▛▀▀▀▀▀▘
        ▐██▌
        ▐██▌
      ▗▟████▙▖
",
        ],
        &[r"
  1111111111111111
 111111111111111111
  1111111111111111
        1111
        1111
      11111111
"],
    )
    .tint("1:yYgGcC")
    .color('g')
    .fps(0.15)
    .weight(4)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "tube-coral",
        &[
            r"
  ◖◗  ◖◗   ◖◗
  ██  ██   ██
  ██  ██◖◗ ██
 ◖██◗ ████ ██◗
 ▐▓█▓██▓██▓█▓▌
",
            r"
  ▖▗  ▖▗   ▖▗
  ██  ██   ██
  ██  ██◖◗ ██
 ◖██◗ ████ ██◗
 ▐▓█▓██▓██▓█▓▌
",
        ],
        &[r"
  11111111111
  11111111111
  11111111111
 1111111111111
 1111111111111
"],
    )
    .tint("1:mMrRyY")
    .color('g')
    .fps(0.2)
    .weight(4)
    .biomes(REEF),
    Plant::sprite(
        "finger-coral",
        &[
            r"
 ▗▖ ▗▖ ▗▖ ▗▖
 ██ ██ ██ ██
 ██▟██▟██▟██
 ▓▒▓▒▓▒▓▒▓▒▓
",
            r"
 ▖▗ ▖▗ ▖▗ ▖▗
 ██ ██ ██ ██
 ██▟██▟██▟██
 ▓▒▓▒▓▒▓▒▓▒▓
",
        ],
        &[r"
 11111111111
 11111111111
 11111111111
 11111111111
"],
    )
    .tint("1:yYwWmM")
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "christmas-tree-worms",
        &[
            r"
  ◢◣    ◢◣
 ◢▓▓◣  ◢▓▓◣
 ◥▒▒◤◢◣◥▒▒◤
  ▐▌◢▓▓◣▐▌
  ▐▌◥▒▒◤▐▌
  ▐▌ ▐▌ ▐▌
",
            r"
  ◢◣    ◢◣
 ◢▒▒◣  ◢▒▒◣
 ◥▓▓◤◢◣◥▓▓◤
  ▐▌◢▒▒◣▐▌
  ▐▌◥▓▓◤▐▌
  ▐▌ ▐▌ ▐▌
",
        ],
        &[r"
  11111111
 1111111111
 1111111111
  11111111
  11111111
  11111111
"],
    )
    .tint("1:rRyYbB")
    .color('g')
    .fps(0.3)
    .weight(5)
    .biomes(REEF),
    Plant::sprite(
        "feather-duster-worm",
        &[
            r"
   ╲╲│╱╱
  ╲╲╲│╱╱╱
  ◖▓▒▓▒▓◗
    ▐█▌
    ▐█▌
",
            r"
    ╲│╱
   ╲╲│╱╱
   ◖▓▒▓◗
    ▐█▌
    ▐█▌
",
            r"
     ╷
     │
    ▐▓▌
    ▐█▌
    ▐█▌
",
            r"
    ╲│╱
   ╲╲│╱╱
   ◖▓▒▓◗
    ▐█▌
    ▐█▌
",
        ],
        &[
            r"
   11111
  1111111
  1111111
    www
    www
",
            r"
    111
   11111
   11111
    www
    www
",
            r"
     1
     1
    111
    www
    www
",
            r"
    111
   11111
   11111
    www
    www
",
        ],
    )
    .tint("1:rRmMyY")
    .color('g')
    .fps(0.2)
    .weight(5)
    .biomes(REEF),
    Plant::sprite(
        "barrel-sponge",
        &[
            r"
 ▗▟▀▀▀▀▀▀▀▙▖
 ▐▓░░░░░░░▓▌
 ▐▒░▒▒░▒▒░▒▌
 ▐▒▒▒▒▒▒▒▒▒▌
 ▝▜▓▓▓▓▓▓▓▛▘
",
            r"
 ▗▟▀▀▀▀▀▀▀▙▖
 ▐▓▒░░░▒░░▓▌
 ▐▒▒░▒▒░▒▒░▌
 ▐▒▒▒▒▒▒▒▒▒▌
 ▝▜▓▓▓▓▓▓▓▛▘
",
        ],
        &[r"
 11111111111
 11111111111
 11111111111
 11111111111
 11111111111
"],
    )
    .tint("1:yYrRmM")
    .color('g')
    .fps(0.15)
    .weight(6)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "tube-sponges",
        &[r"
 ◖◗  ◖◗
 ██  ██  ◖◗
 ██  ██  ██
 ██▄▄██▄▄██
 ▓▒▓▒▓▒▓▒▓▒
"],
        &[r"
 111111
 1111111111
 1111111111
 1111111111
 1111111111
"],
    )
    .tint("1:yYmMrR")
    .color('g')
    .fps(0.0)
    .weight(6)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "vase-sponge",
        &[
            r"
 ▜▙    ▟▛
 ▐▒▙▄▄▟▒▌
 ▐▓▒░░▒▓▌
  ▜▓▒▒▓▛
   ▝██▘
",
            r"
 ▜▙    ▟▛
 ▐▒▙▂▂▟▒▌
 ▐▓▒▒░▒▓▌
  ▜▓▒▒▓▛
   ▝██▘
",
        ],
        &[r"
 11111111
 11111111
 11111111
  111111
   1111
"],
    )
    .tint("1:yYrRmM")
    .color('g')
    .fps(0.15)
    .weight(5)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "pink-anemone",
        &[
            r"
╲╷╱╲╷╱╲╷╱╲╷
│┃│┃│┃│┃│┃│
 ▜▓▒▓▒▓▒▓▛
  ▝▜▒▓▒▛▘
    ▐▓▌
",
            r"
╱╷╲╱╷╲╱╷╲╱╷
┃│┃│┃│┃│┃│┃
 ▜▒▓▒▓▒▓▒▛
  ▝▜▒▓▒▛▘
    ▐▓▌
",
        ],
        &[r"
11111111111
11111111111
 222222222
  2222222
    222
"],
    )
    .tint("1:mMrRyY 2:yw")
    .color('g')
    .fps(0.3)
    .weight(7)
    .biomes(REEF | TANK)
    .traits(SHELTER | STING),
    Plant::sprite(
        "bubble-tip-anemone",
        &[
            r"
 ● ● ● ● ●
 ┃│┃│┃│┃│┃
 ▜▓▓▓▓▓▓▓▛
  ▝▜▒▒▒▛▘
    ▐▓▌
",
            r"
 ○ ○ ○ ○ ○
 │┃│┃│┃│┃│
 ▜▓▓▓▓▓▓▓▛
  ▝▜▒▒▒▛▘
    ▐▓▌
",
        ],
        &[r"
 111111111
 111111111
 111111111
  1111111
    222
"],
    )
    .tint("1:rRmMyY 2:yY")
    .color('g')
    .fps(0.3)
    .weight(6)
    .biomes(REEF | TANK)
    .traits(SHELTER | STING),
    Plant::sprite(
        "sea-grass-meadow",
        &[
            r"
 │ ╲ │ ╱ │ ╲ │
 │╲ ╲│╱ ╱│╲ ╲│
 ││ ╲││╱ ││╲ │
",
            r"
 ╲ │ ╱ │ ╲ │ ╱
 ╱│ ╲│╱ │╱ │╲│
 ││╱ ││ ╲││ ╱│
",
        ],
        &[],
    )
    .color('g')
    .fps(0.3)
    .weight(7)
    .biomes(REEF | OCEAN)
    .traits(SHELTER),
    Plant::sprite(
        "fire-coral",
        &[
            r"
  ▲  ▲  ▲
 ◢█◣◢█◣◢█◣
 ▐▓▒▓▒▓▒▓▌
 ▝▀▀▀▀▀▀▀▘
",
            r"
  △  △  △
 ◢▓◣◢▓◣◢▓◣
 ▐▒▓▒▓▒▓▒▌
 ▝▀▀▀▀▀▀▀▘
",
        ],
        &[r"
  1111111
 111111111
 111111111
 111111111
"],
    )
    .tint("1:yYrR")
    .color('g')
    .fps(0.25)
    .weight(3)
    .biomes(REEF)
    .traits(STING),
    Plant::sprite(
        "sea-rod-fans",
        &[
            r"
 ╲ ╲│╱ ╱   ╲ ╲│╱ ╱
  ╲╲│╱╱     ╲╲│╱╱
   ╲│╱       ╲│╱
    │   ╲│╱   │
    │    │    │
    ┃    ┃    ┃
",
            r"
 ╱ ╱│╲ ╲   ╱ ╱│╲ ╲
  ╱╱│╲╲     ╱╱│╲╲
   ╱│╲       ╱│╲
    │   ╱│╲   │
    │    │    │
    ┃    ┃    ┃
",
        ],
        &[r"
 11111111111111111
  111111111111111
   1111111111111
    1111111111
    1111111111
    1111111111
"],
    )
    .tint("1:rRmMyY")
    .color('g')
    .fps(0.15)
    .weight(4)
    .biomes(REEF)
    .traits(SHELTER),
    Plant::sprite(
        "giant-kelp",
        &[
            r"
  ╲╷╱
  ╭╯◗
 ◖╰╮
  ╭╯◗
 ◖╰╮
  ╭╯◗
 ◖╰╮
  ╭╯◗
  │
  ▟▙
",
            r"
 ╲╷╱
 ◖╰╮
  ╭╯◗
 ◖╰╮
  ╭╯◗
 ◖╰╮
  ╭╯◗
 ◖╰╮
   │
  ▟▙
",
        ],
        &[
            r"

    G
 G
    G
 G
    G
 G
    G
",
            r"

 G
    G
 G
    G
 G
    G
 G
",
        ],
    )
    .color('y')
    .fps(0.2)
    .weight(6)
    .biomes(KELP)
    .traits(SHELTER),
    Plant::sprite(
        "bull-kelp",
        &[
            r"
 ╲╲╲╷╱╱╱
  ╲╲│╱╱
   ▟█▙
   ╰╮
   ╭╯
   ╰╮
   ╭╯
   ╰╮
   ▟▙
",
            r"
 ╱╱╱╷╲╲╲
  ╱╱│╲╲
   ▟█▙
   ╭╯
   ╰╮
   ╭╯
   ╰╮
   ╭╯
   ▟▙
",
        ],
        &[r"


   GGG
"],
    )
    .color('y')
    .fps(0.2)
    .weight(6)
    .biomes(KELP | ARCTIC)
    .traits(SHELTER),
    Plant::sprite(
        "feather-boa-kelp",
        &[
            r"
  ╲ ╷ ╱
 ╲╲╱┃╲╱╱
  ╲╲┃╱╱
 ╲╲╱┃╲╱╱
  ╲╲┃╱╱
    ┃
   ▟█▙
",
            r"
  ╱ ╷ ╲
 ╲╱╲┃╱╲╱
  ╱╲┃╲╱
 ╲╱╲┃╱╲╱
  ╱╲┃╲╱
    ┃
   ▟█▙
",
        ],
        &[r"
  1 1 1
 1111111
  11111
 1111111
  11111
    1
   111
"],
    )
    .tint("1:yYgG")
    .color('y')
    .fps(0.25)
    .weight(5)
    .biomes(KELP)
    .traits(SHELTER),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_plants_drawn(super::PLANTS);
    }
}
