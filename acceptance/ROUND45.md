# Round 45 — document reproducible choice-branch acceptance

Updated the headless acceptance README with a concrete alternate-choice invocation pair and an explicit warning that this harness tests the provider/headless path only—not Aether startup or Windows retail parity. This is documentation, not a host integration claim.

The round-44 second route has been completed and its report now records results at loop 58,368, a different `favMiyako_d=1` endpoint and 3 select requests; screenshots at 24k and 48k were personally read. Primary route evidence remains in round 41. The two observed endpoints demonstrate two distinct headless pathways, not complete branch coverage.

Validation: `cargo test --all-targets --quiet` passed (91 passed, 2 ignored; one headless CLI test passed), `cargo build --bin minori-headless --quiet` passed, and `git diff --check` passed.

Still unverified: Aether host startup and rendering, Windows retail behavior and title screen, full route coverage, exact command parity, and host audio lifecycle. `scr.paz` is rejected as an unsupported REA target format; PAZ behavior continues to be analyzed through the project parser and decoded script corpus, not by claiming REA decompiles that archive.
