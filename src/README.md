# MP4Reader - A Rust program for reading MP4 files
This is a Rust program for reading MP4 files in a streaming fashion. It allows you to read the file content in chunks, improving memory efficiency for large files.

## Features
- Reads MP4 files in chunks of 1024 bytes.
- Handles common file access errors like "file not found" and "permission denied".
- Returns detailed error information for debugging.

## Usage
1. Add the library to your Cargo.toml file:
Ini, TOML
[dependencies]
mp4reader = "0.1.0" // Replace with the actual version

2. Import the library and use the read_file function:
Rust
use mp4reader::read_file;

fn main() {
  let file_path = "/path/to/your/file.mp4";
  match read_file(file_path) {
    Ok(data) => {
      // Process the data here (e.g., iterate over bytes)
      for byte in data.iter() {
        println!("Byte: {}", byte);
      }
    }
    Err(err) => {
      println!("Error reading file: {:?}", err);
    }
  }
}

## Error Handling
The read_file function returns a Result type. On success, it returns a vector containing the entire file content as bytes. On error, it returns an MP4ReaderError variant with specific information:

MP4ReaderError::FileNotFound(String): File not found at the specified path.
MP4ReaderError::PermissionDenied(String): Permission denied while trying to access the file.
MP4ReaderError::IoError(Error): Other I/O errors that may occur during reading.
The error message provides details for troubleshooting.

## Limitations
This library currently only reads the raw byte content of the MP4 file.
It does not handle parsing the complex MP4 format for extracting specific information like video or audio streams.
This are the next steps
