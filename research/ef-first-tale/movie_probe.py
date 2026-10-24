"""Bounded ef movie index/header probe; never extracts movies."""
import hashlib
import struct
import re
import zlib
from paz import B, bf_le, rc4


def probe():
    with open(B + "ef_first_AA.exe", "rb") as f:
        f.seek(0x1a7c00)
        key = bytes((-b) & 255 for b in f.read(32))
    with open(B + "ef_first_AA.exe", "rb") as exe:
        data = exe.read()
    passwords = {"", "hYPH3Fxw", "a7b77qgx", "52AUwF8B"}
    for view in [data, bytes((-b) & 255 for b in data), bytes(b ^ 255 for b in data)]:
        passwords.update(m.group().decode() for m in re.finditer(rb"(?<![A-Za-z0-9])[A-Za-z0-9]{8}(?![A-Za-z0-9])", view))
    with open(B + "mov.paz", "rb") as f:
        f.seek(0x20)
        size = struct.unpack("<I", f.read(4))[0]
        xor = size >> 24
        size ^= xor * 0x01010101
        assert 0 < size <= 1024 * 1024 and size % 8 == 0
        index = bf_le(key, bytes(b ^ xor for b in f.read(size)))
        count = struct.unpack_from("<I", index)[0]
        assert count == 4 and len(index) == 424
        table = index[4:260]
        print("index", size, "count", count, "key", key.hex(), "table_sha256", hashlib.sha256(table).hexdigest())
        pos = 260
        for _ in range(count):
            end = index.index(0, pos)
            name = index[pos:end].decode("shift_jis")
            offset, unpacked, stored, aligned, packed = struct.unpack_from("<QIIII", index, end + 1)
            pos = end + 25
            print("entry", name, offset, unpacked, stored, aligned, packed)
            assert offset + aligned <= f.seek(0, 2)
            f.seek(offset)
            raw = bytes(b ^ xor for b in f.read(min(aligned, 65536)))
            for password in sorted(passwords):
                entry_key = f"{name.lower()} {unpacked:08X} {password}".encode("shift_jis")
                mixed = bytes(table[i] ^ entry_key[i % len(entry_key)] for i in range(256))
                decoded = rc4(mixed, raw[:64])
                if decoded.startswith((b"RIFF", b"\x00\x00\x01")):
                    decoded = rc4(mixed, raw)
                    assert password == "qmzj4Y9C"
                    if name.lower().endswith(".avi"):
                        assert decoded[8:12] == b"AVI "
                        assert struct.unpack_from("<I", decoded, 4)[0] + 8 == unpacked
                        assert decoded[12:16] == b"LIST" and decoded[20:24] == b"hdrl"
                    else:
                        assert decoded[:4] == b"\x00\x00\x01\xba" and decoded[12:16] == b"\x00\x00\x01\xbb"
                    print("MATCH", repr(password), decoded[:64].hex(), "sha256", hashlib.sha256(decoded).hexdigest(), "crc32", hex(zlib.crc32(decoded)))
        print("index_used", pos, "padding", index[pos:].hex())


def verify_logo():
    """Full small-logo check in memory; ffmpeg receives bytes over stdin."""
    import subprocess
    import json
    from paz import d
    with open(B + "mov.paz", "rb") as f:
        f.seek(36)
        index = bf_le(bytes((-b) & 255 for b in d[0x1a7c00:0x1a7c20]), f.read(424))
        f.seek(460)
        raw = f.read(446472)
    key = b"op_logo.mpg 0006D004 qmzj4Y9C"
    stream = rc4(bytes(index[4+i] ^ key[i % len(key)] for i in range(256)), bytes(65536))
    aligned = bytes(b ^ stream[i % 65536] for i, b in enumerate(raw))
    assert aligned[446468:] == bytes(4)
    data = aligned[:446468]
    assert data[-4:] == bytes.fromhex("000001b9")
    assert hashlib.sha256(data).hexdigest() == "4b22979fdf5f813f36ea67b14f9bd63661ab4ec1c3f4c3b7c258a5c0d51e0ead"
    p = subprocess.run(["ffprobe", "-v", "error", "-f", "mpeg", "-i", "pipe:0", "-show_packets", "-of", "json"], input=data, capture_output=True)
    assert p.returncode == 0 and not p.stderr
    packets = json.loads(p.stdout)["packets"]
    print("logo_sha256", hashlib.sha256(data).hexdigest(), "last_packet", packets[-1])
    for label, content in [("stored", data), ("aligned", aligned), ("minus_last_sector", data[:444416])]:
        result = subprocess.run(["ffmpeg", "-v", "error", "-f", "mpeg", "-i", "pipe:0", "-f", "null", "-"], input=content, capture_output=True)
        print(label, "exit", result.returncode, "diagnostic", result.stderr.decode(errors="replace").strip())
        assert result.returncode == 0
    # Sector truncation above is diagnostic only, never a production repair.


if __name__ == "__main__":
    import sys
    verify_logo() if "--verify-logo" in sys.argv else probe()
