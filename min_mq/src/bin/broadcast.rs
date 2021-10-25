//To run, use cargo run --bin test1 -q
use min_mq::broadcast_address;

fn main() {
    println!("Broadcasting Data");
    broadcast_address().unwrap();
    println!("Broadcast complete");
}