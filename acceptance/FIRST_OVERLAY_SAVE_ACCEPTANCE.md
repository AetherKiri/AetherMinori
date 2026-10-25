# Actual first-stage-asset tint and restore acceptance

Added opt-in local-install test using corpus101_01 first-stage asset st_kei10a02-.png and bg_scl002a01_00_f.png, with first-slot or_kei00a.png. Checks tint differs from raw display, bincode manual-state restore then explicit raw-stage rebuild exactly reproduces tinted display, and clearing restored slots exactly recovers original display. This uses real proprietary assets through VFS and production restore path; no screen command included (still unresolved), no original executable oracle.

Default120 passed/12 ignored plus1 CLI passed; all12 opt-in real-install tests passed3.55s, diff check clean. No production changes. Native screen51ff70/parser-slot audit delegated read-only to address next scene occlusion rather than inventing source-over semantics. No new screenshots this step.
