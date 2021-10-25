//To run, use cargo run --bin test1 -q
use min_mq::receive_broadcast;

fn main() {
    println!("Broadcasting Data");
    receive_broadcast().unwrap();
    println!("Broadcast complete");
}