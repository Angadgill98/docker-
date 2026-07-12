use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::Write, mem, os::fd::AsRawFd};


use libc::{
    CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, MS_BIND, SIGCHLD, SYS_clone3, mount, syscall
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

pub fn CreateContainer()->i64{
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

fn CreateContainerFile()->Result<(), Box<std::io::Error>>{
    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("container.json")
    {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("Veth storage file already exists");
            return Ok(());
        }
        Err(e) => return Err(Box::new(e)),
    };

    file.write_all(b"{}")?;
    println!("Created the Veth storage file");

    Ok(())
}


fn CreateContainerStruct(pid:&i64,container_name:&String){
    
}


