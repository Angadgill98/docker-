use libc::{
    syscall,
    SYS_clone3,
    SIGCHLD,
    CLONE_NEWPID,
    CLONE_NEWNET,
    CLONE_NEWNS
};

use std::{default, fs::File, mem, os::fd::AsRawFd};

mod cli;
mod sokcet;
mod brain;
mod ns;

fn main() {
    dotenvy::dotenv().ok();
    cli::CLI();
    
}

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



// fn CLI_printer(){
//     println!("")
// }


fn CreatePID_namespace(){
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

    if pid==0{

    }else if pid==1 {
        
    }
}

fn ChangeNetNamespce(pid:i64)-> Result<(), Box<dyn std::error::Error>>{
    let net_ns=File::open(format!("/proc/{}/ns/net", pid))?;

    let ok=unsafe {
        libc::setns(net_ns.as_raw_fd(), CLONE_NEWNS)
    };
    if ok==-1{

    }
    Ok(())
}

fn ChangeMountNamespce(pid:i64)-> Result<(), Box<dyn std::error::Error>>{
    let net_ns=File::open(format!("/proc/{}/ns/mnt", pid))?;
    let ok=unsafe {
        libc::setns(net_ns.as_raw_fd(), CLONE_NEWNS)
    };
    if ok==-1{

    }
    Ok(())
}

fn ParentProcessCode(){

}

fn ChildProcessCode(){

}