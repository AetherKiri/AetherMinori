# True-Ending Route — Story Reaches the Tail (Acceptance Record)

## Problem (user report, goal round 1)

"剧情根本没有推进到最尾部，完全是半吊子卡住了" — the completed-route run of 2026-10-26
ended at `112_02.sc → Finished`, but that script's tail sets **`.setGlobal BADEND2 = 1`**
(line 215): the default-ordinal choices had steered into a **BAD END**, not the story's tail.
The last visible screenshot stopped mid-sentence (【景】「其实，我很久之前就——」) before the
bad-end black-out — exactly the "half-finished, stuck" the user saw.

## Root cause (corpus evidence)

- `112_01.sc` line 285: final fork `.select 遵守与景的约定:label1 留在宫子身边:label2`
  - label1 → `chain 112_02.sc` = BADEND2 ending (what the default run played)
  - label2 → white transition → `chain 113_01.sc` = **true ending chain**
- True chain: 113_01 → 113_03 → 114_01 → 115_01 → (`.if favMiyako > 1` at line 197)
  115_02 → 116_01 → 117_01 → **700_01mugai** (epilogue; sets `ef_700 = 1`) → chains 200_01.
- Favor gates passed: the default choices yield favMiyako=2 (>1 at 115_01), so only the
  final select ordinal needed changing.

## Fix

Recorder `--choose` indexes branches by ordinal (past-list branches default 0). The played
chain contains exactly 5 selects (branch records in both traces). True-end run:

```
minori-headless ... --choose 0,0,0,0,1
```

Branch records of the true-end run: 继续聊天 / 去看看景 / 去接宫子 / 去看看情况 /
**留在宫子身边** (branch 1) — the recorder's `branch-05.png` shows the select UI with
option 2 highlighted.

## Result (trace + personally read frames)

True-end run trace: 112_01@189061 → 113_01@189146 → 113_03@191747 → 114_01@201009 →
115_01@205974 → **115_02@209924** (favor branch took the true side) → 116_01@217390 →
117_01@223832 → **700_01mugai@227708** → 200_01@229828 (`ef_700=1` set, next tale begins)
→ 201_01/203_01 (run stopped by operator at 256740 to preserve disk).

Frames personally read (parent agent): branch-05.png (select UI, option 2 highlighted),
frame-190500.png (113_01 Miyako apartment, full render), frame-210000.png (115_02 true-path
beach scene), frame-228000.png (700_01mugai epilogue CG, Yuuko), frame-229800.png (200_01
white handoff fade). Ending renders correctly through the tail.

## Independent image audits (3 parallel subagents, 116 images read personally)

- updates 0–66000: 38 frames + programmatic screen of all 221 — **zero failures**
- updates 66000–143000: 38 frames + screen of all 257 — **zero failures**
- updates 143000–193107 (old bad-end run tail): 40 frames, tail sampled 100% — **zero
  failures**; the old run's terminal black frame is the script's own closing fade
  (WaitingTransition → Ready → Finished), not a render fault.
- Frozen-frame check: zero duplicate hashes across all sampled frames; text advances with
  every trace message.

## REA usage (per goal requirement)

REA MCP session opened directly on `ef_first_AA.exe` (`open_binary` succeeded: pe/x86,
SHA-256 verified). Provider string-search calls stalled (same analysis stall as previous
Ghidra attempts), so per the goal's fallback the select/.if handler verification continued
via bounded objdump windows: `.select` name @0x578578 referenced at 0x4cb254, `.if`
@0x5786ac referenced at 0x4cc5c8; handler semantic verdicts recorded in
`ef-re/select-if-native.txt` (subagent task).
