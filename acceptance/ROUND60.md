# Round 60 — save dialog response transaction recovery

The text_input response handler consumed pending_save_slot before validating its payload and permanently latched failure on malformed or duplicate replies. A malformed response therefore lost the dialog transaction and made a corrected reply impossible.

Now validates before consuming the slot; malformed replies return INVALID_ARGUMENT without setting failed, preserving the slot for retry. Cancellation consumes the slot and clears its diagnostic. Unsolicited/duplicate replies return INVALID_STATE without poisoning the VM. Valid save completion still consumes the slot and follows existing save behavior. This is host protocol recovery policy, not original retail dialog parity.

ABI regression verifies malformed reply retains slot 4, valid cancellation clears it and diagnostic, and duplicate cancellation is rejected without fatal failure. Full suite: 101 passed, 4 ignored; 1 CLI test passed; diff check passed. No fresh screenshot or original runtime capture. Full original behavior and actual host acceptance remain incomplete.
