# Snow fadeout production integration acceptance

## Change

Commit b6ad10e wires the previously isolated native units into production
rendering: ParticleState now owns SnowFadeState (0x50b920/0x50bbc0 alpha
semantics), SnowClock (0x5353a0 nominal 10ms divide), and an accepted-update
counter. The authored prologue fadeout (000_01mugai.sc lines 38/43) now
starts the real fade instead of being a cutin-only no-op; particles keep
moving while fading; when alpha drops below one the effect destructs and the
background is recropped, mirroring owner 0x4d7060. The update cadence is the
sentinel delta 1 first, then the 10ms timer period; the unknown dynamic
minimum threshold is still not modeled. Composition keeps the documented
source-over approximation because the native final-layer blend equation
remains unresolved; packed max on the opaque scene stays rejected per
SNOW_FINAL_LAYER_AUDIT.md.

## Corpus chronology evidence

Decoded with ef-re/decode_scr_range.js (Shift-JIS): .effect Snow line10,
messages100-160 lines16-36, [e].effect fadeout line43, .effect * line50,
message170 line52, message180 line55, .vscroll line60. This also corrects
SNOW_REBUILT_SMOKE.md: a run ending at message180 has already passed the
authored fadeout.

## Run

Rebuilt minori-headless; replayed 000_01mugai.sc English, 2001 updates,
delta10, clicks400 (every 5), samples every 25; exit 0, no VM error, final
state WaitingTransition message230 effect=none (past both authored fadeout
commands).

Personally read frames:

- frame-0700.png sha256 78db39fe3df645395f296dfc0aae01d8d6a1b451077c34b5e3a4fd1585b4ba4d:
  full-brightness snow over the black opening monologue, msg160 text visible.
- frame-0725.png sha256 86aa8c806d623d43439d52b725ba327d67bd1c532b6a65fcd15711661d9ac090:
  visibly dimmed gray flakes with the message gone - the fade window.
- frame-0750.png sha256 bd5970d91acacd7986f92910031ef1c5603a3f9c1bbe76dc6ae8c8a58412ad86
  (identical to frame-0775.png): black, snow destructed, awaiting the vista
  transition at lines 47-49.

## Limits

Lightweight ABI backend, not the original Windows executable; particle layout
and motion are still the documented approximation, not native-matched; the
final-layer blend route is unresolved, so fade opacity is applied through the
existing source-over composite; cinematic black crop borders at
800/9200/6400 remain untouched per the accepted review.
