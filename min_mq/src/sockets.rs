use nng::{Message, options::{protocol::pubsub::Subscribe, Options}, Protocol, Socket};

pub(crate) struct SubSocket{
    inner_socket: Socket,
    // TODO AR: For lifetime purposes we put the last message here as a form of ownership, but there
    // might be a better or more efficient way to do this; Look into those things at some point
    last_message: Message,
}

impl SubSocket{
    //TODO AR: Fix this error returns to better stuff
    pub(crate) fn new(address: &str) -> SubSocket{

        let new_socket = Socket::new(Protocol::Sub0).expect("Failed to create a Sub Socket");

        let sub_socket =  SubSocket {
            inner_socket: new_socket,
            last_message: Default::default()
        };

        sub_socket.inner_socket.listen(address).expect(format!("Failed to listen at {}", address).as_str());
        let all_topics = vec![];
        sub_socket.inner_socket.set_opt::<Subscribe>(all_topics).expect(format!("Failed to subscribe all with socket listening at {}", address).as_str());
        return sub_socket;
    }
    //TODO AR: When message structs are created this should be updated
    //Seems to add about and extra 50-80 microsends. But that's actually probably fine (to use a nicer data structure).
    pub(crate) fn recv(&mut self) -> &[u8]{
        self.last_message = self.inner_socket.recv().expect(format!("Receive failure at {:?}", self.inner_socket).as_str());
        &self.last_message
    }
}

pub(crate) struct PubSocket{
    inner_socket: Socket
}

impl PubSocket{
    //TODO AR: Fix this error returns to better stuff
    pub(crate) fn new(address: &str) -> PubSocket{

        let new_socket = Socket::new(Protocol::Pub0).expect("Failed to create a Pub Socket");

        let pub_socket =  PubSocket {
            inner_socket: new_socket
        };

        pub_socket.inner_socket.dial_async(address).expect(format!("Failed to dial to {}", address).as_str());
        return pub_socket;
    }

    pub(crate) fn send(&self, bytes_data: &[u8]) {
        self.inner_socket.send(bytes_data).expect(format!("Error in sending to {:?}", self.inner_socket).as_str());
    }
}