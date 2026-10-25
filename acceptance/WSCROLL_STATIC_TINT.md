# WScroll standing tint / asymmetric background authority

Fresh bounded native audit ef-re/wscroll-filter-native.txt supersedes initial assumption that ordinary BG helper always HSV-filters its destination. WScroll2 frame50a000 refresh50aa30 calls507010 with effect+68 asEDI. Rawload/Overlay/Screen operateEDI, but5070e3 explicitly LEA ctx98 then5070fc SV-filtersctx98, NOTeffect+68. Consequently no BG HSV added to WScroll renderer. Standing path5068e0 producesSV->Overlay->Screen surface atstage10, copied50ac86 to effect188 then tiled50a4aa. This justifies third-slot static helper on WScroll extras only.

New regression first failed raw[91,137,183] vs expected[82,119,156], then passes with standing-only helper. Verifies repeateddrawstable and raworiginalunchanged. Suite128 passed/14ignored plus1CLI; diffcheckclean. No screenshots this step.

Remaining lifetime mismatch: native refresh compares asset names50ab04/50ac22 and can retain effect-owned snapshot on same-name filter changes; current reconstruction prepares extras eachdraw and may update sooner. Setter4ffb30 only storesclamps, no invalidation; outereffectdirtyarming unknown. No claim of snapshot/livefilter equivalence or dynamic ANI hue. BGctx98 sideeffect not fully reconstructed.
