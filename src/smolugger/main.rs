mod executor;

fn main() {
    println!("Host process id : {}", std::process::id());
    let mut cmd = executor::create_process().unwrap();

    let pid = cmd.id();
    println!("Child process id: {}", pid);
    executor::start_tracing(cmd.id());

    println!("Child process pc: {:x}", executor::get_pc(pid));

    executor::dbg_cont(pid);

    println!("Child process pc: {:x}", executor::get_pc(pid));

    let status = cmd.wait().unwrap();

    if status.success() {
        println!("Command executed successfully");
    } else {
        println!("Command failed");
    }
}
