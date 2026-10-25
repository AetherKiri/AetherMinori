# Round 66 — retain saved composition layers

Extended ManualSceneState to carry screen layers, overlay layers and frame stills, copied on save and restored after discarded-timeline state is cleared. Previously only flattened pixels were retained, so the first later redraw could lose the saved layers. This preserves existing reconstruction composition, not original layer-order semantics. Dynamic particle/motion/HSV save state remains incomplete.

New proprietary save version 5 explicitly identifies the extended bincode layout; versions 1–4 remain admitted, with v4 scene decoded as the original four-field tuple and empty layers. Serde defaults alone are not relied on for fixed-layout binary compatibility. Initial test attempted unavailable serde_json; replaced with existing bincode rather than adding a dependency.

Binary layer-state roundtrip regression and nonempty-layer disk save fixture pass. Full suite: 105 passed, 5 ignored; 1 CLI passed; diff check passed. Actual-resource stable-boundary save/load test also passed. Dedicated complete v4 on-disk migration fixture is still outstanding. No new visual/native comparison; overall objective incomplete.
