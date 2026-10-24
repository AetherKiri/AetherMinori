# Round 30 — repeatable long-route capture and historical flat-panel discrepancy

## Reproduction

Re-ran the exact round-20 headless command against the same installed game and current headless binary: `minori-headless "/root/games/ef-first-tale/ef - the first tale" acceptance/round30-replay --frames 240000 --delta 120 --clicks 48000 --every 8000 --choose 0,0,0,0,1,0,0`. It exited 0 and reached `Finished` at iteration 181064. Final debug state is script `none`, message 690, `favMiyako=2`, black terminal frame. Request totals: 5,878 audio, 13,657 messages, 13,044 reveals, 5 movies, 7 choices, 27,028 sprites.

The game executable SHA-256 is `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`. The headless executable SHA-256 is `a8c3fbe764f489d488bec306b22249c59075798f15e03af34da934cab4b9fa22`. The exact replay output at 96,000 is `c8a234e45d254b2a1699a8250ea939791b92b4c3b9a232010174c538cde7b9b2`, identical to round-28 output.

## Visual comparison

I personally inspected the rerun frame 96000. It shows the overlapping character CG, dialogue line and panel fade across the lower scene; it is not a uniform purple fill. Pixel check on historical `round20-current/frame-96000.png` at y=500 reports one unique RGB value `(103,102,142)` across 800 pixels. The rerun has 125 distinct values across that row. The headless screenshot hash from the round-28 full replay is exactly equal to this rerun, so the 96k lavender image is not reproducible under the historical command/current build. This only makes the old PNG anomalous; it does not prove whether that image was stale, captured out of phase, or another host/output issue.

Across saved cadence samples, the first historical/new pixel difference is at tick 32,000, though both images show the same character/scene: one is open-eyed with narration and the other has half-lidded eyes and a speaker line. Images reconverge byte-for-byte at saved samples 48,000 and 88,000, and appear identical again at 136,000 and 176,000. They differ around 96,000 and 104,000. The historical trace row at 96,000 says message 880; repeat runs say message 870. The old capture retained no 96k neighboring frames, so the exact frame/trace timing remains uncertain. Do not label this as a fixed renderer bug.

## Scope and next work

The controlled rerun validates that the route continues past 30k and completes; it does not validate retail Windows or Aether-host startup/render parity. The confirmed cinematic black bars remain intentional. No production code changed. `cargo test --lib`: 90 passed, 2 ignored, 0 failed. Next, focus on missing native commands/runtime parity and host Aether integration; defer visual code changes until a mismatch is reproduced against the original.
