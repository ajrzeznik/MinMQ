use std::sync::{Arc, RwLock};
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
    //Vector here seems to add about and extra 50-80 microsends. But that's actually probably fine (to use a nicer data structure).
    pub(crate) fn receive(&mut self) -> &[u8]{
        self.last_message = self.inner_socket.recv().expect(format!("Receive failure at {:?}", self.inner_socket).as_str());
        &self.last_message
    }
}

//TODO AR: Consider what should and should be combined here
#[derive(Debug)]
struct InnerPubSocket{
    inner_socket: Socket,
    address: String,
    connected: bool
}

#[derive(Clone,Debug)]
pub(crate) struct PubSocket{
    inner: Arc<RwLock<InnerPubSocket>>
}

//TODO AR: Check all the unwrapping stuff contained herein
//TODO AR: Connected being in a separate structure eases off a lot on lock contention
impl PubSocket{
    //TODO AR: Fix this error returns to better stuff
    pub(crate) fn new(address: &str) -> PubSocket{

        let new_socket = Socket::new(Protocol::Pub0).expect("Failed to create a Pub Socket");
        new_socket.dial_async(address).expect(format!("Failed to dial to {}", address).as_str());
        let pub_socket =  PubSocket {
            inner: Arc::new( RwLock::new(InnerPubSocket{
                inner_socket: new_socket,
                address: address.to_string(),
                connected: false
            }))
        };

        return pub_socket;
    }

    //TODO AR: Return &str instead and deal with lifetimes
    pub(crate) fn get_address(&self) -> String {
        self.inner.read().unwrap().address.clone()
    }

    pub(crate) fn update_address(&mut self, address: &str) {
        let mut inner_socket = self.inner.write().unwrap();
        inner_socket.address = address.to_string();
        inner_socket.inner_socket = Socket::new(Protocol::Pub0).expect("Failed to create a new address socket");
    }

    pub(crate) fn set_connected(&mut self) {
        self.inner.write().unwrap().connected = true;
    }

    pub(crate) fn get_connected(&self) -> bool {
        self.inner.read().unwrap().connected
    }

    pub(crate) fn send(&self, bytes_data: &[u8]) {
        self.inner.read().unwrap().inner_socket.send(bytes_data).expect(format!("Error in sending to {:?}", self.inner).as_str());
    }
}