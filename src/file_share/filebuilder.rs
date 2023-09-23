
use std::fs::File;
use std::net::TcpStream;

use anyhow::Error;
use super::metadata::FileMetaData;



pub fn create_file_from_metadata( file: &FileMetaData, tcp_stream: &mut TcpStream) -> Result<(), Error>{
    let mut new_file = File::create(file.file_name.clone())?;
    let _ = std::io::copy(tcp_stream, &mut new_file);
    Ok(())
}