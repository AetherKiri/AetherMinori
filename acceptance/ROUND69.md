# Round 69 — saved layer survives post-load recomposition

Added opt-in local-installation test using mounted ef VFS and the actual restore_manual_state path, with a diagnostic full-canvas layer sentinel. Saves through bincode, removes live layer and redraws, restores snapshot, redraws again and asserts exact sentinel pixels and retained layer count. This distinguishes restoring a flattened screenshot from correctly restoring independently recomposable layer state. No character assets are loaded in this sentinel test; installation is used to enter the real restore API.

Explicit test passed in 0.09s. Default suite 106 passed, 7 ignored; 1 CLI passed; diff check passed. No production change, new screenshot or original execution. Independent read-only audit for another substantive scene defect is running. Overall native fidelity, dynamic effect save state and actual host acceptance remain incomplete.
