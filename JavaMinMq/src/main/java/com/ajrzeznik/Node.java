package com.ajrzeznik;

import java.io.IOException;
import java.net.SocketException;
import java.net.UnknownHostException;
import java.nio.ByteBuffer;
import java.util.HashMap;
import com.ajrzeznik.MQMessage;

public class Node {

    private final HashMap<String, Runnable> callbackMap = new HashMap<>();
    private final SubSocket receiveSocket = SubSocket.create("ipc:///tmp/sock");
    private final TimerQueue timerQueue = TimerQueue.create(PubSocket.create("ipc:///tmp/sock"));
    private final AddressMap addressMap;
    private final DynamicDiscoveryBroadcaster broadcaster;

    public static Node create(String name) throws SocketException, UnknownHostException {
        return new Node(name);
    }

    private Node(String name) throws SocketException, UnknownHostException {

        addressMap = new AddressMap(name);
        //TODO AR: Hande port here
        broadcaster = new DynamicDiscoveryBroadcaster(name, 12345);
    }

    public void addTimer(String name, double interval, Runnable callback) {
        // TODO AR: handle extra naming here
        // TODO AR: HAndle dynamic addition of tiemrs
        // TODO AR: Handle remove of timers maybe???
        callbackMap.put(name, callback);
        timerQueue.addTimer(name, interval);
    }

    void run() throws InterruptedException, SocketException {

        //TODO AR: Unify socket usage
        DynamicDiscoveryListener listener = new DynamicDiscoveryListener(PubSocket.create("ipc:///tmp/sock"));
        listener.start();

        addTimer("Dynamic Discovery", 2.0, () -> {
            try {
                broadcaster.broadcast();
            } catch (IOException e) {
                //Todo AR: handle this IO Error somehow
                e.printStackTrace();
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
                    addressMap.updateAddress(message.origin(), "ADDRESS GOES HERE!!!!");
            }

        }
    }
}
