pub mod bridge;
pub mod veth;




use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::Write, mem, os::fd::AsRawFd, path::Path};


use libc::{
    CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, MS_BIND, SIGCHLD, SYS_clone3, mount, syscall
};
use serde_json::Value;

use crate::namespaces::{self, Namespace};

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


pub struct container{
    pub name:String,

    pub bridge:bridge::Bridge,
    pub veth:veth::VethPair,

    pub mount_ns:namespaces::mount::mount,
    pub net_ns:namespaces::net::Net_ns,
    pub pid_ns:namespaces::pid::pid_ns
}


impl container{
    pub fn Init(&self){
        let child_pid=self.pid_ns.CreateTempProcess(&true, &true,& true);
        
    }
} 
    
