use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::{MutexGuard};

use bincode;

use crate::dsa::char_tree::Tree;
use crate::server::errors::ServerError;


pub fn serialize_node(tree: MutexGuard<Tree>, filename: &str) -> Result<(), ServerError> {
    let encoded: Vec<u8> = bincode::serialize(&*tree).unwrap();
    let mut file = File::create(filename).map_err(ServerError::from_io)?;
    file.write_all(&encoded).map_err(ServerError::from_io)?;
    Ok(())
}


// pub fn deserialize_node(filename: &str) -> Result<(), ServerError>  {
//     let mut file = File::open(filename)?;
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer)?;
//     let node: tree = bincode::deserialize(&buffer[..]).unwrap();
//     Ok(tree)
// }
