use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};

#[derive(Debug)]
pub enum MP4ReaderError {
    FileNotFound(String),
    PermissionDenied(String),
    IoError(Error),
}

pub fn read_file(file_path: &str ) -> Result<Vec<u8>, MP4ReaderError>{
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Err(MP4ReaderError::FileNotFound(file_path.to_owned())),
            ErrorKind::PermissionDenied => return Err(MP4ReaderError::PermissionDenied(file_path.to_owned())),
            _ => return Err(MP4ReaderError::IoError(e)),            
        }
    };

    // read file into BufReader
    let mut reader = BufReader::new(file);
    
    let mut chunks = Vec::with_capacity(1024*1024*10);
    //loop reads data from a file in chunks
    loop{
        let mut chunk = [0; 1024*1024];
        match reader.read(&mut chunk) {
            Ok(n)=>{
                // Collect the read bytes into chunks vector
                chunks.extend_from_slice(&chunk[..n]);
                if n == 0 {
                    break;
                }
            }
            Err(e) => return Err(MP4ReaderError::IoError(e)),
        }
    }
    Ok(chunks) // Return the collected data as a Vec<u8>
}