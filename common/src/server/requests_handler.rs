use std::sync::{Arc, Mutex, MutexGuard};

use crate::dsa::char_tree::Tree;
use crate::server::commands::{FileType, ReadType, ReadWriteType, ServerCommand};
use crate::server::errors::{RequestErrorType, ServerError, SyntaxErrType};
use crate::server::helpers::flush::{flush, update_status_file};
use crate::server::response::ResponseStatus;


fn parse_request(request: &str) -> (ServerCommand, &str, Option<&str>) {
    let mut parts = request.split_whitespace();
    let command = ServerCommand::from(parts.next().unwrap_or(""));
    let path = parts.next().unwrap_or("");
    let value = parts.next();

    (command, path, value)
}

fn execute(
    mut tree: MutexGuard<Tree>,
    command: ServerCommand,
    path: &str,
    value: Option<&str>,
    request: &str,
) -> ResponseStatus {
    match command {
        ServerCommand::ReadWrite(ReadWriteType::Insert) => {
            if let Some(v) = value {
                println!("inserting value: {}", v);
                tree.insert(path, &v);
                ResponseStatus::Ok(format!("{} -> {}", path, v))
            } else {
                ResponseStatus::Error(ServerError::RequestError(RequestErrorType::SyntaxErr(
                    SyntaxErrType::ValueMissing,
                )))
            }
        }
        ServerCommand::Read(ReadType::Get) => {
            if let Some(result) = tree.get(path) {
                ResponseStatus::Ok(format!("{} -> {}", path, result))
            } else {
                ResponseStatus::NoData(format!("{} ->", path))
            }
        }
        ServerCommand::Read(ReadType::Hit) => {
            if let Some(result) = tree.hit(path) {
                ResponseStatus::Ok(format!("{} -> {}", path, result))
            } else {
                ResponseStatus::NoData(format!("{} ->", path))
            }
        }
        ServerCommand::Read(ReadType::Scan) => {
            let result = tree.scan();
            let output = result
                .iter()
                .map(|(path, value)| format!("{} -> {}", path, value))
                .collect::<Vec<_>>()
                .join("\n");
            ResponseStatus::Ok(output)
        }
        ServerCommand::ReadWrite(ReadWriteType::Delete) => {
            tree.deep_delete(path);
            ResponseStatus::Ok(format!("deleted: {}", path))
        }
        ServerCommand::Read(ReadType::Dump(file_type)) => {
            //TODO: move this method to client side, and create write to disc method instead.
            match file_type {
                FileType::Csv => match flush(tree) {
                    Ok(()) => ResponseStatus::Ok("tree saved as csv".to_string()),
                    Err(e) => ResponseStatus::Error(ServerError::DataBaseError(e.to_string())),
                },
            }
        }
        _ => ResponseStatus::Error(ServerError::RequestError(RequestErrorType::SyntaxErr(
            SyntaxErrType::UnknownCommand(request.to_string()),
        ))),
    }
}

pub async fn handle_request(tree: &Arc<Mutex<Tree>>, request: &str) -> ResponseStatus {
    let (command, path, value) = parse_request(request);
    let tree = tree.lock();
    //TODO: implement recovery from poisoned mutex;
    match tree {
        Ok(tree) => execute(tree, command, path, value, request),
        Err(e) => ResponseStatus::Error(ServerError::DataBaseError(e.to_string())),
    }
}
