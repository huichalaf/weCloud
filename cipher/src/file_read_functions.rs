use std::fs::File;
use std::io::Read;

pub fn read_file_vec(name_file: String) -> std::io::Result<()> {
    let mut file = File::open(name_file)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    println!("File contents: {:?}", contents);
    Ok(())
}
