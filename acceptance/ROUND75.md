# Round 75 — character animation redraw preserves full scene

Found refresh_character_frame maintained a separate character/panel-only compositor. Blink, mouth, movement and sequence updates could therefore erase active particle and screen/overlay/frame layers, even after other redraw paths were fixed.

Added deterministic independent pixel sentinels for a particle and frame layer; direct character redraw lost the layer (exit 101). Delegating to full refresh_composed_frame makes both survive and preserves shared transition destination handling. Full suite 110 passed/9 ignored plus 1 CLI passed; all 9 opt-in actual-install tests passed in 3.91s; diff check passed. No native rendering-order claim or new screenshot.

Search for literal .freeze/.charFreeze in corpus returned no matches; this does not imply absence of equivalent commands. Focus this round moved to actual character update path rather than guessed freeze/camera lifetime. Overall fidelity and actual host/Windows acceptance remain incomplete.
