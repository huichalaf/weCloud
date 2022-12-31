#!(allow[unused])
extern crate openssl;
extern crate chrono;
extern crate hex;

use std::env;
use std::time::{Instant};
use hex::encode;
use openssl::hash::MessageDigest;

fn sha512(input: &[u8]) -> Vec<u8> {
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha512()).unwrap();
    hasher.update(input).unwrap();
    hasher.finish().unwrap().to_vec()
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        println!("no ingresaste el argumento");
    }else if args.len() > 3{
        println!("pasaste demasiados argumentos");
    }else{
        
        let amount_of_ceros: i128 = args[2].parse().unwrap();
        let mut status_nonce: bool = false;
        let mut counter: usize = 0;
        let mut hex_string: String = String::new();
        let mut counter_in_while: i128 = 0;
        let mut contenido: String = String::new();
        let mut hash: Vec<u8> = Vec::new();
        let mut hash_hex_cadena: Vec<String> = Vec::new();

        while status_nonce == false{

            contenido = args[1].clone().to_string() + &counter.to_string();
            hash = sha512(&contenido.as_bytes());
            hex_string = encode(hash);
            hash_hex_cadena = hex_string.chars().map(|c| c.to_string()).collect();
            counter_in_while = 0;

            for i in hash_hex_cadena{
                if counter_in_while == amount_of_ceros{
                    println!("minado completo :)");
                    status_nonce = true;
                }
                if i != '0'.to_string(){
                    break;
                }
                counter_in_while += 1;              
            }
            counter+=1;
        }
        println!("final hash: {}, contador: {}", hex_string, counter);
    }
    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Tiempo de ejecución: {} milisegundos", elapsed);
}
