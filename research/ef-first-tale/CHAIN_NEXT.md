# .chain NEXT consumer and local-variable lifetime

## Finding

For the normal successful `.chain` transition, the original engine retains the VM variable stores: switching scripts does **not** perform a blanket local-variable clear. Keeping ordinary locals across `.chain` is supported by original static instructions, rather than only by script dependencies. This is not a runtime test or a claim that arbitrary script instructions, command-specific cleanup hooks, load/save restoration, or engine reinitialization cannot change variables. Production code was not modified.

Target: `/root/games/ef-first-tale/ef - the first tale/ef_first_AA.exe`

SHA-256: `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`. Addresses below are linked PE VAs. Syntax in captures is GNU AT&T.

## Consumer traced

1. Immediate executor `0x4f1490` calls `0x4ddaa0` with its VM, then `0x4c7f70` on VM+`0x1c2c` and writes its object+`0x44` string to key `NEXT` through `0x4c90b0`, flag 1. Importantly, `0x4c7f70` is not a map-clear operation: it conditionally performs backing-file processing (tests store+`0x1a`, uses filename store+`0x14`).
2. Main execution tail calls the selected command executor vtable+8 at `0x4e3647`. On its false return, it enters `0x4e36a1`. `0x4e36ca` selects `NEXT`; `0x4e36ec` loads VM+`0x1c2c`; `0x4e36f3` calls map lookup/copy `0x4c84b0`, copying the value into a temporary string.
3. Nonempty NEXT is split at `#` (literal `0x577b9c`). The left component is the new script name and the optional right component the label.
4. `0x4e378c..0x4e37af` allocates `0x20b8` bytes and constructs a script through `0x4caf70`, passing the same VM. `0x4e37b5..0x4e37bc` flushes old script read-state through `0x4ca070`. `0x4e37cf` loads/parses the new script using `0x4cd100`.
5. On successful load, `0x4e37e7` calls activation `0x4d8770` (EAX=VM; stack argument=new script). Its complete body `0x4d8770..0x4d87d6` deletes the old VM+`0x1c34` script, assigns the new script at `0x4d8795`, zeros instruction index VM+`0x1c3c`, and updates the optional history object. It neither assigns nor clears VM+`0x1c30` or VM+`0x1c2c`. Optional label resolution sets the PC at `0x4e3808`.
6. `0x4e3819..0x4e383c` selects `NEXT` again, takes **VM+0x1c2c**, and calls `0x4c8670` with flag 1. That helper searches one key and erases its found node (`0x4c86c4..0x4c86ca`); it does not clear the map. The consumer then releases temporary strings and returns. Thus NEXT is consumed/erased; it is not a replacement for the entire variable store.

## Store ownership and contrasting explicit clear

- Lookup `0x4d9770` searches VM+`0x1c30` first, then VM+`0x1c2c` (prior evidence in CONTROL_FLOW.md). The production term “locals” should not obscure these two separate stores.
- The original binds these to process-owned stores: `0x4b6995` assigns `0x5aa154` to VM+`0x1c2c`; `0x4b699f` assigns `0x5aa170` to VM+`0x1c30`. The current script is a distinct VM+`0x1c34` object.
- An actual clear of VM+`0x1c30` occurs in `0x4e3940`: `0x4e39ff` loads that store, `0x4e3a0f` destroys its tree, `0x4e3a1c/22/27` reset the sentinel links, and `0x4e3a2a` zeros its size. It conditionally preserves one special key and then restores incoming data. Direct callers found in full objdump output are `0x4e3faf` and `0x4e41f8`; **the NEXT transition does not call this helper**. This is a separate state-restoration path, not evidence for chain clearing.

## Loader/destructor checks and bounds

`0x4cd100` calls `0x4cab60`; that routine reads/decrypts and parses script lines through `0x4cab10`/`0x4ca510`. Command construction calls vtable+4 (argument parser) at `0x4caa0a`, rather than executing vtable+8. The script destructor is `0x431820 -> 0x4ca2b0 -> 0x4ca1a0`, freeing script-owned strings, parsed commands and maps. It has a per-command vtable+0x34 cleanup hook. For `.chain` that slot is `0x4f15f0`, whose complete body is `mov al,1; ret 4` (no variable mutation). We did not enumerate every possible command-specific cleanup hook or every backing-store callback; universal absence of all possible variable effects remains outside this bounded static proof. No game execution was performed.

## Evidence

Complete REA export: `/var/lib/dsh/Minori/ef-re/ef-chain-next-consumer-evidence.json` (17 records, 4 residual unknowns). REA was closed after export. Python/objdump ran under REA process captures, not the game. Each useful capture prints the exact target SHA-256; selected captures print matching before/after hashes. Process-capture subject is null, so this explicit digest is the artifact linkage, not an automatic runtime image binding.

Key records (all exit 0 and untruncated unless stated):

- `ev_5595c96c5d291131fe237794b98847d387713b927c5f8da3dfbd53542c6f3589`: NEXT pointer search; exactly three literal-pointer occurrences, text VAs `0x4e36cb`, `0x4e381a`, `0x4f14d0`. These are immediate operands, not function entries.
- `ev_2ac6f4c205b25ff10b0d95a2852266bd663000d4cc6b464e10196d6d436d4e97`: complete NEXT consumer, command executor call, and contrasting clear.
- `ev_42e396c2e2b236ef9edd0cdc1e7a725ab56e8ecb615eec81c311ae5cdf088025`: complete activation, constructor prefix, old-script state flush.
- `ev_ee3ea2532a230d7aff749ede2a0c050a6b0886fb102feb01eb2217451b5d4455`: complete loader/parser `0x4cab60..0x4caf6d`.
- `ev_4462524ec33be12eeb37ebba7669111d5ef791ddb63113c64687d57e23aeb6f8`: full objdump scanned in memory, printing only literal VM+1c2c/1c30 accesses and direct clear-helper calls. This is a bounded syntactic search, not exhaustive alias analysis.
- `ev_1fe4c3939c85dcff19095c83e6440a3eb9fce2a90c3ceef7982692a425a18cc9`: parsed PE section mapping, NEXT/# bytes, and complete key-erase helper.
- `ev_91d47288a5faedb7420ff5b51c54909e5d8741b4b72cad8540ed98f023ecaad1`: script destructor and teardown.
- `ev_26ebff65c89f67c4c31e761996b7d0da2b337199892c1b8ad7e82c59abc941e8`: chain vtable slots and complete lookup/copy helper.
- `ev_12bf2580bf93d5c59f6f42bfb7a40f05d9b68727dd2b1711791c1df2d6023308`: complete chain cleanup no-op and parsed-command setup.

One exploratory broad loader capture (`ev_760931556019dc8703e05671f29dbf8951fab4813d71dd84b220083088f33013`) reports truncation; do not use it as complete coverage. One capture (`ev_dc4ee141dded185cdae04a120b44c54a122fe9d4e083d48f803d01360e0e4d02`) failed with a Python quoting syntax error and supplies no target evidence. Later corrected capture is the store-access inventory above.
