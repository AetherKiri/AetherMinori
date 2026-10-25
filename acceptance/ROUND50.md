# Round 50 — fix initial headless language selection

## Concrete fix

The headless CLI previously opened the game and loaded the default initial script before applying `minori.language`; it only restarted when `--start` was supplied. Later chained scripts used the selected language, but the initial script did not. The CLI now always restarts after applying language and optional entry settings. This is confined to the diagnostic executable; provider ABI and host integration are unchanged.

## Verification

Full tests passed: 91 library tests passed, 2 ignored, 1 CLI test passed. `cargo build --bin minori-headless --quiet` and `git diff --check` passed. A 400-loop `--language e` startup completed normally; its church-background frame was personally reviewed and showed Chinese dialogue and the intentional cinematic bars. The fixed full route subsequently completed at loop 164,814, no error, 7 selects, 5 movies and `favMiyako=2`, `favMiyako_a=1`, `finderCounter=2421`. The earlier mixed-initial-language run completed at 164,809. The new first trace row is `WaitingPanel` in `test.sc` instead of `PendingChain`, showing the initial variant now changes as intended. The 96k frame was personally reviewed (two characters and Chinese dialogue); terminal image is black at `Finished`. No English dialogue translation or retail visual parity is claimed.

Corpus parser inventories: 75 scripts parse under each selector; English variant 24,137 statements / 24,093 commands, Japanese variant 24,527 / 24,472. Both contain 14,073 message commands. This only measures parser selection; it does not establish original executable semantics.

## Remaining

Aether host launch and screenshot, Windows retail comparison, all branches, full original command parity and host audio lifecycle are still unverified. No repeated Ghidra timeout was used to infer behavior.
