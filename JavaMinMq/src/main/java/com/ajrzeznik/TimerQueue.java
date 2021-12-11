package com.ajrzeznik;

import java.time.Instant;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.PriorityQueue;

public class TimerQueue extends Thread{
    private final PriorityQueue<Timer> queue = new PriorityQueue<>();

    public static TimerQueue create() {
        return new TimerQueue();
    }

    private TimerQueue() {
    }

    public void addTimer(String name, double interval) {
        queue.add(Timer.create(name, interval));
    }

    public void run() {
        try{
            while (true) {
                Timer timer = queue.peek();
                assert timer != null;
                long currentTime = Instant.now().toEpochMilli();
                if (currentTime > timer.getNextTime()) {
                    timer = queue.poll();
                    String timestring = LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss.SSS"));
                    System.out.println(timestring+": Triggered timer: "+ timer.getName());//TODO AR: This needs to be send somewhere
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
}
