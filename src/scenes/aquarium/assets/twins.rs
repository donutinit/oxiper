//! asciiquarium's own sprites drawn again for the world beyond classic: the
//! passers-by the engine moves itself (ship, whale, sea monster, big fish,
//! shark, ducks, swan, dolphins, the fishhook), the bubbles, the splat and
//! the water's surface. Its fish, seaweed and castle are in `classic.rs`.
//! `biome = "classic"` keeps the originals in `../art.rs`.

use super::super::{
    SP_BIG_FISH, SP_BUBBLE, SP_DOLPHIN, SP_DUCKS, SP_HOOK, SP_LINE, SP_MONSTER, SP_POINT, SP_SHARK,
    SP_SHIP, SP_SPLAT, SP_SWAN, SP_WHALE,
};

/// Engine sprites drawn again: (the classic sprite they stand for, frames,
/// masks). Each has the size of the one it stands for, so the engine's
/// offsets still fit (the shark's teeth, the hook's point, the ship's
/// waterline). Those that travel are drawn facing right and mirrored; the
/// classic sprite named is the right-facing one. Until one is drawn, the
/// world uses the original.
pub const SPRITES: &[(u16, &[&str], &[&str])] = &[
    (
        17,
        &[
            r"
        ?
 
    ✦
 
 ",
            r"
        ?
    ∙
   ∙✹∙
    ∙
 ",
            r"
    ∙   ?
  ╲?│?╱
 ∙─?✹?─∙
  ╱?│?╲
    ∙",
            r"
∙??∘???∙
   ∙???∘
∘???✧???∙
 ∙???∘
∙???∙???∘",
        ],
        &[],
    ),
    (
        18,
        &[r"
                           ▗▄▖
 ▗▖????????????????????????▐██▙▖
 ▐█▙???????????????????????▐█████▙▖
  ▝██▙▖???▗▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄████████▄▄▄▄▄▄
    ▝███▙▄▄▄▟███████████████████████████████▙▄▄▄▄▖
      ▝████████████████████████████╲╲╲██████◉████▛▀▀▘
   ▗▟██▛▝▀▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓???▀▀▀▀▀▀▀▀▀▘
 ▗▟█▛▘??????▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒?◥◤◥◤◥◤◥◤◥◤
▐█▛▘??????????▝▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀?▗██▛????▝▀▀▀▀▀▀▀▀▘
                                ▝▀▀▘"],
        &[r"
                           ccc
 cc                        ccccc
 ccc                       cccccccc
  ccccc   ccccccccccccccccccccccccccccccc
    cccccccccccccccccccccccccccccccccccccccccccccc
      ccccccccccccccccccccccccccccccccccccccWcccccccc
   ccccccccccccccccccccccccccccccccccccc   cccccccccc
 ccccc      cccccccccccccccccccccccccccccc WWWWWWWWWW
cccc          ccccccccccccccccccc cccc    cccccccccc
                                cccc"],
    ),
    (
        21,
        &[r"
       ○
       ┃
       ┃
 │?????┃
 │?????┃
 ╰━━━━━╯"],
        &[],
    ),
    (
        22,
        &[r"
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
│
 
 
 
 
 
 
"],
        &[],
    ),
    (
        23,
        &[r"
▴
 
│
 "],
        &[],
    ),
    (
        24,
        &[r"
     ╽▸???╽▸???╽▸
    ▟█▙??▟█▙??▟█▙
   ▟▓█▓▙▟▓█▓▙▟▓█▓▙╲
  ▟▓▓█▓▓▙▟▓▓█▓▓▙▟▓▓█▓▓▙╲╲
▀▀▀▀▀╽▀▀▀▀╽▀▀▀▀╽▀▀▀▀▀▀▀▀▀
◥▓▓▓▓○▓▓▓▓▓○▓▓▓▓▓○▓▓▓◤"],
        &[r"
     yR   yR   yR
    WWW  WWW  WWW
   WWWWWWWWWWWWWWWw
  WWWWWWWWWWWWWWWWWWWWWww
yyyyyyyyyyyyyyyyyyyyyyyyy
yyyyyyyyyyyyyyyyyyyyyy"],
    ),
    (
        26,
        &[
            r"
 
 
 
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
 
 
 
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
 
 
 
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
 
 
 
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
 
 
 
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
 
 
              ┊
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
 
              ┊
              ┊
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
             ∘?∘
             ╶┊╴
              ┊
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
             ∘?∘
            ╭╴┊╶╮
              ┊
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
             ∘?∘
           ◜╭╴┊╶╮◝
           ╷??┊??╷
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
 
            ╭╴?╶╮
           ◟??┊??◞
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
            r"
 
 
           ◟?????◞
        ▂▅█████▅▂
      ▄██████████▙▂
▗▖??▟███████████◉█▙
▝▜▄▄▓▓▓▓▓▓▓▓▓▓▓▓▓▀▘",
        ],
        &[r"
CCCCCCCCCCCCCCCCCCC
CCCCCCCCCCCCCCCCCCC
CCCCCCCCCCCCCCCCCCC
BBBBBBBBBBBBBBBBBBB
BBBBBBBBBBBBBBBBBBB
BBBBBBBBBBBBBBBBBBB
BBBBBBBWWWWWWWWWWWW"],
    ),
    (
        28,
        &[
            r"
                                                          ▗▄▄▄▄▖
                                                        ▗▟██◉████▙
     ▗▄▄▖???????????▗▄▄▖???????????▗▄▄▖?????????????????▐██▙▄▄▄▄▟▛
    ▗▛??▜▖?????????▗▛??▜▖?????????▗▛??▜▖????????????????▐██▌
    ▐▌??▐▌?????????▐▌??▐▌?????????▐▌??▐▌????????????????▐██▌",
            r"
                                                          ▗▄▄▄▄▖
                                                        ▗▟██◉████▙
  ▗▄▄▖???????????▗▄▄▖???????????▗▄▄▖???????????▗▄▄▖?????▐██▙▄▄▄▄▟▛
 ▗▛??▜▖?????????▗▛??▜▖?????????▗▛??▜▖?????????▗▛??▜▖????▐██▌
 ▐▌??▐▌?????????▐▌??▐▌?????????▐▌??▐▌?????????▐▌??▐▌????▐██▌",
            r"
                                                          ▗▄▄▄▄▖
                                                        ▗▟██◉████▙
▄▄▖???????????▗▄▄▖???????????▗▄▄▖???????????▗▄▄▖????????▐██▙▄▄▄▄▟▛
??▜▖?????????▗▛??▜▖?????????▗▛??▜▖?????????▗▛??▜▖???????▐██▌
??▐▌?????????▐▌??▐▌?????????▐▌??▐▌?????????▐▌??▐▌???????▐██▌",
            r"
                                                          ▗▄▄▄▄▖
                                                        ▗▟██◉████▙
           ▗▄▄▖???????????▗▄▄▖???????????▗▄▄▖???????????▐██▙▄▄▄▄▟▛
▖?????????▗▛??▜▖?????????▗▛??▜▖?????????▗▛??▜▖??????????▐██▌
▌?????????▐▌??▐▌?????????▐▌??▐▌?????????▐▌??▐▌??????????▐██▌",
        ],
        &[r"
                                                          GGGGGG
                                                        GGGGWGGGGG
     GGGG           GGGG           GGGG                 GGGGGGGGGG
    GGGGGG         GGGGGG         GGGGGG                GGGG
    GGGGGG         GGGGGG         GGGGGG                GGGG"],
    ),
    (
        30,
        &[r"
▒▒▖?????????????░▒▒▒▒░
░▒▒▒▒▖???????▗▒▒▒▒▒▒▒▒▒▖
  ▝▒▒▒▒▒▖?????▄▄▄▄▄▄▄▄▄▄▄▄
    ▝▒▒▒▒▒▖███████████████████
      ▝▒▒▒▒██████████████╎██████
        ▝▒▒██████████████╎██◉████
        ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓╎▓▓▓▓▓▓▓▓
        ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓╎▓▓▓▓▓▓▓▓
        ▗▒▒▒▒▒▒▒▒▝▒▒▒▒▖▒▒╎▒▒▒▒▒▒▒
      ▗▒▒▒▒▒▒▒▒▒▒▒▝▒▒▖▒▒▒╎▒▒▒▒▒▒
    ▗▒▒▒▒▒▘░░░░░░░░░░░░░░░░░░░
  ▗▒▒▒▒▒▘?????▀▀▀▀▀▀▀▀▀▀▀▀
░▒▒▒▒▘?????????▀▒▒▒▒▒▀
▒▒▘??????????????░▒░"],
        &[r"
221             222222
222221       12222222221
  1222221     111111111111
    12222211111111111111111111
      12222111111111111112111111
        12211111111111111211W1111
        11111111111111111211111111
        11111111111111111211111111
        1222222221222212222222222
      12222222222212212222222222
    12222212222222222222222222
  1222221     111111111111
222221         1222221
221              222"],
    ),
    (
        32,
        &[
            r"
     ▗▄▖????????▗▄▖????????▗▄▖
▗▄▄▄▄▟◉▙▸??▖▄▄▄▄▟◉▙▸??◢▄▄▄▄▟◉▙▸
 ▝█████▛∼???▝█████▛≈???▝█████▛⌒",
            r"
     ▗▄▖????????▗▄▖????????▗▄▖
▖▄▄▄▄▟◉▙▸??◢▄▄▄▄▟◉▙▸??▗▄▄▄▄▟◉▙▸
 ▝█████▛≈???▝█████▛⌒???▝█████▛∼",
            r"
     ▗▄▖????????▗▄▖????????▗▄▖
◢▄▄▄▄▟◉▙▸??▗▄▄▄▄▟◉▙▸??▖▄▄▄▄▟◉▙▸
 ▝█████▛⌒???▝█████▛∼???▝█████▛≈",
        ],
        &[r"
     ggg        ggg        ggg
wwwwwgWgy  wwwwwgWgy  wwwwwgWgy
 wwwwwwwc   wwwwwwwc   wwwwwwwc"],
    ),
    (
        34,
        &[
            r"
        ◢█▖
  ▗▄▄▄▟███▙▄▄▖
▝▜████████◉█▛▀
▜▄?▝▀▓▓▓▓▓▀▘",
            r"
▄▖??????◢█▖
▝▜▄▄▄▄▟███▙▄▄▖
   ▝▜█████◉█▛▀
     ▝▀▓▓▓▀▘",
        ],
        &[r"
 
 
          W
 "],
    ),
    (
        36,
        &[r"
       ▗◉▖▸
       ▐█▌
      ▗▟▛
      ▐█▌
▗▖?▗▄▄▟█▌
▝▜██▓▒▓██▙
  ▝▀▀▀▀▀▀▀▀"],
        &[r"
       WkWy
       WWW
      WWW
      WWW
WW WWWWWW
WWWWWWWWWW
  WWWWWWWWW"],
    ),
    (
        38,
        &[
            r"
∙", r"
∘", r"
○", r"
◎", r"
◎",
        ],
        &[],
    ),
];

/// The sprites `SPRITES` must cover, and the ones that come in pairs (right, left).
pub const WANTED: [(u16, bool); 13] = [
    (SP_SHIP, true),
    (SP_WHALE, true),
    (SP_MONSTER, true),
    (SP_BIG_FISH, true),
    (SP_SHARK, true),
    (SP_DUCKS, true),
    (SP_SWAN, true),
    (SP_DOLPHIN, true),
    (SP_HOOK, false),
    (SP_LINE, false),
    (SP_POINT, false),
    (SP_BUBBLE, false),
    (SP_SPLAT, false),
];

/// The four rows of the water's surface, top to bottom, tiled across the
/// screen (each row repeats every its own length).
pub const WATER: [&str; 4] = [
    "∼≈∼⌒∼≈≈∼∼≈⌒≈∼≈∼∼≈≈⌒∼≈∼≈∼≈≈∼∼≈",
    "∼∼  ≈≈∼   ∼∼∼  ≈∼∼    ∼≈∼∼   ≈≈ ∼∼",
    "╌╌ ∼∼ ┄┄┄ ∼∼ ╌╌╌ ∼ ┄┄ ∼∼∼ ╌╌  ┄ ╌╌ ∼∼",
    "▔▔ ╌╌ ▔▔▔  ┄┄ ▔▔ ╌╌╌  ▔▔▔  ┄┄ ▔  ╌ ▔▔ ┄▔▔",
];

#[cfg(test)]
mod tests {
    use super::super::super::build_sprites;
    use super::super::*;
    use super::*;

    #[test]
    fn drawn_by_hand_in_unicode() {
        for (id, frames, _) in SPRITES {
            assert_drawn(&format!("sprite {id}"), frames.iter().copied());
        }
        assert_drawn("water", WATER);
    }

    #[test]
    fn every_twin_is_drawn_and_fits() {
        let originals = build_sprites();
        for (id, _) in WANTED {
            let Some((_, frames, masks)) = SPRITES.iter().find(|t| t.0 == id) else {
                panic!("sprite {id} has no twin yet");
            };
            let twin = Sprite::new(frames, masks, true, '?');
            let o = &originals[id as usize];
            assert_eq!(
                (twin.w, twin.h),
                (o.w, o.h),
                "sprite {id}: the original's size"
            );
        }
        for row in WATER {
            assert!(
                !row.is_empty() && !row.contains('\n'),
                "water rows are single lines"
            );
        }
    }
}
