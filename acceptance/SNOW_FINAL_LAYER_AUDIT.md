# Snow final layer composition audit (partial)

Hash-matched read-only native followup corrects earlier assumed boundary: layer vtable+0x40 is image visibility setter 0x51adf0, called from Snow at0x50fd3c. Layer output routine0x51cae0 iterates visible records and dispatches to0x521b80,0x523380,0x522d20 according to record flags. Snow intermediate packed-max is not direct compositor.

0x521b80 clips and blends via lookup tables5dca00/5cc930/5bc930, preserving destination high byte; exact table equation unresolved. Snow image initialization defaults flags+0x17/+0x18/+0x1c/+0x20/+0x38/+0x40; report’s conditional analysis suggests the +0x17=0 arm routes through0x522d20, but whether active at draw is a static inference only. No runtime oracle, no exact scene-final alpha result. Threshold getter436da0 dynamic+cc writer unresolved.

**Integration intentionally deferred.** Existing opaque scene destination with packed integer max would be incorrect; guessed source-over may also be wrong. Snow visual fade remains a concrete gap. Do not call this complete, and do not alter known cinematic crop. Evidence verified after handoff at ../ef-re/snow-final-composition.txt, plus source continuations ../ef-re/snow-clock-native.txt and ../ef-re/snow-fadeout-native.txt. The raw final-layer report is static bounded disassembly, not a runtime render observation. REA timed out and was closed.
