mod mp4_reader;

use mp4_reader::read_file;
fn main() {
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
