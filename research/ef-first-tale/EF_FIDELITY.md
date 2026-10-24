# ef graphics-command fidelity: bounded read-only audit

## One actionable finding

**The six .HSVfilter operands represent two HSV triples, not one global HSV/contrast/RGB adjustment.** The current VM maps them to hue, saturation, brightness, contrast, red and green; the scene pass then ignores hue/saturation/contrast and applies brightness plus red/green gains to the already-composited scene. This does not preserve the original command's independently named background and standing-art state.

This is a static command/state-layout finding, **not** a pixel-equivalence or gameplay-runtime claim. Production files were not edited. Another agent was concurrently changing scene/provider title handling, so source line numbers can move; function names below are the durable selectors.

## Exact original evidence (independent of historical handler metadata)

Target: [ef_first_AA.exe](</root/games/ef-first-tale/ef - the first tale/ef_first_AA.exe>), SHA-256 9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e.

All addresses are linked PE VAs, not observed runtime addresses. Used bounded objdump and read-only Python PE-byte parsing under REA process capture; no Ghidra cold analysis or game execution.

1. Registration at 0x4cb9fc installs inline vtable 0x57679c; 0x4cba0c pairs that object with literal pointer 0x5785dc, whose bytes are .HSVfilter followed by NUL. The preceding registration at 0x4cb941 is .message, **not .HSVfilter**. Do not reuse the historical source comments' addresses as proof of handler identity.
2. Vtable slots +4/+8 are parser 0x4edab0 and executor 0x4eda20. Parser compares argument count to exactly 6 at 0x4edad5; six successive parsed integers go to object+0x44, +0x48, +0x4c, +0x50, +0x54, +0x58.
3. Executor 0x4eda20..0x4eda75 copies these, in order, into scene+0x198, +0x19c, +0x1a0, +0x1a4, +0x1a8, +0x1ac through VM+0x40 then +0xc (conditional on the latter pointer being nonnull).
4. State binding 0x4d585a..0x4d58c1 passes these six exact field addresses with literals at 0x57804c, 0x578054, 0x57805c, 0x578064, 0x57806c, 0x578074: **!bgH, !bgS, !bgV, !stH, !stS, !stV**. The three bg / three st names are binary observations; interpreting H/S/V as hue/saturation/value and bg/st as background/standing art is the supported semantic inference, not a recovered original C++ type.

| Operand | Object field | Scene field | Original bound name | Current VM field |
| --- | --- | --- | --- | --- |
| 1 | +44 | +198 | !bgH | hue |
| 2 | +48 | +19c | !bgS | saturation |
| 3 | +4c | +1a0 | !bgV | brightness |
| 4 | +50 | +1a4 | !stH | contrast |
| 5 | +54 | +1a8 | !stS | red |
| 6 | +58 | +1ac | !stV | green |

The [extracted corpus](</var/lib/dsh/Minori/ef-re/extracted/scr/100_01.sc>) contains .HSVfilter 0 0 0 5 -5 0 at line 767 (ASCII command observed with grep; read tool rejects this non-UTF-8 script). Under current implementation this ignores operand 4 and multiplies red by 0.95 globally, while the binary stores operands 4/5 as !stH/!stS and leaves all three !bg fields zero. Similarly [108_03.sc](</var/lib/dsh/Minori/ef-re/extracted/scr/108_03.sc>) line 37 uses 0 0 0 0 5 -5: current global R/G gains cannot represent the original separate state triple. This establishes wrong parameter interpretation; exact expected RGB output still requires the downstream HSV transfer function.

## Minimal fix recommendation (not applied)

In [vm.rs](<../../src/vm.rs>), replace HsvFilter's global hue/saturation/brightness/contrast/red/green/blue model and CommandKind::HsvFilter mapping with two explicitly named triples bg_h/bg_s/bg_v and st_h/st_s/st_v, preserving operand order. In [scene.rs](<../../src/scene.rs>), retain this separation through composition; do not implement operands 5/6 as global red/green gains. Locate and port the original downstream HSV arithmetic before claiming exact color fidelity. The smallest safe first patch is correct state/mapping plus focused tests, **not** guessing a new global filter equation. Keep all-zero identity and alpha unchanged. Test both corpus tuples above and an asymmetric background/standing-sprite fixture to prevent cross-layer contamination. Apply st adjustments before flattening .stage standing-art extras into the background; retain unfiltered originals to avoid cumulative filtering on refresh. Update the misleading UI/contrast/RGB comments alongside the eventual fix.

## Scope inventory and precise residuals

Source inspected: [command.rs](<../../src/command.rs>), [vm.rs](<../../src/vm.rs>) command branches and parse_layer/parse_stage, [provider.rs](<../../src/provider.rs>) graphics-event routing, [scene.rs](<../../src/scene.rs>) apply_screen/apply_overlay/apply_frame_stills/apply_hsv_filter, refresh_composed_frame, strip expansion, composite_layers, apply_stage, and effect dispatch. [FORMAT.md](<FORMAT.md>) is corpus/archive research, not native handler proof; its statement that these commands still need implementation is stale relative to existing branches.

1. **HSV transfer function unresolved:** exact units, hue wrap, saturation/value scaling, clamp/rounding, integer precision, alpha treatment, and the final downstream drawing implementation were not recovered. Separate triples and their state destinations are established; exact pixels are not.
2. **HSV lifetime/integration unresolved:** when triples persist/reset across stage, chain, save/load, scrolling and transitions; whether overlays/frame/panel/effect particles participate; precise treatment of .stage extras. Original executor/state forwarding was inspected, not all consumers.
3. **.screen/.overlay unresolved:** original argument arity, * clear versus preserve semantics, resource-prefix semantics, strip padding/scaling/sampling/blending and exact placement/order. Current source decodes strips and applies screen then overlay after character composition; that conflicts with its screen-under-scene comment, but original order was not independently established here, so it is a source/comment inconsistency, not a second proven ef-binary defect.
4. **.frame unresolved:** original count/selection, * semantics, prefix handling, still origin/dimensions/blending, draw order and lifecycle. Existing source stores decoded stills; native frame parser/executor not independently reconstructed.
5. **.effect unresolved:** native ef dispatch and all kind-specific timing, randomness, motion, stop/fade and resource policies. Existing source supports scrolling/cutin/particle families and rejects other names; its end-stop explanation explicitly says “suggests”, which is hypothesis rather than exact ef proof.
6. **.stage unresolved:** original base-spec grammar, extras count/placement, single/pair coordinate meaning, flattening/order, transition and filter integration. Existing source contains implementations and corpus-based comments; no exact original stage equivalence was established.
7. **Cross-build/metadata unresolved:** no audit of Supipara/Trinoline semantics or repair of OriginalHandler tables. Historical entries are excluded from ef proof; only the independently followed registration/vtable chain above identifies this handler.
8. **Runtime verification absent:** no Windows game execution, screenshot comparison or reconstruction tests were run. This audit does not establish reachability on every language/route or global graphics equivalence.

## Preserved evidence and coverage

Complete export: [ef-fidelity-evidence.json](</var/lib/dsh/Minori/ef-re/ef-fidelity-evidence.json>) (12 retained records including capture unknown bookkeeping, 4 residual-unknown registry heads). These tool-level heads are not a replacement for the eight domain residuals above. Process captures have null artifact subject: explicit target path and printed SHA-256 provide linkage; objdump's own executable digest is not the game's digest. Inspection calls were exit 0 and untruncated; one early registration window began mid-instruction at 0x4cb800, so its misaligned prefix is not used, only the aligned later registration instructions. No native analysis target was opened.

Key Evidence IDs:

- ev_52866ec65bbc30be5031002ae5df282dee1fc4a89c4f98b1465396649f94e25b: bounded registration (use aligned portion only).
- ev_3cddabf0f8aa4976bf70e23d2e6cba95088381ed513de048a93d18228f5cbad2: PE mapping, literal neighborhood, matching before/after game digest; proves 0x4ecc00 belongs to preceding .message object rather than supplying HSV identity.
- ev_136600e6cdb33eea2a49d8df13ee1ba65a68660ad5030e693f993159d3ddec9a: game digest, HSV literal, vtable slots, parser/executor disassembly.
- ev_311f65fd15f000f1e0d137e3aaf2cadaf02c388f9e829aff69eff19c432f6c37: complete short executor and six-field copying.
- ev_b27bdedc2c50ebad9ba406bf3e1703a1f401cc3ba28de0304cdb2405538dd144: game digest and six exact !bg/!st literal bytes.
- ev_2b99cc087aeb91bdb755497f880262b661498448e68f69a6b3990414e6eff49b: complete six state-field/name binding sequence.

Process-capture limitations: inherited environment, short-lived subprocess sampling, non-atomic executable image binding, unobserved network, and no selected filesystem snapshots. Python and objdump inspected original bytes but did not execute the game. No global absence claim is based on syntactic offset searches.
