# Round 57 — real dual-runtime isolation acceptance

Added an opt-in local-installation regression controlled by MINORI_EF_ROOT. Opens two actual ABI Headless instances using the same writable/cache directory, starts 100_01.sc under e selector, ticks/clicks both for 400 updates, then independently drains their queued callbacks. It drives another 400 updates with per-instance drains and records active audio paths. Checks both instances emit dialogue, audio files differ, and destroying the first leaves every active second-instance audio file byte-identical. The second runtime continues ticking afterward.

Executed explicitly against the installed game: test passed (2.70 seconds). Default full suite: 99 passed, 3 ignored (including this opt-in test), plus 1 CLI test passed. Diff check passed. An intermediate assertion assumed open_game alone emits callbacks; that failed. Corrected the test to drive both VMs to dialogue before checking queue isolation. No production change was needed.

This upgrades earlier synthetic queue/file tests to two live reconstructed provider instances, but is not the actual Aether host, concurrent-thread stress, audible playback or original Windows comparison. No new screenshot was taken. Overall objective remains incomplete.
