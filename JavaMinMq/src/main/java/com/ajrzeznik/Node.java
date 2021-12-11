package com.ajrzeznik;

import java.util.HashMap;
import java.util.function.Consumer;

public class Node {

    private final HashMap<String, Consumer<String>> callbackMap = new HashMap<>();
    private final TimerQueue timerQueue = TimerQueue.create();

    public static Node create() {
        return new Node();
    }

    private Node() {

    }



    void run(){

    }
}
