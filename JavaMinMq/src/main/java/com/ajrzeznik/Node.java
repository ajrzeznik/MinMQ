package com.ajrzeznik;

import nanomsg.pubsub.SubSocket;

import java.nio.ByteBuffer;
import java.util.HashMap;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;

public class Node {

    private final HashMap<String, Runnable> callbackMap = new HashMap<>();
    private final BlockingQueue<String> test_queue = new ArrayBlockingQueue<String>(1024);
    private final TimerQueue timerQueue = TimerQueue.create(test_queue);
    private final NngSubSocket receiveSocket = NngSubSocket.create("ipc:///tmp/sock");
    private final NngPubSocket sendSocket = NngPubSocket.create("ipc:///tmp/sock");
    public static Node create() {
        return new Node();
    }

    private Node() {

    }

    public void addTimer(String name, double interval, Runnable callback) {
        // TODO AR: handle extra naming here
        // TODO AR: HAndle dynamic addition of tiemrs
        // TODO AR: Handle remove of timers maybe???
        callbackMap.put(name, callback);
        timerQueue.addTimer(name, interval);
    }

    void run() throws InterruptedException {
        timerQueue.start();
        while (true) {
            String message = test_queue.take();
            byte[] var = new byte[]{2, 3, 5 , 9};
            sendSocket.send(var);
            sendSocket.send(var);
            sendSocket.send(var);
            sendSocket.send(var);
            byte[] buf = new byte[4];
            receiveSocket.receive().get(buf);
            System.out.println("Values: " + buf[0] + "," + buf[1]);
            // System.out.println(message);
            callbackMap.get(message).run();
        }
    }
}
