# Round 59 — live script choice retry acceptance

Added an opt-in test using MINORI_EF_ROOT and the real installed PAZ assets. Starts 102_01.sc under e selector, drives tick/click until minori_select, submits malformed and out-of-range responses, verifies WaitingSelect remains and provider tick still succeeds, then selects index 0 and submits a duplicate response. Continues until a new nonempty dialogue callback arrives, proving actual post-choice story progress through the reconstructed provider. No fixture scripts replace retail resources.

Explicit local-install test passed in 2.13 seconds. Default suite: 100 passed, 4 ignored (including local-install tests); 1 CLI test passed; diff check passed. This validates the round-58 recovery against an actual script path, not original Windows execution or a full route. No new screenshot was taken, and the historical 30k failure cause is not established. Native parity and host acceptance remain incomplete.
