package com.ajrzeznik;

import java.nio.ByteBuffer;
import java.util.HashMap;
import com.ajrzeznik.MQMessage;

public class Node {

    private final HashMap<String, Runnable> callbackMap = new HashMap<>();
    private final SubSocket receiveSocket = SubSocket.create("ipc:///tmp/sock");
    private final TimerQueue timerQueue = TimerQueue.create(PubSocket.create("ipc:///tmp/sock"));
    private final AddressMap addressMap;

    public static Node create(String name) {
        return new Node(name);
    }

    private Node(String name) {
        addressMap = new AddressMap(name);
    }

    public void addTimer(String name, double interval, Runnable callback) {
        // TODO AR: handle extra naming here
        // TODO AR: HAndle dynamic addition of tiemrs
        // TODO AR: Handle remove of timers maybe???
        callbackMap.put(name, callback);
        timerQueue.addTimer(name, interval);
    }

    void run() throws InterruptedException {
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
