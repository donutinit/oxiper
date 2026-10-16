//! Passers-by: whales, submarines, divers and stranger things, drawn in Unicode.

use super::*;

pub const VISITORS: &[Visitor] = &[
    Visitor::new(
        "sperm-whale",
        Lane::Deep,
        &[
            r"
◢▙                                    ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
◥█▙▖                      ▗▄▄▄▄▟██████████████████████▌
 ◥██▙▄▄▄▄▄▄▄▄▄▄▄▄▄██████████████████████████████████████▌
   ▝████████████████████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓◉▓▓▓▓▓▓▌
     ▝▀▀▀▀▀▀▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░▒▌
                  ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▒▒▒▒▒▒▒▒▒▒░░░░░▀▀▘
                                              ▝▀▀▀▀▀▀▀▀▀▘
?
",
            r"
                                      ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
  ◢▙                      ▗▄▄▄▄▟██████████████████████▌
◢▙▐█▄▄▄▄▄▄▄▄▄▄▄▄▄▄██████████████████████████████████████▌
 ▜██████████████████████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓◉▓▓▓▓▓▓▌
◥▛▐█▀▀▀▀▀▀▀▀▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░▒▌
  ◥◣              ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▒▒▒▒▒▒▒▒▒▒░░░░░▀▀▘
                                              ▝▀▀▀▀▀▀▀▀▀▘
?
",
            r"
                                      ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
                          ▗▄▄▄▄▟██████████████████████▌
     ▄▄▄▄▄▄▄▄▄▄▄▄▄██████████████████████████████████████▌
   ▄████████████████████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓◉▓▓▓▓▓▓▌
 ◢▟██▀▀▀▀▀▀▀▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░▒▌
◢█▛▘              ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▒▒▒▒▒▒▒▒▒▒░░░░░▀▀▘
◥█▙▖                                          ▝▀▀▀▀▀▀▀▀▀▘
  ◥▛
",
            r"
                                      ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
  ◢▙                      ▗▄▄▄▄▟██████████████████████▌
◢▙▐█▄▄▄▄▄▄▄▄▄▄▄▄▄▄██████████████████████████████████████▌
 ▜██████████████████████████████▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓◉▓▓▓▓▓▓▌
◥▛▐█▀▀▀▀▀▀▀▀▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░░░░░░▒▌
  ◥◣              ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▒▒▒▒▒▒▒▒▒▒░░░░░▀▀▘
                                              ▝▀▀▀▀▀▀▀▀▀▘
?
",
        ],
        &[
            r"
KK                                    KKKKKKKKKKKKKKKKKKK
KKKK                      KKKKKKKKKKKKKKKKKKKKKKKKKKKKK
 KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK
   KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKWKKKKKKK
     wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
                  WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                                              WWWWWWWWWWW
W
",
            r"
                                      KKKKKKKKKKKKKKKKKKK
  KK                      KKKKKKKKKKKKKKKKKKKKKKKKKKKKK
KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK
 KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKWKKKKKKK
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
  WW              WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                                              WWWWWWWWWWW
W
",
            r"
                                      KKKKKKKKKKKKKKKKKKK
                          KKKKKKKKKKKKKKKKKKKKKKKKKKKKK
     KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK
   KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKWKKKKKKK
 wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
WWWW              WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WWWW                                          WWWWWWWWWWW
  WW
",
            r"
                                      KKKKKKKKKKKKKKKKKKK
  KK                      KKKKKKKKKKKKKKKKKKKKKKKKKKKKK
KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK
 KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKWKKKKKKK
wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
  WW              WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                                              WWWWWWWWWWW
W
",
        ],
    )
    .speed(0.5, 1.0)
    .fps(0.15)
    .weight(4)
    .traits(PREDATOR)
    .biomes(DEEP | OCEAN),
    Visitor::new(
        "narwhal",
        Lane::Deep,
        &[
            r"
◢▙                   ▗▄▄▄▄▄▄▄▄▖
◥█▙▖         ▗▄▄▄▄▟████████████████▜▙▖
 ◥██▙▄▄▄▄▄▄▄████████▓▓∙▓▓▓∙▓▓∙▓▓▓▓∙▓▓█◉▓▌╾━╾━━━━━━━━━━━━━━━╸
   ▝███▒▒▒▓▓▓▒∙▒▒▒∙▒▒∙▒▒▒∙▒▒∙▒▒░░░░░░░░▐
     ▝▀▀▀▀▀▀▒▒▒▒▒▒░░░░░░░░░░░░░░░░▀▘
                     ▝▀▀▀▀▀▀▀▀▀▀▀▘
?
",
            r"
                     ▗▄▄▄▄▄▄▄▄▖
  ◢▙         ▗▄▄▄▄▟████████████████▜▙▖
◢▙▐█▄▄▄▄▄▄▄▄████████▓▓∙▓▓▓∙▓▓∙▓▓▓▓∙▓▓█◉▓▌╾━╾━━━━━━━━━━━━━━━╸
◥▛▐████▒▒▒▓▓▓▒∙▒▒▒∙▒▒∙▒▒▒∙▒▒∙▒▒░░░░░░░░▐
  ◥◣   ▀▀▀▀▀▒▒▒▒▒▒░░░░░░░░░░░░░░░░▀▘
                     ▝▀▀▀▀▀▀▀▀▀▀▀▘
?
",
            r"
                     ▗▄▄▄▄▄▄▄▄▖
             ▗▄▄▄▄▟████████████████▜▙▖
  ▄▄▄▄▄▄▄▄▄▄████████▓▓∙▓▓▓∙▓▓∙▓▓▓▓∙▓▓█◉▓▌╾━╾━━━━━━━━━━━━━━━╸
 ▄█████▒▒▒▓▓▓▒∙▒▒▒∙▒▒∙▒▒▒∙▒▒∙▒▒░░░░░░░░▐
 ◢▟█▀▀▀▀▀▀▀▀▒▒▒▒▒▒░░░░░░░░░░░░░░░░▀▘
◢█▛▘                 ▝▀▀▀▀▀▀▀▀▀▀▀▘
◥▛
",
            r"
                     ▗▄▄▄▄▄▄▄▄▖
  ◢▙         ▗▄▄▄▄▟████████████████▜▙▖
◢▙▐█▄▄▄▄▄▄▄▄████████▓▓∙▓▓▓∙▓▓∙▓▓▓▓∙▓▓█◉▓▌╾━╾━━━━━━━━━━━━━━━╸
◥▛▐████▒▒▒▓▓▓▒∙▒▒▒∙▒▒∙▒▒▒∙▒▒∙▒▒░░░░░░░░▐
  ◥◣   ▀▀▀▀▀▒▒▒▒▒▒░░░░░░░░░░░░░░░░▀▘
                     ▝▀▀▀▀▀▀▀▀▀▀▀▘
?
",
        ],
        &[
            r"
KK                   KKKKKKKKKK
KKKK         KKKKKKKKKKKKKKKKKKKKKKKKK
 wwwwwwwwwwwwwwwwwwwKKKKKKKKKKKKKKKKKwWKwWWWWWWWWWWWWWWWWWWW
   WWWWWWWWWWWKWWWKWWKWWWKWWKWWWWWWWWWWW
     WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                     WWWWWWWWWWWWW
W
",
            r"
                     KKKKKKKKKK
  KK         KKKKKKKKKKKKKKKKKKKKKKKKK
wwwwwwwwwwwwwwwwwwwwKKKKKKKKKKKKKKKKKwWKwWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWKWWWKWWKWWWKWWKWWWWWWWWWWW
  WW   WWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                     WWWWWWWWWWWWW
W
",
            r"
                     KKKKKKKKKK
             KKKKKKKKKKKKKKKKKKKKKKKKK
  wwwwwwwwwwwwwwwwwwKKKKKKKKKKKKKKKKKwWKwWWWWWWWWWWWWWWWWWWW
 WWWWWWWWWWWWWKWWWKWWKWWWKWWKWWWWWWWWWWW
 WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WWWW                 WWWWWWWWWWWWW
WW
",
            r"
                     KKKKKKKKKK
  KK         KKKKKKKKKKKKKKKKKKKKKKKKK
wwwwwwwwwwwwwwwwwwwwKKKKKKKKKKKKKKKKKwWKwWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWKWWWKWWKWWWKWWKWWWWWWWWWWW
  WW   WWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                     WWWWWWWWWWWWW
W
",
        ],
    )
    .speed(0.6, 1.2)
    .fps(0.2)
    .weight(4)
    .biomes(ARCTIC | OCEAN),
    Visitor::new(
        "beluga",
        Lane::Deep,
        &[
            r"
◢▙                        ▗▄▄▄▖
◥█▙▖        ▗▄▄▄▄▄▟███████████▀▀▀▀▜█▙▖
 ◥██▙▄▄▄▄▄▄▄█████████████████████◉█▌
   ▝███▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░‿‿▐
     ▝▀▀▀▀▀▀▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░▀▘
                   ▝▀▀▀▀▀▀▀▀▀▀▀▘
?
",
            r"
                          ▗▄▄▄▖
  ◢▙        ▗▄▄▄▄▄▟███████████▀▀▀▀▜█▙▖
◢▙▐█▄▄▄▄▄▄▄▄█████████████████████◉█▌
◥▛▐████▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░‿‿▐
  ◥◣   ▀▀▀▀▀▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░▀▘
                   ▝▀▀▀▀▀▀▀▀▀▀▀▘
?
",
            r"
                          ▗▄▄▄▖
            ▗▄▄▄▄▄▟███████████▀▀▀▀▜█▙▖
  ▄▄▄▄▄▄▄▄▄▄█████████████████████◉█▌
 ▄█████▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░‿‿▐
 ◢▟█▀▀▀▀▀▀▀▀▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░▀▘
◢█▛▘               ▝▀▀▀▀▀▀▀▀▀▀▀▘
◥▛
",
            r"
                          ▗▄▄▄▖
  ◢▙        ▗▄▄▄▄▄▟███████████▀▀▀▀▜█▙▖
◢▙▐█▄▄▄▄▄▄▄▄█████████████████████◉█▌
◥▛▐████▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░‿‿▐
  ◥◣   ▀▀▀▀▀▒▒▒▒▒▒▒▒░░░░░░░░░░░░░░▀▘
                   ▝▀▀▀▀▀▀▀▀▀▀▀▘
?
",
        ],
        &[
            r"
WW                        WWWWW
WWWW        WWWWWWWWWWWWWWWWWWWWWWWWWW
 WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWKWW
   WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWwwW
     wwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
                   wwwwwwwwwwwww
w
",
            r"
                          WWWWW
  WW        WWWWWWWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWKWW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWwwW
  ww   wwwwwwwwwwwwwwwwwwwwwwwwwwwww
                   wwwwwwwwwwwww
w
",
            r"
                          WWWWW
            WWWWWWWWWWWWWWWWWWWWWWWWWW
  WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWKWW
 WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWwwW
 wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwww               wwwwwwwwwwwww
ww
",
            r"
                          WWWWW
  WW        WWWWWWWWWWWWWWWWWWWWWWWWWW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWKWW
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWwwW
  ww   wwwwwwwwwwwwwwwwwwwwwwwwwwwww
                   wwwwwwwwwwwww
w
",
        ],
    )
    .speed(0.5, 1.0)
    .fps(0.2)
    .weight(5)
    .biomes(ARCTIC),
    Visitor::new(
        "blue-whale",
        Lane::Deep,
        &[
            r"
◢▙                                                        ◢▙
◥█▙▖                      ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟██▙▄▄▄▄▄▄▄▄▖
 ◥██▙▄▄▄▄▄▄▄▄▄▄▄██████████████████████████████████████▓▓▒▓▓▒▓▓▒▓▓▒▓▙▖
   ▝██████████████████████████████████████████████▓▓▒▓▓▒▓▒▓▓▒▓▒▓▓◉▓▓▓▓▓▒▒▛
     ▝▀▀▀▀▀▀▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒░▒░▒░░░░░░░░░░░░░░░░░░▀▘
                              ▝▀▀▀▀▀▒░▒░▒░▒░▒░▒░▒░▒░▒░▒░░░░░░░░░░░░░░░▀▘
                                              ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
                                                          ◢▙
  ◢▙                      ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟██▙▄▄▄▄▄▄▄▄▖
◢▙▐█▄▄▄▄▄▄▄▄▄▄▄▄██████████████████████████████████████▓▓▒▓▓▒▓▓▒▓▓▒▓▙▖
 ▜████████████████████████████████████████████████▓▓▒▓▓▒▓▒▓▓▒▓▒▓▓◉▓▓▓▓▓▒▒▛
◥▛▐█▀▀▀▀▀▀▀▀▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒░▒░▒░░░░░░░░░░░░░░░░░░▀▘
  ◥◣                          ▝▀▀▀▀▀▒░▒░▒░▒░▒░▒░▒░▒░▒░▒░░░░░░░░░░░░░░░▀▘
                                              ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
                                                          ◢▙
                          ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟██▙▄▄▄▄▄▄▄▄▖
     ▄▄▄▄▄▄▄▄▄▄▄██████████████████████████████████████▓▓▒▓▓▒▓▓▒▓▓▒▓▙▖
   ▄██████████████████████████████████████████████▓▓▒▓▓▒▓▒▓▓▒▓▒▓▓◉▓▓▓▓▓▒▒▛
 ◢▟██▀▀▀▀▀▀▀▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒░▒░▒░░░░░░░░░░░░░░░░░░▀▘
◢█▛▘                          ▝▀▀▀▀▀▒░▒░▒░▒░▒░▒░▒░▒░▒░▒░░░░░░░░░░░░░░░▀▘
◥█▙▖                                          ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
                                                          ◢▙
  ◢▙                      ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▟██▙▄▄▄▄▄▄▄▄▖
◢▙▐█▄▄▄▄▄▄▄▄▄▄▄▄██████████████████████████████████████▓▓▒▓▓▒▓▓▒▓▓▒▓▙▖
 ▜████████████████████████████████████████████████▓▓▒▓▓▒▓▒▓▓▒▓▒▓▓◉▓▓▓▓▓▒▒▛
◥▛▐█▀▀▀▀▀▀▀▀▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒░▒░▒░░░░░░░░░░░░░░░░░░▀▘
  ◥◣                          ▝▀▀▀▀▀▒░▒░▒░▒░▒░▒░▒░▒░▒░▒░░░░░░░░░░░░░░░▀▘
                                              ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
Bb                                                        bb
bbbB                      bbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbb
 BbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbBbbBbbBbbBBbb
   bBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbBbbBBBbbBbBbbWbbbBbBBB
     BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
                              WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                                              WWWWWWWWWWWWWWWWWWWWW
",
            r"
                                                          bb
  bB                      bbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbb
bBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbBbbBbbBbbBBbb
 bbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbBbbBBBbbBbBbbWbbbBbBBB
BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
  WW                          WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                                              WWWWWWWWWWWWWWWWWWWWW
",
            r"
                                                          bb
                          bbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbb
     bbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbBbbBbbBbbBBbb
   bBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbBbbBBBbbBbBbbWbbbBbBBB
 BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
WWWW                          WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
WWWW                                          WWWWWWWWWWWWWWWWWWWWW
",
            r"
                                                          bb
  bB                      bbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbb
bBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbBbbBbbBbbBBbb
 bbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbbbbbBbbbBbbbbBbbBBBbbBbBbbWbbbBbBBB
BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
  WW                          WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
                                              WWWWWWWWWWWWWWWWWWWWW
",
        ],
    )
    .speed(0.4, 0.8)
    .fps(0.12)
    .weight(2)
    .traits(FILTER)
    .biomes(OCEAN | DEEP | ARCTIC),
    Visitor::new(
        "giant-whale-shark",
        Lane::Deep,
        &[
            r"
◢▙                          ▗▄▄▄▄▄▄▄▄▖
◥█▙▖              ▗▄▄▄█∙███∙██∙██∙███∙██∙█▟█████▙▄▄▄▖
 ◥██▙▄▄▄▄▄▄▄▄▄██∙███∙█∙█████∙███∙█∙████████∙███∙██∙█▌
   ▝██████████∙█∙███∙████∙█∙███∙████∙█∙███∙██◉██∙███▌
     ▝▀▀▀▀▓▓▓▓▓▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒▒▒▒▒▒▒▒▒▒▌
                ▝▀▀▀░░░░░░░░░░░░░░░░░░░░░░░░━━━━━━━▘
                                        ▝▀▀▀▀▀▀▀▀▀▀▘
?
",
            r"
                            ▗▄▄▄▄▄▄▄▄▖
  ◢▙              ▗▄▄▄█∙███∙██∙██∙███∙██∙█▟█████▙▄▄▄▖
◢▙▐█▄▄▄▄▄▄▄▄▄▄██∙███∙█∙█████∙███∙█∙████████∙███∙██∙█▌
◥▛▐███████████∙█∙███∙████∙█∙███∙████∙█∙███∙██◉██∙███▌
  ◥◣      ▓▓▓▓▓▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒▒▒▒▒▒▒▒▒▒▌
                ▝▀▀▀░░░░░░░░░░░░░░░░░░░░░░░░━━━━━━━▘
                                        ▝▀▀▀▀▀▀▀▀▀▀▘
?
",
            r"
                            ▗▄▄▄▄▄▄▄▄▖
                  ▗▄▄▄█∙███∙██∙██∙███∙██∙█▟█████▙▄▄▄▖
     ▄▄▄▄▄▄▄▄▄██∙███∙█∙█████∙███∙█∙████████∙███∙██∙█▌
   ▄██████████∙█∙███∙████∙█∙███∙████∙█∙███∙██◉██∙███▌
 ◢▟██▀▀▀▀▀▓▓▓▓▓▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒▒▒▒▒▒▒▒▒▒▌
◢█▛▘            ▝▀▀▀░░░░░░░░░░░░░░░░░░░░░░░░━━━━━━━▘
◥█▙▖                                    ▝▀▀▀▀▀▀▀▀▀▀▘
  ◥▛
",
            r"
                            ▗▄▄▄▄▄▄▄▄▖
  ◢▙              ▗▄▄▄█∙███∙██∙██∙███∙██∙█▟█████▙▄▄▄▖
◢▙▐█▄▄▄▄▄▄▄▄▄▄██∙███∙█∙█████∙███∙█∙████████∙███∙██∙█▌
◥▛▐███████████∙█∙███∙████∙█∙███∙████∙█∙███∙██◉██∙███▌
  ◥◣      ▓▓▓▓▓▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒∙▒▒▒▒▒▒▒▒▒▒▒▒▌
                ▝▀▀▀░░░░░░░░░░░░░░░░░░░░░░░░━━━━━━━▘
                                        ▝▀▀▀▀▀▀▀▀▀▀▘
?
",
        ],
        &[
            r"
bb                          bbbbbbbbbb
bbbb              bbbbbWbbbWbbWbbWbbbWbbWbbbbbbbbbbbb
 bbbbbbbbbbbbbbbWbbbWbWbbbbbWbbbWbWbbbbbbbbWbbbWbbWbb
   bbbbbbbbbbbWbWbbbWbbbbWbWbbbWbbbbWbWbbbWbbWbbWbbbb
     BBBBBBBBBBBWBBBWBBBWBBBWBBBWBBBWBBBWBBBBBBBBBBBBB
                WWWWWWWWWWWWWWWWWWWWWWWWWWWWKKKKKKKW
                                        WWWWWWWWWWWW
W
",
            r"
                            bbbbbbbbbb
  bb              bbbbbWbbbWbbWbbWbbbWbbWbbbbbbbbbbbb
bbbbbbbbbbbbbbbbWbbbWbWbbbbbWbbbWbWbbbbbbbbWbbbWbbWbb
bbbbbbbbbbbbbbWbWbbbWbbbbWbWbbbWbbbbWbWbbbWbbWbbWbbbb
  BB      BBBBBBWBBBWBBBWBBBWBBBWBBBWBBBWBBBBBBBBBBBBB
                WWWWWWWWWWWWWWWWWWWWWWWWWWWWKKKKKKKW
                                        WWWWWWWWWWWW
W
",
            r"
                            bbbbbbbbbb
                  bbbbbWbbbWbbWbbWbbbWbbWbbbbbbbbbbbb
     bbbbbbbbbbbWbbbWbWbbbbbWbbbWbWbbbbbbbbWbbbWbbWbb
   bbbbbbbbbbbWbWbbbWbbbbWbWbbbWbbbbWbWbbbWbbWbbWbbbb
 BBBBBBBBBBBBBBBWBBBWBBBWBBBWBBBWBBBWBBBWBBBBBBBBBBBBB
WWWW            WWWWWWWWWWWWWWWWWWWWWWWWWWWWKKKKKKKW
WWWW                                    WWWWWWWWWWWW
  WW
",
            r"
                            bbbbbbbbbb
  bb              bbbbbWbbbWbbWbbWbbbWbbWbbbbbbbbbbbb
bbbbbbbbbbbbbbbbWbbbWbWbbbbbWbbbWbWbbbbbbbbWbbbWbbWbb
bbbbbbbbbbbbbbWbWbbbWbbbbWbWbbbWbbbbWbWbbbWbbWbbWbbbb
  BB      BBBBBBWBBBWBBBWBBBWBBBWBBBWBBBWBBBBBBBBBBBBB
                WWWWWWWWWWWWWWWWWWWWWWWWWWWWKKKKKKKW
                                        WWWWWWWWWWWW
W
",
        ],
    )
    .speed(0.4, 0.9)
    .fps(0.2)
    .weight(3)
    .traits(FILTER)
    .biomes(OCEAN | REEF | DEEP),
    Visitor::new(
        "giant-manta",
        Lane::Deep,
        &[
            r"
                 ▗▄▖
              ▗▟██▙▖
           ▗▟█████████▖
         ▗▟████████▓▓▓████▙▖
╌╌╌╌╌╌╌╌▟█████████████▓▓▓▓▓███◉▙▖
         ▝▜████████▓▓▓████▛▘
           ▝▜█████████▘
              ▝▜██▛▘
                 ▝▀▘
",
            r"

               ▗▄▄▖
           ▗▄▟████▙▄▖
        ▗▟█████████▓▓▓████▙▖
╌╌╌╌╌╌╌╌▟█████████████▓▓▓▓▓███◉▙▖
        ▝▜█████████▓▓▓████▛▘
           ▝▀▜████▛▀▘
               ▝▀▀▘
   ⠂
",
            r"



          ▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▖
╌╌╌╌╌╌╌╌▟█████████████▓▓▓▓▓███◉▙▖
          ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘


     ⠁
",
            r"

               ▗▄▄▖
           ▗▄▟████▙▄▖
        ▗▟█████████▓▓▓████▙▖
╌╌╌╌╌╌╌╌▟█████████████▓▓▓▓▓███◉▙▖
        ▝▜█████████▓▓▓████▛▘
           ▝▀▜████▛▀▘
               ▝▀▀▘
   ⠂
",
        ],
        &[
            r"
                 KKK
              KKKKKK
           KKKKKKKKKKKK
         KKKKKKKKKKWWWKKKKKK
wwwwwwwwKKKKKKKKKKKKKKWWWWWKKKWKK
         KKKKKKKKKKWWWKKKKKK
           KKKKKKKKKKKK
              KKKKKK
                 KKK
",
            r"

               KKKK
           KKKKKKKKKK
        KKKKKKKKKKKWWWKKKKKK
wwwwwwwwKKKKKKKKKKKKKKWWWWWKKKWKK
        KKKKKKKKKKKWWWKKKKKK
           KKKKKKKKKK
               KKKK
   W
",
            r"



          KKKKKKKKKKKKKKKKKKKK
wwwwwwwwKKKKKKKKKKKKKKWWWWWKKKWKK
          KKKKKKKKKKKKKKKKKKKK


     W
",
            r"

               KKKK
           KKKKKKKKKK
        KKKKKKKKKKKWWWKKKKKK
wwwwwwwwKKKKKKKKKKKKKKWWWWWKKKWKK
        KKKKKKKKKKKWWWKKKKKK
           KKKKKKKKKK
               KKKK
   W
",
        ],
    )
    .speed(0.6, 1.2)
    .fps(0.25)
    .weight(3)
    .traits(FILTER)
    .biomes(OCEAN | REEF | DEEP),
    Visitor::new(
        "giant-squid",
        Lane::Deep,
        &[
            r"
                             ▗▄▄▄▄▄▖
                      ▗▄▄▄██████████▙▄▄▖
      ╭──╮     ▄▄▄▄▄██████▓▓▓▓▓▒▒▒▒▒▒▓██▙▖
  ╭──╮╭╯  ╰──╮ ██████◉███▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓██▶
╭─╯  ╰╯ ╭──╮ ╰─████████▓▓▓▒▒▒▒░░░░░░░░▒▒▒▒▓██▶
 ╰─╮  ╭──╯  ╰─╮▀▀▀▀▜██▒▒▒▒▒░░░░░░░░░░▒▒▓██▛▘
   ╰──╯╭╮ ╭──╯      ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
                             ▗▄▄▄▄▄▖
                      ▗▄▄▄██████████▙▄▄▖
     ╭──╮      ▄▄▄▄▄██████▓▓▓▓▓▒▒▒▒▒▒▓██▙▖
   ╭╯  ╰╮ ╭──╮ ██████◉███▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓██▶
 ╭╯ ╭─╮ ╰─╯  ╰─████████▓▓▓▒▒▒▒░░░░░░░░▒▒▒▒▓██▶
╭╯ ╭╯ ╰╮ ╭────╯▀▀▀▀▜██▒▒▒▒▒░░░░░░░░░░▒▒▓██▛▘
╰──╯   ╰─╯          ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
                             ▗▄▄▄▄▄▖
                      ▗▄▄▄██████████▙▄▄▖
   ╭──╮        ▄▄▄▄▄██████▓▓▓▓▓▒▒▒▒▒▒▓██▙▖
 ╭─╯  ╰─╮ ╭───╮██████◉███▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓██▶
─╯ ╭──╮ ╰─╯ ╭─╯████████▓▓▓▒▒▒▒░░░░░░░░▒▒▒▒▓██▶
╭──╯  ╰╮ ╭──╯  ▀▀▀▀▜██▒▒▒▒▒░░░░░░░░░░▒▒▓██▛▘
╰╮   ╭─╯ ╰─╮        ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
                             ▗▄▄▄▄▄▖
                      ▗▄▄▄██████████▙▄▄▖
     ╭──╮      ▄▄▄▄▄██████▓▓▓▓▓▒▒▒▒▒▒▓██▙▖
   ╭╯  ╰╮ ╭──╮ ██████◉███▓▓▓▓▒▒▒▒▒▒▒▒▒▒▒▒▓██▶
 ╭╯ ╭─╮ ╰─╯  ╰─████████▓▓▓▒▒▒▒░░░░░░░░▒▒▒▒▓██▶
╭╯ ╭╯ ╰╮ ╭────╯▀▀▀▀▜██▒▒▒▒▒░░░░░░░░░░▒▒▓██▛▘
╰──╯   ╰─╯          ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
                             rrrrrrr
                      rrrrrrrrrrrrrrrrrr
      RRRR     rrrrrrrrrrrrrrrrRRRRRRrrrrr
  RRRRRR  RRRR rrrrrrWrrrrrrrRRRRRRRRRRRRrrrr
RRR  RR RRRR RRrrrrrrrrrrrRRRRRRRRRRRRRRRRrrrr
 RRR  RRRR  RRRRRRRrrrRRRRRRRRRRRRRRRRRrrrrR
   RRRRRR RRRR      RRRRRRRRRRRRRRRR
",
            r"
                             rrrrrrr
                      rrrrrrrrrrrrrrrrrr
     RRRR      rrrrrrrrrrrrrrrrRRRRRRrrrrr
   RR  RR RRRR rrrrrrWrrrrrrrRRRRRRRRRRRRrrrr
 RR RRR RRR  RRrrrrrrrrrrrRRRRRRRRRRRRRRRRrrrr
RR RR RR RRRRRRRRRRrrrRRRRRRRRRRRRRRRRRrrrrR
RRRR   RRR          RRRRRRRRRRRRRRRR
",
            r"
                             rrrrrrr
                      rrrrrrrrrrrrrrrrrr
   RRRR        rrrrrrrrrrrrrrrrRRRRRRrrrrr
 RRR  RRR RRRRRrrrrrrWrrrrrrrRRRRRRRRRRRRrrrr
RR RRRR RRR RRRrrrrrrrrrrrRRRRRRRRRRRRRRRRrrrr
RRRR  RR RRRR  RRRRrrrRRRRRRRRRRRRRRRRRrrrrR
RR   RRR RRR        RRRRRRRRRRRRRRRR
",
            r"
                             rrrrrrr
                      rrrrrrrrrrrrrrrrrr
     RRRR      rrrrrrrrrrrrrrrrRRRRRRrrrrr
   RR  RR RRRR rrrrrrWrrrrrrrRRRRRRRRRRRRrrrr
 RR RRR RRR  RRrrrrrrrrrrrRRRRRRRRRRRRRRRRrrrr
RR RR RR RRRRRRRRRRrrrRRRRRRRRRRRRRRRRRrrrrR
RRRR   RRR          RRRRRRRRRRRRRRRR
",
        ],
    )
    .speed(0.4, 0.9)
    .fps(0.2)
    .weight(3)
    .traits(PREDATOR | SCARY | LEGENDARY)
    .biomes(DEEP | OCEAN),
    Visitor::new(
        "sea-serpent",
        Lane::Deep,
        &[
            r"
     ◢◣              ◢◣              ◢◣              ◢◣              ◢◣◢◣
   ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗▟█▙▄▄▄▖
  ▟██████▙        ▟██████▙        ▟██████▙        ▟██████▙        ▟█◉█████▙▄▖
 ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟██▛▀▀▀▀▀▀▘╾╸
▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘ ▝▀▀▀▀▀▘
█▘        ▝▜█████▘        ▝▜█████▘        ▝▜█████▘        ▝▜█████▘
",
            r"
 ◢◣              ◢◣              ◢◣              ◢◣                  ◢◣◢◣
▄▄▄▄▖          ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗   ▗▟█▙▄▄▄▖
█████▙        ▟██████▙        ▟██████▙        ▟██████▙        ▟█  ▟█◉█████▙▄▖
▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛ ▟██▛▀▀▀▀▀▀▘╾╸
     ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘ ▟█▘ ▝▀▀▀▀▀▘
      ▝▜█████▘        ▝▜█████▘        ▝▜█████▘        ▝▜█████▘  █▘
",
            r"
             ◢◣              ◢◣              ◢◣              ◢◣      ◢◣◢◣
▖          ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗▄▄▄▄   ▗▟█▙▄▄▄▖
█▙        ▟██████▙        ▟██████▙        ▟██████▙        ▟█████  ▟█◉█████▙▄▖
▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝ ▟██▛▀▀▀▀▀▀▘╾╸
 ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘     ▟█▘ ▝▀▀▀▀▀▘
  ▝▜█████▘        ▝▜█████▘        ▝▜█████▘        ▝▜█████▘      █▘
",
            r"
         ◢◣              ◢◣              ◢◣              ◢◣          ◢◣◢◣
       ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗▄▄▄▄▖          ▗▄▄▄▄▖      ▗▟█▙▄▄▄▖
      ▟██████▙        ▟██████▙        ▟██████▙        ▟██████▙    ▟█◉█████▙▄▖
     ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙      ▟█▛▘  ▝▜█▙  ▟██▛▀▀▀▀▀▀▘╾╸
▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▙   ▟█▘      ▝▜█▟█▘ ▝▀▀▀▀▀▘
█████▘        ▝▜█████▘        ▝▜█████▘        ▝▜█████▘        ▝▜█▘
",
        ],
        &[
            r"
     RR              RR              RR              RR              RRRR
   GGGGGG          GGGGGG          GGGGGG          GGGGGG          GGGGGGGG
  GGGGGGGG        GGGGGGGG        GGGGGGGG        GGGGGGGG        GGYGGGGGGGG
 gggg  gggg      gggg  gggg      gggg  gggg      gggg  gggg      GGGGGGGGGGGRR
ggg      gggg   ggg      gggg   ggg      gggg   ggg      gggg   GGG GGGGGGG
gg        gggggggg        gggggggg        gggggggg        ggggggGG
",
            r"
 RR              RR              RR              RR                  RRRR
GGGGG          GGGGGG          GGGGGG          GGGGGG          G   GGGGGGGG
GGGGGG        GGGGGGGG        GGGGGGGG        GGGGGGGG        GG  GGYGGGGGGGG
g  gggg      gggg  gggg      gggg  gggg      gggg  gggg      ggg GGGGGGGGGGGRR
     gggg   ggg      gggg   ggg      gggg   ggg      gggg   ggg GGG GGGGGGG
      gggggggg        gggggggg        gggggggg        gggggggg  GG
",
            r"
             RR              RR              RR              RR      RRRR
G          GGGGGG          GGGGGG          GGGGGG          GGGGG   GGGGGGGG
GG        GGGGGGGG        GGGGGGGG        GGGGGGGG        GGGGGG  GGYGGGGGGGG
ggg      gggg  gggg      gggg  gggg      gggg  gggg      gggg  g GGGGGGGGGGGRR
 gggg   ggg      gggg   ggg      gggg   ggg      gggg   ggg     GGG GGGGGGG
  gggggggg        gggggggg        gggggggg        gggggggg      GG
",
            r"
         RR              RR              RR              RR          RRRR
       GGGGGG          GGGGGG          GGGGGG          GGGGGG      GGGGGGGG
      GGGGGGGG        GGGGGGGG        GGGGGGGG        GGGGGGGG    GGYGGGGGGGG
     gggg  gggg      gggg  gggg      gggg  gggg      gggg  gggg  GGGGGGGGGGGRR
g   ggg      gggg   ggg      gggg   ggg      gggg   ggg      gggGGG GGGGGGG
gggggg        gggggggg        gggggggg        gggggggg        ggGG
",
        ],
    )
    .speed(0.5, 1.0)
    .fps(0.3)
    .weight(2)
    .traits(PREDATOR | SCARY | LEGENDARY)
    .biomes(OCEAN | DEEP | KELP),
    Visitor::new(
        "rov",
        Lane::Deep,
        &[
            r"
         │
         │
      ▗▄▄▄▄▄▄▖
   ▗▄▄▟█▀▀▀▀█▙▄▄▖⠂
 ◖██▌▐█  ◉   █▌▐██◗══╾◁
 ▝▀▀▘▜█▄▄▄▄▄█▛▝▀▀▘
     ▀▀▜▛▀▀▀▜▛▀▀
",
            r"
         │
         │
      ▗▄▄▄▄▄▄▖
   ▗▄▄▟█▀▀▀▀█▙▄▄▖⠈
 ◖██▌▐█  ◎   █▌▐██◗══╾◀
 ▝▀▀▘▜█▄▄▄▄▄█▛▝▀▀▘
     ▀▀▜▛▀▀▀▜▛▀▀
",
            r"
         │
         │
      ▗▄▄▄▄▄▄▖
   ▗▄▄▟█▀▀▀▀█▙▄▄▖⠐
 ◖██▌▐█  ◉   █▌▐██◗══╾◁
 ▝▀▀▘▜█▄▄▄▄▄█▛▝▀▀▘
     ▀▀▜▛▀▀▀▜▛▀▀
",
            r"
         │
         │
      ▗▄▄▄▄▄▄▖
   ▗▄▄▟█▀▀▀▀█▙▄▄▖⠁
 ◖██▌▐█  ◎   █▌▐██◗══╾◀
 ▝▀▀▘▜█▄▄▄▄▄█▛▝▀▀▘
     ▀▀▜▛▀▀▀▜▛▀▀
",
        ],
        &[
            r"
         K
         K
      YYYYYYYY
   YYYYYYYYYYYYYYW
 KYYYYY  W   YYYYYKwwww
 YYYYYYYYYYYYYYYYY
     YYYYYYYYYYY
",
            r"
         K
         K
      YYYYYYYY
   YYYYYYYYYYYYYYW
 KYYYYY  W   YYYYYKwwww
 YYYYYYYYYYYYYYYYY
     YYYYYYYYYYY
",
            r"
         K
         K
      YYYYYYYY
   YYYYYYYYYYYYYYW
 KYYYYY  W   YYYYYKwwww
 YYYYYYYYYYYYYYYYY
     YYYYYYYYYYY
",
            r"
         K
         K
      YYYYYYYY
   YYYYYYYYYYYYYYW
 KYYYYY  W   YYYYYKwwww
 YYYYYYYYYYYYYYYYY
     YYYYYYYYYYY
",
        ],
    )
    .speed(0.3, 0.7)
    .fps(0.3)
    .weight(3)
    .traits(LURE | GLOW)
    .biomes(DEEP | OCEAN | REEF),
    Visitor::new(
        "toy-submarine",
        Lane::Deep,
        &[
            r"
        ▗▄▖
╲   ▗▄▄▟█▙▄▄▄▖
─▐▌▐██ ◉ ◉ ◉ ██▌
╱   ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
        ▗▄▖
│   ▗▄▄▟█▙▄▄▄▖
│▐▌▐██ ◉ ◉ ◉ ██▌
│   ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
        ▗▄▖
╱   ▗▄▄▟█▙▄▄▄▖
─▐▌▐██ ◉ ◉ ◉ ██▌
╲   ▝▀▀▀▀▀▀▀▀▀▘
",
            r"
        ▗▄▖
│   ▗▄▄▟█▙▄▄▄▖
│▐▌▐██ ◉ ◉ ◉ ██▌
│   ▝▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
        RRR
R   RRRYYYRRRR
RYYYYY C C C YYY
R   RRRRRRRRRRR
",
            r"
        RRR
R   RRRYYYRRRR
RYYYYY C C C YYY
R   RRRRRRRRRRR
",
            r"
        RRR
R   RRRYYYRRRR
RYYYYY C C C YYY
R   RRRRRRRRRRR
",
            r"
        RRR
R   RRRYYYRRRR
RYYYYY C C C YYY
R   RRRRRRRRRRR
",
        ],
    )
    .speed(0.3, 0.8)
    .fps(0.4)
    .weight(6)
    .traits(LURE)
    .biomes(TANK),
    Visitor::new(
        "mermaid",
        Lane::Deep,
        &[
            r"
                       ▗▄▄▖
         ≈≈≈≈≈≈≈≈≈≈≈≈▗▟█◉▙▌
  ▗▖    ≋≋≋≋≋≋≋≋≋≋≋≋▝▜▙▟▛
 ▗▟█▙▖  ▗▄▄▄▄▄▄▄▄▄▄▟██▛▀╾─╴
▐███▙▄▄▟▛◆◇◆◇◆◇◆▜▛▀▀▘
 ▝▀▜█▛▀▀▀▀▝▀▀▀▀▘
    ▝▘
",
            r"
    ▗▖                 ▗▄▄▖
 ▗▟█▙▖  ≈≈≈≈≈≈≈≈≈≈≈≈▗▟█◉▙▌
▐███▙▄▄▖ ≋≋≋≋≋≋≋≋≋≋≋▝▜▙▟▛
 ▝▀▜█▙▄▄▟▄▄▄▄▄▄▄▄▄▄▟██▛▀╾─╴
   ▝▀▀▜▛◆◇◆◇◆◇◆▜▛▀▀▘
      ▝▀▀▀▀▀▀▀▀▘
              ⠁
",
        ],
        &[
            r"
                       WWWW
         YYYYYYYYYYYYWWWKWW
  GG    YYYYYYYYYYYYGWWWW
 GGGGG  GGGGGGGGGGGGGGWWGGG
GGGGGGGGGgggggggGGGGW
 GGGGGGGGGGGGGGG
    GG
",
            r"
    GG                 WWWW
 GGGGG  YYYYYYYYYYYYGWWKWW
GGGGGGGG YYYYYYYYYYYGWWWW
 GGGGGGGGGGGGGGGGGGGGGWWGGG
   GGGGGgggggggGGGGG
      GGGGGGGGGG
              G
",
        ],
    )
    .speed(0.4, 0.9)
    .fps(0.3)
    .weight(1)
    .traits(LURE | LEGENDARY)
    .biomes(REEF | OCEAN | KELP),
    Visitor::new(
        "swimmer",
        Lane::Float,
        &[
            r"
           ╭──╮
   ⠂⠄     ▗▟█◉▙╯
≈≈⠄≈▄▄▄▄▄▄▄██▀▀≈≈
 ∼≈≈∼≈≈∼≈≈∼≈≈∼
",
            r"
      ⠈⠁
          ▗▟█◉▙▖
≈≈⠂≈▄▄▄▄▄▄▄▄██▀╲≈
 ≈∼≈≈∼≈≈∼≈≈∼≈≈
",
            r"
                ╲
  ⠁⠂      ▗▟█◉▙▖ ╲
≈≈⠈≈▄▄▄▄▄▄▄▄██▀≈≈
 ∼≈≈∼≈≈∼≈≈∼≈≈∼
",
        ],
        &[
            r"
           yyyy
   WW     11yK1y
CCWCyyyyyyy11yyCC
 CCCCCCCCCCCCC
",
            r"
      WW
          11yK11
CCWCyyyyyyyy11yyC
 CCCCCCCCCCCCC
",
            r"
                y
  WW      11yK11 y
CCWCyyyyyyyy11yCC
 CCCCCCCCCCCCC
",
        ],
    )
    .tint("1:rRbBgG")
    .speed(0.3, 0.7)
    .fps(0.3)
    .weight(6)
    .traits(LURE)
    .biomes(OCEAN | REEF | KELP | FRESH),
    Visitor::new(
        "surfer",
        Lane::Float,
        &[
            r"
       ●
     ╲▐█▌╱
       █
      ▟ ▙
  ◢▄▄▄▄▄▄▄▄▄◣
 ≈≈≈≈≈≈≈≈≈≈≈≈≈≈
",
            r"
       ●
     ╱▐█▌╲
       █
      ▟▌▐▙
  ◢▄▄▄▄▄▄▄▄▄◣
 ≈≈≈≈≈≈≈≈≈≈≈≈≈≈
",
        ],
        &[
            r"
       y
     yyyyy
       1
      K K
  11111111111
 CCCCCCCCCCCCCC
",
            r"
       y
     yyyyy
       1
      KKKK
  11111111111
 CCCCCCCCCCCCCC
",
        ],
    )
    .tint("1:rRbBgG")
    .speed(0.8, 1.6)
    .fps(0.2)
    .weight(4)
    .biomes(OCEAN | REEF),
    Visitor::new(
        "rubber-duck",
        Lane::Float,
        &[r"
       ▗▄▄▖
      ▐█◉█▌▶
   ▗▖  ▜██▛
 ▗▟█▙▄▄▟██▙▖
 ▝▜███████▛▘
   ▝▀▀▀▀▀▘
"],
        &[r"
       YYYY
      YYKYYR
   YY  YYYY
 YYYYYYYYYYY
 YYYYYYYYYYY
   YYYYYYY
"],
    )
    .speed(0.2, 0.5)
    .weight(5)
    .biomes(FRESH | TANK | OCEAN | KELP),
    Visitor::new(
        "message-in-a-bottle",
        Lane::Float,
        &[
            r"
    ▗▄▄▄▄▄▄▄▄▄▄▖
  ▗▟▘ ▬▬▬▬▬▬▬  ▝▙▀▀▀▀▀▀▀
  ▝▙▖ ▬▬▬▬▬▬▬   ▗▟▄▄▄▄▄▄█▌
    ▝▀▀▀▀▀▀▀▀▀▀▘
",
            r"
    ▗▄▄▄▄▄▄▄▄▄▄▖
  ▗▟▘ ▬▬ ▬▬▬▬  ▝▙▀▀▀▀▀▀▀
  ▝▙▖ ▬▬▬▬ ▬▬   ▗▟▄▄▄▄▄▄█▌
    ▝▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
    CCCCCCCCCCCC
  CCC YYYYYYY  CCCCCCCCC
  CCC YYYYYYY   CCCCCCCCyC
    CCCCCCCCCCCC
",
            r"
    CCCCCCCCCCCC
  CCC YY YYYY  CCCCCCCCC
  CCC YYYY YY   CCCCCCCCyC
    CCCCCCCCCCCC
",
        ],
    )
    .speed(0.1, 0.3)
    .fps(0.1)
    .weight(4)
    .biomes(OCEAN | REEF | KELP | ARCTIC | TANK),
    Visitor::new(
        "bell-buoy",
        Lane::Float,
        &[
            r"
       ✦
      ▐◉▌
     ▟▐█▌▙
    ╱ ◖◗ ╲
   ▗▟████▙▖
  ▐████████▌
  ▝▀▀▀▀▀▀▀▀▘
",
            r"
       ✧
      ▐◎▌
     ▟▐█▌▙
    ╱ ◖◗ ╲
   ▗▟████▙▖
  ▐████████▌
  ▝▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
       Y
      wYw
     wwwww
    w YY w
   11111111
  1111111111
  1111111111
",
            r"
       Y
      wYw
     wwwww
    w YY w
   11111111
  1111111111
  1111111111
",
        ],
    )
    .tint("1:rRgG")
    .speed(0.1, 0.25)
    .fps(0.15)
    .weight(5)
    .biomes(OCEAN | KELP | REEF | FRESH),
    Visitor::new(
        "iceberg",
        Lane::Float,
        &[r"
             ◢◣
            ◢██◣         ◢◣
         ◢◣◢███▓◣      ◢█▓◣
        ◢██████▓▓◣◢◣ ◢███▓▓◣
     ◢▄▄█████▛▀▜▓▓▓███████▓▓▓◣
  ◢▄▟██████▛▀  ∙ ▜▓▓▓█████▛▀▜▓◣
▗▟█████████▌    ∙ ▐▓▓▓▓████▌ ∙ ▐▓▙▖
 ▝▀▀▀▜██████▙▄▄▄▄▄▟▓▓▓▓▓▓▓▄▄▄▟▓▛▀▀▘
"],
        &[r"
             WW
            WWWW         WW
         WWWWWWCW      WWCW
        WWWWWWWCCWWW WWWWCCW
     WWWWWWWWWWWCCCWWWWWWWCCCW
  WWWWWWWWWWW  W WCCCWWWWWWWWCW
WWWWWWWWWWWW    W WCCCCWWWWW W WCWW
 CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC
"],
    )
    .speed(0.1, 0.25)
    .weight(5)
    .biomes(ARCTIC),
    Visitor::new(
        "ice-floe-bear",
        Lane::Float,
        &[
            r"
                 ▗▖
     ▗▄▄▄▄▄▄▄▄▄▟██▙▖
   ▗▟██████████████◉▙▶
  ▐████████████████▛▀
   ▜█▌▐█▌    ▐█▌▐█▌
░▒▒▄▄▄▄▄▟▛▀▀▀▀▀▜▙▄▄▄▄▄▄▒░
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
                  ▗▖
     ▗▄▄▄▄▄▄▄▄▄▟██▙▖
   ▗▟██████████████◉▙▖
  ▐████████████████▛▀▀
   ▜█▌▐█▌    ▐█▌▐█▌
░▒▒▄▄▄▄▄▟▛▀▀▀▀▀▜▙▄▄▄▄▄▄▒░
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
                 WW
     WWWWWWWWWWWWWWW
   WWWWWWWWWWWWWWWWKWK
  WWWWWWWWWWWWWWWWWWW
   WWWWWW    WWWWWW
CCCCCCCCCCCCCCCCCCCCCCCCC
 CCCCCCCCCCCCCCCCCCCCCCCC
",
            r"
                  WW
     WWWWWWWWWWWWWWW
   WWWWWWWWWWWWWWWWKWW
  WWWWWWWWWWWWWWWWWWWW
   WWWWWW    WWWWWW
CCCCCCCCCCCCCCCCCCCCCCCCC
 CCCCCCCCCCCCCCCCCCCCCCCC
",
        ],
    )
    .speed(0.1, 0.3)
    .fps(0.12)
    .weight(4)
    .biomes(ARCTIC),
    Visitor::new(
        "penguin-floe",
        Lane::Float,
        &[
            r"
  ▗▄▖      ▗▄▖        ▗▄▖
  █◉█▶     █◉█▶       █◉█▶
 ▐█▒█▌    ▐█▒█▌      ▐█▒█▌
  ▜▒▛      ▜▒▛        ▜▒▛
  ▀▀▀      ▀▀▀        ▀▀▀
░▒▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▒░
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
            r"
  ▗▄▖      ▗▄▖        ▗▄▖
  █◉█▶     █◉█▶       █◉█▶
 ╱█▒█╲    ╱█▒█╲      ╱█▒█╲
  ▜▒▛      ▜▒▛        ▜▒▛
  ▀▀▀      ▀▀▀        ▀▀▀
░▒▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▒░
 ▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
  KKK      KKK        KKK
  KWKY     KWKY       KWKY
 KKWKK    KKWKK      KKWKK
  KWK      KWK        KWK
  YYY      YYY        YYY
CCCCCCCCCCCCCCCCCCCCCCCCCCC
 CCCCCCCCCCCCCCCCCCCCCCCCCC
",
            r"
  KKK      KKK        KKK
  KWKY     KWKY       KWKY
 KKWKK    KKWKK      KKWKK
  KWK      KWK        KWK
  YYY      YYY        YYY
CCCCCCCCCCCCCCCCCCCCCCCCCCC
 CCCCCCCCCCCCCCCCCCCCCCCCCC
",
        ],
    )
    .speed(0.1, 0.3)
    .fps(0.15)
    .weight(4)
    .biomes(ARCTIC),
    Visitor::new(
        "raft-castaway",
        Lane::Float,
        &[
            r"
           ▕▛▀▜▙
           ▕▌  ▐▶
     ╲▐▌╱  ▕▙▄▟▛
      █    ▕
     ▟ ▙   ▕
▟█▀▀▀▀▀▀▀▀▀▜▙▀▀▀▀▀▀▀▀▙▖
 ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈
",
            r"
           ▕▛▀▀▜▙
           ▕▌  ▐▶
      ▐▌╱  ▕▙▄▄▟▛
     ▗█    ▕
     ▟▌▐   ▕
▟█▀▀▀▀▀▀▀▀▀▜▙▀▀▀▀▀▀▀▀▙▖
 ≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈
",
        ],
        &[
            r"
           wRRRR
           wR  RR
     YYYY  wRRRR
      Y    w
     Y Y   w
yyyyyyyyyyyyyyyyyyyyyyy
 CCCCCCCCCCCCCCCCCCCCC
",
            r"
           wRRRRR
           wR  RR
      YYY  wRRRRR
     YY    w
     YYY   w
yyyyyyyyyyyyyyyyyyyyyyy
 CCCCCCCCCCCCCCCCCCCCC
",
        ],
    )
    .speed(0.15, 0.4)
    .fps(0.2)
    .weight(3)
    .biomes(OCEAN | REEF | KELP),
    Visitor::new(
        "floating-otter",
        Lane::Float,
        &[
            r"
         ▗▄▄▖
   ▗▄▄▄▟█◉◉█▙▖
 ▟██▄▄▄▄▟██▙▄▄█▌
▐█▛◖◗▀▀▀▜██▛▀▀▀▘
  ▝▀▀▀▀▀▀▀▀▀▀▘
",
            r"
         ▗▄▄▖
   ▗▄▄▄▟█◉◉█▙▖
 ▟██▄▄▄▄▟██▙▄▄█▌
▐█▛▀▀▀▀▀▜██▛◖◗▘
  ▝▀▀▀▀▀▀▀▀▀▀▘
",
        ],
        &[
            r"
         yyyy
   yyyyyyKKWWW
 yyyyyyyyyyyyyyy
yyyWWyyyyyyyyyyy
  yyyyyyyyyyyy
",
            r"
         yyyy
   yyyyyyKKWWW
 yyyyyyyyyyyyyyy
yyyyyyyyyyyyWWy
  yyyyyyyyyyyy
",
        ],
    )
    .speed(0.1, 0.3)
    .fps(0.15)
    .weight(4)
    .biomes(KELP | OCEAN | ARCTIC),
    Visitor::new(
        "paper-boat",
        Lane::Boat,
        &[r"
         ◢◣
        ◢█▓◣
       ◢█▓▓▓◣
      ◢█▓▓▓▓▓◣
 ◢▄▄▄▄█▓▓▓▓▓▓▄▄▄▄◣
 ◥▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀◤
"],
        &[r"
         WW
        WWwW
       WWwwwW
      WWwwwwwW
 WWWWWWwwwwwwWWWWW
 WWWWWWWWWWWWWWWWWW
"],
    )
    .speed(0.2, 0.5)
    .weight(5)
    .biomes(TANK | FRESH),
    Visitor::new(
        "deep-sea-diver",
        Lane::Floor,
        &[
            r"
      ▗▄▄▄▖
    ▗▟██████▙▖    ∘
    ▐██▛▀▀▜◉█▌   ◦
    ▝▜█▄▄▄▄█▛▘  ∘
   ▗▄▟▀▀▀▀▀▜▙▄▖
  ▐█▌▒▒▒▒▒▒▐█▌
   ▜▌▒▒▒▒▒▒▐▛
    ▐▌  ▐▌
    ▐▌  ▐▌
   ▟█▌  ▐█▙
",
            r"
      ▗▄▄▄▖
    ▗▟██████▙▖    ◦
    ▐██▛▀▀▜◉█▌   ∘
    ▝▜█▄▄▄▄█▛▘  ◦
   ▗▄▟▀▀▀▀▀▜▙▄▖
  ▐█▌▒▒▒▒▒▒▐█▌
   ▜▌▒▒▒▒▒▒▐▛
    ▐▌ ▐▌
    ▐▌ ▐▌
    ▟█▌▐█▙
",
        ],
        &[
            r"
      YYYYY
    YYYYYYYYYY    W
    YYYYYYYCYY   W
    YYYYYYYYYY  W
   YYYYYYYYYYYY
  yyyyyyyyyyyy
   yyyyyyyyyy
    yy  yy
    yy  yy
   KKK  KKK
",
            r"
      YYYYY
    YYYYYYYYYY    W
    YYYYYYYCYY   W
    YYYYYYYYYY  W
   YYYYYYYYYYYY
  yyyyyyyyyyyy
   yyyyyyyyyy
    yy yy
    yy yy
    KKKKKK
",
        ],
    )
    .speed(0.2, 0.5)
    .fps(0.15)
    .weight(4)
    .traits(LURE | GLOW)
    .biomes(DEEP | OCEAN | REEF | KELP | TANK),
    Visitor::new(
        "sea-rover",
        Lane::Floor,
        &[
            r"
          ▗▄▄▄▖    ╷
    ▗▄▄▄▄▟▀▀▀▜▙▄▄▄▄▖
   ▐█◉█▌▐█ ▪ █▌▐████▌▀╾
  ◖●◗◖●◗◖●◗◖●◗◖●◗◖●◗
",
            r"
          ▗▄▄▄▖    ╷
    ▗▄▄▄▄▟▀▀▀▜▙▄▄▄▄▖
   ▐█◉█▌▐█ ▫ █▌▐████▌▀╾
  ◖◉◗◖◉◗◖◉◗◖◉◗◖◉◗◖◉◗
",
        ],
        &[
            r"
          YYYYY    w
    YYYYYYYYYYYYYYYY
   YYWYYYY W YYYYYYYYYw
  KKKKKKKKKKKKKKKKKK
",
            r"
          YYYYY    w
    YYYYYYYYYYYYYYYY
   YYWYYYY W YYYYYYYYYw
  KKKKKKKKKKKKKKKKKK
",
        ],
    )
    .speed(0.15, 0.4)
    .fps(0.4)
    .weight(4)
    .traits(LURE | GLOW)
    .biomes(DEEP | OCEAN),
];

#[cfg(test)]
mod tests {
    #[test]
    fn drawn_by_hand_in_unicode() {
        super::super::assert_visitors_drawn(super::VISITORS);
    }
}
