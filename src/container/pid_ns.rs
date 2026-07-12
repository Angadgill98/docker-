use std::{fs::File, mem, os::fd::AsRawFd};

use libc::{
    syscall,
    SYS_clone3,
    SIGCHLD,
    CLONE_NEWPID,
    CLONE_NEWNET,
    CLONE_NEWNS
};

#[repr(C)]
#[derive(Default)]
struct CloneArgs {
    flags: u64,
    pidfd: u64,
    child_tid: u64,
    parent_tid: u64,
    exit_signal: u64,
    stack: u64,
    stack_size: u64,
    tls: u64,
    set_tid: u64,
    set_tid_size: u64,
    cgroup: u64,
}

fn TempProcess()->i64{
    println!("Enter the name of the pid namespace:");
    let mut input=String::new() ;
    std::io::stdin().read_line(&mut input).unwrap();

    
    let mut args=CloneArgs::default();
    args.flags = (
        CLONE_NEWPID | 
        CLONE_NEWNET | 
        CLONE_NEWNS
    ) as u64;

    args.exit_signal=SIGCHLD as u64;

    let pid =unsafe {
        syscall(SYS_clone3,&args as *const CloneArgs,mem::size_of::<CloneArgs>())
    };

    if pid == -1 {
        panic!("clone3 failed");
    } else if pid == 0 {
        // Child process
        loop {
            unsafe {
                libc::pause();
            }
        }
    } else {
        // Parent process
        println!("Child PID = {}", pid);
    }
    pid
}


