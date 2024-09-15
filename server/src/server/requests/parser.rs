#[derive(Debug)]
pub enum RequestParserError{
    InvalidRequest,
}


enum DataStructureType {
    Ctree,
}

impl TryFrom<&str> for DataStructureType {
    type Error = RequestParserError;
    fn try_from(value: &str) -> Result<Self, RequestParserError> {
        match value {
            "ctree" => Ok(DataStructureType::Ctree),
            _ => Err(RequestParserError::InvalidRequest),
        }
    }
}


enum PlatformOpType {
    CreateStructure(DataStructureType),
    DestroyStructure(DataStructureType),
    Invalid,
}


pub enum RequestToken {
    PlatformOp(PlatformOpType),
    DsOp(String),
}


impl TryFrom<String> for RequestToken {
    type Error = RequestParserError;

    fn try_from(value: String) -> Result<Self, RequestParserError> {
        let (root_command, leftover) = match value.split_once(" ") {
            Some(result) => result,
            None => return Err(RequestParserError::InvalidRequest),
        };
        
        match root_command {
            "create" => Ok(RequestToken::PlatformOp(PlatformOpType::CreateStructure(DataStructureType::try_from(leftover)?))),
            "destroy" => Ok(RequestToken::PlatformOp(PlatformOpType::DestroyStructure(DataStructureType::try_from(leftover)?))),
            _ => Ok(RequestToken::DsOp(leftover.to_string())),
        }
    }
}


pub fn tokenize_request(request_bytes: &[u8]) -> Result<RequestToken, RequestParserError> {
    let request = match std::str::from_utf8(request_bytes) {
        Ok(v) => v,
        Err(_) => return Err(RequestParserError::InvalidRequest),
    };
    RequestToken::try_from(request.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_request() {
        let request = "create ctree";
        let result = tokenize_request(request.as_bytes());
        assert!(result.is_ok());
        match result.unwrap() {
            RequestToken::PlatformOp(PlatformOpType::CreateStructure(DataStructureType::Ctree)) => (),
            _ => panic!("unexpected result"),
        }
    }
}
