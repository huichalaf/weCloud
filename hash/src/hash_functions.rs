use openssl::hash::MessageDigest;

pub fn sha512(input: &[u8]) -> Vec<u8> {//here we create a function that uses the openssl library, to
                                    //generate a hash of the string
    let mut hasher = openssl::hash::Hasher::new(MessageDigest::sha512()).unwrap();
    hasher.update(input).unwrap();
    hasher.finish().unwrap().to_vec()
}

pub fn add_to_content(content: &Vec<u8>, name: &str, time_pure: &str) -> Vec<u8>{//we merge the 3 values
                                                                        //that are relevant to hash
    let new_content: Vec<u8> = [content, name.to_owned().as_bytes(), time_pure.to_owned().as_bytes()].concat();
    new_content
}

