# ef corpus command coverage audit (2026-10-26)

Method: decoded every .sc in ef-re/extracted/scr with ef-re/decode_scr_lines.js
(Shift-JIS), counted distinct leading commands, and diffed against the
CommandKind match arms in src/vm.rs. The mojibake lines in the count are
full-width-semicolon Japanese comments, which the parser skips.

| corpus command | count | VM branch |
|---|---|---|
| .message 14073, .transition 5254, .stage 4076, .panel 495 | - | Message/Transition/Stage/Panel |
| .playBGM 383, .playSE 230, .playSE2 12, .playVoice 12 | - | PlayBgm/PlaySe/PlaySe2/PlayVoice |
| .wait 360+60, .screen 177, .overlay 148 | - | Wait/Screen/Overlay |
| .effect 88, .chain 76, .endScroll 43, .label 40 | - | Effect/Chain/EndScroll/label table |
| .set 39, .vscroll 35, .scroll 28, .end 28, .pragma 26, .frame 26 | - | Set/VScroll/Scroll/End/Pragma/Frame |
| .if 23, .hscroll 18, .shakeScreen 17, .setGlobal 16 | - | If/HScroll/ShakeScreen/SetGlobal |
| .HSVfilter 14, .goto 10, .movie 8, .select 7 | - | HsvFilter/Goto/Movie/Select |

Result: **no corpus command lacks an implemented handler.** VScroll and
HScroll are separate CommandKinds (vm.rs line 697); labels resolve through the
program label table; comment lines never reach the dispatcher. The observed
zero-error 34001/60000-tick replays exercise most of these live.

Residual fidelity gaps are semantics-level, not command-level: dynamic
character/ANI HSV, the four table blend modes are ported but not selected by
the particle path (they belong to the separate mode dispatcher), active-effect
save restore, and the U1-U5 route unknowns documented in ef-re/snow-route-evidence.txt.
