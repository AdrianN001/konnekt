use anyhow::Error;
use std::fs::*;
use std::io::Write;
use std::net::*;

use super::metadata::FileMetaData;

// Will be called by the gui
#[allow(dead_code)]
pub fn send_file(ip_address: String, port: i32, file_name: String) -> Result<(), Error> {
    let mut stream = TcpStream::connect(format!("{}:{}", ip_address, port))?;
    let mut file_hook = File::open(&file_name)?;

    let metadata = FileMetaData::from_file_system(&file_name)?;
    let header_packet = metadata.as_bit_representation();

    let _ = stream.write_all(&header_packet);

    let _ = std::io::copy(&mut file_hook, &mut stream );

    Ok(())
}
