package com.ajrzeznik;

import com.google.flatbuffers.FlatBufferBuilder;

import java.io.IOException;
import java.net.*;
import java.nio.ByteBuffer;

public class DynamicDiscoveryBroadcaster {

    private final DatagramSocket socket = new DatagramSocket(null);
    private final DatagramPacket packet;

    DynamicDiscoveryBroadcaster(String name, int port) throws SocketException, UnknownHostException {

        //Concise object creation
        //TODO AR: This is relatively succinct, but also has the downside that nanomsg can't message out a specific
        // portion of a bytebuffer, necessitating a copy. Fix that somehow at some point
        FlatBufferBuilder builder = new FlatBufferBuilder();
        NodeAddress.finishNodeAddressBuffer(builder, NodeAddress.createNodeAddress(
                builder,
                builder.createString(name),
                port
        ));
        byte[] buffer = builder.sizedByteArray();
        InetAddress address = InetAddress.getByName("255.255.255.255");
        packet = new DatagramPacket(buffer, buffer.length, address, DynamicDiscoveryListener.DYNAMIC_DISCOVERY_PORT);

        socket.setBroadcast(true);
    }

    void broadcast() throws IOException {
        //TODO AR: Handle this exception somehow
        socket.send(packet);
    }

}
