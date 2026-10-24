# Round 39 — replay through 104,000 loops and personally inspect progression

## Fresh headless replay

Ran the existing lightweight backend against the installed retail data through loop index 103,999, saving a new output set in `acceptance/round39-visual/`. The replay completed with exit code 0. The trace ends at `202_01.sc`, message 3290, `WaitingInput`; provider debug reports `favMiyako=2`, no error, and 5 select requests answered. This is continued script/VM progression, not retail runtime proof.

Checkpoint PNG SHA-256 values at 24k, 32k, 40k, 48k, 56k, 64k, 72k, 80k, 88k, 96k, and terminal loop 103,999 are all distinct. I personally inspected images at 32k, 56k, 64k, 80k, 96k, and 103,999: varied scenes/backgrounds, characters in several scenes, Chinese dialogue, and the translucent message panel are visible; 103,999 shows a letterboxed scene. Thus the deterministic headless rerun does **not** freeze to one image after 30,000 iterations. It does not explain the earlier unrelated 96,000 capture anomaly or prove Windows/Aether visual parity. Cinematic black bars remain intentional per user.

The initial foreground tool budget expired at 120 seconds while the child continued; its returned job ID `bash-f34617e11949488e` was tracked and collected to completion. Final process output: `frames_written=14 last_index=103999`, `state=WaitingInput`, current scene script `202_01.sc`, and empty runtime error. No detached replay jobs remain.

## REA and code

A fresh bounded REA GNU objdump header observation again identifies `ef_first_AA.exe` as `pei-i386`, entry `0x0046825d`, SHA-256 `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`, unchanged after inspection. This establishes file identity only. Ghidra/native behavior comparison was not retried because prior analysis timed out. No production code was changed; no parity inference was made from scripts beyond the routes exercised.

## Remaining acceptance

`project.godot`/Aether host source and Wine are still unavailable; the host itself cannot be launched here. No new whole-route or Aether screenshot acceptance is claimed. Audio/pause integration, Windows behavior comparison, title and retail presentation, and complete original command parity remain unverified.
