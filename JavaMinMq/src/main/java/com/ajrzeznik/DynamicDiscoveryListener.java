package com.ajrzeznik;

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

    public DynamicDiscoveryListener() throws SocketException {
        listenerSocket.setReuseAddress(true);
        listenerSocket.bind(new InetSocketAddress("0.0.0.0", DYNAMIC_DISCOVERY_PORT));

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
            //TODO AR: Should log when the packet length is too short, in order to note the issue and deal with it elsewhere
            System.out.println("Packet length::" + packet.getLength());
            NodeAddress addressMessage = NodeAddress.getRootAsNodeAddress(ByteBuffer.wrap(buffer));
            System.out.println("name: " + addressMessage.name() + ", port: " + addressMessage.port()+ ", address: " + packet.getAddress());
            System.out.println(LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS")));
        }

    }
}
