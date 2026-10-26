# Blink + Mouth (Lip-Sync) — Native Fidelity Record

Source: static analysis of `ef_first_AA.exe` (SHA-256 9824732313d13136cb2300f33038a655ba480ae01cefc2749028d1b1748b4f9e),
reports in `ef-re/blink-mouth-native.txt` + `ef-re/blink-mouth-windows.txt` (24 raw objdump windows).

## Verdict

- **Mouth / lip-sync: already exact.** Native `LipAnimater` 0x52CBF0 matches the Rust
  model field for field: open threshold `cmp eax,0x320` @0x52CC16, close from state 1
  when amplitude < half-of-previous (`shr eax,1`), two-step close 2→1→0, states exactly
  0/1/2, voice-packet dedupe by id. No change needed.
- **Blink: previous fixed 2800ms/100ms timer was wrong and is replaced** (commit 603da1b).
  The binary contains no such constants (full dword scan). The native `EyeAnimater`
  player 0x52CAB0 plays frame-count rhythm tables with RNG jitter.

## What the runtime now implements

1. **Six rhythm tables** read from .rdata 0x573E68..0x574098, entries
   `{image_id, duration_frames}` (0=open, 1=mid, 2=closed, -1=wrap), key 2 default:
   - key 0: 40/40/80/40 ×3 holds 400/3600/1200, blink 40/80/40
   - key 1: 10/20/40/20 ×3 holds 200/1800/600
   - key 2 (default): 10/10/20/10 ×3 holds 100/900/300
   - key 3: 5/5/10/5 ×3 holds 100/450/150
   - key 4: 3/3/7/3 ×3 holds 33/225/75
   - key 5: fixed {0,30}{1,30}{2,30}{1,30}{-1,0} loop
2. **Frame-count clock**: native dispatch (thunk 0x52B780) advances the counter by
   exactly 1 per engine frame; durations are frames, mapped at the assumed 60fps
   (loop rate = explicit unknown 10.1).
3. **MSVC rand() LCG** exactly as at 0x466A6A (imul 0x343FD, add 0x269EC3, >>16,
   mask 0x7FFF). One deterministic stream per character (native shares one TLS
   stream; per-character seeding keeps runs reproducible).
4. **Wrap jitter**: every wrap to the first open hold gains rand()%500+50 frames
   when the table key < 5 (the native gate reads the key field itself at +0xA4,
   which also explains key 5 being the fixed no-jitter loop).
5. **Ctor reset** (0x52C8B0 semantics): open eye immediately, initial hold
   rand()%300 frames.
6. **Hard swap, no blend**: ApplyEntry 0x52C250 and helpers 0x510290/0x5101C0 all
   invoke the image-apply virtual with blend=0; there is no crossfade/alpha
   anywhere. Our overlay draw already hard-composites — aligned.

## Documented approximations / unknowns

- **Mid frame (id 1)** draws the closed-eye overlay: the reimplementation carries
  only open/closed eye slots, while native tables reference three eye images via
  the ANI archive name registry.
- **EYE_SEQ_DEFAULT override** (registry 0x5AA154, loaded from system.dat at
  0x4C0440): if the game data defines it, its frames replace the built-ins at
  runtime. Contents are .sqz data, unresolvable from the exe alone (explicit
  unknown 10.2). Not implemented.
- **Ctor init vs reset**: the ctor tail calls vtable+0x10 (0x52B9C0, body unknown
  10.6); we adopt the reset semantics (rand()%300) for loading.
- **`[noblink:`** is parsed natively (tokens 0x5749D8/0x5749E0, consumers
  0x504E1F/0x504E37) and our `blink=false` gate disables the animater, but the
  native flag-to-field linkage (one-shot stop bytes +0x98/+0x99) is not fully
  traced (unknown 10.3).
- **ANI structure**: native .ani is an image archive (records = name +
  {w,h,bpp,xOff,yOff} + pixels, loader 0x53E010); the five animation image names
  arrive from the `.char` command (format ".char %s %d %d %d %s %s %s %s %s").
  Our `parse_mouth_animation` reads the ANI as a slot manifest instead. The
  corpus's 75 scripts contain no `.char` call (characters stage via `.stage`
  extras), so this pipeline difference is latent on this corpus; wiring the
  five-name `.char` path and a real ANI archive loader remains open work.

## Test

`blink_player_follows_native_rhythm_tables_and_lcg_jitter` drives the player one
60fps frame at a time and asserts the exact key-2 schedule (reset hold →
{0,10}+jitter → {1,10}{2,20}{1,10} → plain 100 hold) against the same recovered
LCG. Full suite: 140 passed, 0 failed, 15 ignored.
