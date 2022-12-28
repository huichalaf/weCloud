extern crate base64;
extern crate hex;
extern crate rustc_serialize;
extern crate crypto;
extern crate rand;

//use rustc_serialize::hex::FromHex;
use rand::Rng;
use core::str;

static mut GENERATED_KEY: String = String::new();
static mut GENERATED_IV: String = String::new();
/*
pub fn hex_to_bytes(s: &str) -> Vec<u8> {
    s.from_hex().unwrap_or_else(|e| {
        panic!("Error al convertir la cadena hexadecimal a bytes: {:?}", e);
    })
}
*/
pub unsafe fn generate_key<'a>(largo: i32) -> &'a str{
    
    let caracteres: [char; 66] = ['a','b','c','d', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r','s','t','u','v','w','x','y','z', 'A', 'B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z', ',', '.', '-', '_', '1', '2', '3','4','5','6','7','8','9','0'];
   
    let mut generated_key_local: String = String::new();
    for _i in 0..largo{
        let random_value: usize = rand::thread_rng().gen_range(0,66);
        generated_key_local += &caracteres[random_value].to_string();
    }
    GENERATED_KEY = generated_key_local;
    &GENERATED_KEY
}

pub unsafe fn generate_iv<'a>(largo: i32) -> &'a str{
    
    let caracteres: [char; 66] = ['a','b','c','d', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r','s','t','u','v','w','x','y','z', 'A', 'B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z', ',', '.', '-', '_', '1', '2', '3','4','5','6','7','8','9','0'];
   
    let mut generated_key_local: String = String::new();
    for _i in 0..largo{
        let random_value: usize = rand::thread_rng().gen_range(0,66);
        generated_key_local += &caracteres[random_value].to_string();
    }
    GENERATED_IV = generated_key_local;
    &GENERATED_IV
}
