package com.ajrzeznik;

import java.io.IOException;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

/**
 * Hello world!
 *
 */
public class App 
{

    public App() {
    }

    public static void main( String[] args ) throws IOException, InterruptedException {
        // TODO AR: Need to join on this as it's not a Daemon
        Node node = Node.create("fun node");
        node.Subscribe("test_topic", (data) -> {
            System.out.println("I JUST RECEIVED A CALLBACL: <<" + data + ">>");
        });
        Node.Publisher pubber = node.addPublisher("test_topic");

        node.addTimer("One Second", 1, () -> {
            String timestring = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS"));
            System.out.println(timestring+": Triggered timer: One Second=====");//TODO AR: This needs to be send somewhere
            pubber.publish("This is some data!!!!!!");
        });

        node.addTimer("Five Second", 5, () -> {
            String timestring = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS"));
            System.out.println(timestring+": Triggered timer: Five Second");//TODO AR: This needs to be send somewhere
        });

        node.run();
    }
}
