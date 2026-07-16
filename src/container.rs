pub mod bridge;
pub mod veth;




use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::{Read, Write}, mem, os::fd::AsRawFd, path::Path};


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


    fn CreateInitFile()->Result<(), Box<std::io::Error>>{
        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("init.json")
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

    fn ReadInitFile()->Result<HashMap<String,String>,Box<dyn std::error::Error>>{
        let json_str=fs::read_to_string("init.json")?;
        let json:HashMap<String,String>=serde_json::from_str(json_str.as_str())?;

        Ok(json)
    }

    fn WriteInitFile(){

    }



    fn CreateConFile()->Result<(), Box<std::io::Error>>{
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

    fn ReadConFile()->Result<HashMap<String,String>,Box<dyn std::error::Error>>{
        let json_str=fs::read_to_string("container.json")?;
        let json:HashMap<String,String>=serde_json::from_str(json_str.as_str())?;

        Ok(json)
    }

    fn WriteConFile(){

    }

} 
    
