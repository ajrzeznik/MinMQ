use std::net::UdpSocket;
use std::io;
use socket2;
use std::mem::MaybeUninit;

const DYNAMIC_DISCOVER_PORT: u16 = 43357;

pub fn broadcast_address() -> io::Result<()> {
    let socket : UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    let call = vec![2,3,5,7];
    socket.send_to(&call, format!("255.255.255.255:{}", DYNAMIC_DISCOVER_PORT)).unwrap();
    Ok(())
}

//TODO: Clean this up to reuse the bound port, since that's needed to listen for messages
pub fn receive_broadcast() -> io::Result<()> {
    let socket = socket2::Socket::new(socket2::Domain::IPV4,
                                      socket2::Type::DGRAM,
                                      Some(socket2::Protocol::UDP)).unwrap();
    socket.set_reuse_address(true).unwrap();
    let address: std::net::SocketAddr = format!("0.0.0.0:{}", DYNAMIC_DISCOVER_PORT).parse().unwrap();
    socket.bind(&address.into())?;
    //TODO AR: Handle some receive/send buffering here!!!!
    let mut buf = [MaybeUninit::<u8>::new(0); 100];
    let (byte_count, sending_address) = socket.recv_from(&mut buf)?;
    let c= buf.iter().map(|a|unsafe {a.assume_init()}).collect::<Vec<u8>>();
    println!("Received {:?}", c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;
    use crate::{receive_broadcast, broadcast_address};

    #[test]
    //TODO: Clean this up a bit, remove the unwraps and have proper asserts
    fn send_and_receive() {
        let receive_thread_1 = thread::spawn(|| {
            assert!(receive_broadcast().is_ok())
        });
        let receive_thread_2 = thread::spawn(|| {
            assert!(receive_broadcast().is_ok())
        });
        //TODO: Have a synchronization method to test this instead of a wait
        thread::sleep(Duration::from_millis(1000));

        assert!(broadcast_address().is_ok());
        assert!(receive_thread_1.join().is_ok());
        assert!(receive_thread_2.join().is_ok());
    }
}
