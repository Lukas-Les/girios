pub enum ClientError {
    FailedConfig(String),
}


impl ClientError {
    pub fn from_failed_config(err: String) -> Self {
        ClientError::FailedConfig(err)
    }
}


impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::FailedConfig(e) => write!(f, "{}", e)
        }
    }
}
