use std::{collections::HashMap };

use crate::traits::file::{Changeable, Serializable};

use super::file::FileDescriptor;

use anyhow::Error;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

#[derive(Debug)]
pub enum Changes{
    Added,                  // It wan't in the prev. snapshot, but it is now
    Deleted,                // It was in the prev. snapshot, but now it isn't
    Renamed,                // Same content, different name
    Modified,                // Same name, but different content

    Unmodified                    // Nothing has Changed
}


type ChangesToFile<'a> = HashMap<&'a FileDescriptor, Changes>;

#[derive(Debug)]
pub struct Snapshot{
    snapshot_id: String,
    pub files: Vec<FileDescriptor>
}


impl Snapshot { 
    // Creates a Snapshot from the current state of the directory
    pub async fn create(snapshot_id: &str) -> Result<Self, Error>{
        let base_dir = "/home/noirangel/Shared/";

        let paths = std::fs::read_dir(base_dir)?;

        let mut files_in_snapsh = vec![];
        for path in paths {
           
            if path.as_ref().unwrap().metadata()?.is_dir(){
                continue;
            }
            let filtered_file = FileDescriptor::new_from_fs(&path.unwrap().path().display().to_string()).await?;
            files_in_snapsh.push(filtered_file);
        }

        Ok(Self{
            snapshot_id: snapshot_id.to_string(),
            files: files_in_snapsh
        })
    }
    // Reads a previous snapshot from the file
    pub async fn read_from_fs(snapshot_id: &str) -> Result<Self, Error>{

        let snapshot_path = format!("/home/noirangel/Shared/.snapshots/{}.snsh", snapshot_id);

        let content_of_snapshot = fs::read(snapshot_path).await?;
        let lines_in_content = content_of_snapshot.split(| byte: &u8| *byte == (0x0A as u8) ); // 0x0A => New line
       
   

        let files = lines_in_content
                                        .filter(| slice: &&[u8] | slice.len() != 0) 
                                        .map( | file_metadata: &[u8] |  FileDescriptor::deserialize_from_bin(&mut file_metadata.to_vec().clone())).collect::<Vec<FileDescriptor>>();

        

        Ok(Self{
            snapshot_id:  snapshot_id.to_string(),
            files,
        })
    }
    // Writes the snapshot to fs
    pub async fn write_to_file(&self) -> Result<(), Error> {
        let mut new_file = fs::File::create(format!("/home/noirangel/Shared/.snapshots/{}.snsh", self.snapshot_id)).await?;
        for snapshot_controlled_file in &self.files{
            let binary_output_of_the_file = snapshot_controlled_file.serialize_to_bin();
            let _ = new_file.write_all(&binary_output_of_the_file).await?;
            let _ = new_file.write_all(&"\n".as_bytes()).await;
        }
        Ok(())
     }

}

impl Snapshot {
    // Compares the snapshot with an other one and returns a hashmap of FileDescriptor (:key) => Change (:value)
    pub fn compare<'a>(&'a self, previous_snapshot:&'a Self) -> ChangesToFile {

        let mut changes: ChangesToFile = HashMap::new();
        
        
        // Add the files that were: Modified, Added, Renamed, or wasn't changed
        for file in &self.files{
            let changed_since_the_last_snapsh = file.changes_made_to_file(&previous_snapshot);
            changes.insert(&file, changed_since_the_last_snapsh);
        }

        for file in &previous_snapshot.files{
            if !changes.contains_key(&file){
                changes.insert(&file, Changes::Deleted);
            }
        }
        changes
    }
}

impl Snapshot{ 
    pub async fn fetch_files_from_snapshot(&self) -> Vec<Result<File, std::io::Error>> {

        let mut result = vec![];
        for file in self.files.iter(){
            result.push(File::open(format!("/home/noirangel/Shared/{}", file.file_name)).await)
        }
        result

    }
}