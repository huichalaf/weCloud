extern crate base64;
extern crate hex;
extern crate rustc_serialize;
extern crate crypto;
extern crate rand;

use crypto::aes_gcm::AesGcm;
use crypto::aead::{AeadDecryptor, AeadEncryptor};
use rustc_serialize::hex::FromHex;
use rand::Rng;
use std::env;
use core::str;
use std::iter::repeat;

static mut GENERATED_KEY: String = String::new();
static mut GENERATED_IV: String = String::new();

pub fn hex_to_bytes(s: &str) -> Vec<u8> {
    s.from_hex().unwrap_or_else(|e| {
        panic!("Error al convertir la cadena hexadecimal a bytes: {:?}", e);
    })
}

pub unsafe fn generate_key<'a>(largo: i32) -> &'a str{
    
    let mut generated_key_local: String = String::new();
    for _i in 0..largo{
        let random_value: i32 = rand::thread_rng().gen_range(0,2);
        println!("{}",random_value.to_string());
        generated_key_local += &random_value.to_string();
    }
    GENERATED_KEY = generated_key_local;
    &GENERATED_KEY
}

pub unsafe fn generate_iv<'a>(largo: i32) -> &'a str{
    
    let mut generated_key_local: String = String::new();
    for _i in 0..largo{
        let random_value: i32 = rand::thread_rng().gen_range(0,2);
        println!("{}",random_value.to_string());
        generated_key_local += &random_value.to_string();
    }
    GENERATED_IV = generated_key_local;
    &GENERATED_IV
}
