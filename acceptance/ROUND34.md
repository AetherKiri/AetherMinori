# Round 34 — confirm host-boundary and retail-analysis limits

## Retail target analysis

REA opened the installed `ef_first_AA.exe` in Ghidra and identified it as PE/x86 with the known SHA-256 `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`. Procedure search and binary overview each timed out during provider analysis, so they yielded no usable native function evidence. Closed the target afterward; REA reports no open analysis session. The failure is a Ghidra analysis timeout, not a change or a new finding about retail behavior.

## Host contract

Reviewed the integration documentation and provider table. The Minori package identifies AetherKiri as its parent and implements the `engine_runtime_provider` descriptor with scene frame/input callbacks. However, this workspace contains only `AetherMinori` and no `project.godot`, host CMake project, or AetherKiri source; therefore the host cannot be linked/launched here. The existing `pause` and `resume` callbacks are the `unary` validator, not evidence of pause-state behavior. Menus and other unsupported host services must not be assumed to work merely because the provider compiles. The headless backend drives provider callbacks but is not a host substitute.

## Tests and remaining scope

`cargo test --all-targets` passed in round 34: 90 tests passed, 2 installation-dependent tests were ignored, and all binary targets completed. No production source changed. Previous round’s CMake Release target and exported provider symbol remain verified, but neither validates actual Aether host startup. No Windows runner/Wine is available, and no further retail decompilation is supported by the timed-out Ghidra session. The full-route headless captures remain the available visual evidence; do not claim original-player/host parity. Keep cinematic bars.
