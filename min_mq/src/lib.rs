use std::net::UdpSocket;
use std::io;

pub fn broadcast_address() -> io::Result<()> {
    let socket : UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    let call = vec![2,3,5,7];
    socket.send_to(&call, "255.255.255.255:55555").unwrap();
    Ok(())
}

//TODO: Clean this up to reuse the bound port, since that's needed to listen for messages
pub fn receive_broadcast() -> io::Result<()> {
    let socket : UdpSocket = UdpSocket::bind("0.0.0.0:55555")?;
    let mut buf = [0; 100];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();
    println!("Received: {:?}", &buf[0..number_of_bytes]);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
