# Stage foreground retained during camera scroll

Independent read-only audit found viewport-space base_spec overlays were drawn on initial stage application and retained in effect_layers.foreground, but omitted after tick_scroll replaced background_rgba with a camera crop of background_source. WScroll already reapplies these layers. Source-contract defect reaches real corpus 100_01.sc stage past_window.png followed by e/j vscroll, and 000_01mugai.sc vista_window2.png followed by vscroll. No new native rendering-order claim.

Deterministic regression installs a foreground sentinel and moves camera: before fix first pixel became black instead of sentinel color (exit 101). Camera crop now reapplies foreground at viewport coordinates before normal composition. Regression passes, full suite 107 passed/7 ignored plus 1 CLI passed, diff check passed. No fresh screenshot yet; actual-window asset regression and clear_effect equivalent crop path remain to check. Overall objective incomplete.
