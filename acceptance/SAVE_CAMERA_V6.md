# Manual save v6 — preserve static camera recrop inputs

Reviewed delegated implementation in scene/persistence. Saves retain original background source, camera coordinates and fixed stage foreground. Previously restore substituted the cropped viewport and camera0; later scroll could not reproduce uninterrupted rendering. Static source/foreground frame byte lengths are validated. Dynamic scrolling/particles/HSV remain outside this change.

Writer now v6. Explicit fixed-layout v5 seven-field and v4 four-field migrations preserve older saves with historical cropped-source fallback; v1-v3 reader support remains. Tests independently encode old envelopes, migrate and rewrite as v6; full v6 envelope includes source/foreground. Synthetic decoded scene tests compare down/up scrolling after restore at multiple ticks and nonzero coordinates, with and without foreground.

Parent independently reran tests:113 library passed/9 ignored plus1 CLI passed; all9 opt-in real-install tests passed3.41s; diff check clean. ABI unchanged. New dedicated synthetic camera test exercises production background restore helper; real-asset full restore+scroll is still useful follow-up. No claim of original proprietary save compatibility, dynamic effect continuity or retail parity. Round78 flat background visual discrepancy remains to investigate independently.
