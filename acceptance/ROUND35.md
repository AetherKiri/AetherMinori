# Round 35 — implement host lifecycle pause state

## Change

The integration audit identified that the provider exposed `pause` and `resume` function pointers but both were mapped to a unary null-handle validator. Added private `Runtime::paused`; the callbacks validate the runtime handle and toggle it. The provider tick and input callbacks now return `OK` without advancing scene/VM state or consuming input while paused; `open_game` resets the flag so a prior pause cannot leak into a new game. The provider descriptor still contains the same ABI entries and function signatures; no struct shared with the host changed. Updated the audit’s lifecycle statement.

This implements the provider contract locally. It does not prove how Aether schedules callbacks while paused, whether it separately pauses audio, or match retail pause/menu policy.

## Verification

Added `provider_pause_resume_and_tick_state_are_consistent`: calls pause/tick/resume on a test runtime and verifies both null-handle cases fail with `INVALID_ARGUMENT`. `cargo test --all-targets` passed: 91 library tests passed, 2 installation-dependent tests ignored, and the headless CLI test passed. `git diff --check` passed. Repository-wide `cargo fmt -- --check` exposes numerous pre-existing formatting differences in unrelated targets; no unrelated files were reformatted.

## Scope

No game screenshots were regenerated: this callback-only change does not affect unpaused route rendering or story content. Host integration and Windows retail remain unverified; no Aether host project or Wine is available in this workspace. Existing route screenshot evidence and user-confirmed cinematic bars are unchanged.
