use chrono;
//To run, use cargo run --bin test1 -q
use min_mq::receive_broadcast;

fn main() {
    loop{
        println!("Waiting for Broadcasting Data");
        receive_broadcast().unwrap();
        let local= chrono::Local::now();
        println!("{:?}", local.to_rfc3339());
        println!("Broadcast Receive complete");
    }
}