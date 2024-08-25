use std::sync::{Arc, Mutex};

use crate::dsa::char_tree::Tree;
use crate::server::commands::ServerCommand;


fn parse_request(request: &str) -> (ServerCommand, &str, Option<&str>) {
    let mut parts = request.split_whitespace();
    let command_str = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");
    let value = parts.next();

    // TODO: implement From trait for ServerCommand;
    let command = match command_str {
        "delete" => ServerCommand::Delete,
        "DELETE" => ServerCommand::Delete,
        "get" => ServerCommand::Get,
        "GET" => ServerCommand::Get,
        "isert" => ServerCommand::Insert,
        "INSERT" => ServerCommand::Insert,
        _ => ServerCommand::Nop,
    };
    (command, path, value)
}

pub async fn handle_request(tree: &Arc<Mutex<Tree>>, request: &str) -> String {
    let (command, path, value) = parse_request(request);

    let mut tree = tree.lock().unwrap(); // Acquire lock for safe mutable access
    match command {
        ServerCommand::Insert => {
            if let Some(v) = value {
                tree.insert(path, v);
                format!("inserted: {} -> {}", v, path)
            } else {
                format!("error: value was not provided")
            }
        },
        ServerCommand::Get => {
            if let Some(result) = tree.get(path) {
                format!("ok: {} -> {}", path, result)
            } else {
                format!("no data: {path} -> x")
            }
        }
        ServerCommand::Delete => {
            tree.deep_delete(path);
            format!("Deleted value at path {}\n", path)
        }
        _ => "Unknown command\n".to_string(),
    }
}
