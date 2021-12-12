package com.ajrzeznik;

import java.nio.charset.StandardCharsets;
import java.time.Instant;
import java.util.PriorityQueue;

// TODO AR: clean up the message queue stuff to use a socket
public class TimerQueue extends Thread{
    private final PriorityQueue<Timer> queue = new PriorityQueue<>();
    private final PubSocket pubSocketToNode;

    public static TimerQueue create(PubSocket pubSocket) {
        return new TimerQueue(pubSocket);
    }

    private TimerQueue(PubSocket pubSocket) {
        pubSocketToNode = pubSocket;
    }

    public void addTimer(String name, double interval) {
        queue.add(new Timer(name, (long) (interval * 1000)));
    }

    public void run() {
        try{
            while (true) {
                Timer timer = queue.peek();
                assert timer != null;
                long currentTime = Instant.now().toEpochMilli();
                if (currentTime > timer.getNextTime()) {
                    timer = queue.poll();
                    pubSocketToNode.send(timer.name.getBytes(StandardCharsets.UTF_8));
                    timer.tick();
                    queue.add(timer);
                } else {
                    Thread.sleep(timer.getNextTime() - currentTime);
                }
            }
        } catch (InterruptedException e) {
            System.out.println("Failure in timer queue thread:");
            e.printStackTrace();
        }
    }

    static class Timer implements Comparable<Timer>{
        //Publicly facing static constructors use double time, but internally timer runs off of milliseconds

        private long nextTime;
        private final long interval;
        private final String name;

        private Timer(String name, long interval) {
            this.name = name;
            this.interval = interval;
            this.nextTime = Instant.now().toEpochMilli() + this.interval;
        }

        void tick() {
            nextTime += interval;
        }

        long getNextTime() {
            return nextTime;
        }

        String getName() {
            return name;
        }

        @Override
        public int compareTo(Timer timer) {
            return (int) (this.nextTime - timer.nextTime);
        }
    }

}
