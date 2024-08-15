use sha3::digest::block_buffer::Error;
use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::{Result, BufReader, Read};

pub fn generate_hash(file : &File) -> std::io::Result<String> {
    // create a SHA3-256 object
    let mut reader = BufReader::new(file);
    let mut hasher = Sha3_256::new();
    let mut buffer = [0; 1024];

    loop {
        let read = reader.read(&mut buffer);
        match read {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    // read hash digest
    let hash = hasher.finalize().to_vec();
    Ok(hex::encode(hash))
}