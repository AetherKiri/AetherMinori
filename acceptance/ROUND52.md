# Round 52 — isolate diagnostic platform callback queues

Source inspection found a process-global request queue shared by every Headless instance. A second instance could drain another instance’s message, select, movie or audio requests, or inherit stale queued requests after destruction. Replaced it with an instance-owned boxed mutex queue passed through the existing ABI host user_data field. Its address stays stable when Headless moves; it remains alive while the provider destroy callback runs. The CLI now drains `runtime.take_requests()`. This changes the diagnostic Rust helper API, not the C provider ABI or Aether host.

Regression exercises two separate host callback contexts, distinct payloads, a moved boxed owner, null user_data/operation and null argument. 93 library tests passed, 2 ignored; 1 CLI test passed. Headless build and diff checks passed.

Real-install smoke: 400 loops, delta 120, clicks 80, start 100_01.sc, default j selector, output round52-queue. Exit 0; message 200, WaitingTransition, empty error; 11 message callbacks and 1 audio request. Personally inspected frame-0399.png: transitioning sepia church exterior with readable Chinese narration. This was a short callback/render check, not a full-route or original Windows comparison.

Original command fidelity and actual host validation remain incomplete. The callback isolation bug is source-established; it is not claimed as the cause of the earlier 30k stall.
