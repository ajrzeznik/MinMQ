package com.ajrzeznik;

import nanomsg.pubsub.PubSocket;

public class NngPubSocket {

    public static NngPubSocket create(String address) {
        return new NngPubSocket(address);
    }
    private PubSocket socket = new PubSocket();

    private NngPubSocket(String address) {
        socket.bind(address);
    }

    public void send(byte[] data){
        socket.send(data);
    }

}
