use std::collections::BinaryHeap;
use std::{thread, time};
use std::cmp::Ordering;
use std::time::Instant;
use core::time::Duration;

#[derive(Clone, Eq, PartialEq)]
struct Timer {
    name: String,
    interval: Duration,
    next_event: Instant
}

impl Timer {
    pub fn new(name: &str, interval: f64) -> Self{
        Timer {
            name: name.to_string(),
            interval: Duration::from_secs_f64(interval),
            next_event: time::Instant::now() + Duration::from_secs_f64(interval)
        }
    }
}

impl PartialOrd<Self> for Timer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timer {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.next_event.cmp(&self.next_event)
            .then_with(|| self.interval.cmp(&other.interval)) //TODO AR: Maybe this ordering on interval matters, as it might differentiate a bit
            .then_with(|| self.name.cmp(&other.name))
    }
}

pub fn start_timer() {
    let mut timer_queue = BinaryHeap::<Timer>::new();
    timer_queue.push(Timer::new("OneSec", 1.0));
    timer_queue.push(Timer::new("FiveSec", 5.0));
    loop {
        let next_timer = timer_queue.peek().expect("Timer queue was empty!!!");
        let current_time = time::Instant::now();
        if next_timer.next_event <= current_time {
            let mut old_timer = timer_queue.pop().expect("Timer queue was empty!!!");
            old_timer.next_event += old_timer.interval;
            println!("Event triggered! Name: {}", old_timer.name);
            timer_queue.push(old_timer);
        } else {
            thread::sleep(next_timer.next_event-current_time);
        }
    }


}