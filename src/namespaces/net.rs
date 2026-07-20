use crate::namespaces;




use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::Write, mem, os::fd::{AsRawFd, OwnedFd}, path::Path};


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



use serde::{Serialize, Deserialize};


#[derive(Clone,Serialize, Deserialize)]
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

    pub fn CreateFile(&self)->Result<(), Box<std::io::Error>>{
        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("net_ns.json")
        {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                println!("net ns storage file already exists");
                return Ok(());
            }
            Err(e) => return Err(Box::new(e)),
        };

        file.write_all(b"{}")?;
        println!("Created the net ns storage file");

        Ok(())
    }

    pub fn ReadFile(&self)->Result<HashMap<String,Self>,Box<dyn std::error::Error>>{
        let paylaod=fs::read_to_string("net_ns.json")?;
        let obj:HashMap<String,Self>=serde_json::from_str(&paylaod)?;
        Ok(obj)
    }

    pub fn WriteToFile(&self,mut obj:HashMap<String,Self>)->Result<(),Box<dyn std::error::Error>>{
    
        fs::write("net_ns.json", serde_json::to_string_pretty(&obj)?)?;
        Ok(())
    } 
    
    fn RemoveBindMount(&self){
        let target = CString::new(format!("dc_ns/net/{}",self.name)).unwrap();
        let ret=unsafe {
            libc::umount2(target.as_ptr(), libc::MNT_DETACH)
        };

        if ret != 0 {
            eprintln!("umount failed: {}", std::io::Error::last_os_error());
        }
        
    }

    fn DeleteFile(&self)->Result<(),Box<dyn std::error::Error>>{
        fs::remove_file(format!("dc_ns/net/{}", self.name))?;
        Ok(())
    }

    pub fn GetNetFS(&self,pid:&i64)->Result<OwnedFd,Box<dyn std::error::Error>>{
        let target=format!("/proc/{}/ns/net",pid);
        let file=File::open(target)?;
        let fd: OwnedFd = file.into();

        Ok(fd)
    }
    
}

impl namespaces::Namespace for Net_ns {
    fn Create(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool) {
        let child_pid=self.CreateTempProcess(net_flag, pid_flag, mount_flag);

    }

    fn BindMount(&self,child_pid:i64)->Result<(),Box<dyn std::error::Error>>{
        
        //name is with self 
        let source=CString::new(format!("/proc/{}/ns/net",child_pid)).unwrap();
        
        let target=CString::new(format!("dc_ns/net/{}",self.name)).unwrap();

        self.verify_file(source.to_str()?);
        
        std::fs::create_dir_all(format!("dc_ns/net"))?;
        File::create(&target.to_str()?)?;

        let ret=unsafe {
            mount(
                source.as_ptr(),
                target.as_ptr(),
                std::ptr::null(),
                MS_BIND,
                std::ptr::null(),
            )

        };
        if ret != 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        Ok(())
    }

    fn Delete(&self,target_mount:&String) {
        self.RemoveBindMount();
        self.DeleteFile().unwrap();
        println!("Bind mount is reomved the namspce will be remove d once the process referencing to it stops");
    }
}