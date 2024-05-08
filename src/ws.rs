use std::sync::mpsc::{Receiver, Sender};

use crate::{header, request::Request, HttpResult};

pub struct WebSocket {
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl WebSocket {
    pub fn split(self) -> (Sender<String>, Receiver<String>) {
        (self.sender, self.receiver)
    }

    pub fn from_request<S: Clone>(req: Request<S>) -> HttpResult<Self> {
        let Some(origin) = req.headers().get(header::ORIGIN) else {
            todo!("Continue WebSocket handling");
            //return Err();
        };
        let sec_websocket_version = req.headers().get(header::SEC_WEBSOCKET_VERSION);
        let sec_websocket_key = req.headers().get(header::SEC_WEBSOCKET_KEY);
        let sec_websocket_accept = req.headers().get(header::SEC_WEBSOCKET_ACCEPT);
        let connection = req.headers().get(header::CONNECTION);
        todo!("Continue WebSocket handling");
    }
}
