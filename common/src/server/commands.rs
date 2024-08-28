pub enum FileType {
    Csv,
}

pub enum ReadWriteType {
    Delete,
    Insert,
}

pub enum ReadType {
    Get,
    Hit,
    Scan,
    Dump(FileType),
}


pub enum ServerCommand {
    ReadWrite(ReadWriteType),
    Read(ReadType),
    Nop,
}

impl From<&str> for ServerCommand {
    fn from(value: &str) -> Self {
        match value {
            "delete" | "DELETE" => ServerCommand::ReadWrite(ReadWriteType::Delete),
            "get" | "GET" => ServerCommand::Read(ReadType::Get),
            "hit" | "HIT" => ServerCommand::Read(ReadType::Hit),
            "insert" | "INSERT" => ServerCommand::ReadWrite(ReadWriteType::Insert),
            "scan" | "SCAN" => ServerCommand::Read(ReadType::Scan),
            "dump" | "DUMP" => ServerCommand::Read(ReadType::Dump(FileType::Csv)),
            _ => ServerCommand::Nop,
        }
    }
}
