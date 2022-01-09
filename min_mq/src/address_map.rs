use std::borrow::Borrow;
use std::cell::{Cell, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use crate::dynamic_discovery::broadcast_address;
use crate::sockets::PubSocket;

pub(crate) struct AddressMap {
    pub(crate) all_connected: Rc<Cell<bool>>,
    node_name: String,
    socket_map: Rc<RefCell<HashMap<String, PubSocket>>>
}

impl AddressMap {
    pub fn new(name: &str) -> Self{
        let mut socket_map = HashMap::new();
        socket_map.insert(name.to_string(), PubSocket::new("tcp://localhost:55555"));

        AddressMap {
            all_connected: Rc::new(Cell::new(false)),
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
        self.all_connected.set(false);
        true
    }

    pub fn all_newly_connected(&self) -> bool {
        if self.all_connected.get() {
            false
        } else {
            let ref_current_map: &RefCell<HashMap<String, PubSocket>> = self.socket_map.borrow();
            let current_map = ref_current_map.borrow();
            let result = !current_map.iter().any(|item| !item.1.get_connected());
            self.all_connected.set(result);
            result
        }
    }

    pub fn send(&self, name: &str, data: &[u8]) {
        let ref_current_map: &RefCell<HashMap<String, PubSocket>> = self.socket_map.borrow();
        ref_current_map.borrow().get(name).unwrap().send(data); //TODO AR: Add some more checking here!!!!
    }

    pub fn send_to_all(&self, data: &[u8]) {
        let run_map: &RefCell<HashMap<String, PubSocket>> = self.socket_map.borrow();
        let map = run_map.borrow();
        for (_ ,socket) in map.iter(){
            socket.send(data);
        }
    }

    pub fn send_if_present(&self, name: &str, data: &[u8]) {
        let ref_current_map: &RefCell<HashMap<String, PubSocket>> = self.socket_map.borrow();
        if let Some(socket) = ref_current_map.borrow().get(name){
            socket.send(data);
        } else {
            println!("Socket {} not yet present in address map!!!!!", name);
        }
    }

    pub fn set_connected(&mut self, name: &str) {
        let mut ref_current_map: RefMut<HashMap<String, PubSocket>> = self.socket_map.borrow_mut();
        ref_current_map.get_mut(name).unwrap().set_connected(); //TODO AR: Add some more checking here!!!!
    }

    pub fn get_socket(&self, name: &str) -> PubSocket{
        let ref_current_map: &RefCell<HashMap<String, PubSocket>> = self.socket_map.borrow();
        ref_current_map.borrow().get(name).unwrap().clone() //TODO AR: Add some more checking here!!!!
    }

    pub fn get_socket_map(&self) -> Rc<RefCell<HashMap<String, PubSocket>>>{
        self.socket_map.clone()
    }
}