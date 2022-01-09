//To run, use cargo run --bin test1 -q
use min_mq::node::Node;

fn main() {
    println!("Broadcasting Data");
    let mut node = Node::new("rust_test_node");
    let mut a = 45;

    let pubber = node.add_text_publisher("test_topic");

    node.add_timer("one_second", 1.0, move || {
        a += 1;
        println!("Testclosure: {}", a);
        pubber.publish(&format!("RustData!!!!!! Number {} !!!!!!!",a));
    });

    node.subscribe_text("test_topic", move |text| {
        println!("==I just received the test: {}", text)
    });



    node.run();
    println!("Broadcast complete");
}