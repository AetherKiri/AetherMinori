# Round 38 — harden pause-state input validation

## Change

Extended the existing pause/resume provider regression test. While paused, a non-null but deliberately undersized input pointer now verifies the callback returns `OK` without dereferencing or consuming that event. A null event pointer still returns `INVALID_ARGUMENT` even while paused, and the runtime does not enter failed state. This confirms the intended ordering of pointer validation, failure-state validation, pause gating, and event decoding in the provider callback. No host ABI or runtime behavior changed; this is regression coverage for the round-35 implementation.

## Verification

The focused `provider_pause_resume_and_tick_state_are_consistent` test passed. `cargo test --all-targets --quiet` passed: 91 library tests passed, 2 installation-dependent tests ignored, and the one headless CLI test passed. `rustfmt --edition 2024 src/provider.rs` and `git diff --check` passed; the changed source diff is limited to the new assertions.

## Acceptance limits

The repository has no `project.godot`/Aether host project, and `wine` is unavailable. No host launch or new game screenshot was possible this round. Earlier personally inspected route frames show the 96,000-iteration screenshot contains characters, text, and scene imagery rather than a uniform frozen screen; the saved terminal frame is black, consistent with the route ending but not a retail reference. User-confirmed black cinematic bars remain intentional. Aether host rendering, real pause integration (including host audio), original Windows parity, and full visual/story acceptance remain open.
