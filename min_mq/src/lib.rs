use std::net::UdpSocket;
use std::io;

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
    let socket : UdpSocket = UdpSocket::bind(format!("0.0.0.0:{}", DYNAMIC_DISCOVER_PORT))?;
    let mut buf = [0; 100];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();
    println!("Received: {:?}", &buf[0..number_of_bytes]);
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
        let receive_thread = thread::spawn(|| {
            assert!(receive_broadcast().is_ok())
        });
        //TODO: Have a synchronization method to test this instead of a wait
        thread::sleep(Duration::from_millis(1000));

        assert!(broadcast_address().is_ok());
        assert!(receive_thread.join().is_ok());
    }
}
