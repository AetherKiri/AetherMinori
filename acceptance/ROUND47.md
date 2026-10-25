# Round 47 — language selector route replay

Ran the main route again with `--language e`, isolated in `acceptance/round47-english/`. It completed at loop 164,809 with VM `Finished`, `favMiyako=2`, `favMiyako_a=1`, `finderCounter=2421`, no error, 7 select, 5 movie and 13,647 message requests. The late route screenshot (loop 96k) was personally inspected: two characters, school background and visible Chinese dialogue. The terminal screenshot is fully black, matching the completed primary headless run.

Important finding: in the existing decoded Chinese-selected PAZ installation, `--language e` still produced the Chinese dialogue seen in the frame and message callback. The VM language option is accepted, but this run did not validate a genuinely English/localized asset set. The executable chains through the same script route as default; does not prove English localization parity. Existing language source notes say ef variants can select prefixed lines and differ in pacing, but captured result is direct evidence this installation/run remains Chinese.

Validation only covers headless provider callbacks. No Windows title/GUI/runtime comparison or Aether host screenshot; Wine/Godot and host project files remain absent. Exact command parity, complete routes, and host audio pause behavior remain unverified.
