# Round 54 — prevent lossy manual-save control-flow snapshots

Source-established bug: VM manual_state copied the next program counter but not pending state; from_manual_state always restored WaitingInput. Saving during WaitingSelect would lose branch target labels and resume after the select command. PendingChain and animation/timer waits likewise were not representable in this envelope.

Scoped safety fix: snapshots are accepted only at WaitingInput, the boundary this format actually restores. Included-script snapshots remain unsupported as before. This deliberately rejects unsupported saves rather than writing a slot that silently changes story flow. It does not implement original save compatibility or save-anywhere semantics; the provider currently surfaces save errors using its existing error route. Previously written unsafe slots are not repaired.

Regression enumerates every unsupported VM state (including choice, chain, timer, movie, transition and movement), checks rejection without changing state/PC, and confirms WaitingInput acceptance. Existing actual-select test now rejects a snapshot before choosing option 1 and still reaches the correct branch. Existing message-boundary snapshot/restore roundtrip remains passing.

Full suite: 95 library tests passed, 2 ignored; 1 CLI test passed; diff check passed. No new visual rendering or retail execution claim; this is control-flow/save safety validation. Native command fidelity, host validation and unsupported save-state serialization remain incomplete.
