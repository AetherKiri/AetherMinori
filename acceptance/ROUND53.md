# Round 53 — fail closed on missing dialogue font

The screenshot runner silently converted TextRenderer startup failure to None and could exit successfully with all dialogue absent. Its fallback font list also included Latin-only DejaVu without checking CJK mappings. The runner now fails with an explicit font error rather than producing textless acceptance images. Font selection requires nonzero glyph IDs for representative Latin, Chinese, hiragana, katakana and punctuation characters. This is a sample coverage guard, not a guarantee of every glyph in every script or original-font fidelity.

Added deterministic coverage-predicate regression (Latin-only and missing individual CJK representatives rejected). Full tests: 94 library tests passed, 2 ignored; 1 CLI test passed. Build and diff checks passed. Real-install chapter smoke (`--language e --start 100_01.sc`, 400 loops, delta 120, clicks 80) exited 0 with message 270, WaitingInput and empty error. Personally inspected frame-0399.png: character closeup, church background and readable Chinese speaker/dialogue. No host or retail equivalence is inferred.

Original command parity and actual host acceptance remain incomplete. This improves the trustworthiness of screenshot acceptance; it does not fix or identify the historical 30k stall.
