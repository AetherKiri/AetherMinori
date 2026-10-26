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
An out-of-range answer fails rather than silently selecting a different route. Compare branches by running the same command in separate output directories with different `--choose` lists; for example, use `0,0,0,0,1,0,0` versus `1,1,1,1,1,1,1`. The backend writes a diagnostic choice screenshot before answering, streams a story trace even when a later tick fails, and stops on `Finished`. It preserves the final frame when the tick budget expires. Screenshots use a system CJK font and the provider reveal schedule, not retail font parity. This harness validates provider/headless behavior only, not Aether host startup or Windows retail parity.

Movie requests require `ffmpeg` on PATH: every clip is decoded offline to a null sink, a
representative decoded video frame is saved, and completion is returned through the normal
host-response ABI. Successful decoder warnings remain in the trace; the ef logo has a
documented terminal MP2-packet defect. This is accelerated decoding, not real-time playback.

`--verify-audio` fully decodes each requested Ogg to PCM and records channel count, sample
rate, interleaved sample count and peak amplitude. It is opt-in because repeatedly decoding
whole BGM tracks is expensive. It does not mix channels or send sound to a device. Without
it, ordinary audio requests are only recorded (the provider still decodes voice for lip analysis).
No licensed game assets are included in this repository.

## ef renderer and save verification (2026-10-25)

The ef static renderer now uses instruction-derived, asset-local Overlay and Screen strip
operations rather than opaque full-frame strip coverage. The three command slots target the
first stage argument, background tuple and standing extras respectively. Static BG/ST S/V
processing precedes Overlay → Screen; first-stage surfaces and dynamic character/ANI HSV
are not silently treated as the same path. WScroll standing preparation retains S/V; its
background destination and snapshot-refresh lifetime remain distinct native cases.

Manual saves currently write runtime-specific `MINORISV` version 9 with raw filter state,
original stage inputs and camera state. Older runtime save envelopes have explicit migrations;
this does **not** establish compatibility with retail saves or full active-effect restoration.
Snow-family particles now drive the recovered native fade state machine (authored fadeout,
10ms accepted updates, destruct when done) and composite through the decoded native
final-layer route (0x521b80 alpha-keyed masked copy over the zero-filled intermediate);
see [snow fadeout integration](<acceptance/SNOW_FADE_INTEGRATION.md>) and
[masked route acceptance](<acceptance/SNOW_ROUTE_MASKED.md>). The four table blend modes
and the remaining route unknowns (U1-U5) are documented in the linked evidence.

To include tests requiring your own local game installation:

```bash
EF_GAME_ROOT="/path/to/ef - the first tale" \
MINORI_EF_ROOT="/path/to/ef - the first tale" \
cargo test --lib -- --ignored
```

Both variables are needed. The ordinary suite omits these asset-dependent tests. See the
[Screen visual acceptance](<acceptance/SCREEN_VISUAL_ACCEPTANCE.md>),
[34,000-update replay](<acceptance/ROUND84.md>),
[60,000-update alternate replay](<acceptance/ROUND85_ALTERNATE.md>) and
[static HSV limitations](<acceptance/STATIC_HSV_INTEGRATION.md>) for observed results and
remaining boundaries. These tests and screenshots do not certify all routes, endings,
native cache semantics, original-executable pixel parity or actual Aether GUI startup.

## License

GPL-3.0-or-later, consistent with the AetherKiri project it was extracted from.
