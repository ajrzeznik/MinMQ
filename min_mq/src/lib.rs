pub mod node;
mod dynamic_discovery;
mod timer;
mod sockets;

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
