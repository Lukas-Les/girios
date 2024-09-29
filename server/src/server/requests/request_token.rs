use common::dsa::char_tree::CharTree;

use crate::platform;

#[derive(PartialEq, Debug)]
pub enum RequestParserError {
    InvalidRequest,
    FailedToReadBytes(String),
}

impl RequestParserError {
    fn from_request(err: String) -> Self {
        Self::FailedToReadBytes(err)
    }
}

#[derive(Debug)]
pub enum DataStructureType {
    Ctree { name: String },
}

impl TryFrom<String> for DataStructureType {
    type Error = RequestParserError;
    fn try_from(value: String) -> Result<Self, RequestParserError> {
        let (structure_type, structure_name) = match value.split_once(" ") {
            Some(result) => result,
            None => return Err(RequestParserError::InvalidRequest),
        };
        match structure_type {
            "ctree" => Ok(DataStructureType::Ctree {
                name: structure_name.to_string(),
            }),
            _ => Err(RequestParserError::InvalidRequest),
        }
    }
}

#[derive(Debug)]
pub enum PlatformRwOpType {
    CreateStructure(DataStructureType),
    DestroyStructure(DataStructureType),
    Invalid,
}

impl PlatformRwOpType {
    fn execute(&self, data_structures: &mut platform::DataStructures) {
        match self {
            PlatformRwOpType::CreateStructure(DataStructureType::Ctree { name }) => {
                let new_ctree = CharTree::new(name.clone());
                data_structures.insert_ctree(new_ctree);
            }
            PlatformRwOpType::DestroyStructure(DataStructureType::Ctree { name }) => {
                // Remove CharTree from DataStructures
                data_structures.remove_ctree(name.clone());
            }
            _ => return,
        }
    }
}


#[derive(Debug)]
pub enum RequestToken {
    PlatformRwOp(PlatformRwOpType),
    DsOp(String),
}

impl RequestToken {
    fn from_string(value: String) -> Result<Self, RequestParserError> {
        let (root_command, leftover_str) = match value.split_once(" ") {
            Some(result) => result,
            None => return Err(RequestParserError::InvalidRequest),
        };
        let leftover = leftover_str.to_string();
        match root_command {
            "create" => Ok(RequestToken::PlatformRwOp(
                PlatformRwOpType::CreateStructure(DataStructureType::try_from(leftover)?),
            )),
            "destroy" => Ok(RequestToken::PlatformRwOp(
                PlatformRwOpType::DestroyStructure(DataStructureType::try_from(leftover)?),
            )),
            _ => Ok(RequestToken::DsOp(leftover.to_string())),
        }
    }
}

impl TryFrom<String> for RequestToken {
    type Error = RequestParserError;

    fn try_from(value: String) -> Result<Self, RequestParserError> {
        Self::from_string(value)
    }
}

impl TryFrom<&[u8]> for RequestToken {
    type Error = RequestParserError;

    fn try_from(value: &[u8]) -> Result<Self, RequestParserError> {
        let request_str = std::str::from_utf8(value)
            .map_err(|e| RequestParserError::from_request(e.to_string()))?;
        Self::from_string(request_str.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_request() {
        let request = "create ctree my_tree".to_string();
        let result = RequestToken::from_string(request);
        assert!(result.is_ok());
        match result.unwrap() {
            RequestToken::PlatformRwOp(PlatformRwOpType::CreateStructure(
                DataStructureType::Ctree { name },
            )) => {
                assert_eq!(name, "my_tree".to_string())
            }
            _ => panic!("unexpected result"),
        }
    }
}
