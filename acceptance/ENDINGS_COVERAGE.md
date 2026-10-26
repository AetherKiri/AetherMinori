# Every Route Ending — Coverage & Acceptance Record

## Ending map (corpus-exhaustive, 75 scripts)

Terminal points found by scanning all scripts for BADEND markers and chain-less `.end`:

| Ending | Script | Reached by | Status |
|---|---|---|---|
| BADEND1 | 108_03.sc | `108_01` `.if favMiyako <= 0` -> miyakobad | **verified this round** |
| BADEND2 | 112_02.sc | `112_01` select option 1 (遵守与景的约定) | verified (previous round, 40-image audit) |
| BADEND3 | 115_03.sc | `115_01` `.if favMiyako <= 1` -> miyakobad | **verified this round** |
| BADEND4 | 216_03.sc | `216_01` select option 2 (label2) | **verified this round** |
| First-tale true end | 700_01mugai.sc (ef_700=1) | 112_01 option 2 -> 113..117 | verified (previous round) |
| 2xx finale | 701_01.sc (.end x5) | 216_01 option 1 -> 216_02/04/06 -> 217/218 | **verified this round** |
| 215_02_ero / 111_05_mem | included scenes | `#include` (215_02.sc line 2) / censored-variant — scene variants, not separate endings | documented |

The 212_01 select (212_02/212_03) is a scene variant: both sides chain 212_04.

## Verification runs (all `Finished`, traces exact)

- `end-badend1`: 108_01 -> **108_03** -> Finished (13,437 updates)
- `end-badend3`: 115_01 -> **115_03** -> Finished (8,067 updates)
- `end-badend4`: 216_01 -branch 1-> **216_03** -> Finished (7,062 updates)
- `end-finale`: 216_01 -branch 0-> 216_02 -> 216_04 -> 216_06 -> 217_01 -> 218_01 -> **701_01** -> Finished (43,467 updates)

Frames in `/var/lib/dsh/Minori/screenshots/end-*/`.

## Frames personally read (parent agent)

- end-badend1 frame-13200: empty church, 「如果我真的有那个心情，再来找她可能也不错。」 — full render
- end-badend3 frame-7800: parting intimate CG, 「恋情也未必仅此一次。」 — full render
- end-badend4 frame-1500: Kyousuke/Kei rooftop, 「等你什么时候想和我独处了…」 — full render; frame-6900: intentional dark monologue 「我都仍想一直看着她。」 (staging, not a render fault)
- end-finale frame-9000 (216_04 church exterior + Yuuko narration), frame-33000 white = 217_01's own timer transition (trace WaitingTimer, single sample), frame-43200 (701_01 church meeting CG, Yuuko & Yuu)
- end-finale movie snapshots: 5 ED-movie requests consumed; movie-043229 shows the ED title card "a sto / 故事" — decoder verified working; the four black snapshots are movies opening on black (offline backend takes a representative early frame)

## REA usage

REA MCP session live on `ef_first_AA.exe` (Ghidra 12.1.4 provider, auto-analysis gating
deep queries — same stall as prior rounds). Select/.if semantics already carry the objdump
verdicts (ef-re/select-if-native.txt): 0-based labels, signed comparisons — the primitives
this round's forks rely on are native-verified.

## Self-audit of the four runs (parent agent, after one audit subagent failed mid-run)

Programmatic census: every run's only duplicate md5 pair is (frame-0000 boot black,
final-frame terminal fade) - the same verified terminal-fade behavior as the original
full-route run. Tiny (<5KB) frames are exactly: boot black, terminal fade black, and
end-finale frame-33000 (217_01's own timer white transition, single sample, trace
WaitingTimer). No other anomalies.

Personally read additions: end-badend1 frame-6000 (Kyousuke low-angle dialogue) and
frame-11700 (BADEND1 church narrative); end-badend3 frame-1500 (Chihiro church, flowers,
"教堂后的陵园") and frame-5100 (115_03 closing narration over split white/dark staging).
All four bad/true endings render with full scenes, correct text, and intentional staging.
