#!/usr/bin/env bash
# Regenerates the README animations in assets/ with oxiper's own recorder.
# Needs: a release build, agg (github.com/asciinema/agg) and ffmpeg.
#   scripts/gifs.sh            # everything
#   scripts/gifs.sh neo fire   # only these
set -euo pipefail
cd "$(dirname "$0")/.."

OX=./target/release/oxiper
AGG=${AGG:-agg}
BG=0d1117
# GitHub's dark terminal palette, so the GIFs sit naturally on the page.
THEME=$BG,e6edf3,484f58,ff7b72,3fb950,d29922,58a6ff,bc8cff,39c5cf,b1bac4,6e7681,ffa198,56d364,e3b341,79c0ff,d2a8ff,56d4dd,ffffff
FONTS="DejaVu Sans Mono,DejaVu Sans,Symbola,Noto Sans Mono CJK JP"
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

# name | seconds | skip | size | fps | oxiper flags
SHOTS=(
  "bonsai|10|0|80x24|24|-s bonsai --seed 21 -o bonsai.step=0.012"
  "aquarium|10|20|80x24|24|-s aquarium --seed 11 -o aquarium.biome=reef"
  "neo|8|3|80x24|24|-s neo --seed 4"
  "nyancat|6|0|80x24|24|-s nyancat --seed 1 -o nyancat.scale=1"
  "fire|6|3|80x24|15|-s fire --seed 2"
  "constellation|10|0|80x24|24|-s constellation --seed 5 -t catppuccin"
  "waveform|8|0|80x24|24|-s waveform --seed 3 -t synthwave"
  "pipes|10|0|80x24|24|-s pipes --seed 8"
  "rain|8|2|80x24|24|-s rain --seed 6 -o rain.lightning=12"
  "dvd|10|0|80x24|24|-s dvd --seed 9 -o dvd.corner_assist=true"
  "boids|8|3|80x24|24|-s boids --seed 12 -t dracula"
  "orrery|10|2|80x24|24|-s orrery --seed 3"
  "particles|6|3|80x24|15|-s particles --seed 7 -t rosepine"
  "dog|8|0|80x24|15|-s dog --seed 2"
  "shark|8|0|80x24|12|-s shark --seed 3"
  "tiger|10|1|80x24|12|-s tiger --seed 6 -o tiger.ufo=3600"
  "ocean|8|20|80x24|12|-s ocean --seed 5 -o ocean.visits=3"
  "fireworks|10|3|80x24|24|-s fireworks --seed 4"
  "metaballs|4|0|80x24|10|-s metaballs --seed 3 -t synthwave"
  "aurora|8|10|80x24|6|-s aurora --seed 18 -o aurora.stay=20"
  "hollywood|10|4|80x24|15|-s hollywood --seed 3"
  "muertos|18|0|80x24|6|-s dog,tiger,aurora,fireworks,dvd,fire -d 3 --seed 5 -o fiestas=muertos"
  "tour|24|0|100x30|15|-s bonsai,aquarium,neo,nyancat,fire,constellation,pipes,orrery -d 3 --seed 4 -o bonsai.step=0.004"
)

want=" $* "
for shot in "${SHOTS[@]}"; do
  IFS='|' read -r name secs skip size fps flags <<<"$shot"
  [[ $# -gt 0 && $want != *" $name "* ]] && continue
  echo "• $name"
  # shellcheck disable=SC2086
  $OX record "$TMP/$name.cast" --size "$size" --seconds "$secs" --skip "$skip" --fps "$fps" -o bg_hint=#$BG -o fiestas=none $flags 2>/dev/null
  "$AGG" --theme "$THEME" --font-family "$FONTS" --font-size 14 --fps-cap "$fps" \
    --last-frame-duration 0 "$TMP/$name.cast" "assets/$name.gif" >/dev/null 2>&1
done
# The theme sampler: pipes frozen mid-run in six themes (needs ImageMagick).
if [[ $# -eq 0 || $want == *" themes "* ]]; then
  echo "• themes"
  set -- classic dracula gruvbox nord sakura synthwave
  for t in "$@"; do
    $OX record "$TMP/t-$t.cast" -s pipes --size 52x16 --seconds 0.5 --skip 5 --fps 4 -t "$t" --seed 5 \
      -o pipes.style=heavy -o pipes.count=7 -o pipes.limit=0 -o bg_hint=#$BG 2>/dev/null
    "$AGG" --theme "$THEME" --font-family "$FONTS" --font-size 14 --last-frame-duration 1 \
      "$TMP/t-$t.cast" "$TMP/t-$t.gif" >/dev/null 2>&1
    ffmpeg -loglevel error -y -i "$TMP/t-$t.gif" -vframes 1 "$TMP/$t.png"
  done
  (cd "$TMP" && montage -label '%t' "${@/%/.png}" -tile 3x2 -geometry +6+6 -background "#$BG" \
    -fill '#8b949e' -font DejaVu-Sans-Mono -pointsize 16 "$OLDPWD/assets/themes.png")
fi
# The biome sampler: the aquarium frozen in each of its places (needs ImageMagick).
if [[ $# -eq 0 || $want == *" biomes "* ]]; then
  echo "• biomes"
  set -- classic reef ocean kelp deep fresh arctic tank
  for b in "$@"; do
    $OX record "$TMP/b-$b.cast" -s aquarium --size 72x22 --seconds 0.5 --skip 25 --fps 4 --seed 3 \
      -o aquarium.biome=$b -o bg_hint=#$BG 2>/dev/null
    "$AGG" --theme "$THEME" --font-family "$FONTS" --font-size 14 --last-frame-duration 1 \
      "$TMP/b-$b.cast" "$TMP/b-$b.gif" >/dev/null 2>&1
    ffmpeg -loglevel error -y -i "$TMP/b-$b.gif" -vframes 1 "$TMP/$b.png"
  done
  (cd "$TMP" && montage -label '%t' "${@/%/.png}" -tile 2x4 -geometry +6+6 -background "#$BG" \
    -fill '#8b949e' -font DejaVu-Sans-Mono -pointsize 16 "$OLDPWD/assets/biomes.png")
fi
ls -la assets/
