# Round 74 — frozen character pixels survive redraw

Scene-level test found freeze_character_layers correctly returned baked pixels and cleared characters, but caller never persisted the baked pixels into background_rgba. Next full redraw therefore lost the frozen character. Existing unit test only covered the helper, not repeated scene composition.

New scene regression failed before fix and passes afterward. Freeze now retains character-only baked pixels in composition background before adding transient particles/layers/panel. This does not bake transient elements or alter camera-source semantics; later camera recrop behavior of frozen pixels remains a separate fidelity question.

Full suite 109 passed/9 ignored plus 1 CLI passed; diff check passed. Initial failed Vec equality produced excessive output; changed assertion to bounded diagnostic. No screenshot or native execution. This follows existing freeze-bakes contract, not newly recovered original lifetime semantics. Overall objective remains incomplete.
