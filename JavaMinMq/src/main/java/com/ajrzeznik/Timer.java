package com.ajrzeznik;

import java.time.Instant;

public class Timer implements Comparable<Timer>{
    //Publicly facing static constructors use double time, but internally timer runs off of milliseconds

    private long nextTime;
    private final long interval;
    private String name;

    public static Timer create(String name, double interval){
        return new Timer(name, (long) (interval*1000));
    }
    private Timer(String name, long interval) {
        this.name = name;
        this.interval = interval;
        this.nextTime = Instant.now().toEpochMilli() + this.interval;
    }

    void tick() {
        nextTime += interval;
    }

    public long getNextTime() {
        return nextTime;
    }

    public String getName() {
        return name;
    }

    @Override
    public int compareTo(Timer timer) {
        return (int) (this.nextTime - timer.nextTime);
    }

    //@Override
    //public boolean equals(Object obj){
    //
    //}
}
