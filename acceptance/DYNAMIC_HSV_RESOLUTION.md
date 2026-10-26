# Dynamic character/ANI HSV: native semantics resolved, production verified (2026-10-26)

## Native findings (ef-re/char-hsv-native.txt, 8 sections, 14 objdump windows)

- .HSVfilter handler 0x4cba0c region; exactly six %d operands committed into
  the screen-state object S+0x198..0x1AC (names proven by save-serializer
  keys "!bgH".."!stV"); per-frame clamped copies land in the compositor
  C+0x42C..0x440 via 0x4ffb30 (H +/-180, S/V +/-100).
- Per-pixel kernel 0x51f430: q=((S+100)<<16)/100, K=65536-q,
  coef_i=((int32)(K*w_i)>>16)+Vt with w=[0x4c8b,0x9646,0x1d2f] (ITU-R 601),
  out_c=((sum(rgb_i*coef_i)+c*q)>>16).clamp(0,255). Hue NEVER enters the
  pixel math; the low-32 IMUL wrap on 0x9646 for |S|>=86 (signed -27066 at
  S=-100) makes grays go black - an authentic native cliff.
- Timing: BG re-filtered every draw (dynamic); ST/ANI baked at load, cached
  by (file,H,S,V), layer-wide, with an embedded name[A:B:C] filename override.

## Production verification

- AetherMinori/src/scene.rs ef_sv_matrix_rgba (line 2303) was extracted from
  the same function in an earlier round. The reconciliation re-simulated the
  exact instruction sequence (low-32 IMUL, sar, mod-2^32 accumulation, magic
  divide): ZERO divergences over 12 targeted cases + 4000 randomized
  (S,V,pixel) tuples = 64000 pixels. Bit-exact; the signed wrap and the +q
  diagonal are both faithfully modeled. No code change required.
- Our runtime stores the six operands unclamped, applies S/V clamps at use,
  recomputes BG from unfiltered originals each compose (draw-time dynamic),
  and applies no hue rotation. All four match the native.

## Corpus equivalence

- All 14 .HSVfilter uses decoded: nine are identity resets (0 0 0 0 0 0),
  the nonzero st adjustments are (0 0 0 5 -5 0), (0 0 0 0 5 -5) and
  (0 0 0 0 -25 -8). The bg half is never used.
- 216_04.sc line 838 changes the st values while the ST layer is EMPTY
  (line 835 stages a bare background); the next character load is line 842
  (st_kei10g01.png). So the native load-time keying and our immediate
  application are observationally identical on every corpus use.
- The name[A:B:C] filename override has ZERO uses in the entire corpus;
  it is documented here rather than implemented.

## Limits

- The hypothetical filename-embedded |V|>100 divergence (native does not
  clamp inside 0x51f430) cannot trigger: the corpus never uses the override.
- Cache-key separators and the message-face draw path remain untraced
  (native side); neither is observable through the script corpus.
