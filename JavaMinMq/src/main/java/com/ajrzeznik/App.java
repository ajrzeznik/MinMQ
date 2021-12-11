package com.ajrzeznik;

import java.io.IOException;

/**
 * Hello world!
 *
 */
public class App 
{

    public App() {
    }

    public static void main( String[] args ) throws IOException {
        DynamicDiscoveryListener listener = new DynamicDiscoveryListener();
        listener.start();
        TimerQueue timer = TimerQueue.create();
        timer.addTimer("one second", 1.0);
        timer.addTimer("five seconds", 5.0);
        timer.run();
    }
}
