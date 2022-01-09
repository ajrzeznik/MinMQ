use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{BinaryHeap, HashMap};
use std::ops::Bound;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;
use mq_message_base::{MessageType, MQMessage, MQMessageArgs, root_as_mqmessage};
use crate::address_map::AddressMap;
use crate::dynamic_discovery::{broadcast_address, receive_broadcast};
use crate::sockets::{PubSocket, SubSocket};
use crate::timer::{start_timer, Timer};

pub struct Node<'a>{
    name: String,
    timer_thread: JoinHandle<()>,
    timer_sender: Sender<Option<Timer>>,
    main_socket: SubSocket,
    //TODO AR: I am not sure how I feel about sotring FnMut specifically here, but it is probably ok.
    //Part of me definitely feels like there is a more elegant/better way of handling the callback map in rust
    callback_map: HashMap<String, Box<dyn FnMut(&MQMessage)>>,
    address_map: AddressMap,
    ack_bytes: flatbuffers::FlatBufferBuilder<'a>,
    ping_bytes: flatbuffers::FlatBufferBuilder<'a>
}

impl Node<'_> {
    pub fn new(name: &str) -> Self {
        let (sender, receiver) = channel();


        let passed_name = name.to_string();
        let handle_timer = thread::spawn(move || {
            start_timer(receiver, passed_name, 55555);
        });


        //Ack and ping bytes created
        let mut ack_builder = flatbuffers::FlatBufferBuilder::with_capacity(256);
        //TODO AR: Clean this up to remove allocations possibly
        let origin = ack_builder.create_string(name);
        let msg = MQMessage::create(&mut ack_builder, &MQMessageArgs{
            topic: None,
            origin: Some(origin),
            message_type: MessageType::Ack,
            data: None
        });
        ack_builder.finish(msg, None);

        //Ack and ping bytes created
        let mut ping_builder = flatbuffers::FlatBufferBuilder::with_capacity(256);
        //TODO AR: Clean this up to remove allocations possibly
        let origin = ping_builder.create_string(name);
        let msg = MQMessage::create(&mut ping_builder, &MQMessageArgs{
            topic: None,
            origin: Some(origin),
            message_type: MessageType::Ping,
            data: None
        });
        ping_builder.finish(msg, None);

        Node {
            name: name.to_string(),
            timer_thread: handle_timer,
            timer_sender: sender,
            main_socket: SubSocket::new("tcp://*:55555"),
            callback_map: Default::default(),
            address_map: AddressMap::new(name),
            ack_bytes: ack_builder,
            ping_bytes: ping_builder,
        }
    }

    //TODO AR: Check out this static stuff
    pub fn add_timer(&mut self, name: &str, interval: f64, mut callback: impl FnMut() + 'static){
        self.callback_map.insert(name.to_string(), Box::new(move |a: &MQMessage|  callback()));
        self.timer_sender.send(Some(Timer::new(name, interval)));
    }

    pub fn run(&mut self) {
        let passed_name = self.name.to_string();
        let handle_receive = thread::spawn(move || {
            receive_broadcast( 55555);
        });
        //Send None to trigger the timer to start running
        //-----------------Ping timer stuff---------------------------------------
        //TODO AR: Sync this with the recv sockets
        let passed_name = self.name.clone();
        //TODO AR: replace with the main out socket, probably? could also just use here again
        // Send out ping message
        //TODO AR: Check the size here
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(256);
        //TODO AR: Clean this up to remove allocations possibly
        let origin = builder.create_string(&passed_name);
        let msg = MQMessage::create(&mut builder, &MQMessageArgs{
            topic: None,
            origin: Some(origin),
            message_type: MessageType::Ping,
            data: None
        });
        builder.finish(msg, None);
        //-------------------End timer state stuff----------------------------------
        let broadcast_name = self.name.clone();

        //TODO AR: Can clean this up quite a bit; the Refcell should probably be at a different level to keep stuff clean
        let send_sockets = self.address_map.socket_map.clone();
        let all_connected = self.address_map.all_connected.clone();
        self.add_timer("dynamic_broadcast_ping", 1.0, move|| {
            broadcast_address(broadcast_name.clone(), 55555);
            if all_connected.get() {
                return;
            }
            let socket_ref : &RefCell<HashMap<String, PubSocket>> = send_sockets.borrow();
            let sockets = socket_ref.borrow();
            for (_, socket) in sockets.iter() {
                if !socket.get_connected() {
                    socket.send(builder.finished_data());
                }
            }
        });
        self.timer_sender.send(None);
        loop {
            let buffer = self.main_socket.receive();
            let msg = root_as_mqmessage(buffer).expect("Failed to unwrap incoming MQMessage buffer");
            match msg.message_type() {
                MessageType::Topic => {
                    //TODO AR: Cleanup these checks!!!
                    self.callback_map.get_mut(msg.topic()
                        .expect("Got message with an empty topic"))
                        .expect("Topic did not have callback")(&msg);
                }
                MessageType::Address => {
                    println!("Received ADDRESS {:?}", msg);
                    self.address_map.update_address(msg.origin().unwrap(), &(msg.topic().unwrap()));
                }
                MessageType::Ping => {
                    println!("Received PING {:?}", msg);
                    let ref_cell_map : &RefCell<HashMap<String, PubSocket>> = self.address_map.socket_map.borrow();
                    let current_map = ref_cell_map.borrow();
                    if current_map.contains_key(msg.origin().unwrap()) {
                        current_map.get(msg.origin().unwrap()).unwrap().send(self.ack_bytes.finished_data())
                    } else {
                        println!("NOT YET IN ADDRESS MAP!!!");
                    }

                }
                MessageType::Ack => {
                    println!("Received ACK {:?}", msg);
                    {
                        let mut current_map: RefMut<HashMap<String, PubSocket>> = self.address_map.socket_map.borrow_mut();
                        current_map.get_mut(msg.origin().unwrap()).unwrap().set_connected();
                    }
                    self.address_map.all_newly_connected();
                    //TODO AR: If all newly connected here, send out messages everywhere!!!!
                }
                MessageType::PubSub => {
                    println!("Received PUBSUB {:?}", msg);
                    //TODO AR: Handle the pubsub registration!!!!!
                }
                _ => panic!("Unexpected message of type: {:?}", msg.message_type())
            }

        }
    }
}