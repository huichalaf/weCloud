#!(allow[unused])
extern crate openssl;
extern crate chrono;

use std::fs;
use std::env;
use std::time::{Instant};

mod hash_functions;
use hash_functions::{add_to_content, create_hash, write_hash_to_file, sha512};
//mod cipher_functions;
//use cipher_functions::cipher_aes;

fn main() {
    
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("no ingresaste el argumento");
    }else if args.len() > 2{
        println!("pasaste demasiados argumentos");
    }else{
        let contents = fs::read_to_string(args[1].clone());
        let mut contenido: String = String::new();
        match contents{
            Ok(contents) => contenido = contents,
            Err(e) => println!("Error el leerlo: {}", e),
        }
        let metadata = fs::metadata(args[1].clone()).unwrap();
        
        let created = metadata.created().unwrap();
        let modified = metadata.modified().unwrap();
        let time_pure = modified.duration_since(created).unwrap().as_secs();

        let hashable_content = add_to_content(&contenido, &args[1].clone(), &time_pure.to_string());
        let hash = sha512(hashable_content.as_bytes());
        let hash_usable: [char; 64] = create_hash(hash);
        //println!("El hash SHA-512 de '{}' es: {:?}", String::from_utf8_lossy(args[1].as_bytes()), hash_usable);
        let _response: bool = write_hash_to_file(args[1].clone(), hash_usable);
    }
    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Tiempo de ejecución: {} milisegundos", elapsed);
}
