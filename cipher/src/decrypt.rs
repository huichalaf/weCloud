extern crate base64;
extern crate hex;
extern crate rustc_serialize;
extern crate crypto;
extern crate rand;

use openssl::symm::{Cipher};
use hex::decode;

pub fn descifrar(key: &[u8], iv: &[u8], content_of_file_str: String) -> std::io::Result<String>{
    let string = decode(content_of_file_str.clone()).unwrap_or_else(|e| {
        panic!("Error al convertir la cadena hexadecimal a string: {:?}", e);
    });
    let cipher = Cipher::aes_256_cbc();
    let decrypted_message = openssl::symm::decrypt(cipher, key, Some(iv), &string).unwrap();
    let decrypted_data = String::from_utf8(decrypted_message).unwrap();
    Ok(decrypted_data.to_string())
}

