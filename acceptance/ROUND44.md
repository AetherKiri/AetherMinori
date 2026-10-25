# Round 44 — alternate-branch route replay

## Work

Started a second 240,000-loop headless replay using alternate choices (`--choose 1,1,1,1,1,1,1`) and a separate output directory, preserving the round-41 route artifacts. It is still in progress at report time; its trace is shorter than the completed primary route and initial emitted trace rows show script startup. No alternate-route completion or branch outcomes are claimed yet. The tracked job is `bash-44a6212474804fff`; collect that exact job before claiming completion.

## Host / test boundary

Read the project README and rechecked the workspace. The documented lightweight program calls provider callbacks and draws its own diagnostic captures; it is not a host replacement and does not prove Aether invokes the library. Aether project files and Wine/Godot remain unavailable. Source-level provider tests and headless route tests are not host smoke tests.

## Retail static inspection

No fresh decompilation was run. The PE’s previous resource inventory is retained under Evidence ID `ev_b08ce7bdba14bd4359a82ab509b608bbc165cdfb7ebdad747ea9c4fe0d330b90`, complete for its 41 directory entries/16 resources but not behavior evidence.

## Remaining acceptance

Host startup/render screenshot, retail title/window behavior, executable-vs-runtime comparison, all branches, exact full original command parity, and host audio lifecycle remain open. Do not mark the objective complete.
