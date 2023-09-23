
use anyhow::Error;
use std::path::Path;
use log;


// ----
//  8 bytes for the size
// 60 bytes for the file_name
// ----
pub type FileHeader = [u8; 68];
pub type FileNameHeader = [u8; 60];
pub type FileSizeHeader = [u8; 8];


pub struct FileMetaData {
    pub file_size: u64,
    pub file_name: String,
}



impl FileMetaData {
    pub fn from_bit_represention(header: &FileHeader) -> Result<Self, Error> {

        let file_size_in_bytes: FileSizeHeader = match header[0..8].try_into() {
            Ok(x) => x,
            Err(x) => return Err(x.into()),
        };
        let file_size_in_unsigned = u64::from_be_bytes(file_size_in_bytes);

        let file_name_in_bytes: FileNameHeader = header[8..header.len()].try_into().unwrap();

        let file_name_in_string: String = match String::from_utf8(file_name_in_bytes.to_vec()) {
            Ok(x) => x,
            Err(x) => return Err(x.into()),
        };

        Ok(FileMetaData {
            file_size: file_size_in_unsigned,
            file_name: file_name_in_string.to_owned(),
        })
    }

    pub fn as_bit_representation(&self) -> Vec<u8>{



        let mut file_size_in_bytes = u64::to_be_bytes(self.file_size as u64).to_vec();
        file_size_in_bytes.resize(8, 0);

        let mut file_name_in_bytes = self.file_name.as_bytes().to_vec();
        
        file_name_in_bytes.resize(60, 0);

        file_size_in_bytes.append(&mut file_name_in_bytes);
        file_size_in_bytes


    }

    pub fn from_file_system(file_path: &str) -> Result<Self, Error>{

        // Read the file 
        let file = std::fs::File::open(file_path)?;


        // Get the size and the name of the file
        let size = file.metadata().unwrap().len();
        let name = {
                        let path = Path::new(file_path);
                        let filename = path.file_name().unwrap();
                
                        filename.to_str().unwrap().to_owned()
        };


        Ok(FileMetaData{
            file_size: size,
            file_name: name
        })
    }

    pub fn log_info(&self) -> () {
        log::info!(
            " You've received a new file ({}) \n The file size is '{}' ",
            self.file_name, self.file_size
        );
    }
}
