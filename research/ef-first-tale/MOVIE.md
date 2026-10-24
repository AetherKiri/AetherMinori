# ef first tale movie PAZ: index and bounded headers resolved

## Recovery

The old assumption that mov uses index slot 2 at 0x1a7ae0 is false.
The correct 32-byte index key is negated executable bytes at **0x1a7c00**:


Key hex: 5a4caf2034cc9510780ce2b9bae97b9c2026274e7fdd36c58d10325f0337e1ea

The bounded discovery scanned offsets 0x1a7800..0x1a8200 every four bytes,
32-byte keys, raw/negated/complemented forms. Only 0x1a7c00 negated produced
a sane count and all four valid directory records. This address also overlaps
the previously named data-key region: table labels were inferred, not authoritative.

mov.paz index size is 424 bytes at file offset 0x24; XOR byte is zero.
LE Blowfish decoding gives count=4, then a **256-byte movie key table**,
then standard name + QIIII records. Records consume 420 bytes; four zero pad bytes.
Table SHA-256: d84de26a7348c68eeffe7f380937709fb2e87e46af8b8644779c09bd502da2fb.

| Entry | Offset | Unpacked/stored bytes | Aligned bytes |
|---|---:|---:|---:|
| op_logo.mpg | 460 | 446468 | 446472 |
| ef_first_sound.avi | 446932 | 39668126 | 39668128 |
| ef_ED_sound.avi | 40115060 | 10658946 | 10658952 |
| ed_moji_test.avi | 50774012 | 229440 | 229440 |

All packed flags are zero. Password **qmzj4Y9C** was uniquely identified by
matching MPEG pack/AVI signatures among executable raw/negated/complemented
8-character alphanumeric string candidates (including empty and old assumptions).
Entry key text: lowercased original name + space + uppercase 8-digit hex unpacked
size + space + password, encoded Shift-JIS.

No payload Blowfish is used. XOR the 256-byte index table with the repeating
entry-key bytes, use that mixed table as standard RC4 key with zero skip,
generate 65536 keystream bytes and repeat that block over the payload.
This is the existing VFS movie algorithm and matches the version>=1 path in
[GARbro ArcPAZ.cs](https://raw.githubusercontent.com/morkt/GARbro/master/ArcFormats/Musica/ArcPAZ.cs).

## Bounded proof

Run python3 research/ef-first-tale/movie_probe.py. It reads the index and only
65536 encrypted bytes per movie, does not write assets, and prints these decoded
prefix SHA-256 values:

| Entry | SHA-256 of first 65536 decoded bytes | CRC32 |
|---|---|---|
| op_logo.mpg | 2f6e0eca4fbdc1e758893bae181187dffcabfe7845aced7bb83a79a3b9019e51 | 3ffaaa25 |
| ef_first_sound.avi | 9bf59dcbfef576b83352e536b0e128db45fd10d1f877bf54bfd61823c9937bf5 | bf874704 |
| ef_ED_sound.avi | db3c1348206d4b3afe6f9ff9f698507adffefefc393f6eada961fc2b04a645b2 | d658d340 |
| ed_moji_test.avi | ca64f9bdbbd428f1e690a6b95a33c28418a2918057bbbc691da3518dce5b00f6 | b3b8983b |

All AVI prefixes have RIFF length+8 equal to index unpacked size, AVI form,
LIST/hdrl header. MPEG prefix has pack BA and system BB start codes.
The opt-in local Rust test checks these structures and independent Python CRCs
through the production profile and PazArchive::read_movie_prefix API:


env EF_GAME_ROOT="/root/games/ef-first-tale/ef - the first tale" cargo test --lib ef_movie_local_headers_match_independent_probe -- --ignored

The synthetic regression checks 256-byte table placement, truncation rejection,
per-archive key text, 64-KiB repeat, short prefix and empty decode.
The production ef profile now mounts all eight archives including mov.

## Scope and residual unknowns

This step proves directory decoding and the first 64 KiB of all four payloads,
not complete movie integrity, codec decode/playback, end-of-file validity,
or real-time host integration. Keystream repetition past 64 KiB is covered by
synthetic regression/reference, not complete shipped movie validation here.
No movie extraction or REA lifecycle was performed. Packed movie prefixes
are explicitly rejected; none occur in this installation.
