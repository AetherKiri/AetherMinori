# Round 36 — recheck retail binary evidence and full suite

## REA observations

Revisited the installed retail EXE using REA’s bounded GNU objdump operations. `file_headers` reports `pei-i386`, architecture `i386`, and entry point `0x0046825d`; the artifact digest before/after is unchanged at `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`. `section_headers` observes a `.text` section at VA `0x00401000`, size `0x001542ec`, file offset `0x400`; `.rdata`, `.data`, and `.rsrc` are also present. `symbols` reports no symbols; `relocations` and `dwarf` output no records. These support an executable-image identity and explain why portable symbol/source debugging is not available from these operations. They do **not** establish original transition or gameplay semantics.

Attempted a direct Ghidra decompilation of an already-known address (`0x456fa0`, previously described as the shared finalizer) again timed out. A default objdump disassembly result was over the MCP response budget and could not be consumed inline; no content was retained from that attempt as a behavioral finding. The target was closed afterward and the session reports no open binary. Exported two short successful metadata observations (header and relocation results) to `/var/lib/dsh/Minori/ef-re/round36-evidence.json`; their canonical REA Evidence is the authority for those facts.

## Tests and source scope

`cargo test --all-targets` passed: 91 library tests passed, 2 installation-dependent tests were ignored, and the one headless CLI test passed. No production code changed this round. The existing script-corpus transition commands predominantly include types 0 and 1; that observation is not a retail implementation specification. I made no guessed transition changes.

## Remaining acceptance

The route’s previous headless screenshots remain personally inspected, but none were regenerated this round. The CMake provider archive and exported registration symbol were verified in round 33, and a tick/input pause guard was implemented and tested in round 35. Neither is an actual Aether host launch. AetherKiri host source, `project.godot`, Wine, and a Windows runtime are absent here. Thus the retail title, host visuals, audio/lifecycle integration, full Windows command parity, and original screenshot comparison remain unverified. Preserve the user-confirmed cinematic bars; the objective is still incomplete.
