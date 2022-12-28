#!(allow[unused])
extern crate base64;
extern crate hex;
extern crate rustc_serialize;
extern crate crypto;
extern crate rand;
extern crate openssl;

use openssl::symm::{Cipher};

pub fn cifrar(key: &[u8], iv: &[u8], content_of_file_vec: Vec<u8>) -> std::io::Result<String>{ 
    let cipher = Cipher::aes_256_cbc();
    let encrypted_message = openssl::symm::encrypt(cipher, key, Some(iv), &content_of_file_vec).unwrap();

    let encrypted_data = hex::encode(encrypted_message.clone());

    Ok(encrypted_data)
}
