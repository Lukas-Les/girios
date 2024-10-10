use std::sync::Arc;

use common::dsa::char_tree::CharTree;
use log::{debug, info};
use tokio::sync::RwLock;

use crate::platform::Platform;

fn split_once_or_err<'a>(
    input: &'a str,
    delimiter: &'a str,
) -> Result<(&'a str, &'a str), RequestParserError> {
    input
        .split_once(delimiter)
        .ok_or(RequestParserError::InvalidRequest)
}

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
        let (structure_type, structure_name) = split_once_or_err(&value, " ")?;
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
}

#[derive(Debug)]
pub enum ListableType {
    Ctrees,
}

impl TryFrom<String> for ListableType {
    type Error = RequestParserError;

    fn try_from(value: String) -> Result<Self, RequestParserError> {
        match value.as_str() {
            "ctrees" => Ok(ListableType::Ctrees),
            _ => Err(RequestParserError::InvalidRequest),
        }
    }
}

#[derive(Debug)]
pub enum PlatformReadOpType {
    ListStructures(ListableType),
}

#[derive(Debug)]
pub enum CtreeOpType {
    Insert {
        target: String,
        key: String,
        value: String,
    },
    Remove {
        target: String,
        key: String,
    },
    Get {
        target: String,
        key: String,
    },
    Hit {
        target: String,
        key: String,
    },
    Scan {
        target: String,
    },
}

impl TryFrom<String> for CtreeOpType {
    type Error = RequestParserError;

    fn try_from(value: String) -> Result<Self, RequestParserError> {
        if value.is_empty() {
            debug!("CtreeOpType from string: empty");
            return Err(RequestParserError::InvalidRequest);
        }

        debug!("CtreeOpType from string: {}EOL", &value);

        let (target, leftover) = split_once_or_err(&value, " ")?;

        if leftover == "scan" {
            return Ok(CtreeOpType::Scan {
                target: target.to_owned(),
            });
        }

        let (operation, key_value) = split_once_or_err(leftover, " ")?;
        match operation {
            "insert" => {
                let (key, value) = split_once_or_err(key_value, " ")?;
                Ok(CtreeOpType::Insert {
                    target: target.to_owned(),
                    key: key.to_owned(),
                    value: value.to_owned(),
                })
            }
            "remove" => Ok(CtreeOpType::Remove {
                target: target.to_owned(),
                key: key_value.to_owned(),
            }),
            "get" => Ok(CtreeOpType::Get {
                target: target.to_owned(),
                key: key_value.to_owned(),
            }),
            "hit" => Ok(CtreeOpType::Hit {
                target: target.to_owned(),
                key: key_value.to_owned(),
            }),
            _ => Err(RequestParserError::InvalidRequest),
        }
    }
}

#[derive(Debug)]
pub enum RequestToken {
    PlatformRwOp(PlatformRwOpType),
    PlatformReadOp(PlatformReadOpType),
    CtreeOp(CtreeOpType),
}
impl RequestToken {
    fn from_string(value: String) -> Result<Self, RequestParserError> {
        debug!("Received input: {}", value);
        let (root_command, leftover_str) = split_once_or_err(&value, " ")?;
        let leftover = leftover_str.to_string();
        match root_command {
            "create" => Ok(RequestToken::PlatformRwOp(
                PlatformRwOpType::CreateStructure(DataStructureType::try_from(leftover)?),
            )),
            "destroy" => Ok(RequestToken::PlatformRwOp(
                PlatformRwOpType::DestroyStructure(DataStructureType::try_from(leftover)?),
            )),
            "list" => Ok(RequestToken::PlatformReadOp(
                PlatformReadOpType::ListStructures(ListableType::try_from(leftover)?),
            )),
            "ctree" => Ok(RequestToken::CtreeOp(CtreeOpType::try_from(leftover)?)),
            _ => Err(RequestParserError::InvalidRequest),
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
        let request_str = request_str.trim();
        Self::from_string(request_str.to_string())
    }
}

pub async fn process_token(
    token: RequestToken,
    platform: &Arc<RwLock<Platform>>,
) -> Result<String, String> {
    println!("Processing token: {:?}", token);
    match token {
        RequestToken::PlatformRwOp(PlatformRwOpType::CreateStructure(
            DataStructureType::Ctree { name },
        )) => {
            let platforn_lock = platform.write().await;
            let data_structures_lock = platforn_lock.data_structures.write().await;
            data_structures_lock
                .insert_ctree(CharTree::new(name.clone()))
                .await;
            Ok(format!("Ctree {} created", name))
        }
        RequestToken::PlatformRwOp(PlatformRwOpType::DestroyStructure(
            DataStructureType::Ctree { name },
        )) => {
            let platforn_lock = platform.write().await;
            let data_structures_lock = platforn_lock.data_structures.write().await;
            data_structures_lock.remove_ctree(&name).await;
            Ok(format!("Ctree {} removed", name))
        }
        RequestToken::CtreeOp(CtreeOpType::Insert { target, key, value }) => {
            let platforn_lock = platform.write().await;
            let data_structures_lock = platforn_lock.data_structures.write().await;
            let ctree = data_structures_lock.get_ctree(&target).await;
            if ctree.is_none() {
                return Err("Ctree not found".to_string());
            }
            let ctree_lock = ctree.unwrap();
            let mut ctree_write = ctree_lock.write().await;
            ctree_write.insert(&key, &value);
            Ok(format!("Key {} inserted", key))
        }
        RequestToken::CtreeOp(CtreeOpType::Remove { target, key }) => {
            let platforn_lock = platform.write().await;
            let data_structures_lock = platforn_lock.data_structures.write().await;
            let ctree = match data_structures_lock.get_ctree(&target).await {
                Some(ctree) => ctree,
                None => return Err("Ctree not found".to_string()),
            };
            let mut ctree_write = ctree.write().await;
            ctree_write.deep_delete(&key);
            Ok(format!("Key {} removed", key))
        }
        RequestToken::CtreeOp(CtreeOpType::Get { target, key }) => {
            let platforn_lock = platform.read().await;
            let data_structures_lock = platforn_lock.data_structures.read().await;
            let ctree = data_structures_lock.get_ctree(&target).await;
            if ctree.is_none() {
                info!("Ctree {} not found", &target);
                return Err("Ctree not found".to_string());
            }
            dbg!(&ctree);
            let ctree_lock = ctree.unwrap();
            let ctree_read = ctree_lock.read().await;
            let value = ctree_read.get(&key);
            println!("Value: {:?}", value);
            match value {
                Some(value) => Ok(value.clone()),
                None => Err("Key not found".to_string()),
            }
        }
        RequestToken::CtreeOp(CtreeOpType::Hit { target, key }) => {
            let platforn_lock = platform.read().await;
            let data_structures_lock = platforn_lock.data_structures.read().await;
            let ctree = data_structures_lock.get_ctree(&target).await;
            if ctree.is_none() {
                return Err("Ctree not found".to_string());
            }
            let ctree_lock = ctree.unwrap();
            let ctree_read = ctree_lock.read().await;
            ctree_read.hit(&key);
            Ok("Key hit".to_string())
        }
        RequestToken::CtreeOp(CtreeOpType::Scan { target }) => {
            let platforn_lock = platform.read().await;
            let data_structures_lock = platforn_lock.data_structures.read().await;
            let ctree = data_structures_lock.get_ctree(&target).await;
            if ctree.is_none() {
                return Err("Ctree not found".to_string());
            }
            let ctree_lock = ctree.unwrap();
            let ctree_read = ctree_lock.read().await;
            let keys = ctree_read.scan();
            Ok(format!("{:?}", keys))
        }
        RequestToken::PlatformReadOp(PlatformReadOpType::ListStructures(ListableType::Ctrees)) => {
            let platforn_lock = platform.read().await;
            let data_structures_lock = platforn_lock.data_structures.read().await;
            let ctrees = data_structures_lock.get_all_ctrees().await;
            Ok(format!("{:?}", ctrees))
        }

        _ => Err("Invalid request".to_string()),
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
