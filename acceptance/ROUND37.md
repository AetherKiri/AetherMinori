# Round 37 — recheck visual acceptance and retain retail identity evidence

## Image review

Personally reopened two preserved headless-route captures. `round30-replay/frame-96000.png` shows an illustrated school scene with two characters, Chinese dialogue, and the translucent text panel; this is not a flat/lavender frozen image. `round30-replay/frame-181064.png` is black at the route’s terminal iteration. This is consistent with the recorded VM route ending, but an isolated final black frame does not establish the retail ending or explain all black frames. These are prior headless captures, not freshly generated screenshots or an Aether host capture.

## Retail identity

A fresh REA `file_headers` observation confirms the installed artifact digest remains `9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e`, PE format `pei-i386`, i386, and start address `0x0046825d` (`ev_a4aa3e0f73a15e0bb8e93a50f717fd4a524550b07625da2942476144ca8169ed`). Exported to `/var/lib/dsh/Minori/ef-re/round37-evidence.json` and reimported after closing the target; its retained bundle summary reports the complete record. This is image-identity evidence only, not game behavior.

## Status

No production code changed and no source-level parity claim is made. The existing round-35 provider lifecycle change remains tested only at provider unit level. The Aether host project/source and Wine are not available in this workspace, so its actual startup, frame presentation, host pause integration, and Windows retail comparison remain unverified. The user-confirmed cinematic black bars remain intentional and unchanged. Acceptance goal stays active.
