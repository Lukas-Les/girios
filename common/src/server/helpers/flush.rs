use std::sync::MutexGuard;

use crate::dsa::char_tree::Tree;
use crate::server::errors::ServerError;
use crate::server::helpers::csv::dump_as_csv;


pub fn flush(tree: MutexGuard<Tree>) -> Result<(), ServerError>{
    let result = tree.scan();
    let file_name = format!("{}.csv", &tree.name);
    dump_as_csv(result, &file_name).map_err(ServerError::from_database)?;
    Ok(())
}
