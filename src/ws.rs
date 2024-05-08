use std::sync::mpsc::{Receiver, Sender};

pub struct WebSocket {
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl WebSocket {
    pub fn split(self) -> (Sender<String>, Receiver<String>) {
        (self.sender, self.receiver)
    }
}
