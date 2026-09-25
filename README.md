# AetherMinori

Independent Rust runtime for the [Minori](https://minori.ph) visual novel engine (WHITE ALBUM2 etc.),
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

## License

MIT, as declared in `Cargo.toml`, inherited from the AetherKiri project.
