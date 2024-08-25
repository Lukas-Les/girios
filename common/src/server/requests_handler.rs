use std::sync::{Arc, Mutex};

use crate::dsa::char_tree::Tree;
use crate::server::commands::ServerCommand;
use crate::server::errors::{RequestErrorType, ServerError, SyntaxErrType};
use crate::server::response::ResponseStatus;


fn parse_request(request: &str) -> (ServerCommand, &str, Option<&str>) {
    let mut parts = request.split_whitespace();
    let command = ServerCommand::from(parts.next().unwrap_or(""));
    let path = parts.next().unwrap_or("");
    let value = parts.next();

    (command, path, value)
}


pub async fn handle_request(tree: &Arc<Mutex<Tree>>, request: &str) -> ResponseStatus {
    let (command, path, value) = parse_request(request);

    let mut tree = tree.lock().unwrap(); // Acquire lock for safe mutable access
    match command {
        ServerCommand::Insert => {
            if let Some(v) = value {
                tree.insert(path, v);
                ResponseStatus::Ok(format!("{} -> {}", v, path))
            } else {
                ResponseStatus::Error(ServerError::RequestError(RequestErrorType::SyntaxErr(SyntaxErrType::ValueMissing)))
            }
        },
        ServerCommand::Get => {
            if let Some(result) = tree.get(path) {
                ResponseStatus::Ok(format!("{} -> {}", path, result))
            } else {
                ResponseStatus::NoData(format!("{} -> x", path))
            }
        },
        ServerCommand::Hit => {
            if let Some(result) = tree.hit(path) {
                ResponseStatus::Ok(format!("{} -> {}", path, result))
            } else {
                ResponseStatus::NoData(format!("{} -> x", path))
            }
        },
        ServerCommand::Delete => {
            tree.deep_delete(path);
            ResponseStatus::Ok(format!("deleted: {}", path))
        },
        _ => ResponseStatus::Error(ServerError::RequestError(RequestErrorType::SyntaxErr(SyntaxErrType::UnknownCommand(request.to_string())))),
    }
}
