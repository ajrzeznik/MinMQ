use std::net::UdpSocket;
use std::io;
use socket2;
use std::mem::MaybeUninit;
use mq_message_base;
use mq_message_base::{get_root_as_mqmessage, NodeAddress, NodeAddressT, root_as_node_address};

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
pub fn receive_broadcast() -> io::Result<()> {
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
        println!("Received {:?}", node_address);
    }
}