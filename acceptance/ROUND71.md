# Round 71 — initial stage and transition composition consistency

Audited apply_stage after animation composition fixes. It independently composed only characters/panel, omitting active particles and screen/overlay/frame layers from initial target and transition.new_rgba. Those states stayed active and appeared on a later ordinary redraw, causing inconsistent frames.

Actual-asset stage regression with a diagnostic frame-layer sentinel failed (asset pixel white instead of sentinel), then passed after extracting compose_current_scene and sharing it between full redraw and stage target construction. Test covers immediate stage frame and active transition destination. This enforces current layer persistence contract, not newly recovered original lifecycle/order.

Explicit test passed 0.15s; default suite 107 passed/9 ignored plus 1 CLI passed; diff check passed. An ambiguous literal edit was rejected without applying that replacement; inspected partial factoring and completed targeted edit before compiling. No fresh screenshot/native runtime capture. Overall objective incomplete.
