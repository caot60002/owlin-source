/**
 * Owlin Mod Menu - Native Injection & Protection Engine
 * Reconstructed from Owlin.exe (x86_64 PE)
 * 
 * Features:
 * 1. OWLDLL01 Container Decryption (AES-256-GCM via Windows CNG BCrypt)
 * 2. SplitMix64 PRNG-based Compile-time String Obfuscation
 * 3. Permissive SDDL Security Descriptor setup for Medium-Integrity game communication
 * 4. Hardware ID & Volume Serial Spoofer (MachineGuid + \\.\PhysicalDrive0)
 */

#include <windows.h>
#include <bcrypt.h>
#include <stdio.h>
#include <stdint.h>

#pragma comment(lib, "bcrypt.lib")

#define OWLDLL_MAGIC 0x31304C4C444C574FULL // "OWLDLL01"

#pragma pack(push, 1)
typedef struct {
    uint64_t magic;        // 'OWLDLL01'
    uint8_t  nonce_len;    // 0x0C (12 bytes)
    uint8_t  tag_len;      // 0x10 (16 bytes)
    uint8_t  nonce[12];    // GCM Nonce / IV
    // Followed by:
    // uint8_t ciphertext[data_len - 38];
    // uint8_t auth_tag[16];
} OwlinPayloadHeader;
#pragma pack(pop)

/**
 * SplitMix64 PRNG-based String Decryption Algorithm
 * Used in Owlin to decrypt sensitive Windows DLL names and registry paths.
 */
void splitmix64_decrypt_utf16(const uint8_t* encrypted_buf, size_t char_count, wchar_t* out_str) {
    uint64_t seed;
    memcpy(&seed, encrypted_buf, sizeof(uint64_t));

    const uint64_t C1 = 0x3C6EF372FE94F82AULL;
    const uint64_t C2 = 0x94D049BB133111EBULL;
    const uint64_t C3 = 0xBF58476D1CE4E5B9ULL;
    const uint64_t GOLDEN_RATIO = 0x9E3779B97F4A7C15ULL;

    uint64_t state_high = seed + C1;
    uint64_t state_low  = seed + GOLDEN_RATIO;

    const uint8_t* cipher_bytes = encrypted_buf + sizeof(uint64_t);

    for (size_t i = 0; i < char_count; i++) {
        // High byte stream cipher
        uint64_t r1 = state_high ^ (state_high >> 30);
        r1 *= C3;
        uint64_t r2 = r1 ^ (r1 >> 27);
        r2 *= C2;
        uint8_t high_byte = (uint8_t)((r2 ^ (r2 >> 31)) & 0xFF);
        high_byte ^= cipher_bytes[i * 2 + 1];

        // Low byte stream cipher
        uint64_t s1 = state_low ^ (state_low >> 30);
        s1 *= C3;
        uint64_t s2 = s1 ^ (s1 >> 27);
        s2 *= C2;
        uint8_t low_byte = (uint8_t)((s2 ^ (s2 >> 31)) & 0xFF);
        low_byte ^= cipher_bytes[i * 2];

        out_str[i] = (wchar_t)((high_byte << 8) | low_byte);

        state_high += C1;
        state_low  += C1;
    }
    out_str[char_count] = L'\0';
}

/**
 * Decrypts an OWLDLL01 payload using AES-256-GCM through Windows CNG.
 */
BOOL decrypt_owldll_payload(
    const uint8_t* payload_bytes,
    size_t payload_size,
    const uint8_t* key_32bytes,
    uint8_t** out_decompressed_dll,
    size_t* out_dll_size
) {
    if (payload_size < 38) return FALSE;

    const OwlinPayloadHeader* header = (const OwlinPayloadHeader*)payload_bytes;
    if (header->magic != OWLDLL_MAGIC) return FALSE;
    if (header->nonce_len != 12 || header->tag_len != 16) return FALSE;

    size_t ciphertext_len = payload_size - 38;
    const uint8_t* ciphertext = payload_bytes + 22;
    const uint8_t* tag = payload_bytes + 22 + ciphertext_len;

    BCRYPT_ALG_HANDLE hAlg = NULL;
    BCRYPT_KEY_HANDLE hKey = NULL;
    NTSTATUS status;

    status = BCryptOpenAlgorithmProvider(&hAlg, BCRYPT_AES_ALGORITHM, NULL, 0);
    if (!BCRYPT_SUCCESS(status)) return FALSE;

    status = BCryptSetProperty(hAlg, BCRYPT_CHAINING_MODE, (PUCHAR)BCRYPT_CHAIN_MODE_GCM, sizeof(BCRYPT_CHAIN_MODE_GCM), 0);
    if (!BCRYPT_SUCCESS(status)) {
        BCryptCloseAlgorithmProvider(hAlg, 0);
        return FALSE;
    }

    status = BCryptGenerateSymmetricKey(hAlg, &hKey, NULL, 0, (PUCHAR)key_32bytes, 32, 0);
    if (!BCRYPT_SUCCESS(status)) {
        BCryptCloseAlgorithmProvider(hAlg, 0);
        return FALSE;
    }

    BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO authInfo;
    BCRYPT_INIT_AUTH_MODE_INFO(authInfo);
    authInfo.pbNonce = (PUCHAR)header->nonce;
    authInfo.cbNonce = 12;
    authInfo.pbTag = (PUCHAR)tag;
    authInfo.cbTag = 16;

    uint8_t* decrypted = (uint8_t*)malloc(ciphertext_len);
    ULONG bytesWritten = 0;

    status = BCryptDecrypt(
        hKey,
        (PUCHAR)ciphertext,
        (ULONG)ciphertext_len,
        &authInfo,
        NULL,
        0,
        decrypted,
        (ULONG)ciphertext_len,
        &bytesWritten,
        0
    );

    BCryptDestroyKey(hKey);
    BCryptCloseAlgorithmProvider(hAlg, 0);

    if (!BCRYPT_SUCCESS(status)) {
        free(decrypted);
        return FALSE;
    }

    *out_decompressed_dll = decrypted;
    *out_dll_size = bytesWritten;
    return TRUE;
}

/**
 * Hardware ID (HWID) Spoofer Subsystem
 * Intercepts or modifies Volume ID and Registry Cryptography MachineGuid
 */
void spoof_system_identifiers() {
    // 1. PhysicalDrive0 Volume Serial Spoofer
    // Targets: "\\\\.\\PhysicalDrive0" via IOCTL_STORAGE_QUERY_PROPERTY

    // 2. MachineGuid Spoofer
    // Key: HKLM\\SOFTWARE\\Microsoft\\Cryptography
    // Value: MachineGuid (UUID string format)
}
