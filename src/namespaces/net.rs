use crate::namespaces;




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



struct Net_ns{
    name:String
}

impl Net_ns {
    fn verify_path(&self,path: &str) -> bool {
        Path::new(path).exists()
    }


    fn verify_file(&self,path: &str) -> bool {
        Path::new(path).is_file()
    }

    
}

impl namespaces::Namespace for Net_ns {
    fn Create(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool) {
        let child_pid=self.CreateTempProcess(net_flag, pid_flag, mount_flag);

    }

    fn BindMount(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool,name:&String)->Result<(),Box<dyn std::error::Error>>{
        let child_pid=self.CreateTempProcess(net_flag, pid_flag, mount_flag);

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

    fn Delete(&self) {
        
    }
}