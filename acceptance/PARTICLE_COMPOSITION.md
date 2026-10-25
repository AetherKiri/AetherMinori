# Particle animation composition repair

Read-only independent audit identified Snow/SnowH state creation and tick_effect calling render_panel_frame, which omitted particles and screen/overlay/frame layers. Full refresh already included these. A deterministic one-white-pixel stationary particle regression reproduced the bug: white after full refresh, black after tick_effect(16), exit 101.

Changed animation redraw helper to delegate to existing full refresh. This preserves layer and panel composition and active transition destination handling without inventing native particle physics. Regression now checks repeated ticks keep the white particle and clearing the effect restores black. Full suite 102 passed, 5 ignored; 1 CLI passed; build/diff checks passed.

Actual 108_01.sc run, j selector, 600 updates/16ms/6 clicks: exit 0, message 130 WaitingInput, effect Snow, no runtime error. Personally inspected acceptance/snow-active/frame-0599.png: white background and readable Chinese narration; white particles cannot be visually distinguished here, so this screenshot is NOT visual snow proof. The earlier 180-update smoke had not reached Snow and is not counted as particle acceptance.

The real corpus contains Snow at 108_01.sc:8 and Snow/SnowH in 000_01mugai.sc. This repairs omission of the existing renderer, not native timing/count/physics/blend equivalence. Further live dark-background particle acceptance and native fidelity remain outstanding.
