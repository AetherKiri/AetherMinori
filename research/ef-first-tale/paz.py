import struct, warnings, zlib, os
warnings.filterwarnings("ignore")
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
B = "/root/games/ef-first-tale/ef - the first tale/"
d = open(B + "ef_first_AA.exe", "rb").read()
ORDER = ("scr","bg","st","sys","bgm","voice","se","mov")
DAT = {n: bytes((256-b)&0xFF for b in d[0x1a7bc0+i*32:0x1a7bc0+(i+1)*32]) for i,n in enumerate(ORDER)}
IDX = {n: bytes((256-b)&0xFF for b in d[0x1a7ae0+i*32:0x1a7ae0+(i+1)*32]) for i,n in enumerate(ORDER)}
PW = {"png": b"36TXHE5N", "ogg": b"oRQCAU1o", "sc": b"102ycz1s", "avi": b"hYPH3Fxw"}

def bf_le(key, data):
    dec = Cipher(algorithms.Blowfish(key), modes.ECB()).decryptor()
    out = bytearray()
    for i in range(0, len(data) - len(data) % 8, 8):
        blk = data[i:i+8]
        be = blk[0:4][::-1] + blk[4:8][::-1]
        r = dec.update(be)
        out += r[0:4][::-1] + r[4:8][::-1]
    return bytes(out)

def rc4(key, data):
    s = list(range(256)); j = 0
    for i in range(256):
        j = (j + s[i] + key[i % len(key)]) & 0xFF
        s[i], s[j] = s[j], s[i]
    out = bytearray(); i = j = 0
    for b in data:
        i = (i+1) & 0xFF; j = (j + s[i]) & 0xFF
        s[i], s[j] = s[j], s[i]
        out.append(b ^ s[(s[i]+s[j]) & 0xFF])
    return bytes(out)

def pwfor(nm, arc):
    n = nm.lower()
    if n.endswith(".png"): return PW["png"]
    if n.endswith(".ogg") or arc in ("bgm","se","voice"): return PW["ogg"]
    if n.endswith(".sc"): return PW["sc"]
    if n.endswith((".avi",".mpg",".mpeg")): return PW["avi"]
    return None

class Paz:
    def __init__(self, arc, data_key_name):
        self.arc = arc
        self.key = DAT[data_key_name]
        self.f = open(B + arc + ".paz", "rb")
        head = self.f.read(0x24)
        size = struct.unpack("<I", head[0x20:0x24])[0]
        self.xor = size >> 24 & 0xFF
        if self.xor: size ^= self.xor*0x01010101
        idx = bytearray(self.f.read(size))
        if self.xor:
            for i in range(len(idx)): idx[i] ^= self.xor
        dec = bf_le(IDX[arc], bytes(idx))
        self.entries = {}
        p = 4; n = struct.unpack_from("<I", dec, 0)[0]
        for _ in range(n):
            e = dec.find(b"\x00", p); nm = dec[p:e].decode("shift_jis","replace"); p = e+1
            off, dk, up, pk, st, typ = struct.unpack_from("<IIIIII", dec, p); p += 24
            self.entries[nm] = dict(offset=off, unpacked=up, stored=st, type=typ)

    def read(self, nm):
        e = self.entries[nm]
        self.f.seek(e["offset"])
        raw = bytearray(self.f.read((e["stored"]+7)&~7))
        if self.xor:
            for i in range(len(raw)): raw[i] ^= self.xor
        out = bf_le(self.key, bytes(raw))[:e["stored"]]
        if e["type"] == 1:
            return zlib.decompress(out)[:e["unpacked"]]
        pw = pwfor(nm, self.arc)
        if pw:
            out = rc4(f"{nm.lower()} {e['unpacked']:08X} {pw.decode()}".encode("shift_jis"), out)
        return out[:e["unpacked"]]

if __name__ == "__main__":
    out = "/var/lib/dsh/Minori/ef-re/extracted/scr"
    os.makedirs(out, exist_ok=True)
    p = Paz("scr", "sys")
    ok = 0
    for nm in sorted(p.entries):
        try:
            open(os.path.join(out, nm), "wb").write(p.read(nm)); ok += 1
        except Exception as ex:
            print("FAIL", nm, ex)
    print("extracted", ok, "of", len(p.entries))
