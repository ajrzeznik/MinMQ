use std::collections::{BinaryHeap, HashMap};
use std::ops::Bound;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;
use mq_message_base::{MQMessage, root_as_mqmessage};
use crate::dynamic_discovery::receive_broadcast;
use crate::sockets::SubSocket;
use crate::timer::{start_timer, Timer};

pub struct Node{
    name: String,
    timer_thread: JoinHandle<()>,
    timer_sender: Sender<Option<Timer>>,
    main_socket: SubSocket,
    callback_map: HashMap<String, Box<dyn FnMut(&MQMessage)>>
}

impl Node {
    pub fn new(name: &str) -> Self {
        let (sender, receiver) = channel();


        let passed_name = name.to_string();
        let handle_timer = thread::spawn(move || {
            start_timer(receiver, passed_name, 55555);
        });
        Node {
            name: name.to_string(),
            timer_thread: handle_timer,
            timer_sender: sender,
            main_socket: SubSocket::new("tcp://*:55555"),
            callback_map: Default::default()
        }
    }

    //TODO AR: Check out this static stuff
    pub fn add_timer(&mut self, name: &str, interval: f64, mut callback: impl FnMut() + 'static){
        self.callback_map.insert(name.to_string(), Box::new(move |a: &MQMessage|  callback()));
        self.timer_sender.send(Some(Timer::new(name, interval)));
    }

    pub fn run(&mut self) {
        let handle_receive = thread::spawn(|| {
            receive_broadcast();
        });
        //Send None to trigger the timer to start running
        //TODO AR: Sync this with the recv sockets
        self.timer_sender.send(None);
        loop {
            let buffer = self.main_socket.receive();
            let node_address = root_as_mqmessage(buffer).expect("Failed to unwrap incoming MQMessage buffer");
            println!("Receive node address {:?}", node_address);
        }
    }
}