#!(allow[unused])
extern crate openssl;
extern crate chrono;

use std::fs;
use openssl::hash::MessageDigest;
use std::env;
use std::time::{Instant};

fn sha512(input: &[u8]) -> Vec<u8> {//here we create a function that uses the openssl library, to
                                    //generate a hash of the string
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha512()).unwrap();
    hasher.update(input).unwrap();
    hasher.finish().unwrap().to_vec()
}

fn add_to_content(content: &str, name: &str, time_pure: &str) -> String{//we merge the 3 values
                                                                        //that are relevant to hash
    let new_content: String = content.to_owned() + &name.to_owned() + &time_pure.to_owned();
    new_content
}

fn create_hash(hash: Vec<u8>) -> [char; 64]{//we convert the [0-255] values of the hash, to ascii
                                            //dictionary

    let mut ascii_dict: [char; 256] = ['\0'; 256];
    let mut hash_diccionario: [char; 64] = ['\0'; 64];
    let mut counter: usize = 0;

    for i in 0..256 {
        ascii_dict[i] = char::from(i as u8);
    }
     
    for caracter in hash{
        let x: &u8 = &caracter;
        let counter2: usize = (*x) as usize;
        hash_diccionario[counter] = ascii_dict[counter2];
        counter+=1;
    }
    hash_diccionario
}

fn write_hash_to_file(name_file: String, hash: [char; 64]) -> bool{
    
    let prefix: &str = "hash_of_";
    let new_name_file: String = prefix.to_string() + &name_file.to_string();
    let content_to_write: String = String::from_iter(hash.iter());
    let result = fs::write(new_name_file, content_to_write.clone());
    let mut to_return_value: bool = false;
    
    println!("{}",content_to_write);
    match result{
        Ok(_contents) => to_return_value = true,
        Err(_e) => to_return_value = false,
    }
    println!("{}", to_return_value);
    to_return_value
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
        let metadata = fs::metadata(args[1].clone()).unwrap();
        
        let created = metadata.created().unwrap();
        let modified = metadata.modified().unwrap();
        let time_pure = modified.duration_since(created).unwrap().as_secs();

        let hashable_content = add_to_content(&contents, &args[1].clone(), &time_pure.to_string());
        let hash = sha512(hashable_content.as_bytes());
        let hash_usable: [char; 64] = create_hash(hash);
        //println!("El hash SHA-512 de '{}' es: {:?}", String::from_utf8_lossy(args[1].as_bytes()), hash_usable);
        let _response: bool = write_hash_to_file(args[1].clone(), hash_usable);
    }
    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Tiempo de ejecución: {} milisegundos", elapsed);
}
