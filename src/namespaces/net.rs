use crate::namespaces;




use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::Write, mem, os::fd::AsRawFd, path::Path};


use futures::TryStreamExt;
use libc::{
    CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, IN_UNMOUNT, MS_BIND, SIGCHLD, SYS_clone3, mount, syscall
};   
use rtnetlink::Handle;
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



pub struct Net_ns{
    pub name:String
}

impl Net_ns {
    fn verify_path(&self,path: &str) -> bool {
        Path::new(path).exists()
    }


    fn verify_file(&self,path: &str) -> bool {
        Path::new(path).is_file()
    }

    fn JoinProcessNS(&self,target_pid:&i64)->Result<(),Box<dyn std::error::Error>>{
        let file = File::open(format!("/proc/{}/ns/net",target_pid))?;
        let fd = file.as_raw_fd();
        
        let result = unsafe {
            libc::setns(fd, CLONE_NEWNET)
        };

        if result != 0 {
            panic!("setns failed");
        }
        Ok(())
    }

    fn JoinNamedNS(&self,target_ns_name:&String)->Result<(),Box<dyn std::error::Error>>{
        //insert path here
        let file = File::open(format!("{}",target_ns_name))?;
        let fd = file.as_raw_fd();

        let result = unsafe {
            libc::setns(fd, CLONE_NEWNET)
        };

        if result != 0 {
            panic!("setns failed");
        }
        Ok(())
    }


    
}

impl namespaces::Namespace for Net_ns {
    fn Create(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool) {
        let child_pid=self.CreateTempProcess(net_flag, pid_flag, mount_flag);

    }

    fn BindMount(&self,child_pid:i64)->Result<(),Box<dyn std::error::Error>>{
        
        //name is with self 
        let source=CString::new("").unwrap();
        
        let target=CString::new("").unwrap();

        self.verify_file(source.to_str()?);
        
        File::create(&target.to_str()?);

        unsafe {
            mount(
                source.as_ptr(),
                target.as_ptr(),
                std::ptr::null(),
                MS_BIND,
                std::ptr::null(),
            );

        }

        Ok(())
    }

    fn Delete(&self,target_mount:&String) {
        let target=CString::new("").unwrap();

        let ret = unsafe {
            libc::umount(target.as_ptr())
        };

        if ret != 0 {
            panic!("umount failed");
        }

        println!("Bind mount is reomved the namspce will be remove d once the process referencing to it stops");
    }
}