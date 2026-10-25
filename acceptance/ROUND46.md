# Round 46 — transition corpus and host-scope review

## Original script corpus

Compared literal `.transition` usage in the extracted ef scripts with the renderer’s static type dispatch. The corpus contains 5,254 matching commands across all scripts; exact literal types observed include 0, 1, 2, 14, 16, and 17. The renderer has defined paths for 0–18 with a few type-specific mask/tile requirements. This is corpus-level script evidence, not execution in the retail engine or a proof of pixel equivalence. Effect names/masks and runtime selection can still produce behavior the headless route did not exercise. No speculative transition algorithm changes were made.

REA’s Ghidra `procedure_info` request for an address in the retail EXE timed out again; target was closed. Static header/PE-resource observations remain usable but do not reveal transition semantics.

## Routes and verification

Two headless pathways are separately completed and have different final script variables: primary path ends at loop 181,064 (`favMiyako=2`, `favMiyako_a=1`); alternate path ends at loop 58,368 (`favMiyako_d=1`). This shows the runner processes different choices to two distinct terminal states. It is not exhaustive branch coverage or Windows retail comparison.

`cargo test --all-targets --quiet`: 91 passed, 2 ignored, 1 headless CLI integration test passed. No production code changed this round.

## Remaining

Aether host project and Wine/Godot remain absent in the current environment. Actual host launch and screenshot, retail title/GUI and behavior comparison, every route/branch, original exact command parity, and host audio pause lifecycle are not verified.
