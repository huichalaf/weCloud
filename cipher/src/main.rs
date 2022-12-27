#!(allow[unused])
extern crate base64;
extern crate hex;
extern crate rustc_serialize;
extern crate crypto;
extern crate rand;

mod key_functions;
mod file_read_functions; 

use key_functions::{generate_iv, generate_key, hex_to_bytes};
use file_read_functions::{read_file_vec};
use crypto::aes_gcm::AesGcm;
use crypto::aead::{AeadDecryptor, AeadEncryptor};
use rustc_serialize::hex::FromHex;
use rand::Rng;
use std::env;
use core::str;
use std::iter::repeat;

fn main() {
    
    let mut path_file: String = "Cargo.lock".to_string();
    let largo_clave: i32 = 32;
    let largo_iv: i32 = 24;
    let mut mykey="00000000000000000000000000000000";
    unsafe{mykey=generate_key(largo_clave);}
    let mut msg="Hola mundo";
    let myadd="Additional Data for authentication!";

    let mut myiv="000000000000000000000000";
    unsafe{myiv = generate_iv(largo_iv);}

    let args: Vec<String> = env::args().collect();
  
    if args.len() >1 { path_file = args[1].as_str().to_string();}
    if args.len() >2 { msg = args[2].as_str();}
    if args.len() >3 { mykey = args[3].as_str();}
    if args.len() >4 { myiv = args[4].as_str();}

    read_file_vec(path_file);
    println!("== AES GCM ==");
    println!("Message: {:?}",msg);
    println!("Key: {}",mykey);
    println!("IV: {:?}",myiv);
    println!("Additional: {:?}",myadd);
 
    let key=&hex_to_bytes( mykey)[..];
    let iv=&hex_to_bytes( myiv)[..];
    let plain=msg.as_bytes();
    let add=myadd.as_bytes();

    let key_size=crypto::aes::KeySize::KeySize128;
    let mut cipher = AesGcm::new(key_size, key, iv, add);
    let mut out: Vec<u8> = repeat(0).take(plain.len()).collect();
    let mut out_tag: Vec<u8> = repeat(0).take(16).collect();

    cipher.encrypt(plain, &mut out[..],&mut out_tag[..]);

    let mut decipher = AesGcm::new(key_size, key, iv, add);
    let mut out2: Vec<u8> = repeat(0).take(plain.len()).collect();

    let result = decipher.decrypt(&out[..], &mut out2[..], &out_tag[..]);

    println!("\nEncrypted: {}",hex::encode(out.clone()));
    if result==true { 
        println!("Successful decryption");
        println!("\nDecrypted {}",str::from_utf8(&out2[..]).unwrap());
    }
}
