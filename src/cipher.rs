extern crate pbkdf2;
extern crate hmac;
extern crate sha2;
extern crate aes;

use self::hmac::Hmac;
use self::sha2::Sha256;

pub type KeyType = [u8; 16];


use self::aes::block_cipher_trait::generic_array::GenericArray;
use self::aes::block_cipher_trait::BlockCipher;
use self::aes::{Aes128, Aes256};

pub fn get_cipher_key(a_password: &str, a_iterations: usize) -> KeyType {
    let salt = b"saltysalt";
    let mut res: KeyType = [0u8; 16];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(a_password.as_bytes(), salt, a_iterations, res.as_mut());
    return res;
}

 pub fn decrypt_value(encrypted: &String, key: &KeyType) -> String {
     let k = GenericArray::from_slice(key);

     let mut block = GenericArray::clone_from_slice(&[0u8; 16]);
     let mut i: i32 = 15;
     for c in encrypted.chars().rev() {
         block[i as usize] = c as u8;
         i -= 1;
         if i < 0 {
             break;
         }
     }

     let cipher = Aes128::new(&k);
     cipher.decrypt_block(&mut block);

     let mut block2 = block.clone();
     let mut res = String::new();
     for e in block2.iter() {
         res.push(char::from(e.to_owned()));
     }
     println!("res={}", res);
     return res;
//     return String::new();
 }