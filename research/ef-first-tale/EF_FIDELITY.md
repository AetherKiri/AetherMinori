# ef graphics-command fidelity: audit and scoped state correction

## Production follow-up (2026-10-24)

The production VM maps exactly six operands into `bg_h/bg_s/bg_v/st_h/st_s/st_v`, and validates exactly six arguments. The fabricated global RGB gains were removed. A native instruction-derived S/V kernel is implemented but remains isolated and is **not applied to rendered frames**; hue transfer and the standing-state renderer remain unproved. The runtime therefore preserves the two triples without claiming nonzero pixel fidelity. All-zero output is unchanged. Regression tests cover distinct signed operands, corpus tuples, wrong arity, state retention, and the intentionally no-rendering fallback.

### Additional bounded native evidence

Target identity remains SHA-256 `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`. Initial export: [ef-hsv-implementation-evidence.json](</var/lib/dsh/Minori/ef-re/ef-hsv-implementation-evidence.json>). Follow-up exports: [round19 renderer](</var/lib/dsh/Minori/ef-re/ef-st-renderer-round19.json>), [round20 downstream](</var/lib/dsh/Minori/ef-re/ef-hsv-downstream-consumer-evidence.json>). No game execution; bounded objdump inspected original bytes. Ghidra cold decompilation timed out. All REA targets closed after capture.

- `ev_b86f7f21bfa7d1135b2752559278fcc25031091bce02386391f3402baa96f388`: exact getter bodies 0x42ed40..0x42ed90 read +0x198..+0x1ac.
- `ev_6dde9515537cdb2ebbab8bda36ef89c20d39d9fd714b01b2a04a30f276ea26a5` and `ev_552167b56fe0cadaf08754f473d8c55ec13641d1a246779b73faa77a322e0cc7`: bounded raw e8/literal-pointer searches did not identify getter callers; syntactic negative results do not prove absence.
- `ev_26bd46426885e731d7af0c4a9cff0e9039a4ea7bc6fda9a1ad12ad2ab954f33c`: displacement-based candidate search; unrelated objects reuse these offsets, so matches alone are not renderer authority.
- `ev_bf44c9e19f8c824d514ecf16ab227cfd1f2847b47d5c5b7b57c9d1a8ccd33c10`: 0x5070c8 tests renderer +0x42c/+0x430/+0x434; if any nonzero, passes **only S/V** (+0x430/+0x434) and image +0x98 into 0x51f430 before later composition. Thus a hue rotation must not be invented for this call.
- `ev_3dd9e09efb1b4ac0a8aa94a6e107b8ab13255e0c0b1f3e0784850e9cd4d9ea19`: 0x51f430 begins a fixedpoint S/V matrix, not textbook HSV. It computes `q = (clamp(S+100,0,200)<<16)/100`; off-diagonal coefficients derive from `(65536-q)` times 0x4c8b/0x9646/0x1d2f with arithmetic shifts, plus `V<<8`. Diagonal coefficients add q. Per-pixel sums shift right16 and clamp to bytes. Its 32-bit path preserves alpha and skips alpha-zero pixels. The 32-bit path is now ported as isolated `ef_sv_matrix_rgba` with instruction-derived fixtures (identity, V contribution, endpoint saturation, transparent pixels, preserved alpha), but is intentionally not connected to scene rendering until base/extras ownership is corrected. Signed 32-bit IMUL overflow is reproduced: at S=-100 the green coefficient wraps negative. It has not been verified against an executable oracle or the alternate pixel-format path.
- `ev_23e397be43e4fe0cb1f233aaa4b31466f2ee2babc70162684c945359deaed239`: aligned 0x5069ca copies renderer +0x438/+43c/+440 into a standing-object +4/+8/+c.
- Round 19 exported bounded disassemblies: [standing consumer](</var/lib/dsh/Minori/ef-re/ef-st-consumer-round19.json>), [helper](</var/lib/dsh/Minori/ef-re/ef-st-helper-round19.json>), and [renderer path](</var/lib/dsh/Minori/ef-re/ef-st-renderer-round19.json>); all from executable SHA-256 9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e. The consumer at 0x5069ca demonstrably copies the three fields and subsequently passes the constructed object through indirect calls to helper 0x4d91f0. In caller 0x539297, further retrieved 0x4d91f0-created data is supplied as arguments to indirect renderer vtable methods (e.g. 0x5392a9..0x5392e1). This narrows the ST route to standing-object rendering, but object offsets at that later consumer are not proven to be these same +4/+8/+c fields, and the indirect method bodies remain unresolved. This is evidence of forwarding, not proof that the S/V kernel is the standing renderer. Hue, renderer lifetime/reset, channel layout, alternate pixel formats, and stage-extra handling remain unresolved. All captures exited 0 and were bounded; no game execution occurred.

A parallel audit handoff also exported [ef-hsv-math-handoff-evidence.json](</var/lib/dsh/Minori/ef-re/ef-hsv-math-handoff-evidence.json>): reported chain 0x4df7f7..0x4df824 to setter 0x4ffb30 and clamping H[-180,180], S/V[-100,100]. These are downstream renderer clamps, not justification to clamp raw command state at parse time. The residuals below are current and remain unless explicitly resolved above.

---

## Historical initial finding (superseded by implementation above)

**The six `.HSVfilter` operands are two separately named triples, not one global adjustment.** The production VM now preserves `bg_h/s/v` and `st_h/s/v`; the old global red/green gain pass was removed. The original background S/V kernel has been ported but intentionally remains unconnected because correct source-layer ownership and standing path are not established.

This is a static command/state-layout finding, **not** a pixel-equivalence or gameplay-runtime claim. Correct state mapping and the isolated kernel have since been implemented; the latter is not connected to frames. The historical source snapshot below is retained as audit lineage, while current implementation status is summarized above.

## Exact original evidence (independent of historical handler metadata)

Target: [ef_first_AA.exe](</root/games/ef-first-tale/ef - the first tale/ef_first_AA.exe>), SHA-256 9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e.

All addresses are linked PE VAs, not observed runtime addresses. Used bounded objdump and read-only Python PE-byte parsing under REA process capture; no Ghidra cold analysis or game execution.

1. Registration at 0x4cb9fc installs inline vtable 0x57679c; 0x4cba0c pairs that object with literal pointer 0x5785dc, whose bytes are .HSVfilter followed by NUL. The preceding registration at 0x4cb941 is .message, **not .HSVfilter**. Do not reuse the historical source comments' addresses as proof of handler identity.
2. Vtable slots +4/+8 are parser 0x4edab0 and executor 0x4eda20. Parser compares argument count to exactly 6 at 0x4edad5; six successive parsed integers go to object+0x44, +0x48, +0x4c, +0x50, +0x54, +0x58.
3. Executor 0x4eda20..0x4eda75 copies these, in order, into scene+0x198, +0x19c, +0x1a0, +0x1a4, +0x1a8, +0x1ac through VM+0x40 then +0xc (conditional on the latter pointer being nonnull).
4. State binding 0x4d585a..0x4d58c1 passes these six exact field addresses with literals at 0x57804c, 0x578054, 0x57805c, 0x578064, 0x57806c, 0x578074: **!bgH, !bgS, !bgV, !stH, !stS, !stV**. The three bg / three st names are binary observations; interpreting H/S/V as hue/saturation/value and bg/st as background/standing art is the supported semantic inference, not a recovered original C++ type.

| Operand | Object field | Scene field | Original bound name | Current VM field |
| --- | --- | --- | --- | --- |
| 1 | +44 | +198 | !bgH | bg_h |
| 2 | +48 | +19c | !bgS | bg_s |
| 3 | +4c | +1a0 | !bgV | bg_v |
| 4 | +50 | +1a4 | !stH | st_h |
| 5 | +54 | +1a8 | !stS | st_s |
| 6 | +58 | +1ac | !stV | st_v |

The [extracted corpus](</var/lib/dsh/Minori/ef-re/extracted/scr/100_01.sc>) contains `.HSVfilter 0 0 0 5 -5 0` at line 767; [108_03.sc](</var/lib/dsh/Minori/ef-re/extracted/scr/108_03.sc>) line 37 contains `0 0 0 0 5 -5`. Production state now preserves these exact tuples in separate triples; expected pixels are not yet implemented.

## Scoped fix applied; rendering remains unimplemented

The production state-model patch is committed as `0b1f490`; the instruction-derived, isolated S/V kernel is `1917d70`. Rendering integration is deliberately deferred: determine original layer ownership and standing-state callback behavior before applying the kernel. Do not replace the unresolved hue/standing transform with guessed math. The tests assert state preservation and the no-filter fallback, not original pixel values.

## Scope inventory and precise residuals

Source inspected: [command.rs](<../../src/command.rs>), [vm.rs](<../../src/vm.rs>) command branches and parse_layer/parse_stage, [provider.rs](<../../src/provider.rs>) graphics-event routing, [scene.rs](<../../src/scene.rs>) apply_screen/apply_overlay/apply_frame_stills/apply_hsv_filter, refresh_composed_frame, strip expansion, composite_layers, apply_stage, and effect dispatch. [FORMAT.md](<FORMAT.md>) is corpus/archive research, not native handler proof; its statement that these commands still need implementation is stale relative to existing branches.

1. **HSV transfer function unresolved:** exact units, hue wrap, saturation/value scaling, clamp/rounding, integer precision, alpha treatment, and the final downstream drawing implementation were not recovered. Separate triples and their state destinations are established; exact pixels are not.
2. **HSV lifetime/integration unresolved:** when triples persist/reset across stage, chain, save/load, scrolling and transitions; whether overlays/frame/panel/effect particles participate; precise treatment of .stage extras. Round 19/20 evidence shows field copies into a standing object and unresolved indirect renderer callbacks; the exact standing filter-to-pixel consumer was not proved.
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
