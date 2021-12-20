use std::process::Command;

fn main() {
    println!("Begin custom build script");
    let output = if cfg!(target_os = "windows") {
        Command::new("flatc.exe")
            .args(["-r", "-j", "--gen-object-api", "--gen-mutable", "-o", "generated", ".\\src\\message_definitions\\NodeAddress.fbs"])
            .output()
            .expect("failed to execute process")
    } else {
        println!("Performing Linux Build");
        Command::new("./flatc")
            .args(["-r", "-j", "--gen-object-api", "--gen-mutable", "-o", "generated", "./src/message_definitions/NodeAddress.fbs", "./src/message_definitions/MQMessage.fbs"])
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
    let result_string = std::str::from_utf8(&hello).unwrap();
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    println!("{}", result_string);
}