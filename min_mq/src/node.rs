use std::thread;
use std::thread::JoinHandle;
use crate::dynamic_discovery::receive_broadcast;
use crate::timer::start_timer;

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
        let handle_receive = thread::spawn(|| {
            receive_broadcast();
        });

        let handle_timer = thread::spawn(|| {
            start_timer();
        });
        handle_timer.join();
    }
}