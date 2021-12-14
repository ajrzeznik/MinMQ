package com.ajrzeznik;

public class PubSocket {

    public static PubSocket create(String address) {
        return new PubSocket(address);
    }
    private nanomsg.pubsub.PubSocket socket = new nanomsg.pubsub.PubSocket();

    private PubSocket(String address) {
        socket.connect(address);
    }

    public void send(byte[] data){
        //TODO AR: Send a byte buffer portion
        socket.send(data);
    }

}
