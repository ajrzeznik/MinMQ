package com.ajrzeznik;

import com.google.gson.Gson;
import com.google.gson.internal.LinkedTreeMap;

import java.io.IOException;

/**
 * Hello world!
 *
 */
public class App 
{

    public App() {
    }

    public static void main( String[] args ) throws IOException, InterruptedException {
        //JSON parsing testing
        // TODO AR: Need to join on this as it's not a Daemon
        Gson gson = new Gson();
        Node node = Node.create("fun node");
        node.Subscribe("test_topic", (data) -> {
            System.out.println("I JUST RECEIVED A CALLBACK: <<" + data + ">>");
        });

        node.Subscribe("test_topic_json", (data) -> {
            System.out.println("JSON CALLBACK: <<" + gson.fromJson(data , LinkedTreeMap.class) + ">>");
        });

        Node.TextPublisher pubber = node.addTextPublisher("test_topic");
        Node.JsonPublisher pubberJson = node.addJsonPublisher("test_topic_json");

        node.addTimer("One Second", 1, () -> {
            //String timestring = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS"));
            //System.out.println(timestring+": Triggered timer: One Second=====");//TODO AR: This needs to be send somewhere
            pubber.publish("This is some data!!!!!!");
            pubberJson.publish(new Object(){
                boolean data1 = true;
                String string1 = "This is anonymous";
                double double1 = 3.5;
            });
        });

        node.addTimer("Five Second", 5, () -> {
            //String timestring = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS"));
            //System.out.println(timestring+": Triggered timer: Five Second");//TODO AR: This needs to be send somewhere
        });

        node.run();
    }
}
