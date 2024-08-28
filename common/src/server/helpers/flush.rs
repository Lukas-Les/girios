use std::sync::{Arc, MutexGuard};

use crate::dsa::char_tree::Tree;
use crate::server::errors::ServerError;
use crate::server::helpers::csv::dump_as_csv;


pub fn flush(tree: MutexGuard<Tree>) -> Result<(), ServerError>{
    let result = tree.scan();
    dump_as_csv(result, &tree.name).map_err(ServerError::from_database);
    Ok(())
}
