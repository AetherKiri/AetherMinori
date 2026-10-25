# Round 51 — configure before opening, not after restarting

The round-50 workaround loaded the default game once before reopening it with the selected options. That could emit stale platform requests or fail in the default entry script before a requested chapter was ever reached. Added `Headless::open_with_options`: create the ABI runtime, set options, then call `open_game` exactly once. Existing `Headless::open` delegates with no options for compatibility. The CLI uses the configured constructor, avoiding the double load. Provider ABI and native host remain unchanged.

Added a regression test that requests an invalid language with a nonexistent game root: the error must come from language validation, not game opening. Full suite: 92 library tests passed, 2 ignored; 1 existing CLI test passed. Build and diff checks passed.

Executed a real-install chapter-start probe: `--language e --start 100_01.sc --frames 400 --delta 120 --clicks 80 --every 399`. Exit 0, first trace row is `100_01.sc`, final state WaitingInput/message 270 with empty error. Personally inspected frame-0399.png: character closeup, church background, speaker label and Chinese dialogue visible. This is a short chapter-entry acceptance, not another complete route replay, and does not prove English dialogue translation or Windows retail equivalence.

Remaining: original behavior/command fidelity and actual host integration acceptance. Earlier route completions and cinematic crop acceptance remain scoped to their recorded runs.
