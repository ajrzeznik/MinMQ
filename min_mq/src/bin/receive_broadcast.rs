//To run, use cargo run --bin test1 -q
use min_mq::receive_broadcast;

fn main() {
    println!("Waiting for Broadcasting Data");
    receive_broadcast().unwrap();
    println!("Broadcast Receive complete");
}