"""Bounded offline ef audio probe; no extraction or production imports.
Default: indexes and at most 64 payload bytes per selected entry.
--verify: one full stream per archive in memory, capped below 2 MiB each.
"""
import importlib.util, struct, json, re, zlib
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from pathlib import Path
spec = importlib.util.spec_from_file_location("ef_research_paz", Path(__file__).with_name("paz.py"))
paz = importlib.util.module_from_spec(spec)
spec.loader.exec_module(paz)

def index(arc, slot):
    f = open(paz.B + arc + ".paz", "rb")
    h = f.read(36)
    size = struct.unpack_from("<I", h, 32)[0]
    xor = size >> 24
    size ^= xor * 0x01010101
    assert 0 < size < 4 * 1024 * 1024
    key = bytes(-b & 255 for b in paz.d[0x1a7ae0+slot*32:0x1a7ae0+(slot+1)*32])
    dec = paz.bf_le(key, bytes(b ^ xor for b in f.read(size)))
    count, = struct.unpack_from("<I", dec)
    assert count < 100000
    entries = []; p = 4
    for _ in range(count):
        e = dec.index(0, p); name = dec[p:e].decode("shift_jis"); p = e+1
        off, up, sz, aligned, packed = struct.unpack_from("<QIIII", dec, p); p += 24
        entries.append((name, off, up, sz, aligned, packed))
    return f, xor, entries

def extended():
    passwords = {b"", b"a7b77qgx", b"oRQCAU1o", b"102ycz1s", b"36TXHE5N", b"hYPH3Fxw"}
    for transform in [paz.d, bytes(-b & 255 for b in paz.d), bytes(b ^ 255 for b in paz.d)]:
        passwords.update(m.group() for m in re.finditer(rb"(?<![A-Za-z0-9])[A-Za-z0-9]{8}(?![A-Za-z0-9])", transform))
    print("extended password candidates", len(passwords))
    keys = {}
    for off in range(0x1a7ae0, 0x1a7e00, 32):
        raw = paz.d[off:off+32]
        for label, key in [("neg",bytes(-b & 255 for b in raw)), ("raw",raw), ("not",bytes(b ^ 255 for b in raw))]:
            keys[f"{off:x}/{label}"] = key
    for arc, slot in [("bgm",5), ("voice",6), ("se",7)]:
        f,xor,entries = index(arc,slot)
        name, off, up, sz, aligned, packed = entries[0]
        f.seek(off); raw = f.read(64); f.close()
        variants = {"raw":raw}
        for label,key in keys.items():
            variants[label+"/le"] = paz.bf_le(key,raw)
            variants[label+"/be"] = Cipher(algorithms.Blowfish(key),modes.ECB()).decryptor().update(raw)
        trials = 0
        for password in passwords:
            for nm in {name, name.lower(), name.lower()+".ogg"}:
                key = nm.encode("shift_jis") + f" {up:08X} ".encode() + password
                ks = paz.rc4(key,bytes(320))
                for skip in {0, (zlib.crc32(key)>>12)&255}:
                    for label, data in variants.items():
                        out = bytes(a^b for a,b in zip(data,ks[skip:]))
                        trials += 1
                        if out.startswith((b"OggS",b"RIFF")):
                            print("EXTENDED_MATCH",arc,name,label,password,skip,out.hex(),flush=True)
        print("extended",arc,"trials",trials,flush=True)

def main():
    for arc, slot in [("bgm", 5), ("voice", 6), ("se", 7)]:
        f, xor, entries = index(arc, slot)
        print(arc, "count", len(entries), "xor", xor, "examples", entries[:3])
        for name, off, up, sz, aligned, packed in entries[:3]:
            f.seek(off); raw = bytes(b ^ xor for b in f.read(64))
            for ds in range(12):
                key = bytes(-b & 255 for b in paz.d[0x1a7bc0+ds*32:0x1a7bc0+(ds+1)*32])
                out = paz.bf_le(key, raw)
                for password in [None, "a7b77qgx", "oRQCAU1o", "102ycz1s", "36TXHE5N", "hYPH3Fxw"]:
                    result = out if password is None else paz.rc4(f"{name.lower()} {up:08X} {password}".encode("shift_jis"), out)
                    if result.startswith((b"OggS", b"RIFF")):
                        print("MATCH", json.dumps(dict(arc=arc, name=name, slot=ds, key=key.hex(), password=password, header=result.hex())))
        f.close()

def verify():
    """Verify one full stream per archive in memory (largest about 1.6 MB)."""
    import hashlib
    for arc, ix, ds in [("bgm", 5, 8), ("voice", 6, 9), ("se", 7, 10)]:
        f, xor, entries = index(arc, ix)
        name, off, up, size, aligned, packed = entries[0]
        assert aligned < 2 * 1024 * 1024 and not packed and xor == 0
        f.seek(off); raw = f.read(aligned); f.close()
        key = bytes(-b & 255 for b in paz.d[0x1a7bc0+ds*32:0x1a7bc0+(ds+1)*32])
        out = paz.rc4(f"{name.lower()} {up:08X} 52AUwF8B".encode(), paz.bf_le(key, raw))[:up]
        p = 0; pages = 0; flags = []
        while p < len(out):
            assert out[p:p+4] == b"OggS"
            count = out[p+26]
            end = p+27+count+sum(out[p+27:p+27+count])
            page = bytearray(out[p:end])
            assert len(page) == end-p
            crc, = struct.unpack_from("<I", page, 22)
            page[22:26] = bytes(4); value = 0
            for b in page:
                value ^= b << 24
                for _ in range(8):
                    value = ((value << 1) ^ (0x04c11db7 if value & 0x80000000 else 0)) & 0xffffffff
            assert value == crc
            flags.append(out[p+5]); pages += 1; p = end
        assert p == len(out) and flags[0] & 2 and flags[-1] & 4
        print("VERIFIED", arc, name, key.hex(), len(out), pages, hashlib.sha256(out).hexdigest())

if __name__ == "__main__":
    import sys
    if "--verify" in sys.argv:
        verify()
    else:
        main()
        extended()
