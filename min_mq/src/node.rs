use std::thread;
use std::thread::JoinHandle;
use crate::dynamic_discovery::receive_broadcast;

pub struct Node{
    name: String,
    //receive_thread: Option<JoinHandle<()>>
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            //receive_thread: None,
        }
    }

    pub fn run(&mut self) {
        let handle = thread::spawn(|| {
            receive_broadcast();
        });
    }
}