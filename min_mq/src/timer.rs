use std::collections::BinaryHeap;
use std::{thread, time};
use std::cmp::Ordering;
use std::time::Instant;
use core::time::Duration;
use std::sync::mpsc::Receiver;
use mq_message_base::{MessageType, MQMessageT};
use crate::sockets::PubSocket;

#[derive(Clone, Eq, PartialEq)]
pub struct Timer {
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

pub fn start_timer(receiver: Receiver<Option<Timer>>, name: String, port: u16) {
    //TODO AR: For now pass the whole timer queue in for simplicity. In the future, will need
    // to use a channel to pass the data across.
    let mut timer_queue = BinaryHeap::<Timer>::new();
    let socket = PubSocket::new(&("tcp://localhost:".to_owned() + &port.to_string()));

    loop {
        let new_timer_result =  receiver.recv();
        let new_timer = match new_timer_result {
            Ok(t) => t,
            Err(e) => {println!("{}", e.to_string()); panic!()}
        };
        match new_timer {
            Some(t) => timer_queue.push(t),
            None => break
        }
    }

    loop {
        let next_timer = timer_queue.peek().expect("Timer queue was empty!!!");
        let current_time = time::Instant::now();
        if next_timer.next_event <= current_time {
            let mut old_timer = timer_queue.pop().expect("Timer queue was empty!!!");
            old_timer.next_event += old_timer.interval;

            //TODO AR: Use more standare MQMessage interface
            let mut msg = MQMessageT::default();
            msg.message_type = MessageType::Topic;
            msg.topic = Some(old_timer.name.clone());
            msg.origin = Some(name.clone());
            let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(256);
            let mut temp = msg.pack(&mut fbb);
            fbb.finish(temp, None);
            socket.send(fbb.finished_data());
            println!("Event triggered! Name: {}", old_timer.name);
            timer_queue.push(old_timer);
        } else {
            thread::sleep(next_timer.next_event-current_time);
        }
    }


}