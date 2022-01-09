use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::{BinaryHeap, HashMap};
use std::ops::Bound;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use mq_message_base::{MessageType, MQMessage, MQMessageArgs, PubSub, PubSubArgs, root_as_mqmessage, root_as_pub_sub};
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
    local_subscribers: Vec<String>,
    local_publishers: Vec<String>,
    publisher_map: HashMap<String, Arc<RwLock<HashMap<String, PubSocket>>>>,
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
            local_subscribers: Vec::new(),
            local_publishers: Vec::new(),
            publisher_map: HashMap::new(),
            ack_bytes: ack_builder,
            ping_bytes: ping_builder,
        }
    }

    //TODO AR: Check out this static stuff
    pub fn add_timer(&mut self, name: &str, interval: f64, mut callback: impl FnMut() + 'static){
        self.callback_map.insert(name.to_string(), Box::new(move |a: &MQMessage|  callback()));
        self.timer_sender.send(Some(Timer::new(name, interval)));
    }

    //TODO AR: Lots of type checking to be done here
    pub fn subscribe_text(&mut self, topic: &str, mut callback: impl FnMut(&str) + 'static){
        self.local_subscribers.push(topic.to_string());
        self.callback_map.insert(topic.to_string(), Box::new(move |a: &MQMessage| callback(std::str::from_utf8(a.data().unwrap()).unwrap())));
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
        let send_sockets = self.address_map.get_socket_map();
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
                    self.address_map.send_if_present(msg.origin().unwrap(), self.ack_bytes.finished_data());

                }
                MessageType::Ack => {
                    println!("Received ACK {:?}", msg);
                    {
                        self.address_map.set_connected(msg.origin().unwrap());
                    }
                    if self.address_map.all_newly_connected() {
                        println!(">>>>>>>>>>>>>>>FULL CONNECTION, SEND OUT PUBSUB DATA");



                        let mut data_builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
                        //TODO AR: Almost certainly a better way to handle this
                        let mut testvec: Vec<&str> = Vec::new();
                        for item in &self.local_subscribers{
                            testvec.push(&item);
                        }
                        let sub_data = data_builder.create_vector_of_strings(&testvec);
                        let pubsub_data = PubSub::create(&mut data_builder, &PubSubArgs{
                            pub_ : None,
                            sub: Some(sub_data)
                        });
                        data_builder.finish(pubsub_data, None);


                        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
                        //TODO AR: Clean this up to remove allocations possibly
                        let origin = builder.create_string(&self.name);
                        let data_offset = builder.create_vector(data_builder.finished_data());
                        let msg = MQMessage::create(&mut builder, &MQMessageArgs{
                            topic: None,
                            origin: Some(origin),
                            message_type: MessageType::PubSub,
                            data: Some(data_offset)
                        });
                        builder.finish(msg, None);
                        self.address_map.send_to_all(builder.finished_data());

                    };
                    //TODO AR: If all newly connected here, send out messages everywhere!!!!
                }
                MessageType::PubSub => {
                    println!("Received PUBSUB {:?}", msg);
                    let pubsub_data = root_as_pub_sub(msg.data().unwrap()).expect("Had issue getting pubsub data");
                    for topic in pubsub_data.sub().unwrap(){
                        let topic = topic.to_string(); //TODO AR: Clean up another useless allocation;
                        if self.local_publishers.contains(&topic) {
                            let mut new_flag = false;
                            //TODO AR: It should be GUARANTEED that the publisher map contains the topic
                            let map = self.publisher_map.get(&topic).expect("Publisher not found in publsiher map!!!!");
                            let map_to_check: &RwLock<HashMap<String, PubSocket>> = map.borrow();
                            {
                                let interior = map_to_check.read().unwrap(); //TODO AR: Clean up these result here
                                if !interior.contains_key(msg.origin().unwrap()) {
                                    new_flag = true
                                }
                            }
                            if new_flag {
                                let mut write_map = map_to_check.write().unwrap(); //TODO AR: Clean this up also
                                write_map.insert(msg.origin().unwrap().to_string(), self.address_map.get_socket(msg.origin().unwrap()));
                            }

                        }
                    }
                }
                _ => panic!("Unexpected message of type: {:?}", msg.message_type())
            }

        }
    }
}