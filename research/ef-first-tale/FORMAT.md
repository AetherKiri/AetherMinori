# ef - the first tale: PAZ archive and script format

Verified against every archive in the retail Chinese build.

## Container

    header        0x24 bytes
      [0x08]      XOR key k (0 when the index is not XORed)
      [0x20]      u32 index size; every byte is XORed with k, so the high byte carries k
    index         XOR k, then Blowfish-LE (32-bit words byte-swapped within each block)
      u32 count
      per entry:  name\0 (Shift-JIS)
                  u32 offset
                  u32 data_key      (always 0 in this build)
                  u32 unpacked_size
                  u32 packed_size
                  u32 stored_size
                  u32 type          (1 = zlib, 0 = raw)
    payload       read stored_size rounded up to 8 bytes, XOR k, Blowfish-LE,
                  then RC4 for .png/.ogg/.sc/.avi and for the audio archives,
                  then inflate when type == 1

RC4 key text: `<lowercased name> <unpacked_size:08X> <type password>`

## Keys

Two tables of 8 x 32 bytes live in `ef_first_AA.exe`; both are stored **negated**
(`key[i] = 256 - byte`).

| table | file offset | use |
| --- | --- | --- |
| index | 0x1a7ae0 | PAZ index Blowfish |
| data  | 0x1a7bc0 | PAZ payload Blowfish |

File order is scr, bg, st, sys, bgm, voice, se, mov. The archive -> slot mapping is
rotated relative to that order; the slots that actually open each archive are:

| archive | index slot | data slot |
| --- | --- | --- |
| scr   | scr   | sys   |
| st    | sys   | sys   |
| sys   | bgm   | mov   |
| bgm   | voice | voice |
| voice | se    | se    |
| se    | mov   | mov   |
| bg    | bg    | unresolved |
| mov   | unresolved | - |

Type passwords: png `36TXHE5N`, ogg `oRQCAU1o`, sc `102ycz1s`, avi `hYPH3Fxw`.

## Script corpus

75 scripts, 25,838 lines. Commands observed:

    message transition stage panel playBGM wait playSE screen overlay effect
    chain endScroll label set vscroll end scroll pragma frame if hscroll
    shakeScreen setGlobal HSVfilter playSE2 playVoice goto movie select

`screen`, `overlay`, `frame` and `HSVfilter` are **not** in the Supipara/Trinoline
command registry: they are ef-era additions the runtime still has to implement.
The `[e]` / `[j]` line prefixes are per-language variants (English / Japanese)
that the VM must select between, not skip.
