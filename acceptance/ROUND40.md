# Round 40 — clarify pause audit and verify replay artifacts

## Audit correction

Updated the integration audit’s lifecycle entry to include the new round-38 paused-input pointer assertions, and to explicitly distinguish callback-level tests from unverified host pause scheduling, audio pause/resume, and retail lifecycle policy. No production code changed.

## Verification

`cargo test --all-targets --quiet` passed: 91 library tests passed, 2 installation-dependent tests ignored, and the 1 headless CLI test passed. `git diff --check` passed.

Fresh round-39 PNGs were visually checked; the checkpoint PNGs from 24,000 through 96,000 plus 103,999 had distinct SHA-256 digests. The route trace progressed to `202_01.sc`, message 3290, `WaitingInput` with `favMiyako=2` and no runtime error. These are headless results only. REA Ghidra opening of the retail PE succeeded, but the targeted text-drawer decompilation and procedure-name inventory timed out again; the active binary target was closed after failure.

## Open acceptance

The actual Aether host project and Wine remain unavailable. Host startup/screenshot, audio pause semantics, Windows retail comparison, title parity, and full command fidelity are not demonstrated. No original behavior was inferred from a blocked decompilation. User-confirmed cinematic black bars are intentional.
