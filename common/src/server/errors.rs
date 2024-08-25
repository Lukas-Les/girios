use std::fmt;

pub enum SyntaxErrType {
    ValueMissing,
    UnknownCommand(String),
}

pub enum RequestErrorType {
    SyntaxErr(SyntaxErrType),
}

impl fmt::Display for RequestErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request err:")?;
        match self {
            RequestErrorType::SyntaxErr(SyntaxErrType::ValueMissing) => write!(f, "value missing"),
            RequestErrorType::SyntaxErr(SyntaxErrType::UnknownCommand(cmd)) => {
                write!(f, "{}", format!("unknown command: {}", cmd))
            }
        }
    }
}

pub enum ServerError {
    RequestError(RequestErrorType),
    DataBaseError(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::RequestError(e) => write!(f, "{}", e),
            ServerError::DataBaseError(e) => write!(f, "{}", e),
        }
    }
}
