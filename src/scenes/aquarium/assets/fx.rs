//! Small effects: food, plankton, scraps, ink, a trawl net and a fishing line.

/// Fish food sprinkled from above; each frame is one look a flake can have.
pub const FLAKE: &[&str] = &["\n▪", "\n◦", "\n▫", "\n⠢"];
/// Plankton; each frame is one look.
pub const PLANKTON: &[&str] = &["\n⠂", "\n⠄", "\n⠐", "\n⠈", "\n∙"];
/// What is left when something is eaten; each frame is one look.
pub const SCRAP: &[&str] = &["\n▖", "\n▝", "\n⠪", "\n▫"];
/// A cloud of ink about 5x3, thinning out frame by frame.
pub const INK: &[&str] = &[
    "\n ▟█▙\n█████\n ▜█▛",
    "\n ▟▓▙\n▓███▓\n ▜▓▛",
    "\n ▒▓▒\n▒▓▒▓▒\n ░▒░",
    "\n ░▒░\n░ ▒ ░\n ░ ░",
    "\n ⠂ ⠐\n⠄  ⠂\n  ⠁",
];
/// A trawl net behind a boat that moves right (mirrored for left): a rope
/// from the stern at the top right and a mesh, frames swaying. `?` is
/// transparent, so what the net holds shows through.
pub const NET: &[&str] = &[
    "
            ╱
           ╱
  ╭─╲╱╲╱╲╱╲╱
  │?╱╲╱╲╱╲╱╲
  ╰─╲╱╲╱╲╱╲╱
    ╰─╱╲╱╲─╯",
    "
            ╱
           ╱
  ╭─╱╲╱╲╱╲╱╲
  │?╲╱╲╱╲╱╲╱
  ╰─╱╲╱╲╱╲╱╲
    ╰─╲╱╲╱─╯",
];
/// A fishing line: one piece per row, and the hook at its end.
pub const LINE_BAR: &str = "│";
pub const LINE_HOOK: &str = "╯";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawn_by_hand_in_unicode() {
        let all = [FLAKE, PLANKTON, SCRAP, INK, NET, &[LINE_BAR, LINE_HOOK]];
        super::super::assert_drawn("fx", all.into_iter().flatten().copied());
    }
}
