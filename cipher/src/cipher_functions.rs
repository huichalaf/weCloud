extern crate openssl;
extern crate rand;

use std::fs;
use std::io::prelude::*;
use openssl::symm::{self, Cipher, Encryptor};
use rand::random;
use openssl::pkcs5::pbkdf2_hmac;

pub fn cipher_aes(file_name: String, password: String) -> bool{
    
    let first_check: bool = false;
    let second_check: bool = false;

    let salt: [u8; 8] = rand::random();
    let key = pkcs5::pbkdf2_hmac(
        Cipher::aes_256_cbc(),
        password.as_bytes(),
        &salt,
        100000,
        32,
    );
    let iv: [u8; 16] = rand::random();

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_name)
        .unwrap();

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let cipher = symm::Cipher::aes_256_cbc();
    let mut encryptor = symm::Encryptor::new(cipher, &key, Some(&iv)).unwrap();
    let mut encrypted = Vec::new();
    encryptor.update(&contents, &mut encrypted).unwrap();
    
    let response = encryptor.finalize(&mut encrypted).unwrap();
    match response{
        Ok(_response) => first_check = true,
        Err(e) => println!("error en la encriptacion: {}", e),
    }

    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let response2 = file.write_all(&encrypted).unwrap();
    return first_check;
}
