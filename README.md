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

All supported games now open directly into their configured entry script; the runtime no longer
loads title artwork or routes title-menu clicks. The host library owns launching. Save/load and
the in-game menu remain separate and are retained, including work-specific assets and geometry.
This intentionally omits original title UI; it does not establish a host replacement for every
original menu function or original-player parity.

See the [integration audit](<research/ef-first-tale/INTEGRATION_AUDIT.md>) for the exact baseline,
remaining fidelity gaps, save/ABI preservation boundaries and a focused removal plan.

## Headless acceptance backend

The lightweight backend drives the same provider callbacks as Aether, not a replacement VM.
Build the executable explicitly after source edits; `cargo test --lib` alone does not rebuild it.

```bash
cargo build --bin minori-headless
./target/debug/minori-headless "/path/to/ef - the first tale" /tmp/ef-acceptance \
  --frames 240000 --delta 120 --clicks 48000 --every 4000 \
  --choose 0,0,0,0,1,0,0
```

`--choose` contains zero-based answers by branch ordinal; unspecified branches use zero.
An out-of-range answer fails rather than silently selecting a different route. The backend
writes a diagnostic choice screenshot before answering, streams a story trace even when a
later tick fails, and stops on `Finished`. It preserves the final frame when the tick budget
expires. Screenshots use a system CJK font and the provider reveal schedule, not retail font parity.

Movie requests require `ffmpeg` on PATH: every clip is decoded offline to a null sink, a
representative decoded video frame is saved, and completion is returned through the normal
host-response ABI. Successful decoder warnings remain in the trace; the ef logo has a
documented terminal MP2-packet defect. This is accelerated decoding, not real-time playback.

`--verify-audio` fully decodes each requested Ogg to PCM and records channel count, sample
rate, interleaved sample count and peak amplitude. It is opt-in because repeatedly decoding
whole BGM tracks is expensive. It does not mix channels or send sound to a device. Without
it, ordinary audio requests are only recorded (the provider still decodes voice for lip analysis).
No licensed game assets are included in this repository.

## License

GPL-3.0-or-later, consistent with the AetherKiri project it was extracted from.
