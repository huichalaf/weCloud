use std::fs::File;
use std::io::Read;
use std::fs::{OpenOptions};
use std::io::{Write, Seek, SeekFrom};

fn write_to_file_str(name_file: String, data: String) -> std::io::Result<()>{

    let mut file = OpenOptions::new().write(true).open(name_file.clone()).unwrap();
    file.seek(SeekFrom::End(0)).unwrap();

    file.write_all(data.as_bytes()).unwrap();
    Ok(())
}

pub fn write_hash_to_file(hash: String) -> bool{
    
    let new_name_file: String = "../blockchain".to_string();
    let content_to_write: String = hash.to_string();
    let result = write_to_file_str(new_name_file.clone(), content_to_write.clone());
    let mut to_return_value: bool = false;
    
    println!("{}",content_to_write);
    match result{
        Ok(_contents) => to_return_value = true,
        Err(e) => println!("error: {}",e),
    }
    println!("{}", to_return_value);
    to_return_value
}

pub fn read_file_to_vec(name_file: String) -> std::io::Result<Vec<u8>>{
    
    let mut file = File::open(name_file)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)

}
