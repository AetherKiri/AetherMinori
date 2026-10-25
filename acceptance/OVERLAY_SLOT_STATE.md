# Overlay parser preserves native slot identity

Based on independently verified parser4eb9a0, added overlay-specific minimum2 validation and preserved raw three-slot operand mapping in LayerRequest.overlay_slots, before wildcard/resource-prefix normalization. Two operands map to empty optional slot plus operands1/2; three-or-more map first3. Generic screen parser unchanged. Renderer still consumes historical images pending native downstream math; gym occlusion NOT fixed by this patch. ABI unchanged.

Regression covers two/three operands, retained wildcard, invalid single operand and screen separation. Full suite114 passed/10 ignored plus1 CLI passed; diff check clean.

Native audit update: overlay resource18c8 feeds renderer context40c through4df781/4df790, then50696a copies it to staging object14. Other resource slots1568->418 and17a8->408. Crucially resource+8c=4 is NOT established blend mode: source-bank loader524290 computes global bank base5e0148+index*270 and stores index at+8c. Must not implement guessed mode4 mixing. Downstream staging consumer audit ongoing.
