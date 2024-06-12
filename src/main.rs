mod file_encryption;
mod mp4_reader;

use file_encryption::encrypt_file;
use mp4_reader::read_file;
fn main() {
    let metadata = b"title: My Movie, author: John Doe";
    match read_file(
        "/home/wanjiku/Development/code/rust/streaming_baased_reading/src/How to use FFMPEG.mp4",
    ) {
        Ok(data) => {
            //Process the data (e'g print the bytes)
            // println!("Read the bytes: {:?}", &data);
            match encrypt_file(
                &data,
                "/home/wanjiku/Development/code/rust/streaming_baased_reading/src",
                metadata,
            ) {
                Ok(_) => println!("File encrypted Successfully."),
                Err(e) => println!("Error encrypting file {:?}", e),
            }
        }
        Err(e) => {
            println!("Error reading file: {:?}", e)
        }
    }
}
