#!(allow[unused variables])
extern crate base64;
extern crate hex;
extern crate rustc_serialize;
extern crate crypto;
extern crate rand;

mod key_functions;
mod file_read_functions; 
mod encrypt;
mod decrypt;

use decrypt::{descifrar};
use encrypt::{cifrar};
use key_functions::{generate_iv, generate_key};
use file_read_functions::{read_file_vec, write_to_file_str, read_file_str};
use std::env;
use std::time::{Instant};

fn main() {
    
    let start = Instant::now();
    let mut content_of_file_vec: Vec<u8> = Vec::new();
    let mut content_of_file_str: String = String::new();
    let mut path_file: String = "Cargo.lock".to_string();
    
    let mut option: i32 = 0;
    let largo_clave: i32 = 32;
    let largo_iv: i32 = 24;
    
    let mut mykey="my secret key345";
    println!("{}", mykey);
    unsafe{mykey=generate_key(largo_clave);}
    let mut myiv="my secret iv1345";
    println!("{}", myiv);
    unsafe{myiv = generate_iv(largo_iv);}

    let args: Vec<String> = env::args().collect();
  
    if args.len() >1 { option = args[1].parse().unwrap();}
    if args.len() >2 { path_file = args[2].as_str().to_string();}
    if args.len() >3 { mykey = args[3].as_str();}
    if args.len() >4 { myiv = args[4].as_str();}
 
    println!("== AES GCM ==");
    println!("Key: {}",mykey);
    println!("IV: {}",myiv);   

    let key = mykey.as_bytes();
    let iv = myiv.as_bytes();
    println!("{:?}",key);
    println!("{:?}",iv);
    println!("{}", key.len());

    if option==1{ 

        let new_path_file: String = "Cifrated_".to_string()+&path_file.clone();
        let response1 = read_file_vec(path_file.clone());
        let mut cifrated_text: String = String::new();

        match response1{
            Ok(value1) =>  content_of_file_vec = value1,
            Err(err1) => println!("lectura por vec incorrecta {}", err1),
        } 

        let response_cifrado = cifrar(key, iv, content_of_file_vec);
        match response_cifrado{
            Ok(value2) => cifrated_text = value2,
            Err(e) => println!("hubo un error: {}", e),
        }
        //println!("{}", cifrated_text);
        let result_write = write_to_file_str(new_path_file, cifrated_text);
        match result_write{
            Ok(_s) => println!("escritura exitosa"),
            Err(e) => println!("escritura fallida: {}", e),
        }
    }

    else if option==2{

        let new_path_file: String = "Descifrated_".to_string()+&path_file.clone();
        let response1 = read_file_str(path_file.clone());
        let mut descifrated_text: String = String::new();

        match response1{
            Ok(value1) =>  content_of_file_str = value1,
            Err(err1) => println!("lectura por vec incorrecta {}", err1),
        } 
        let response_descifrado = descifrar(key, iv, content_of_file_str);
        match response_descifrado{
            Ok(value2) => descifrated_text = value2,
            Err(e) => println!("hubo un error: {}", e),
        }
        let result_write = write_to_file_str(new_path_file, descifrated_text);
        match result_write{
            Ok(_s) => println!("escritura exitosa"),
            Err(e) => println!("escritura fallida: {}", e),
        }
    }

    else{
        println!("no se ha suministrado ni opcion ni ruta :(");
    }
    let end = Instant::now();
    let elapsed = end.duration_since(start).as_millis();
    println!("Tiempo de ejecución: {} milisegundos", elapsed);
}
