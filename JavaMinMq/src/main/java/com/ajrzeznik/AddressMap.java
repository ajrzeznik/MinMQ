package com.ajrzeznik;

import java.util.HashMap;

public class AddressMap {

    private final String nodeName;
    // TODO AR: Thread safety is probably NOT needed here, as each publicher can hold a pubsocket as a reference, and
    // the pubsocket itself is guaranteed to be threadsafe. But this assumption should be checked.
    final HashMap<String, PubSocket> socketMap = new HashMap<>();

    AddressMap(String nodeName) {
        this.nodeName = nodeName;
    }

    // Returns true if a new address was added.
    boolean updateAddress(String name, String address) {
        if (socketMap.containsKey(name)) {
            PubSocket socket = socketMap.get(name);
            if (socket.getAddress().equals(address)){
                return false;
            }
            socket.updateAddress(address);

        } else {
            socketMap.put(name, PubSocket.create(address));
        }
        return true;
    }
}
