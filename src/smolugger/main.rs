use std::process::{Command, Stdio};

fn main() {

    let cmd = Command::new("/workspaces/smolugger_rs/target/debug/target")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    let status = cmd.unwrap().wait();

    if status.unwrap().success() {
        println!("Command executed successfully");
    } else {
        println!("Command failed");
    }

    return;


}
