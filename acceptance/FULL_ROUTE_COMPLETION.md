# Full route playthrough to story completion (2026-10-26)

## Run

Current build (post masked-route commit ca95f71). Default entry, English,
delta120, clicks6000, sampled every 10000. 3,873,501 updates; exit 0.

- story-trace.txt: ZERO error lines across the whole run.
- VM reached state Finished at tick 3873500 (script none, message 1250).
- 25 script chains crossed, in order: test.sc, 000_01mugai.sc, 100_01,
  101_01, 102_01, 102_02, 102_04, 103_01, 104_01, 105_01, 106_01, 107_01,
  107_02, 107_04, 107_05, 108_01, 108_02, 109_01, 110_01, 111_01, 111_02,
  111_04, 111_05, 111_06, 112_01 (ending epilogue 112_02 ran in-place).

The default-choice path is the main route through the epilogue. Earlier
alternate-choice runs (rounds 85/86) reached BADEND1 and a different branch,
so branching works.

## Personally read frames (all sha256 recorded by the harness)

- frame-240000.png (2e1326d799bb8e9a96b9ebcbfc6dbd854db50dc75691bbd66ab23223d0402661):
  snowy hillside event CG, two characters embracing, message text rendering.
- frame-996000.png (0be1ebd83b8d9c4e8f58008017ffebce0ebb024a8947632d853326cd2000314f):
  Yuuko standing sprite with speaker tag and dialogue in a detailed interior.
- frame-3790000.png (fe00025b803526237acf77d0b80cc34c08418015472c326578ff20140197987d):
  final-chapter face closeup, speaker + dialogue correct.
- frame-3860000.png (bf53e2f273856973307104d3a168f89d94f2ea4d0b0e03e68bad3425633a64ea):
  ending scene, Yuuko smiling in the sunset gym, text correct.
- frame-3873500.png (bd5970d91acacd7986f92910031ef1c5603a3f9c1bbe76dc6ae8c8a58412ad86):
  post-story black frame at Finished - the natural end state.

## Notes

- An earlier 4M-tick attempt died at disk-full (os error 28) mid-run; the disk
  was reclaimed by pruning superseded run frame PNGs (story traces and every
  committed acceptance record kept), and this run reran clean from scratch.
- The 30,000-frame freeze reported by the user remains absent: this run passed
  3.87M updates with WaitingInput/WaitingScroll/WaitingSelect states only.
- The title screen is intentionally skipped (direct story start) per the
  original-integration decision; everything after it is exercised here.

## Limits

- This is the default-choice route to one completion; other choice branches
  were spot-checked in earlier rounds, not exhaustively played.
- No runtime oracle against the original executable; rendering fidelity is
  bounded by the documented approximations (source-over replaced by the
  decoded masked route; final-present alpha U4 still open).
