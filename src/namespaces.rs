pub mod net;


use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::Write, mem, os::fd::AsRawFd, path::Path};


use libc::{
    CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, MS_BIND, SIGCHLD, SYS_clone3, mount, syscall
};
use serde_json::Value;

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

trait Namespace {
    fn Create(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool);

    fn BindMount(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool,name:&String)->Result<(),Box<dyn std::error::Error>>;

    fn Delete(&self);

    fn CreateTempProcess(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool)->i64{
        
        let mut flags=0;
        unsafe{
            if *pid_flag{
                flags |= libc::CLONE_NEWPID as u64;
            }

            if *net_flag {
                flags |= libc::CLONE_NEWNET as u64;
            }
            if *mount_flag{
                flags |= libc::CLONE_NEWNS as u64;
            }
        }
        let mut args = CloneArgs {
            flags,
            ..Default::default()
        };

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

}