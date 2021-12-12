package com.ajrzeznik;

import nanomsg.pubsub.SubSocket;

import java.nio.ByteBuffer;

public class NngSubSocket {

    public static NngSubSocket create(String address) {
        return new NngSubSocket(address);
    }
    private SubSocket socket = new SubSocket();

    private NngSubSocket(String address) {
        socket.connect(address);
        socket.subscribe("");
    }

    public ByteBuffer receive() {
        return socket.recv();
    }

}
