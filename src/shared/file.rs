
use std::{path::Path};
use tokio::fs::{self, File};
use sha256::try_digest;


use std::hash::{Hash};

use anyhow::Error;

use crate::traits::file::{Changeable, Serializable};
use super::snapshot::{Snapshot, Changes};


#[derive( Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileDescriptor{
    pub file_name: String,
    file_size: u64,
    checksum: String
}


impl FileDescriptor {

    pub async fn new_from_fs(path: &str) -> Result<Self, Error>{

          // Read the file 
        let file = fs::File::open(path).await?;


        // Get the size and the name of the file
        let size = file.metadata().await.unwrap().len();
        let name = {
                        let path = Path::new(path);
                        let filename = path.file_name().unwrap();
                
                        filename.to_str().unwrap().to_owned()
        };

        let checksum = calculate_checksum(path).await?;
  
  
          Ok(Self{
              file_size: size,
              file_name: name,
              checksum
          })

    }

    pub fn same_file(file_a: &Self,file_b: &Self) -> bool{
        return ( file_a.checksum == file_b.checksum ) || ( file_a.file_name == file_b.file_name );
    }
    
}

impl Serializable for FileDescriptor {

    fn serialize_to_bin(&self) -> Vec<u8>{
        let mut buffer: Vec<u8> = vec![];

        buffer.append(&mut self.file_name.as_bytes().to_vec());
        buffer.append(&mut b"<>".to_vec()); // delimiter

        buffer.append(&mut self.file_size.to_be_bytes().to_vec());
        buffer.append(&mut b"<>".to_vec());

        buffer.append(&mut self.checksum.as_bytes().to_vec());
        buffer.append(&mut b"<>".to_vec());


        buffer
    }
    fn deserialize_from_bin(file: &mut Vec<u8>) -> Self {       

        let mut buffer: Vec<Vec<u8>> = vec![];
        buffer.push(vec![]);
        buffer.push(vec![]);
        buffer.push(vec![]);
        
        // File name 
        loop {
            let current_element = file.remove(0);
            if current_element == b'<' && file[0] == b'>'{
                file.remove(0);
                break;
            }
            buffer[0].push(current_element);
        }
        // File size
        loop {
            let current_element = file.remove(0);
            if current_element == b'<' && file[0] == b'>'{
                file.remove(0);
                break;
            }
            buffer[1].push(current_element);
        }
        // Checksum
        loop {
            let current_element = file.remove(0);
            if current_element == b'<' && file[0] == b'>'{
                file.remove(0);
                break;
            }
            buffer[2].push(current_element);
        }

        let file_name = std::str::from_utf8(& buffer[0]).unwrap().to_string();
        let file_size = u64::from_be_bytes(buffer[1].clone().try_into().unwrap());
        let checksum = std::str::from_utf8(& buffer[2]).unwrap().to_string();

        Self { file_name, file_size, checksum }

        

    }

    
}

impl Changeable for FileDescriptor{
    //
    // Can return: None/ Modified/ Renamed/ Added
    // If a file is in a previous snapshot, but it isn't it now
    // That means it has been Deleted 
    //

    fn changes_made_to_file(&self, last_snapshot: &Snapshot ) -> Changes{

        for file in last_snapshot.files.iter(){
            if file.file_name == self.file_name && self.checksum == file.checksum {
                return Changes::Unmodified;
            }else if file.file_name == self.file_name && self.checksum != file.checksum {
                return Changes::Modified;
            }else if file.file_name != self.file_name && self.checksum == file.checksum {
                return Changes::Renamed;
            }
        }
        Changes::Added
    }
}

pub async fn calculate_checksum(file_path: &str) -> Result<String, Error> {
    let path = Path::new(file_path);
    let chksum = try_digest(path)?;
    Ok(chksum)
}
