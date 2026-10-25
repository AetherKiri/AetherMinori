# Round 70 — stage foreground survives effect clear

Extended deterministic scroll regression to clear_effect; it failed because that path independently re-cropped background_source without fixed foreground. Extracted a shared viewport-aware crop helper used by both camera scrolling and effect clear. Actual corpus audit also found vista_window2 stage immediately followed by effect * in 000_01mugai.sc, making this more than a synthetic path.

Added real-install opt-in test staging actual past_window.png plus bgw_normal011a01_03_10001_sepia.png, scrolling three changed camera ticks then clearing effects. Every opaque window pixel is compared to its original asset value and survives. Explicit test passed in 0.19s. Default suite 107 passed, 8 ignored; 1 CLI passed; diff check passed. No fresh screenshot or native runtime capture; this fixes existing viewport-layer contract, not original pixel parity. Overall objective remains incomplete.
