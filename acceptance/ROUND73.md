# Round 73 — rebuilt scrolling scene visual check and freeze transition coverage

Rebuilt headless after stage/freeze fixes. Real 100_01.sc e selector: 2401 updates at 16ms, 12 clicks, sampled every 400. Exit 0; final WaitingScroll/message170 with no runtime error. Personally inspected frame-2000.png and frame-2400.png: sepia image reveals church/tree as camera advances; fixed horizontal frame boundaries and readable narration remain. White upper viewport and violet panel are visibly present; no claim of original color/crop equivalence. Artifacts untracked at acceptance/round73-window.

Expanded freeze regression to check panel pixel in transition.new_rgba, retained frame-layer pixel and unchanged displayed frame. Initial fixture wrongly assumed a one-pixel panel was placed at offset_y; actual rule is bottom alignment plus offset. Corrected fixture to 247px panel and derived top. No production defect found by this extension.

Default suite 108 passed/9 ignored plus 1 CLI passed. All nine opt-in tests with both root env vars passed after current fixes (3.57s). Diff check passed. Native fidelity and real Aether/Windows parity remain incomplete.
