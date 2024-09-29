use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{platform::Platform, server::requests::request_token::RequestToken};


enum State {
    AwaitingRequest,
    ProcessingRequest,
    ExecutingOperation,
    Error(String),
    Finished,
}

enum Event {
    IncomingToken(RequestToken),
    InvalidToken,
    OperationSuccess,
    OperationFailed(String),
}

struct FSM {
    state: State,
}

impl FSM {
    fn new() -> Self {
        Self {
            state: State::AwaitingRequest,
        }
    }

    pub fn process_event(&mut self, event: Event, platform: &Arc<RwLock<Platform>>) {
        match (&self.state, event) {
            (State::AwaitingRequest, Event::IncomingToken(token)) => {
                self.state = State::ProcessingRequest;
                self.process_token(token, platform);
            }
            (State::AwaitingRequest, Event::InvalidToken) => {
                self.state = State::Error("Invalid token received".to_string());
            }
            (State::ProcessingRequest, Event::OperationSuccess) => {
                self.state = State::ExecutingOperation;
            }
            (State::ProcessingRequest, Event::OperationFailed(err)) => {
                self.state = State::Error(err);
            }
            (State::ExecutingOperation, Event::OperationSuccess) => {
                self.state = State::Finished;
            }
            (State::ExecutingOperation, Event::OperationFailed(err)) => {
                self.state = State::Error(err);
            }
            _ => (),
        }
    }

    async fn process_token(&mut self, token: RequestToken, platform: &Arc<RwLock<Platform>>) -> Event {
        let operation_result = token.execute(platform);
        match operation_result.await {
            Ok(_) => Event::OperationSuccess,
            Err(err) => Event::OperationFailed(err),
        }
    }

}