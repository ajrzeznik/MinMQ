use std::process::Command;

fn main() {

    let output = if cfg!(target_os = "windows") {
        Command::new("flatc.exe")
            .args(["-r", "--gen-object-api", "-o", "generated", ".\\src\\message_definitions\\NodeAddress.fbs"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("flatc")
            .args(["-r", "--gen-object-api", "-o", "generated", "./src/message_definitions/NodeAddress.fbs"])
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;
    let result_string = std::str::from_utf8(&hello).unwrap();
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    println!("{}", result_string);
}