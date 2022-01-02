use std::net::UdpSocket;
use std::io;
use socket2;
use std::mem::MaybeUninit;
use mq_message_base;
use mq_message_base::{get_root_as_mqmessage, MessageType, MQMessage, MQMessageArgs, NodeAddress, NodeAddressT, root_as_node_address};
use crate::sockets::PubSocket;

const DYNAMIC_DISCOVER_PORT: u16 = 43357;

pub fn broadcast_address() -> io::Result<()> {
    let socket : UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    //TODO AR: Remove the object api here
    let mut msg = NodeAddressT::default();
    msg.name = Some("Rust_Data".to_string());
    msg.port = 32121;
    let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(256);
    let mut temp = msg.pack(&mut fbb);
    fbb.finish(temp, None);
    socket.send_to(fbb.finished_data(), format!("255.255.255.255:{}", DYNAMIC_DISCOVER_PORT)).unwrap();
    Ok(())
}

//TODO: Clean this up to reuse the bound port, since that's needed to listen for message_generator
pub fn receive_broadcast(port: u16) -> io::Result<()> {
    let pub_socket = PubSocket::new(&("tcp://localhost:".to_owned() + &port.to_string()));
    let socket = socket2::Socket::new(socket2::Domain::IPV4,
                                      socket2::Type::DGRAM,
                                      Some(socket2::Protocol::UDP)).unwrap();
    socket.set_reuse_address(true).unwrap();
    let address: std::net::SocketAddr = format!("0.0.0.0:{}", DYNAMIC_DISCOVER_PORT).parse().unwrap();
    socket.bind(&address.into())?;
    //TODO AR: Handle some receive/send buffering here!!!!
    loop {
        let mut buf = [MaybeUninit::<u8>::new(0); 256];
        let (byte_count, sending_address) = socket.recv_from(&mut buf)?;
        //TODO AR: Can clean this up with nightly
        let c = buf.iter().map(|a| unsafe { a.assume_init() }).collect::<Vec<u8>>();
        //TODO AR: Error handling here
        let node_address = root_as_node_address(&c).expect("Failed to unwrap incoming node address buffer");

        //TODO AR: Check the size here
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(256);
        //TODO AR: Clean this up to remove allocations possibly
        let topic = builder.create_string(&("tcp://".to_string() + &sending_address.as_socket_ipv4().unwrap().ip().to_string() + ":" + &node_address.port().to_string()));
        let origin = builder.create_string(node_address.name().unwrap());
        let msg = MQMessage::create(&mut builder, &MQMessageArgs{
            topic: Some(topic),
            origin: Some(origin),
            message_type: MessageType::Address,
            data: None
        });
        builder.finish(msg, None);
        pub_socket.send(builder.finished_data());

        //println!("Received {:?}\nWith addr: {:?}", node_address, );
    }
}