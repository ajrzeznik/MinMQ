use std::collections::HashMap;
use crate::sockets::PubSocket;

pub(crate) struct AddressMap {
    all_connected: bool,
    node_name: String,
    pub(crate) socket_map: HashMap<String, PubSocket>
}

impl AddressMap {
    pub fn new(name: &str) -> Self{
        AddressMap {
            all_connected: false,
            node_name: name.to_string(),
            socket_map: Default::default()
        }
    }

    pub fn update_address(&mut self, name: &str, address: &str) -> bool{
        if self.socket_map.contains_key(name){
            let socket = self.socket_map.get_mut(name).unwrap();
            if socket.get_address() == address {
                return false;
            }
            socket.update_address(address);
        } else {
            self.socket_map.insert(name.to_string(), PubSocket::new(address));
        }
        true
    }

    pub fn all_newly_connected(&mut self) -> bool {
        if self.all_connected {
            false
        } else {
            self.all_connected = !self.socket_map.iter().any(|item| !item.1.get_connected());
            self.all_connected
        }
    }
}