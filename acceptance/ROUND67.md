# Round 67 — independent on-disk legacy save migration fixture

Closed the round-66 validation gap with a v4 envelope built independently of the v5 writer: old four-field bincode scene, compressed payload, original header, metadata/variable fields and RGBA tail. Writes it to disk, loads via load_manual_slot, verifies metadata, PC/global, background/RNG, pixels and default empty new layers. Rewrites through current writer and checks version 5 plus complete semantic equality after reload.

Full suite: 106 passed, 5 ignored; 1 CLI passed; diff check passed. No production change. This validates our proprietary v4-to-v5 migration, not retail save compatibility. No screenshot/native capture this round. Native render fidelity, dynamic effect serialization and actual host acceptance remain incomplete.
