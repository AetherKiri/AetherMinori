# Gym occlusion isolated to opaque overlay strip

Compared personally read round63 vs round78 frame30000: same dialogue, former gym/Kei visible, latter blue-grey flat. Corpus104_01.sc line20 sets overlay * or_kei00a.png, line216 clears it. Shared stage compositor now correctly retains this active layer, exposing pre-existing incorrect/unverified source-over semantics.

Real asset diagnostic: or_kei00a.png=1x601, padding first pixel[255,255,255,255]; first expanded pixel[118,144,157,255]; all480000 expanded pixels opaque. Source-over therefore necessarily obliterates stage while strip remains active. This is measured asset evidence and source consequence, NOT proof of original blend equation or which argument selects affected geometry. Do not fix by guessed alpha or silently ignoring overlay.

REA retained evidence inventory currently empty (summary returned0 records); initial unsupported operation filter rejected then removed. Need reopen/import archived evidence to recover original overlay consumer. Skill loaded. Diagnostic test passed0.04s; no native target opened this step. Overall visual discrepancy unresolved.
