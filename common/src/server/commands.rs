pub enum ServerCommand {
    Delete,
    Insert,
    Get,
    Hit,
    Scan,
    Nop,
}

impl From<&str> for ServerCommand {
    fn from(value: &str) -> Self {
        match value {
            "delete" | "DELETE" => ServerCommand::Delete,
            "get" | "GET" => ServerCommand::Get,
            "hit" | "HIT" => ServerCommand::Hit,
            "insert" | "INSERT" => ServerCommand::Insert,
            "scan" | "SCAN" => ServerCommand::Scan,
            _ => ServerCommand::Nop,
        }
    }
}
