# Native Screen strip kernel

Independently reviewed ef-re/screen-native-unique.txt223..267:51ff70 loop complements source/destination RGB, unsigned byte MUL, INC AH, XORffffff, retains destination alpha, skips destinationalpha0. Thus channel result254-(((255-s)*(255-d))>>8), not textbook /255 Screen; source255 yields254. Sourcex0 repeated across row; -1 bottom-aligned row sampling/prefix extension same as Overlay.

Added isolated ef_screen_strip plus exhaustive65536 channel pairs against instruction-shaped MUL/highbyteINC/XOR reference, destinationalpha0 unchanged, sourcealpha ignored, black strip high-end quantization fixture. Suite121 passed/12 ignored plus1 CLI; diffcheck clean. Static-derived reference not original-executed oracle. No production screen integration or new screenshots yet.

Native parser confirmed2 operands map(empty,operand0,operand1),3+ mapfirst3; fewer2 no setters. Screen resources first1688/second19e8/third1b08 publishctx41c/410/414 at4df7ce/e4/ee. Same asset scopes as paired Overlay; Screen runs AFTER Overlay. Must not put strips in covering/full-frame source-over layer. Next integrate slot-aware raw resources and versioned persistence analogous Overlay.
