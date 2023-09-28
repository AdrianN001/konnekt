use crate::traits::server;
use anyhow::Result;
use std::{self, io::Read, thread};

use super::filebuilder::decompress_file;

// ----
//  8 bytes for the size
// 60 bytes for the file_name
// ----
type FileHeader = [u8; 132];
type CheckSum  = [u8; 64];
type FileNameHeader = [u8; 60];
type FileSizeHeader = [u8; 8];

pub struct FileShareService {
    listener: std::net::TcpListener,
}

#[derive(Debug)]
pub struct FileMetaData {
    pub file_size: u64,
    pub checksum: CheckSum,
    pub file_name: String,
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
            let mut header: FileHeader = [0; 132];

            stream.read(&mut header);

            // Debug
            // TODO Create something for println

            match FileMetaData::new(&header) {
                Ok(metadata) => {
                    metadata.print_info();

                    // Read the actuall file
                    thread::spawn(move ||{

                      


                            let mut whole_buffer: Vec<u8> = Vec::new();
                            

                            std::io::copy(&mut stream, &mut whole_buffer).unwrap();

                            println!("BODY LEN {}", &whole_buffer.len());
                            
                            if let Err(x) = decompress_file(&whole_buffer, metadata){
                                println!("{}", x);
                            }
                    });
                }, 
                Err(x) => {
                    println!("{}", x);
                    continue;
                }
            }

        }
    }
}

impl FileMetaData {
    fn new(header: &FileHeader) -> Result<Self, String> {
        let checksum: CheckSum = match header[0..64].try_into(){
            Ok(x) => x, 
            Err(x) => return Err("Couldn't fetch the checksum".to_string()),
        };

        let file_size_in_bytes: FileSizeHeader = match header[64..72].try_into() {
            Ok(x) => x,
            Err(x) => return Err("Couldn't fetch the file_size".to_string()),
        };
        let file_size_in_unsigned = u64::from_be_bytes(file_size_in_bytes);

        let file_name_in_bytes: FileNameHeader = header[72..132].try_into().unwrap();
        println!("{:?} FILENAME", &file_name_in_bytes);
        let filtered_filename: Vec<u8> = file_name_in_bytes.iter()
                                                           .filter(|&x| x.is_ascii() && *x != 0)
                                                           .map(|x| x.to_owned())
                                                           .collect();
        println!("{:?}", &filtered_filename);

        let file_name_in_string: String = match String::from_utf8(filtered_filename) {
            Ok(x) => x,
            Err(x) => return Err(format!("Couldn't fetch the file name : {}", x)),
        };
        println!("{}", &file_name_in_string);

        Ok(FileMetaData {
            checksum: checksum,
            file_size: file_size_in_unsigned,
            file_name: file_name_in_string.to_owned(),
        })
    }

    fn print_info(&self) -> () {
        println!(
            " You've received a new file ({}) \n The file size is '{}' ",
            self.file_name, self.file_size
        );
    }
}
