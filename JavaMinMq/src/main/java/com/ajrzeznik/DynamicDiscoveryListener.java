package com.ajrzeznik;

import com.google.flatbuffers.FlatBufferBuilder;

import java.io.IOException;
import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetSocketAddress;
import java.net.SocketException;
import java.nio.ByteBuffer;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

public class DynamicDiscoveryListener extends Thread{

    public static final int DYNAMIC_DISCOVERY_PORT = 43357;

    private final DatagramSocket listenerSocket = new DatagramSocket(null);
    private final byte[] buffer = new byte[256]; //TODO AR: Consider some size changes here
    private final PubSocket socket;

    public DynamicDiscoveryListener(PubSocket pubSocket) throws SocketException {
        listenerSocket.setReuseAddress(true);
        listenerSocket.bind(new InetSocketAddress("0.0.0.0", DYNAMIC_DISCOVERY_PORT));
        this.socket = pubSocket;
    }

    public void run() {
        System.out.println( "Dynamic Discovery Listener Startup" );
        DatagramPacket packet = new DatagramPacket(buffer, buffer.length);
        while (true) {
            try {
                listenerSocket.receive(packet);
            } catch (IOException e) {
                System.out.println("Error: IO Exception on Receive");
                e.printStackTrace();
                break;
            }
            NodeAddress addressMessage = NodeAddress.getRootAsNodeAddress(ByteBuffer.wrap(buffer));
            //TODO AR: Should log when the packet length is too short, in order to note the issue and deal with it elsewhere
            FlatBufferBuilder builder = new FlatBufferBuilder();
            //TODO AR: Clean up this creation/work here on these types
            MQMessage.finishMQMessageBuffer(builder, MQMessage.createMQMessage(builder,
                    builder.createString(packet.getAddress().toString().substring(1)+ ":" + addressMessage.port()),
                    builder.createString(addressMessage.name()),
                    MessageType.Address,
                    builder.createByteVector(new byte[0]))
            );
            //TODO AR: Send a byte buffer portion
            socket.send(builder.sizedByteArray());

            //System.out.println("Packet length::" + packet.getLength());

            //System.out.println("name: " + addressMessage.name() + ", port: " + addressMessage.port()+ ", address: " + packet.getAddress());
        }

    }
}
