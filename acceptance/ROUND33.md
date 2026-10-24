# Round 33 — verify Aether CMake integration artifact

## Build

Configured this repository with its own CMake glue in Release mode and built the `aether_minori_runtime_build` target. Configure and build both exited 0. This executes the project’s CMake-to-Cargo path rather than bypassing it with `cargo build`: CMake produced `/tmp/minori-cmake-acceptance/cargo-target/release/libaether_minori_runtime.a`. `nm` on that exact archive shows the unmangled export `aetherkiri_minori_register_runtime_provider`. The separate direct Cargo release archive exposes the same symbol.

This verifies the local Linux static-library integration artifact and registration symbol only. It does not link the absent AetherKiri host, invoke its engine registration function, or validate the Windows target. CMake emitted its existing developer warning that this subproject lacks a literal `project()` call; it did not prevent the build. The build also surfaced a non-fatal GNU make jobserver descriptor warning. No production code or CMake was changed.

## Tests and remaining acceptance

`cargo test --all-targets` passed: 90 library tests passed, 2 installation-dependent tests ignored, all binary harnesses passed, and the headless CLI integration test passed. The exact retail EXE remains installed at `/root/games/ef-first-tale/ef - the first tale/ef_first_AA.exe` (SHA-256 `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`). REA PE resource parsing had previously confirmed this is the retail PE32 x86 target, but the generic binary-layout REA provider is unavailable here because its caller-supplied pwntools/pyelftools/Unicorn toolchain is not configured. No new target binary analysis was available this round.

The Linux headless provider has prior manually inspected title/scene/dialogue route frames and a full route to Finished; this is not visual acceptance of Aether’s real host window or Windows retail. `wine`, AetherKiri host source/build, and a native Windows runner are unavailable in this checkout/environment. Continue with code/source and provider-based tests; do not claim host/retail parity until a target host is available. User-approved cinematic bars remain unchanged.
