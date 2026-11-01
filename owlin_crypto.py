"""
Owlin Mod Menu - Crypto & String Decryption Utility
Reconstructed from Owlin.exe
"""

import struct

def splitmix64_decrypt(buf: bytes, length: int) -> str:
    """
    Decrypts compile-time obfuscated UTF-16 strings using Owlin's dual-state SplitMix64 stream cipher.
    """
    seed = struct.unpack('<Q', buf[:8])[0]
    C1 = 0x3C6EF372FE94F82A
    C2 = 0x94D049BB133111EB
    C3 = 0xBF58476D1CE4E5B9
    GOLDEN_RATIO = 0x9E3779B97F4A7C15

    state_high = (seed + C1) & 0xFFFFFFFFFFFFFFFF
    state_low  = (seed + GOLDEN_RATIO) & 0xFFFFFFFFFFFFFFFF

    chars = []
    for i in range(length):
        # High byte stream
        r1 = state_high ^ (state_high >> 30)
        r1 = (r1 * C3) & 0xFFFFFFFFFFFFFFFF
        r2 = r1 ^ (r1 >> 27)
        r2 = (r2 * C2) & 0xFFFFFFFFFFFFFFFF
        high = (r2 ^ (r2 >> 31)) & 0xFF
        high ^= buf[8 + i * 2 + 1]

        # Low byte stream
        s1 = state_low ^ (state_low >> 30)
        s1 = (s1 * C3) & 0xFFFFFFFFFFFFFFFF
        s2 = s1 ^ (s1 >> 27)
        s2 = (s2 * C2) & 0xFFFFFFFFFFFFFFFF
        low = (s2 ^ (s2 >> 31)) & 0xFF
        low ^= buf[8 + i * 2]

        chars.append(chr((high << 8) | low))

        state_high = (state_high + C1) & 0xFFFFFFFFFFFFFFFF
        state_low  = (state_low  + C1) & 0xFFFFFFFFFFFFFFFF

    return ''.join(chars)

def inspect_pe_rcdata_header(file_path: str):
    """
    Reads and prints the header metadata of Owlin's OWLDLL01 payloads.
    """
    resources = [
        (101, 10671872, 6505510, "ETS2 Cheat Hook DLL"),
        (102, 17177384, 6827558, "ATS Cheat Hook DLL"),
        (103, 24004944, 88,      "ETS2 Config / Crypto Block"),
        (104, 24005032, 121,     "ATS Config / Crypto Block")
    ]

    with open(file_path, 'rb') as f:
        for rid, off, sz, desc in resources:
            f.seek(off)
            raw = f.read(min(sz, 64))
            magic = raw[:8]
            nonce_len = raw[8]
            tag_len = raw[9]
            nonce = raw[10:22]
            print(f"RCDATA {rid} ({desc}):")
            print(f"  Size: {sz} bytes")
            print(f"  Magic: {magic!r}")
            print(f"  Nonce length: {nonce_len} bytes")
            print(f"  Tag length: {tag_len} bytes")
            print(f"  Nonce hex: {nonce.hex()}\n")

if __name__ == '__main__':
    inspect_pe_rcdata_header(r"C:\Users\Tu\Downloads\rea\ow\Owlin.exe")
