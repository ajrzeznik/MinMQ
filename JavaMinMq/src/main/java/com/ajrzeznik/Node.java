package com.ajrzeznik;

import java.nio.ByteBuffer;
import java.util.HashMap;

public class Node {

    private final HashMap<String, Runnable> callbackMap = new HashMap<>();
    private final SubSocket receiveSocket = SubSocket.create("ipc:///tmp/sock");
    private final TimerQueue timerQueue = TimerQueue.create(PubSocket.create("ipc:///tmp/sock"));
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
            ByteBuffer message = receiveSocket.receive();
            int ref_len = message.remaining();
            byte[] buf = new byte[ref_len];
            message.get(buf);
            String strMsg = new String(buf);
            // System.out.println(message);
            callbackMap.get(strMsg).run();
        }
    }
}
