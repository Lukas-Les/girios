pub enum ServerCommand {
    Delete,
    Insert,
    Get,
    Hit,
    Nop,
}

impl From<&str> for ServerCommand {
    fn from(value: &str) -> Self {
        match value {
            "delete" | "DELETE" => ServerCommand::Delete,
            "get" | "GET" => ServerCommand::Get,
            "hit" | "HIT" => ServerCommand::Hit,
            "insert" | "INSERT" => ServerCommand::Insert,
            _ => ServerCommand::Nop,
        }
    }
}
