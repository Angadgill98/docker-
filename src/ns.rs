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


fn Create_PID_namespace(){

}

fn CreateNetNamespaceReference(){
    let temp_pid=CreateTempProcess();
    let temp_process_id=GetChildPID(temp_pid);
    let net_ns_ref=GetProcNS(temp_pid);
    if temp_pid<1{

    }else if temp_pid==-1 {
        
    }else{

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

fn CreateTempProcess()->i64{
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

    pid
}

fn GetChildPID(host_pid:i64)->i32{
    host_pid as libc::pid_t
}

fn GetProcNS(pid:i64)->Result<File,std::io::Error>{
    let path = format!("/proc/{}/ns/net", pid);
    let netns = File::open(path)?;
    Ok(netns)
}




