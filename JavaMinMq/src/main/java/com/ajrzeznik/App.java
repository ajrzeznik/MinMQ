package com.ajrzeznik;

import com.google.gson.Gson;
import com.google.gson.internal.LinkedTreeMap;

import java.io.IOException;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;

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
        String nestedJSON = "{\"id\":\"1\",\"message\":\"web_didload\",\"content\":{\"success\":1}, \"testar\":[1,2,3]}";
        Gson gson = new Gson();
        LinkedTreeMap result = gson.fromJson(nestedJSON , LinkedTreeMap.class);
        System.out.println(result.toString());
        System.out.println(((ArrayList) (result.get("testar"))).get(2));
        // TODO AR: Need to join on this as it's not a Daemon



        Node node = Node.create("fun node");
        node.Subscribe("test_topic", (data) -> {
            System.out.println("I JUST RECEIVED A CALLBACK: <<" + data + ">>");
        });
        Node.Publisher pubber = node.addPublisher("test_topic");

        node.addTimer("One Second", 1, () -> {
            //String timestring = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS"));
            //System.out.println(timestring+": Triggered timer: One Second=====");//TODO AR: This needs to be send somewhere
            pubber.publish("This is some data!!!!!!");
        });

        node.addTimer("Five Second", 5, () -> {
            //String timestring = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS"));
            //System.out.println(timestring+": Triggered timer: Five Second");//TODO AR: This needs to be send somewhere
        });

        node.run();
    }
}
