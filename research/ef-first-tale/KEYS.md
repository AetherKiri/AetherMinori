# ef - the first tale: recovered key tables

**Current status:** all eight archives now mount. Historical unresolved notes
below are retained as discovery history; audio is resolved in AUDIO.md and movie
index/payload prefix recovery is resolved in MOVIE.md. Movie uses negated key at
0x1a7c00, not the originally assumed index slot 2; password qmzj4Y9C.

Both tables live in `ef_first_AA.exe`, 8 x 32 bytes each, stored **negated**
(`key[i] = (256 - byte) & 0xFF`). Reproduce them with:

```python
d = open("ef_first_AA.exe", "rb").read()
def key_table(offset):
    return [bytes((256 - b) & 0xFF for b in d[offset + i*32: offset + (i+1)*32])
            for i in range(8)]
index_keys = key_table(0x1a7ae0)
data_keys  = key_table(0x1a7bc0)
```

## Verified archive -> slot mapping

For the seven archives whose index parses (key = index_keys[slot]):

| archive | index slot | payload slot |
| --- | --- | --- |
| scr | 0 | 3 (sys) |
| bg  | 1 | unresolved |
| st  | 3 | unresolved |
| sys | 4 | 7 (mov) |
| bgm | 5 | unresolved |
| voice | 6 | unresolved |
| se  | 7 | unresolved |
| mov | 2 (index still fails) | - |

The mapping is rotated relative to the literal in-file order
(`scr bg st sys bgm voice se mov`), which is why a naive slot==archive
assumption fails.

## Type passwords

```
png 36TXHE5N      ogg oRQCAU1o      sc 102ycz1s      avi hYPH3Fxw
```

They are used as the third field of the RC4 key text:

```
"<lowercased entry name> <unpacked_size:08X> <password>"
```

encoded as Shift-JIS, applied after the Blowfish payload decrypt and before
inflate (for `type == 1` entries the RC4 layer is skipped).

## Status

- `scr.paz`: all 75 scripts extract correctly with index slot 0 + payload
  slot 3 + the `sc` password. 25,838 lines of script recovered.
- `bg`, `st`, `bgm`, `voice`, `se`: index opens, payload key not yet
  identified. `WHITE.png` (bg.paz, offset 143164) is a clean oracle -- it must
  decrypt to `\x89PNG\r\n\x1a\n` -- but none of the 16 recovered keys, in
  either endianness, with or without the RC4 layer, produces it.
- `mov.paz`: index does not parse with slot 2; the movie archive carries an
  extra 256-byte key block after the entry count (GARbro's `MovPazArchive`),
  so it needs its own path.

## Resolved (round 2)

All four image/script archives now decode. The archive-to-slot rotation is a
literal `+3` shift over the file order `scr bg st sys bgm voice se mov`:

| archive | index slot | payload slot |
| --- | --- | --- |
| scr | 0 scr | 3 sys |
| bg  | 1 bg  | 4 bgm |
| st  | 3 sys | 6 se  |
| sys | 4 bgm | 7 mov |

Signature entries verified end to end: `000_01mugai.sc` (text), `WHITE.png`
(2309 B), `st_yuko13g05.png` (999x600 sprite), `topmenu.png` (800x600 title).

The type password for ef is **per-archive**, not per-extension: `a7b77qgx` is
what every encrypted image entry uses. `102ycz1s` is the script password.
The earlier audio failure was due to bounding the data-key table too early,
not a different cipher. Audio is now resolved (see below); `mov.paz` remains separate.

## Resolved audio (round 3)

All audio uses negated 32-byte keys beyond the original eight-slot boundary:

| archive | index slot at 0x1a7ae0 | payload slot at 0x1a7bc0 | payload key file offset |
| --- | ---: | ---: | ---: |
| bgm | 5 | 8 | 0x1a7cc0 |
| voice | 6 | 9 | 0x1a7ce0 |
| se | 7 | 10 | 0x1a7d00 |

```text
bgm   8B7731E7AA9DAE4C1F216750C32B9D395794B257C353A3C9DFA880733026D32B
voice 151694FDCF2CE124A14E6D1E46E1A7B17395C0CC172DA47CA3AD8CAE42043258
se    3DCA17D9518DE518087077409C8BCB396A6BE53DA76E650238E09905096D1AA8
```

Password: **52AUwF8B** for all three. LE Blowfish ECB, then standard RC4
with zero skip. Key text uses lowercase original index name and unpacked
size: `<name> <size:08X> 52AUwF8B`. Do not append `.ogg` to voice names.

`python3 research/ef-first-tale/audio_probe.py --verify` validates one complete
Ogg/Vorbis stream per archive entirely in memory, including every page CRC,
exact extent, BOS and EOS. Verified: BGM `efBGM056.ogg` 1616925 bytes/382 pages;
voice `yuk-701_01-0051` 47347 bytes/13 pages; SE `zidouhanbaiki.ogg`
12097 bytes/5 pages. See [AUDIO.md](AUDIO.md) for decoded SHA-256 hashes and
probe coverage. No mass extraction or playback test was performed.
