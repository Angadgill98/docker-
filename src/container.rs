pub mod bridge;
pub mod veth;




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


struct container_init{
    container_name:String,
    pid_as_master:Option<u8>,
    pid_as_child:Option<i64>,


    net_flag:bool,
    pid_flag:bool,
    mount_flag:bool,


    net_ns_fd_path:Option<String>,
    process_ns_fd_path:Option<String>,
    mount_ns_fd_path:Option<String>,
    
}

impl container_init {
    fn init(&mut self,data:Value)->Result<(),Box<dyn std::error::Error>>{

        let net_flag=data["net_ns"].as_bool().ok_or("not valid ent flag")?;
        let pid_flag=data["net_ns"].as_bool().ok_or("not valid ent flag")?;
        let mount_flag=data["net_ns"].as_bool().ok_or("not valid ent flag")?;


        let pid=self.CreateTempProcess(&net_flag,&pid_flag,&mount_flag);
        self.pid_as_child=Some(pid);


        


        Ok(())
    }


    pub fn CreateTempProcess(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool)->i64{
        
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

    pub fn Create_BM_net(&self)->Result<(),Box<dyn std::error::Error>>{
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
    
    pub fn Create_BM_mount(&self)->Result<(),Box< dyn std::error::Error>>{
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


    fn verify_path(&self,path: &str) -> bool {
        Path::new(path).exists()
    }


    fn verify_file(&self,path: &str) -> bool {
        Path::new(path).is_file()
    }
    //define path here 
    fn GetProcNS(pid:i64)->Result<File,std::io::Error>{
        let path = format!("{}", pid);
        let netns = File::open(path)?;
        Ok(netns)
    }

    fn ChangeNetns(){

    }


}