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
        listener.run();
    }
}
