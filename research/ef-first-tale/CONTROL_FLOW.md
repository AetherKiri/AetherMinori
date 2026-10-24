# Original ef conditional semantics

Target: /root/games/ef-first-tale/ef - the first tale/ef_first_AA.exe
SHA-256: 9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e

## Conclusion

The original .if branches to its label when the comparison is TRUE, not false. This is static instruction evidence, not a runtime test of the game. No source was edited. Target was closed after exporting evidence.

## Exact discovery chain

- .if literal VA 0x5786ac (file 0x176eac).
- Registration block 0x4cc5a5 allocates 0xd4 bytes, calls constructor 0x4ed940 at 0x4cc5bc, then pairs object with the .if literal at 0x4cc5c8 onward.
- Constructor sets vtable 0x57683c at 0x4ed970. Vtable slot +8 points to executor 0x4ed3d0. Slot +4 points to four-argument parser 0x4ed2f0; arguments stored at object +0x64 (left), +0x80 (operator), +0x9c (right), +0xb8 (label).
- Operator literals: != 0x576820, == 0x576824, > 0x576828, < 0x575698, >= 0x57682c, <= 0x576830.
- Executor resolves operands and obtains integer left in EBX and right in ESI: left object+0x64 is selected at 0x4ed40f and its parsed value loaded into EBX at 0x4ed4e0; right object+0x9c is selected at 0x4ed45e and its parsed value loaded into ESI at 0x4ed52a. Comparison notation below is GNU objdump AT&T source,destination: `cmp %esi,%ebx` (bytes `3b de`) sets flags for EBX - ESI, i.e. left - right. In Intel destination,source notation this is `cmp ebx,esi`. Register mapping and branch conclusions are unchanged; the syntax distinction prevents reversing the comparison.
- For >: 0x4ed5a8 selects literal 0x576828; 0x4ed5b6 `cmp %esi,%ebx` (AT&T); 0x4ed5b8 JLE 0x4ed628 skips label handling when left <= right. Otherwise 0x4ed5ba jumps to label handling 0x4ed570.
- For <=: 0x4ed5e4 selects literal 0x576830; 0x4ed5f2 `cmp %esi,%ebx` (AT&T); 0x4ed5f4 JG 0x4ed628 skips when left > right. Otherwise 0x4ed5f6 jumps to label handling.
- Label handling loads current script from VM+0x1c34, looks up object+0xb8 label in script+0x4c. A valid nonnegative index from node+0x28 is stored to VM+0x1c3c at 0x4ed618 and optionally VM->0xc +0x24 at 0x4ed625.
- Hence favMiyako > 0 selects miyakotrue; <= 0 selects miyakobad. Historical 0x449010 was not used to identify this handler.

## .chain (consumer followed; see CHAIN_NEXT.md)

- .chain literal VA 0x578728, registration constructor call at 0x4cccff -> 0x4f13e0, vtable 0x5760e4, executor slot +8 -> 0x4f1490.
- Executor calls 0x4ddaa0, then calls 0x4c7f70 on VM+0x1c2c and stores its argument (object+0x44) under literal NEXT (0x5760d4), using 0x4c90b0 with flag 1.
- Variable lookup 0x4d9770 searches VM+0x1c30 first and VM+0x1c2c second.
- Follow-up: [CHAIN_NEXT.md](CHAIN_NEXT.md) traces NEXT consumer `0x4e36a1..0x4e384c` through script activation `0x4d8770` and single-key NEXT removal. The normal successful transition retains both VM variable stores; no blanket local clear occurs. Explicit VM+0x1c30 clearing belongs to a separate restoration helper `0x4e3940`, not this transition. See the follow-up for evidence IDs and command-specific cleanup/runtime limitations.

## REA evidence

Export: /var/lib/dsh/Minori/ef-re/ef-if-chain-evidence-final.json (10 records, 4 residual unknowns).
- ev_39ed07e1402edea5174807dcd1539c243637156af28aa70ad213b12adef3de33: target-bound section layout, objdump.
- ev_638d90fec47c7324864d7b83f2aef366116b598ed93ebf52ab47eaf1aa5563b5: broad objdump. IMPORTANT output budget stopped this around 0x4dd9be: useful for registration only, not entire executable coverage.
- ev_82b0eed85ca45d0079fcf5487e9a9873d2322a76716d1f18dd21bc5c69586cba: bounded objdump of executor 0x4ed3d0..0x4ed673 via REA process capture; exit 0, no truncation.
- ev_b70beabbc32ca4c6383debc060bc305c7413b015e52143483a68e3292349f9a8: bounded registration/constructors/chain executor, literal bytes and vtables, before/after target SHA-256 via REA Python process capture; exit 0.
- ev_91ba3c56e84f8980e4980b5701abf71a6eff9eac60ef5a469e6e8d1320ed6538: bounded variable lookup and setter, NEXT literal and before/after SHA-256; exit 0, no truncation.

Ghidra cold analysis timed out twice through MCP. Used REA objdump and controlled process captures of read-only Python byte inspection and objdump instead. Process captures execute the inspection tools, NOT the game. Their subject is not automatically artifact-bound; explicit printed target digests and exact command paths provide the linkage. Addresses are statically linked PE VAs, not measured runtime relocation addresses.
