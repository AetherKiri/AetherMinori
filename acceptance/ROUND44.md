# Round 44 — alternate-branch route replay

## Work

Ran a second 240,000-loop headless replay using alternate choices (`--choose 1,1,1,1,1,1,1`) in a separate output directory. It exited successfully at loop 58,368; trace ends `58368	none	880	Finished`, runtime state `Finished`, local `favMiyako_d=1`, no error, and three select requests. It differs from the primary route, which completed at 181,064 with `favMiyako=2`/`favMiyako_a=1`. The alternate route thus reaches a different finite story endpoint and demonstrates at least one additional choice branch in headless mode. Four checkpoint PNGs were written; I personally inspected 24,000 and 48,000, which show different character artwork and Chinese dialogue. This does not imply all route combinations were exercised or that endings match retail. Job `bash-44a6212474804fff` was collected at exit code 0.

## Host / test boundary

Read the project README and rechecked the workspace. The documented lightweight program calls provider callbacks and draws its own diagnostic captures; it is not a host replacement and does not prove Aether invokes the library. Aether project files and Wine/Godot remain unavailable. Source-level provider tests and headless route tests are not host smoke tests.

## Retail static inspection

No fresh decompilation was run. The PE’s previous resource inventory is retained under Evidence ID `ev_b08ce7bdba14bd4359a82ab509b608bbc165cdfb7ebdad747ea9c4fe0d330b90`, complete for its 41 directory entries/16 resources but not behavior evidence.

## Remaining acceptance

Host startup/render screenshot, retail title/window behavior, executable-vs-runtime comparison, all branches, exact full original command parity, and host audio lifecycle remain open. Do not mark the objective complete.
