//To run, use cargo run --bin test1 -q
use min_mq::node::Node;

fn main() {
    println!("Broadcasting Data");
    let mut node = Node::new("rust_test_node");
    node.run();
    println!("Broadcast complete");
}