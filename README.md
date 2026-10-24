# AetherMinori

Independent Rust runtime for minori visual novels (ef - the first tale, Supipara, Trinoline),
extracted from the [AetherKiri](https://github.com/AetherKiri/AetherKiri) monorepo.

Consumed by AetherKiri as a git submodule (`packages/AetherMinori`) and registered into the host
through the `engine_runtime_provider` ABI. History of the original `rust/minori_runtime`
development is preserved.

## Layout

- `src/` — the runtime: VFS, script VM, scene graph, save persistence, provider registration
- `src/bin/minori_vfs.rs`, `src/bin/minori_script.rs` — standalone inspection CLIs
- `CMakeLists.txt` — build glue used by the AetherKiri host (cross-compile target mapping for
  Linux/macOS/iOS/Android/Emscripten)

## Standalone build

```bash
cargo build --release
cargo test
```

## Integration status

This is a partial independent implementation, not a verified drop-in recreation of the original
engine. Archive/game detection, decryption keys, canvas sizes and startup scripts remain
work-specific; they must not be removed just because UI layout is being consolidated.

At the source-audit baseline, ef bypasses the runtime title screen and starts its entry script;
Supipara and Trinoline still have runtime-rendered title menus. This is **not** global removal
of native-style title UI. Save/load and the in-game menu are separate from the title screen.
The host library is the intended launcher, but that design intent does not establish an
implemented host replacement for every original menu function.

See the [integration audit](<research/ef-first-tale/INTEGRATION_AUDIT.md>) for the exact baseline,
remaining fidelity gaps, save/ABI preservation boundaries and a focused removal plan.

## License

GPL-3.0-or-later, consistent with the AetherKiri project it was extracted from.
