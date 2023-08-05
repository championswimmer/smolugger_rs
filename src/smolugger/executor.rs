use nix::{
    sys::ptrace::{self, AddressType},
    unistd::{self, Pid},
};
use std::{
    io::Error,
    process::{Child, Command, Stdio},
};

pub fn create_process() -> Result<Child, Error> {
    let cmd = Command::new("/workspaces/smolugger_rs/target/debug/target")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    cmd
}

pub fn start_tracing(id: u32) -> Pid {
    let _pid = id as i32;
    println!("Attaching to process {}", _pid);
    let _ = ptrace::traceme();
    let pid = unistd::Pid::from_raw(_pid);
    ptrace::attach(pid).unwrap_or_else(|_| panic!("Failed to attach to process {}", _pid));
    println!("Attached to process {}", _pid);
    pid
}

pub fn dbg_cont(pid: u32) {
    let _pid = unistd::Pid::from_raw(pid as i32);

    ptrace::cont(_pid, None);
}

pub fn get_pc(pid: u32) -> u64 {
    let _pid = unistd::Pid::from_raw(pid as i32);

    let addr: AddressType = std::ptr::null_mut();
    ptrace::read(_pid, addr);
    addr as u64
}
