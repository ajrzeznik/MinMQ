package com.ajrzeznik;

import java.nio.ByteBuffer;

public class SubSocket {

    public static SubSocket create(String address) {
        return new SubSocket(address);
    }
    private nanomsg.pubsub.SubSocket socket = new nanomsg.pubsub.SubSocket();

    private SubSocket(String address) {
        socket.setRecvTimeout(-1);
        socket.bind(address);
        socket.subscribe("");
    }

    public ByteBuffer receive() {
        return socket.recv();
    }

}
