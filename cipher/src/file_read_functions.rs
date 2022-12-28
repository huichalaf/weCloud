use std::fs::File;
use std::io::Read;
use std::fs;

pub fn read_file_str(name_file: String) -> std::io::Result<String> {
    let mut file = File::open(name_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_file_vec(name_file: String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(name_file)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

/*pub fn write_to_file_vec(name_file: String, data: Vec<u8>) -> std::io::Result<()>{
    let slice: &[u8] = &data;
    fs::write(&name_file, slice)?;
    Ok(()) 
}*/

pub fn write_to_file_str(name_file: String, data: String) -> std::io::Result<()>{
    fs::write(name_file, &data)?;
    Ok(())
}
