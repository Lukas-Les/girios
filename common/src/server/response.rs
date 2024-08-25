use std::fmt;

use crate::server::errors::ServerError;


pub enum ResponseStatus {
    Ok(String),
    NoData(String),
    Error(ServerError),
}


impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseStatus::Ok(ref s) => writeln!(f, "ok:{}", s),
            ResponseStatus::NoData(ref s) => writeln!(f, "nodata:{}", s),
            ResponseStatus::Error(ref e) => writeln!(f, "e: {}", e),
        }
    }
}

impl ResponseStatus {
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
