# ef first tale audio: resolved offline

The recovered key table was prematurely bounded to eight slots. Audio uses the
next three 32-byte negated keys after that boundary. Index and payload processing
are ordinary little-endian Blowfish ECB, then RC4 with no discarded bytes.

| Archive | index slot from 0x1a7ae0 | data slot from 0x1a7bc0 | executable data-key offset |
|---|---:|---:|---:|
| bgm | 5 | 8 | 0x1a7cc0 |
| voice | 6 | 9 | 0x1a7ce0 |
| se | 7 | 10 | 0x1a7d00 |

Keys are negated bytewise (`(-b) & 255`):

```text
bgm   8b7731e7aa9dae4c1f216750c32b9d395794b257c353a3c9dfa880733026d32b
voice 151694fdcf2ce124a14e6d1e46e1a7b17395c0cc172da47ca3ad8cae42043258
se    3dca17d9518de518087077409c8bcb396a6be53da76e650238e09905096d1aa8
```

All three use password **52AUwF8B**, not the image password or previously assumed
extension password. RC4 key text is `<lowercase-original-index-name> <unpacked-size:08X> 52AUwF8B`.
Voice names must remain extensionless when generating that text.

## Verification

`python3 research/ef-first-tale/audio_probe.py` reproduces the bounded discovery
matrix: first three 64-byte prefixes per archive, then a larger matrix over one
prefix per archive. It tests 374 candidate passwords from the executable raw,
negated and complemented byte views, nearby aligned 32-byte key candidates,
LE/BE Blowfish, name variants, and zero/CRC32-based RC4 skip.

`python3 research/ef-first-tale/audio_probe.py --verify` reads only one full stream
per archive in memory (largest 1,616,928 encrypted bytes), verifies every Ogg page
CRC with polynomial 0x04c11db7, exact stream extent, BOS and EOS. No audio files
are written. Verified results:

| Entry | decoded bytes | Ogg pages | decoded SHA-256 |
|---|---:|---:|---|
| bgm: efBGM056.ogg | 1616925 | 382 | 2d0eddef697857a74dcb8df6e0e3f7ce5ae556bef2b7d6157c29ef5dc0058c76 |
| voice: yuk-701_01-0051 | 47347 | 13 | f0a10677ca214e21d4ecb4ad12aa6944fa2267722b87eb9b5473fc1593355344 |
| se: zidouhanbaiki.ogg | 12097 | 5 | 1d8f857b8223491660631bbd8819f32e904e81f20e5576f90e3fea3b0448522c |

All start with OggS and contain a Vorbis identification packet. CRC validation
is stronger than a four-byte magic match; codec playback was not tested.
Archive indexes contain 29 BGM, 5830 voice, 89 SE entries; XOR byte is zero.
No claim that every audio entry has been decoded: only the three complete
streams above and bounded candidate prefixes were exercised.

GARbro reference inspected:
https://raw.githubusercontent.com/morkt/GARbro/master/ArcFormats/Musica/ArcPAZ.cs
It confirms ordinary Blowfish then RC4, with CRC skip only for scheme version >=2.
This investigation did not use REA. The verified keys are now mounted by the
production ef profile. VFS tests cover extensionless voice key construction and
an opt-in local installation test checks mounted audio entry counts plus the
same full-stream Ogg CRC proof through production decryption:

```sh
EF_GAME_ROOT="/root/games/ef-first-tale/ef - the first tale" cargo test --lib ef_audio_local_streams_have_valid_ogg_page_crcs -- --ignored
```
