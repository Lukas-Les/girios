use std::sync::MutexGuard;

use common::dsa::char_tree::CharTree;
use crate::server::errors::ServerError;
use crate::server::helpers::csv::{dump_as_csv, update_file};


pub fn flush(tree: MutexGuard<CharTree>) -> Result<(), ServerError>{
    let result = tree.scan();
    let file_name = format!("{}.csv", &tree.name);
    dump_as_csv(result, &file_name).map_err(ServerError::from_database)?;
    Ok(())
}


pub fn update_status_file(path: &str, value: &str, out_file_name: &str) {
    let out_file_name = format!("{}.csv", out_file_name);
    update_file(path, value, &out_file_name);
}
