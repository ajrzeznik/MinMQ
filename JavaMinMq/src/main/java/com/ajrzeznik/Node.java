package com.ajrzeznik;

import java.util.HashMap;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;

public class Node {

    private final HashMap<String, Runnable> callbackMap = new HashMap<>();
    private final BlockingQueue<String> test_queue = new ArrayBlockingQueue<String>(1024);
    private final TimerQueue timerQueue = TimerQueue.create(test_queue);


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
            // System.out.println(message);
            callbackMap.get(message).run();
        }
    }
}
