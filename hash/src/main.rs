#!(allow[unused])
extern crate openssl;
extern crate chrono;
extern crate hex;

use std::fs;
use std::env;
use std::time::{Instant};

mod hash_functions;
mod file_read_write_functions;
use file_read_write_functions::{write_hash_to_file, read_file_to_vec};
use hash_functions::{add_to_content, sha512};
use hex::encode;

fn main() {
    
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("no ingresaste el argumento");
    }else if args.len() > 2{
        println!("pasaste demasiados argumentos");
    }else{
        let contents = read_file_to_vec(args[1].clone());
        let mut contenido: Vec<u8> = Vec::new();
        match contents{
            Ok(contents) => contenido = contents,
            Err(e) => println!("Error el leerlo: {}", e),
        }
        let metadata = fs::metadata(args[1].clone()).unwrap();
        
        let created = metadata.created().unwrap();
        let modified = metadata.modified().unwrap();
        let time_pure = modified.duration_since(created).unwrap().as_secs();

        let hashable_content = add_to_content(&contenido, &args[1].clone(), &time_pure.to_string());
        let hash = sha512(&hashable_content);
        let hex_string = encode(hash);
        println!("El hash SHA-512 de '{}' es: {:?}", String::from_utf8_lossy(args[1].as_bytes()), hex_string);
        let _response: bool = write_hash_to_file(hex_string);
    }
    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Tiempo de ejecución: {} milisegundos", elapsed);
}
