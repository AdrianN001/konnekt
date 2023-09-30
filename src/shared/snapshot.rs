use std::{collections::HashMap, fs::File};

use crate::traits::file::{Changeable, Serializable};

use super::file::FileDescriptor;

use anyhow::Error;
use tokio::{fs, io::AsyncWriteExt};

pub enum Changes{
    Added,                  // It wan't in the prev. snapshot, but it is now
    Deleted,                // It was in the prev. snapshot, but now it isn't
    Renamed,                // Same content, different name
    Modified,                // Same name, but different content

    None                    // Nothing has Changed
}


type Changes_to_File<'a> = HashMap<&'a FileDescriptor, Changes>;
#[derive(Debug)]
pub struct Snapshot{
    pub files: Vec<FileDescriptor>
}


impl Snapshot { 

    pub async fn create() -> Result<Self, Error>{
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
            files: files_in_snapsh
        })
    }

    pub fn compare<'a>(&'a self, previous_snapshot:&'a Self) -> Changes_to_File {

        let mut changes: Changes_to_File = HashMap::new();
        
        
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

    pub async fn write_to_file(&self, file_name: &str) -> Result<(), Error> {
        let mut new_file = fs::File::create(file_name).await?;
        for snapshot_controlled_file in &self.files{
            let binary_output_of_the_file = snapshot_controlled_file.serialize_to_bin();
            new_file.write_all(&binary_output_of_the_file).await?;
            new_file.write_all(&"\n".as_bytes()).await;
        }
        Ok(())
     }
}