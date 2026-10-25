# Round 55 — isolate provider media cache paths

Source-established collision: audio filenames contained only PID and slot, movie filenames PID and movie ID, and message icon used a fixed name. Two runtimes sharing the host cache directory could overwrite one another; clearing one audio/movie could delete the other runtime’s media.

Added a process-local atomic runtime ID to private Runtime state and included it (with PID) in audio, movie and icon cache filenames. Provider C ABI is unchanged; hosts still receive explicit paths. This isolates concurrent runtime instances within a process, not a hardened filesystem/security boundary or asynchronous playback synchronization fix. Same-runtime slot replacement semantics are unchanged. Icon cache cleanup behavior is unchanged.

Regression writes two runtime-specific files for audio/movie/icon suffixes in a shared temporary directory, deletes the first and verifies the second remains unchanged. 98 library tests passed, 2 ignored; 1 CLI test passed. Build and diff checks passed.

Real-install chapter smoke with e selector and start 100_01.sc, 400 loops: exit 0, message 270 WaitingInput, empty error, 5 audio requests. Callback output confirms the unique icon path. Personally inspected frame-0399.png: character, background and Chinese dialogue visible. This is not audible playback, dual live-runtime integration, full movie validation or original retail comparison. Overall fidelity and actual host acceptance remain incomplete.
