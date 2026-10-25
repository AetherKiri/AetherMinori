# Native Screen strip kernel

Independently reviewed ef-re/screen-native-unique.txt223..267:51ff70 loop complements source/destination RGB, unsigned byte MUL, INC AH, XORffffff, retains destination alpha, skips destinationalpha0. Thus channel result254-(((255-s)*(255-d))>>8), not textbook /255 Screen; source255 yields254. Sourcex0 repeated across row; -1 bottom-aligned row sampling/prefix extension same as Overlay.

Added isolated ef_screen_strip plus exhaustive65536 channel pairs against instruction-shaped MUL/highbyteINC/XOR reference, destinationalpha0 unchanged, sourcealpha ignored, black strip high-end quantization fixture. Suite121 passed/12 ignored plus1 CLI; diffcheck clean. Static-derived reference not original-executed oracle. No production screen integration or new screenshots yet.

Native parser confirmed2 operands map(empty,operand0,operand1),3+ mapfirst3; fewer2 no setters. Screen resources first1688/second19e8/third1b08 publishctx41c/410/414 at4df7ce/e4/ee. Same asset scopes as paired Overlay; Screen runs AFTER Overlay. Must not put strips in covering/full-frame source-over layer. Production integration is now implemented as described below.

## Production integration

- VM preserves raw Screen operands independently in `LayerRequest.screen_slots`: two map to (empty, arg0, arg1), three-or-more use the first three, fewer than two report a runtime parse error. Prefix/wildcard normalization cannot collapse slot indices. Interface ABI is unchanged.
- Native Screen filters the same raw surfaces as Overlay, immediately after Overlay and before placement/camera crop: slot 0 base-spec foreground, slot 1 background, slot 2 stage extras and character surfaces. Mouth/eye patches sample parent-sprite row coordinates. Raw decoded/cached assets are never modified; every redraw starts from original surfaces. Historical `screen_layers` remains for migrated saves only; new Screen commands clear that sentinel and do not create covering layers.
- Manual saves now write v8 with `native_screen: [Option<Frame>; 3]`. Explicit v7 nine-field tuple migration preserves native Overlay inputs and defaults Screen to empty; v4/v5/v6 migrations and earlier envelope support remain. Raw stage sources and placements continue to be serialized in the existing native Overlay stage state, so clear after restore reconstructs the originals.
- Regressions cover synthetic all-three-slot Overlay-then-Screen ordering, local row sampling before placement, character/animation patch parent rows, repeated redraw, raw-cache preservation, v7 migration and v8 roundtrip. The installed-game regression uses `ev01_054a01.png` with `sc_evening01.png` in slot 1, compares the exact local-kernel result, checks horizontal image structure (not a covering gradient), exact clear, restage/reapply idempotence, and restore/rebuild/clear.

Validation: `cargo test` passed 124 library tests (13 opt-ins ignored), plus 1 CLI test. `cargo test -- --ignored` passed all 13 opt-ins with both `EF_GAME_ROOT` and `MINORI_EF_ROOT` set to `/root/games/ef-first-tale/ef - the first tale`. `git diff --check` clean. No commit.

Limits: kernel authority remains static-derived native instructions plus exhaustive integer-domain reference, not an original-executable runtime oracle. This work does not claim new HSV/transition equivalence or original-runtime pixel parity. Final visual screenshots are handled separately by the parent investigation.
