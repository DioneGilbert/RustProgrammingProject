#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use openssl::aes;

#[target_feature(enable = "sse2")]

unsafe fn expand_key(key: &[u8; 16]) -> [__m128i; 11] {
    // let round_keys = aes::AesKey::new_encrypt(key);
    // let key_into_m128i: [__m128i; 11] ;
    // for i in 0..10 {
    //     round_keys.Ae
    //     key_into_m128i[i] = _mm_loadu_si128(round_keys[i].as_ptr().cast());
    // }
    // let key_into_m128i: __m128i = _mm_loadu_si128(key.as_ptr().cast());
    todo!()
    //let key = aes::Key::generate();
}

unsafe fn encrypt1(keys: &[__m128i; 11], block: &mut [u8; 16]) {
    //let cipher_in_m128i = aes::AesKey::new_encrypt(key);
    let block_in_m128i: __m128i = _mm_loadu_si128(block.as_ptr().cast());
    let mut round_ciphertext = Vec::<__m128i>::new();
    // Add Round Key
    round_ciphertext[0] = _mm_add_epi32(keys[0], block_in_m128i);
    // Rounds: 1 to 9
    for i in 1..10 {
        round_ciphertext[i] = _mm_aesenc_si128(round_ciphertext[i - 1], keys[i]);
    }
    // last round
    round_ciphertext[10] = _mm_aesenclast_si128(round_ciphertext[9], keys[10]);
    block = _mm_storeu_epi8(block, round_ciphertext[10]);
}

unsafe fn decrypt1(keys: &[__m128i; 11], block: &mut [u8; 16]) {}

unsafe fn encrypt8(keys: &[__m128i; 11], block: &mut [u8; 128]) {}

unsafe fn decrypt8(keys: &[__m128i; 11], block: &mut [u8; 128]) {}
