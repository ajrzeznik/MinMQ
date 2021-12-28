package com.ajrzeznik;

import java.io.IOException;
import java.net.ServerSocket;
import java.net.SocketException;
import java.nio.charset.StandardCharsets;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.function.Consumer;

import com.google.flatbuffers.FlatBufferBuilder;
import com.google.gson.Gson;

public class Node {

    private final HashMap<String, Consumer<String>> callbackMap = new HashMap<>();
    private final SubSocket receiveSocket;
    private final int port;
    private final TimerQueue timerQueue;
    private final AddressMap addressMap;
    private final String name;
    private final DynamicDiscoveryBroadcaster broadcaster;
    private final HashMap<String, HashMap<String, PubSocket>> publisherMap = new HashMap<>();
    private final HashSet<String> localPublishers = new HashSet<>();
    private final HashSet<String> localSubscribers = new HashSet<>(); //TODO AR: Can we support double/multi subscriptions? i.e. multiple callbacks per topic?
    private final Gson gson = new Gson(); //TODO AR: Look into making lazy instantiated, and possibly threading issues here.
    //From online notes, Gson instances are threadsafe
    private final byte[] pingBytes;
    private final byte[] ackBytes;


    public static Node create(String name) throws IOException {
        return new Node(name);
    }

    private Node(String name) throws IOException {
        // Get a free port by binding a socket , closing, and pulling from that
        this.name = name;
        ServerSocket serverSocket = new ServerSocket(0);
        port = serverSocket.getLocalPort();
        serverSocket.close();

        receiveSocket = SubSocket.create("tcp://*:" + port);
        addressMap = new AddressMap(name);
        timerQueue = TimerQueue.create(PubSocket.create("tcp://localhost:"+port));
        broadcaster = new DynamicDiscoveryBroadcaster(name, port);

        // Fixed Message Creation

        FlatBufferBuilder builder = new FlatBufferBuilder();
        //TODO AR: Clean up this creation/work here on these types
        MQMessage.finishMQMessageBuffer(builder, MQMessage.createMQMessage(builder,
                builder.createString(""),
                builder.createString(name),
                MessageType.Ping,
                builder.createByteVector(new byte[0]))
        );
        pingBytes = builder.sizedByteArray();


        builder = new FlatBufferBuilder();
        //TODO AR: Clean up this creation/work here on these types
        MQMessage.finishMQMessageBuffer(builder, MQMessage.createMQMessage(builder,
                builder.createString(""),
                builder.createString(name),
                MessageType.Ack,
                builder.createByteVector(new byte[0]))
        );
        ackBytes = builder.sizedByteArray();


    }

    abstract class Publisher<T> {
        protected final String topic;
        protected Publisher(String topic) {
            this.topic = topic;
        }
        abstract void publish(T data);
    }

    //TODO AR: Stringify
    class TextPublisher extends Publisher<String> {
        //TODO AR: This can be a lot more efficient if we use a reference instead, maybe


        private TextPublisher(String topic) {
            super(topic);
        }

        public void publish(String data){
            //TODO AR: Clean up this if check, it's not strictly needed here
            if (publisherMap.containsKey(topic)) {
                FlatBufferBuilder builder = new FlatBufferBuilder();
                //TODO AR: Clean up this creation/work here on these types
                MQMessage.finishMQMessageBuffer(builder, MQMessage.createMQMessage(builder,
                        builder.createString(topic),
                        builder.createString(name),
                        MessageType.Topic,
                        builder.createByteVector(data.getBytes(StandardCharsets.UTF_8)) //TODO AR: Add proper serialization here!!!!
                ));
                byte[] pubBytes = builder.sizedByteArray();
                for (Map.Entry<String, PubSocket> item : publisherMap.get(topic).entrySet()) {
                    item.getValue().send(pubBytes);
                }
            }
        }
    }

    //TODO AR: Stringify
    class JsonPublisher extends Publisher<Object> {
        //TODO AR: This can be a lot more efficient if we use a reference instead, maybe


        private JsonPublisher(String topic) {
            super(topic);
        }

        public void publish(Object data){
            //TODO AR: Clean up this if check, it's not strictly needed here
            if (publisherMap.containsKey(topic)) {
                FlatBufferBuilder builder = new FlatBufferBuilder();
                //TODO AR: Clean up this creation/work here on these types
                MQMessage.finishMQMessageBuffer(builder, MQMessage.createMQMessage(builder,
                        builder.createString(topic),
                        builder.createString(name),
                        MessageType.Topic,
                        builder.createByteVector(gson.toJson(data).getBytes(StandardCharsets.UTF_8)) //TODO AR: Add proper serialization here!!!!
                ));
                byte[] pubBytes = builder.sizedByteArray();
                for (Map.Entry<String, PubSocket> item : publisherMap.get(topic).entrySet()) {
                    item.getValue().send(pubBytes);
                }
            }
        }
    }

    public void addTimer(String name, double interval, Runnable callback) {
        // TODO AR: handle extra naming here
        // TODO AR: Handle dynamic addition of tiemrs
        // TODO AR: Handle remove of timers maybe???
        callbackMap.put(name, (data) -> {callback.run();});
        timerQueue.addTimer(name, interval);
    }

    public TextPublisher addTextPublisher(String topic){
        //TODO AR: Check this syncing and possibly clean it up elsewhere
        localPublishers.add(topic);
        publisherMap.put(topic, new HashMap<>());
        return new TextPublisher(topic);
    }

    public JsonPublisher addJsonPublisher(String topic){
        //TODO AR: Check this syncing and possibly clean it up elsewhere
        localPublishers.add(topic);
        publisherMap.put(topic, new HashMap<>());
        return new JsonPublisher(topic);
    }

    public void Subscribe(String topic, Consumer<String> callback){
        //TODO AR: TYPE CHECKING!!!!!! DO A LOT OF THAT!!!!! And make this actually work with the right types
        localSubscribers.add(topic);
        callbackMap.put(topic, callback);
    }



    void run() throws InterruptedException, SocketException {

        //TODO AR: Unify socket usage
        DynamicDiscoveryListener listener = new DynamicDiscoveryListener(PubSocket.create("tcp://localhost:" + port));
        listener.start();

        addTimer("Dynamic Discovery and Ping", 2.0, () -> {
            try {
                broadcaster.broadcast();
            } catch (IOException e) {
                //Todo AR: handle this IO Error somehow
                e.printStackTrace();
            }

            // Ping anything that isn't properly connected

            for (Map.Entry<String,PubSocket> item : addressMap.socketMap.entrySet()) {
                PubSocket socket= item.getValue();
                if (!socket.isConnected()) {
                    System.out.println("Pinging Socket: " + item.getKey());
                    socket.send(pingBytes);
                }
            }


        });


        //TODO AR: Add in timer/main thread synchronization.
        timerQueue.start();
        while (true) {
            MQMessage message = MQMessage.getRootAsMQMessage(receiveSocket.receive());
            switch (message.messageType()) {
                case MessageType.Topic:
                    String strMsg = message.topic();
                    System.out.println("Received message of topic: "+ strMsg );
                    //if message.getByteBuffer();
                    callbackMap.get(strMsg).accept(StandardCharsets.UTF_8.decode(message.dataAsByteBuffer()).toString());
                    break;
                case MessageType.Address:
                    //TODO AR: Deserialize data to node address
                    //TODO AR: Clean up and send out the pub/sub additions
                    String address = "tcp:/" + message.topic();
                    if (addressMap.updateAddress(message.origin(), address)){
                        System.out.println("==NEW ADDRESS: name: " + message.origin()+ ", address: " + address);
                        System.out.println(LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS")));
                    }
                    break;
                case MessageType.Ping:
                    System.out.println("Received Ping from " + message.origin() );
                    //If we don't contain the key, we deal with that issue by waiting for it to be added by an address msg
                    if (addressMap.socketMap.containsKey(message.origin())){
                        addressMap.socketMap.get(message.origin()).send(ackBytes);
                    }
                    break;

                case MessageType.Ack:
                    System.out.println("Received Ack from " + message.origin() );
                    // Our address map MUST contain the origin in the case of an Ack; if not we would fail here
                    addressMap.socketMap.get(message.origin()).setConnected();

                    if (addressMap.allNewlyConnected()) {
                        //TODO AR: Clean this up to send out all the required messages for synchronization
                        System.out.println("***SEND OUT ALL CONNECTED MESSAGES***");



                        FlatBufferBuilder builder = new FlatBufferBuilder();
                        int[] subOffsets = new int[localSubscribers.size()];

                        int i = 0;
                        for (String item : localSubscribers) {
                            subOffsets[i] = builder.createString(item);
                            i += 1;
                        }
                        int finalSubOffset = PubSub.createSubVector(builder, subOffsets);
                        //TODO AR: Add publisher
                        PubSub.startPubSub(builder);
                        PubSub.addSub(builder, finalSubOffset);
                        builder.finish(PubSub.endPubSub(builder));

                        byte[] dataBytes = builder.sizedByteArray();

                        //TODO AR: Clean up this creation/work here on these types
                        builder = new FlatBufferBuilder();
                        PubSub.finishPubSubBuffer(builder, MQMessage.createMQMessage(builder,
                                builder.createString(""),
                                builder.createString(name),
                                MessageType.PubSub,
                                builder.createByteVector(dataBytes)
                        ));
                        byte[] msgBytes = builder.sizedByteArray();

                        for (Map.Entry<String, PubSocket> item: addressMap.socketMap.entrySet()){
                            item.getValue().send(msgBytes);
                        }

                    }
                    break;

                case MessageType.PubSub:
                    System.out.println("Received PubSub from " + message.origin() );
                    // Our address map MUST contain the origin in the case of an Ack; if not we would fail here
                    addressMap.socketMap.get(message.origin()).setConnected();

                    PubSub pubsubData = PubSub.getRootAsPubSub(message.dataAsByteBuffer());

                    //TODO AR:Do I even need the pub data? Really I just need sub and can cross-reference
                    for (int i = 0; i < pubsubData.subLength(); i++) {
                        String topic = pubsubData.sub(i);
                        System.out.println("***SUBREQ: " + topic);
                        if (localPublishers.contains(topic)){
                            if (publisherMap.containsKey(topic)) {
                                HashMap<String, PubSocket> activePubMap = publisherMap.get(topic);
                                //TODO AR: message.origin should probably be a variable here
                                if (!activePubMap.containsKey(message.origin())) {
                                    //TODO AR: This is a critical place to check thread safety, BE CAREFUL. Need to copy replace here/
                                    activePubMap.put(message.origin(), addressMap.socketMap.get(message.origin()));
                                }
                            } else {
                                //TODO AR***: This should PROBABLY be created when a new local publisher is created? Yeah,
                                // That would make things more efficient with sharing the publication hashmap
                                HashMap<String, PubSocket> newPubMap = new HashMap<>();
                                newPubMap.put(message.origin(), addressMap.socketMap.get(message.origin()));
                                publisherMap.put(topic, newPubMap);
                            }
                        }
                    }

                    break;

            }

        }
    }
}
