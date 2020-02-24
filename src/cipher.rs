extern crate ring;
extern crate aes;

use std::num::NonZeroU32;
use self::ring::{digest, pbkdf2};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
pub type Key = [u8; 16];



use cipher::aes::block_cipher_trait::generic_array::GenericArray;
use cipher::aes::block_cipher_trait::BlockCipher;
use cipher::aes::Aes128;


pub fn get_cipher_key(a_password: & String, a_iterations: usize) -> Key {
    let mut salt = Vec::with_capacity(9);
    salt.extend(String::from("saltysalt").as_bytes());
    let length = 16;

    let mut res : Key = [0u8; 16];;
    pbkdf2::derive(PBKDF2_ALG, NonZeroU32::new(a_iterations as u32).unwrap() , &salt, a_password.as_bytes(), &mut res);

    return res;
//    return String::new();
}

pub fn decrypt_value(encrypted: &String, key: &Key) -> String {
    // let k = GenericArray::from_slice(key.);
    // let cipher = Aes128::new(&k);

    // let mut block = GenericArray::from_slice(encrypted.as_bytes());
    // let mut bl2 = block.clone();
    // cipher.decrypt_block(&mut bl2);
    return String::new();
}