package com.ajrzeznik;

public class PubSocket {

    public static PubSocket create(String address) {
        return new PubSocket(address);
    }
    private nanomsg.pubsub.PubSocket socket = new nanomsg.pubsub.PubSocket();
    private String address;

    private PubSocket(String address) {
        this.address = address;
        socket.connect(address);
    }

    String getAddress(){
        return this.address;
    }

    void updateAddress(String address){
        this.address = address;
        socket = new nanomsg.pubsub.PubSocket();
        socket.connect(address);
    }

    public void send(byte[] data){
        //TODO AR: Send a byte buffer portion
        socket.send(data);
    }

}
