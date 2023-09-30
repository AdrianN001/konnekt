use crate::shared::snapshot::{Snapshot, Changes};



pub trait Serializable { 

    fn serialize_to_bin(&self) -> Vec<u8>; 
    fn deserialize_from_bin(binary: &mut Vec<u8>) -> Self;
}

pub trait Changeable {

    fn changes_made_to_file(&self, snapshot: &Snapshot) -> Changes;

}