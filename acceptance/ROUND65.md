# Round 65 — remove discarded-timeline render contamination on restore

After unifying animation redraws, audited restore_save_capture and found it cleared scrolling/cutin/characters/panel but retained particles, screen/overlay layers, frame stills and HSV state from the discarded timeline. Subsequent redraw could therefore overlay unrelated old visuals on a loaded capture.

Deterministic regression installed a particle and layer sentinels, restored a blank capture and ticked/redrew: failed before fix because particles remained. Restore now clears those unsaved old states; provider also clears current_effect/fade bookkeeping. This prevents leakage, but does not reconstruct effect state from the save. Existing save format does not serialize these dynamic/layer states; saving within an effect remains a fidelity limitation, not claimed fixed.

Regression now passes; default suite 104 passed, 5 ignored; 1 CLI passed. Actual-resource manual save/restore test also passed (1.31s), preserving VM/message/pixels and continuation. Diff check passed. No fresh screenshot or retail execution. Overall native fidelity and actual host acceptance remain incomplete.
