# Round 58 — choice response retry safety

Provider previously latched fatal failure for malformed, out-of-range or duplicate minori_select host responses. The VM itself rejects these without consuming choice labels or changing PC, so a host retry could select successfully but subsequent provider ticks remained permanently blocked.

Choice response errors now retain return codes and diagnostics without setting the fatal latch; a valid selection clears its prior diagnostic. No C ABI change and no claim that retail has this callback interface.

ABI regression submits malformed index, out-of-range index, valid option 1 then duplicate callback. Confirms waiting state preserved through invalid responses, failed remains false, valid response clears error, and destination assignment result=2 executes after the duplicate rejection. Full suite: 100 library tests passed, 3 opt-in tests ignored; 1 CLI test passed; diff check passed. No fresh screenshot or original execution performed; overall original fidelity and host acceptance remain incomplete. This is a source-established potential choice freeze, not proof of the historical 30k failure cause.
