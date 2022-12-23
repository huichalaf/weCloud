extern crate openssl;

use std::fs;
use openssl::hash::MessageDigest;
use std::env;
use std::time::{Instant};
//use std::io;

fn sha512(input: &[u8]) -> Vec<u8> {
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha512()).unwrap();
    hasher.update(input).unwrap();
    hasher.finish().unwrap().to_vec()
}

fn main() {
    
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("no ingresaste el argumento");
    }else if args.len() > 2{
        println!("pasaste demasiados argumentos");
    }else{
        let contents = fs::read_to_string(args[1].clone()).ok().expect("error al leer el archivo");
        let hash = sha512(contents.as_bytes());
        println!("El hash SHA-512 de '{}' es: {:?}", String::from_utf8_lossy(args[1].as_bytes()), hash);
        //println!("{}",contents);
    }
    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Tiempo de ejecución: {} milisegundos", elapsed);
}
