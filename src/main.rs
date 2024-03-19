use std::fs::File;
use std::io::{BufReader, Error, Read};

fn read_file(file_path: &str ) -> Result<Vec<u8>, Error>{
    let mut file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    
    let mut chunks = Vec::new();
    loop{
        let mut chunk = [0; 1024];
        match reader.read(&mut chunk) {
            Ok(n)=>{
                chunks.extend_from_slice(&chunk[..n]);
                if n == 0 {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(chunks)
}

fn main() {
    println!("Hello, world!");
    match read_file("/home/wanjiku/Development/code/rust/streaming_baased_reading/src/How to use FFMPEG.mp4") {
        Ok(data) => {
        //Process the data (e'g print the bytes)
        println!("Read the bytes: {:?}", &data);
        }
        Err(e) => {
        println!("Error reading file: {:?}", e)
        }      
   } 
}
