# Round 41 — complete route replay and terminal screenshot review

## Full headless route

Ran a new replay against the installed game data with 240,000-loop budget, fixed route choices, 16,000-loop checkpoints, and 48,000 scheduled clicks. The process exited successfully at loop 181,064. Trace ends `181064	none	690	Finished`; provider debug reports `state=Finished`, `favMiyako=2`, `favMiyako_a=1`, `finderCounter=2421`, and empty error. It records five movie requests, seven select requests, and continued messages/audio requests. This is a fresh, deterministic headless route result, not proof of the Windows executable’s behavior.

Thirteen PNG checkpoints were emitted through the terminal frame. I personally inspected the 96,000-loop image (two characters, illustration, Chinese dialogue) and the 176,000-loop image (a different close-up character scene and dialogue). Their distinct visuals substantiate continued image/story changes after the previously reported 30,000-loop threshold in this backend. I also inspected terminal `frame-181064.png`: it is fully black, with no text or characters. The trace says `Finished`; this matches the runtime’s observed final black frame, but does not confirm that the original retail ending is visually identical or establish why retail should render black. The user-confirmed earlier cinematic bars are intentional and untouched.

## REA and source changes

REA `file_headers`/`section_headers` continue to observe the installed PE identity and file-backed sections; identity digest remains `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`. No new code changes were made. A fresh targeted Ghidra text-drawer analysis was not repeated this round; earlier exact-address decompilation timed out.

## Acceptance still open

This verifies only the existing headless provider path and one branch/route. The Aether host project and Wine/Windows execution are unavailable. Host launch, renderer screenshot, title behavior, Windows retail comparison, full route/choice coverage, exact full-command parity, and audio pause/resume integration remain unverified; the goal is not complete.
