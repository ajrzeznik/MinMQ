package com.ajrzeznik;

import java.io.IOException;
import java.net.ServerSocket;
import java.net.SocketException;
import java.net.UnknownHostException;
import java.nio.ByteBuffer;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.HashMap;
import java.util.Map;

import com.ajrzeznik.MQMessage;
import com.google.flatbuffers.FlatBufferBuilder;

public class Node {

    private final HashMap<String, Runnable> callbackMap = new HashMap<>();
    private final SubSocket receiveSocket;
    private final int port;
    private final TimerQueue timerQueue;
    private final AddressMap addressMap;
    private final String name;
    private final DynamicDiscoveryBroadcaster broadcaster;

    public static Node create(String name) throws IOException {
        return new Node(name);
    }

    private Node(String name) throws IOException {
        // Get a free port by binding a socket , closing, and pulling from that
        this.name = name;
        ServerSocket serverSocket = new ServerSocket(0);
        port = serverSocket.getLocalPort();
        serverSocket.close();

        receiveSocket = SubSocket.create("tcp://*:" + port);
        addressMap = new AddressMap(name);
        timerQueue = TimerQueue.create(PubSocket.create("tcp://localhost:"+port));
        broadcaster = new DynamicDiscoveryBroadcaster(name, port);
    }

    public void addTimer(String name, double interval, Runnable callback) {
        // TODO AR: handle extra naming here
        // TODO AR: Handle dynamic addition of tiemrs
        // TODO AR: Handle remove of timers maybe???
        callbackMap.put(name, callback);
        timerQueue.addTimer(name, interval);
    }

    void run() throws InterruptedException, SocketException {

        //TODO AR: Unify socket usage
        DynamicDiscoveryListener listener = new DynamicDiscoveryListener(PubSocket.create("tcp://localhost:" + port));
        listener.start();

        addTimer("Dynamic Discovery and Ping", 2.0, () -> {
            try {
                broadcaster.broadcast();
            } catch (IOException e) {
                //Todo AR: handle this IO Error somehow
                e.printStackTrace();
            }

            // Ping anything that isn't properly connected

            // TODO AR: If everything is connected, no need to allocate a new message, so can probably optimize that out

            FlatBufferBuilder builder = new FlatBufferBuilder();
            //TODO AR: Clean up this creation/work here on these types
            MQMessage.finishMQMessageBuffer(builder, MQMessage.createMQMessage(builder,
                    builder.createString(""),
                    builder.createString(name),
                    MessageType.Ping,
                    builder.createByteVector(new byte[0]))
            );
            //TODO AR: Send a byte buffer portion
            byte[] messageBytes = builder.sizedByteArray();

            for (Map.Entry<String,PubSocket> item : addressMap.socketMap.entrySet()) {
                PubSocket socket= item.getValue();
                if (!socket.isConnected()) {
                    System.out.println("Pinging Socket: " + item.getKey());
                    socket.send(messageBytes);
                }
            }


        });


        //TODO AR: Add in timer/main thread synchronization.
        timerQueue.start();
        while (true) {
            MQMessage message = MQMessage.getRootAsMQMessage(receiveSocket.receive());
            switch (message.messageType()) {
                case MessageType.Topic:
                    String strMsg = message.topic();
                    System.out.println("Received message of topic: "+ strMsg );
                    callbackMap.get(strMsg).run();
                    break;
                case MessageType.Address:
                    //TODO AR: Deserialize data to node address
                    //TODO AR: Clean up and send out the pub/sub additions
                    String address = "tcp:/" + message.topic();
                    if (addressMap.updateAddress(message.origin(), address)){
                        System.out.println("==NEW ADDRESS: name: " + message.origin()+ ", address: " + address);
                        System.out.println(LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS")));
                    }
                    break;
                case MessageType.Ping:
                    String pingMsg = message.topic();
                    System.out.println("Received Ping from " + message.origin() );

                    break;

            }

        }
    }
}
