use crate::traits::server;
use anyhow::Result;
use std::{self, io::Read, thread};
use crate::file_share::metadata::*;

use super::filebuilder::create_file_from_metadata;

// ----
//  8 bytes for the size
// 60 bytes for the file_name
// ----

pub struct FileShareService {
    listener: std::net::TcpListener,
}



impl server::ConnectableService for FileShareService {
    fn new(port: i32, debug_text: &str) -> Result<FileShareService, String>
    where
        FileShareService: Sized,
    {
        let tcp_listener = match std::net::TcpListener::bind(format!("0.0.0.0:{}", port)) {
            Ok(x) => x,
            Err(x) => return Err(x.to_string()),
        };

        let new_instance = FileShareService {
            listener: tcp_listener,
        };
        println!("{}", debug_text);
        return Ok(new_instance);
    }

    fn start(&mut self) -> ! {

        loop {
            let (mut stream, _addr) = self.listener.accept().unwrap();

            // Read the header from the file;
            let mut header: FileHeader = [0; 68];

            let _ = stream.read_exact(&mut header).unwrap();

            // Debug
            // TODO Create something for println
            if let Ok(metadata) = FileMetaData::from_bit_represention(&header) {
                thread::spawn(move ||{
                    metadata.log_info();

                    let _ = create_file_from_metadata(&metadata, &mut stream);
                });
               
            }
        }
    }
}

