use std::sync::Arc;

use tokio::sync::RwLock;

use common::dsa::char_tree::CharTree;

use crate::{platform::Platform, server::requests::request_token::{RequestToken, PlatformRwOpType, CtreeOpType, DataStructureType}};

#[derive(Debug)]
enum State {
    AwaitingRequest,
    ProcessingRequest,
    ExecutingOperation,
    Error(String),
    Finished,
}

#[derive(Debug)]
pub enum Event {
    IncomingToken(RequestToken),
    InvalidToken,
    OperationSuccess(String),
    OperationFailed(String),
}

#[derive(Debug)]
pub struct FSM {
    state: State,
}

impl FSM {
    pub fn new() -> Self {
        Self {
            state: State::AwaitingRequest,
        }
    }

    pub async fn process_event(&mut self, event: Event, platform: &Arc<RwLock<Platform>>) {
        println!("Processing event: {:?}", event);
        println!("Current state: {:?}", self.state);
        println!("Platform: {:?}", platform);
        match (&self.state, event) {
            (State::AwaitingRequest, Event::IncomingToken(token)) => {
                self.state = State::ProcessingRequest;
                let result = self.process_token(token, platform).await;
                match result {
                    Event::OperationSuccess(_) => self.state = State::AwaitingRequest,
                    Event::OperationFailed(_) => self.state = State::Error("Operation failed".to_string()),
                    _ => (),
                }
            }
            (State::AwaitingRequest, Event::InvalidToken) => {
                self.state = State::Error("Invalid token received".to_string());
            }
            (State::ProcessingRequest, Event::OperationSuccess(s)) => {
                self.state = State::ExecutingOperation;
            }
            (State::ProcessingRequest, Event::OperationFailed(err)) => {
                self.state = State::Error(err);
            }
            (State::ExecutingOperation, Event::OperationSuccess(s)) => {
                self.state = State::Finished;
            }
            (State::ExecutingOperation, Event::OperationFailed(err)) => {
                self.state = State::Error(err);
            }
            _ => (),
        }
    }

    async fn process_token(&mut self, token: RequestToken, platform: &Arc<RwLock<Platform>>) -> Event {
        println!("Processing token: {:?}", token);
        match token {
            RequestToken::PlatformRwOp(PlatformRwOpType::CreateStructure(DataStructureType::Ctree { name })) => {
                println!("Creating ctree");
                let platforn_lock = platform.write().await;
                let data_structures_lock = platforn_lock.data_structures.write().await;
                data_structures_lock.insert_ctree(CharTree::new(name.clone())).await;
                Event::OperationSuccess("Ctree created".to_string())
            }
            RequestToken::PlatformRwOp(PlatformRwOpType::DestroyStructure(DataStructureType::Ctree { name })) => {
                println!("Removing ctree");
                let platforn_lock = platform.write().await;
                let data_structures_lock = platforn_lock.data_structures.write().await;
                data_structures_lock.remove_ctree(&name).await;
                Event::OperationSuccess("Ctree removed".to_string())
            }
            RequestToken::CtreeOp(CtreeOpType::Insert { target, key, value }) => {
                println!("Insert request");
                let platforn_lock = platform.write().await;
                let data_structures_lock = platforn_lock.data_structures.write().await;
                let mut ctree = data_structures_lock.get_ctree(&target).await;
                if ctree.is_none() {
                    return Event::OperationFailed("Ctree not found".to_string());
                }
                let ctree_lock = ctree.unwrap();
                let mut ctree_write = ctree_lock.write().await;
                ctree_write.insert(&key, &value);
                Event::OperationSuccess("Key inserted".to_string())
            }
            RequestToken::CtreeOp(CtreeOpType::Remove { target, key }) => {
                println!("Remove request");
                let platforn_lock = platform.write().await;
                let data_structures_lock = platforn_lock.data_structures.write().await;
                let mut ctree = match data_structures_lock.get_ctree(&target).await {
                    Some(ctree) => ctree,
                    None => return Event::OperationFailed("Ctree not found".to_string()),
                };
                let mut ctree_write = ctree.write().await;
                ctree_write.deep_delete(&key);
                Event::OperationSuccess("Key removed".to_string())
            }
            RequestToken::CtreeOp(CtreeOpType::Get { target, key }) => {
                println!("Get request");
                let platforn_lock = platform.write().await;
                let data_structures_lock = platforn_lock.data_structures.write().await;
                let ctree = data_structures_lock.get_ctree(&target).await;
                if ctree.is_none() {
                    println!("Ctree not found");
                    return Event::OperationFailed("Ctree not found".to_string());
                }
                dbg!(&ctree);
                let ctree_lock = ctree.unwrap();
                let ctree_read = ctree_lock.read().await;
                let value = ctree_read.get(&key);
                println!("Value: {:?}", value);
                match value {
                    Some(value) => Event::OperationSuccess(value.clone()),
                    None => Event::OperationFailed("Key not found".to_string()),
                }
            }

            _ => Event::OperationFailed("Invalid token".to_string()),
        }
    }

}