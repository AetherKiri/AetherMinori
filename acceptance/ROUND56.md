# Round 56 — rejected saves must not freeze gameplay

Follow-up to the safe snapshot restriction: provider text-input save completion routed any save rejection through Runtime::fail, setting the permanent failed latch. A rejected save at a choice could therefore freeze all subsequent ticks even though the VM state remained valid.

Separated error reporting from fatal failure. Manual-save errors still return IO_ERROR and expose a readable diagnostic but do not set or clear the fatal latch. Other fatal error routes remain unchanged. This is reconstruction error-recovery policy, not a claim about original Windows dialogs.

ABI regression uses a real VM at WaitingSelect plus an ef script scene, submits a text-input save response, checks the exact unsupported boundary error and verifies failed remains false. The pending save request is consumed and a subsequent minori_select response succeeds, returning VM to Ready. A preexisting fatal latch is not cleared by recoverable reporting. Tests: 99 library tests passed, 2 ignored; 1 CLI test passed; diff check passed. No visual or original executable execution performed for this error-state regression. Actual host acceptance and full native command fidelity remain incomplete.
