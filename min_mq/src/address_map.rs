use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::sockets::PubSocket;

pub(crate) struct AddressMap {
    all_connected: bool,
    node_name: String,
    pub(crate) socket_map: Rc<RefCell<HashMap<String, PubSocket>>>
}

impl AddressMap {
    pub fn new(name: &str) -> Self{
        let mut socket_map = HashMap::new();
        socket_map.insert(name.to_string(), PubSocket::new("tcp://localhost:55555"));

        AddressMap {
            all_connected: false,
            node_name: name.to_string(),
            socket_map: Rc::new(RefCell::new(socket_map))
        }
    }
    //TODO AR: Refcell stuff here is pretty ugly, probably should be fixed
    pub fn update_address(&mut self, name: &str, address: &str) -> bool{
        let mut current_map = self.socket_map.borrow_mut();
        if current_map.contains_key(name){
            let socket = current_map.get_mut(name).unwrap();
            if socket.get_address() == address {
                return false;
            }
            socket.update_address(address);
        } else {
            current_map.insert(name.to_string(), PubSocket::new(address));
        }
        true
    }

    pub fn all_newly_connected(&mut self) -> bool {
        if self.all_connected {
            false
        } else {
            let ref_current_map: &RefCell<HashMap<String, PubSocket>> = self.socket_map.borrow();
            let current_map = ref_current_map.borrow();
            self.all_connected = !current_map.iter().any(|item| !item.1.get_connected());
            self.all_connected
        }
    }
}