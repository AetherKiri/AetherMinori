# Scoped static HSV integration acceptance

Production now applies native S/V matrix only to ef static image-tuple base(BG) and static stage extras(ST) before Overlay->Screen. First/base_spec excluded per observed zero arguments; dynamic char/ANI excluded. H only gates; no invented hue rotation. Raw6 state preserved, S/V clamped only renderer boundary. v9 saves HSV with explicitv8 tuple migration. Originals retained, immediate updates/repeat/clear reversible on ordinary static path.

Parent independently verified127 library+1CLI,14ignored default;all14optins passed3.35s including actualgym nonzeroST kernel/save/clear. Rebuilt2001update104_01 replay exit0, personally read gym0500 andevening2000; both hashes exactly match previously accepted Screen candidate. This tests no regression there, not nonzeroHSV visual proof (nonzero path checked by tests).

IMPORTANT unresolved visible discontinuity: WScroll renderer still reads raw layers through native_surface, bypassing new staticHSV helper; activation/redraw may discard staticHSV tint. apply_hsv_filter whileWScroll active also enters that path. This is NOT accepted native equivalence; requires original effect-path audit before widening scope. Dynamicchar/hue/originalcache/save lifetimes likewise unknown. Lightweight backend not actualAether/Windows comparison.
