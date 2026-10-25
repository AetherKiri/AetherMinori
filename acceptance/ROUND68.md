# Round 68 — controlled real-sprite snow motion and stop redraw

Added opt-in MINORI_EF_ROOT scene test: load actual snow sprite via mounted PAZ VFS, start Snow over fixed black scene, verify nonblack particles, tick 60 times at 16ms without other scene changes, verify changed frame, then issue end and require black again. This closes prior confounding from simultaneous stage/camera/dialogue changes, but uses a diagnostic black background rather than a full script scene.

Test found a second renderer defect: end/End/stop dispatch cleared effect state without redrawing, leaving the final particle image visible until another command triggered composition. Test failed on final black-frame assertion. Added composition after clearing states; test passed in 0.09s. No native claim about exact end timing/physics; this enforces existing immediate-clear renderer intent.

Full suite 106 passed, 6 ignored (one additional real-install test); 1 CLI passed; diff check passed. An initial compile error in the new test omitted EffectRequest.third and was corrected. No new screenshot/native execution. Overall native fidelity and host acceptance incomplete.
