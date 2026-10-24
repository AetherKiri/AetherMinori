# Integration audit: title ownership and profile duplication

Source audit, 2026-10-24. This records the pre-global-removal baseline inspected in this task, not an assertion that subsequent parent-agent edits have already shipped. No REA or game execution was used.

## Current cleanup status

After the baseline audit, the parent authorized global title removal. The runtime now has no `TitleProfile`, title resource/rectangle fields, `MainMenuAction`, title constructor/hit dispatch or `SceneKind::MainMenu`. All supported profiles use `script_only` and unconditional `start_new_game` after existing global-state restoration. The provider ABI table and its `get_main_menu_json: None` field remain intact. In-game save/load, menu assets and layout are retained. This fulfills **global runtime title UI removal**, not complete elimination of repeated non-title defaults or original-player parity. A new three-profile scene test checks generic startup without title assets and correct canvas size.

## Verdict at baseline

**The requested global title-UI removal was not fulfilled.** Only ef opts out: `EF_PROFILE.ui.title = None`. Supipara and Trinoline still have `Some(TitleProfile)` with artwork and New/Load hit rectangles. The generic title implementation and input routes are active, not merely unused declarations.

- [Profile declarations and selection](<../../src/profile.rs>): `TitleProfile`, `UiProfile.title`, `GameProfile.has_title_screen`, both non-ef title resource blocks and all three game profiles remain.
- [Scene implementation](<../../src/scene.rs>): `SceneSystem::main_menu` loads the selected title PNG into `SceneKind::MainMenu`; `main_menu_action` hit-tests New/Load. `script_only` instead starts with a black frame in `SceneKind::Script`.
- [Provider lifecycle](<../../src/provider.rs>): `open_game` selects the title constructor when `has_title_screen()` is true, otherwise creates the script scene and calls `start_new_game`. The latter loads `minori.start`/`start_script` if set, else the profile entry script; it restores persisted globals into the new VM. Pointer input dispatches title New/Load before in-game controls.

The comment that ef has no title in its script corpus is a source-level rationale, not a new exhaustive corpus or original-engine proof from this audit. Bypassing a title is a deliberate integration policy, not recreation of its original behavior.

## Redundant profiles: partial consolidation, not elimination

There is one shared `SUPIPARA_GAME_MENU_ITEMS` slice used by all three games. Nonetheless each game repeats `GameMenuProfile` background/margins/trigger settings; Supipara and Trinoline repeat identical message-wait settings and panel settings. Several save-layout fields repeat too. The `UiProfile` comment claiming the duplication “used to” exist overstates the achieved cleanup.

Do not collapse everything into one profile: archive executable markers, archive/index/data keys, per-extension versus per-archive password scheme, RC4 skip, canvas dimensions and work-specific save artwork/geometry differ. ef uses 800x600 rather than 1280x720; identical resource strings do not prove identical artwork or geometry. Existing `ArchiveProfile`/`GameProfile` distinctions remain functional.

## Smallest proposed global title-removal patch

The parent subsequently authorized this focused production patch. It has now been implemented as described below; this section preserves the reasoning behind the change.

1. Delete `TitleProfile`, `UiProfile.title`, title resource/rectangle blocks and `has_title_screen`. Keep game identity/archive/startup/canvas and non-title UI fields.
2. Always construct `SceneSystem::script_only` during `open_game`; call `start_new_game` after the existing reset/system-global load for every selected profile. Preserve language/start-script options and error propagation.
3. Delete title-only scene constructor/action/enum, title pointer dispatch and stale title debug/test states. Check every `SceneKind::MainMenu` reference before removing the variant; load/save overlay operation must not depend on it.
4. Preserve in-game save/load constructors, slot hit-testing, thumbnails, save-name request/response and persistence paths. Title removal must not remove the only retained load route.
5. Add regressions that every supported profile opens directly into script execution, no title artwork is required, entry override works, and in-game Save/Load still works. A later separate deduplication can factor identical non-title defaults without flattening genuine per-game differences.

## Preservation boundaries and remaining fidelity gaps

These are concrete limits, not a claim of exhaustive missing-feature discovery:

- **Save compatibility:** [provider save/load](<../../src/provider.rs>) uses the host writable directory, `save/slot-01.sav` through slot 10 and `system.dat`; [persistence](<../../src/persistence.rs>) uses the runtime-specific `MINORISV` manual-save envelope. Do not migrate/delete/relabel these in a title cleanup. A matching original save format has not been established here. Legacy slots without a resumable PC cannot resume; [VM manual state](<../../src/vm.rs>) rejects saves during included scripts.
- **In-game controls:** provider consumes QuickSave, System and Skip menu actions with `OK` without implementing those actions in that dispatch branch. Save/Load have actual routes. Keeping the painted menu is not proof all its buttons work like the original.
- **Lifecycle:** the round-35 follow-up replaced the original no-op `unary` pause/resume callbacks with runtime pause state. `tick` and `send_input` return without advancing while paused; opening a game clears the paused state. A provider unit test covers pause/resume, paused tick, and null-handle rejection. `set_surface_size` still only validates nonzero dimensions. Host scheduling/scaling behavior was not exercised. `destroy` clears movie/audio then drops the runtime; it is not an implicit promise to save VM progress.
- **Host integration:** provider menu-enumeration/activation callbacks are `None`. The host-library-launcher comment alone is not evidence that chapter select, original configuration, extras or title-load functionality have host replacements. No GUI smoke test or original/native UI comparison was performed.
- **ABI:** preserve the C-compatible provider layout, `API_VERSION = 0x0100_0000`, runtime ID `minori`, function-pointer table, reserved slots and `aetherkiri_minori_register_runtime_provider` export in [provider](<../../src/provider.rs>). This is the existing host ABI, not proof of equivalence to the original retail engine ABI.
- **Audio:** [audio findings](<AUDIO.md>) report full Ogg-page CRC verification for one BGM, voice and SE stream plus bounded prefixes, not all entries or original/host playback parity.
- **Movies:** [movie findings](<MOVIE.md>) report first-64-KiB checks of all four movies and full small-logo codec decoding. Full AVI integrity and real-time host synchronization are unproven; the logo has a terminal MP2 diagnostic and exact original-player handling remains unknown. No production trimming is justified by the audit.
- **Control flow:** [conditional findings](<CONTROL_FLOW.md>) and [chain findings](<CHAIN_NEXT.md>) supply bounded original static evidence for true-branch `.if` and normal `.chain` local retention. They do not prove all commands, state restoration, cleanup hooks or complete routes match.

## Changes and validation in this audit

- Corrected [README](<../../README.md>) identification (removed the unrelated WHITE ALBUM2 example and misleading external Minori link), and documented partial implementation/global-removal baseline.
- With explicit parent permission, added the missing `ui.title.is_none()` and `!has_title_screen()` assertions to the existing ef title-named profile test. Previously its name was not backed by any title assertion. No production behavior, keys, save data or ABI was changed by this audit. Parent may replace these assertions with all-game startup tests when deleting the title type.
- Initial `cargo test --lib profile::tests -- --nocapture` attempt exited **127**, `cargo: command not found`. Parent supplied the installed toolchain path `/var/lib/dsh/.cargo/bin`; rerunning with that directory in PATH succeeded: `cargo test --all-targets`, exit **0**, **86 library tests passed, 2 local-installation tests ignored; 1 headless CLI test passed**. This includes the new all-profile script-scene test and existing save-grid/radial-menu/save-round-trip tests. It does not exercise provider `open_game` against all three real installations or constitute an end-to-end GUI save/load smoke test. `git diff --check` passed.
- Metadata-only check confirmed the existing installation directory `/root/games/ef-first-tale/ef - the first tale` and its executable (2,048,104 bytes). No assets were extracted and no installation content was modified. Download/removal evidence was not investigated: it is unnecessary to decide the source-level title-removal question, and no deletion claim is made.
