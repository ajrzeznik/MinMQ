use std::collections::BinaryHeap;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::dynamic_discovery::receive_broadcast;
use crate::timer::{start_timer, Timer};

pub struct Node{
    name: String,
    timer_thread: JoinHandle<()>,
    timer_sender: Sender<Option<Timer>>
}

impl Node {
    pub fn new(name: &str) -> Self {
        let (sender, receiver) = channel();

        let handle_timer = thread::spawn(|| {
            start_timer(receiver);
        });
        Node {
            name: name.to_string(),
            timer_thread: handle_timer,
            timer_sender: sender
        }
    }

    pub fn add_timer(&self, name: &str, interval: f64){
        self.timer_sender.send(Some(Timer::new(name, interval)));
    }

    pub fn run(&mut self) {
        let handle_receive = thread::spawn(|| {
            receive_broadcast();
        });
        self.timer_sender.send(None);
        loop {}
    }
}