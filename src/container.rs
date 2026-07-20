pub mod bridge;
pub mod veth;




use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::{Read, Write}, mem, net::Ipv4Addr, os::fd::AsRawFd, path::Path};


use libc::{
    CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, MS_BIND, SIGCHLD, SYS_clone3, mount, syscall
};
use rtnetlink::Handle;
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



use serde::{Serialize, Deserialize};

#[derive(Clone,Serialize,Deserialize)]
pub struct container{
    pub name:String,

    pub bridge:bridge::Bridge,
    pub veth:veth::VethPair,

    pub mount_ns:Option<namespaces::mount::mount>,
    pub net_ns:Option<namespaces::net::Net_ns>,
    pub pid_ns:Option<namespaces::pid::pid_ns>,


    pub pid:Option<i32>
}


impl container{
    
    pub async  fn Init(&self,handle:&Handle)->Result<i32,Box<dyn std::error::Error>>{
        let pid_flag=true;
        let mount_flag=true;
        let net_flag=true;

        let mut flags=0;
        unsafe{
            if pid_flag{
                flags |= libc::CLONE_NEWPID as u64;
            }
            if net_flag {
                flags |= libc::CLONE_NEWNET as u64;
            }
            if mount_flag{
                flags |= libc::CLONE_NEWNS as u64;
            }
        }
        let mut args = CloneArgs {
            flags,
            ..Default::default()
        };

        args.exit_signal=SIGCHLD as u64;

        let parent_pid = unsafe { libc::getpid() };
        let parent_net_ns_fd=self.net_ns.as_ref().unwrap().GetNetFD(&parent_pid)?;
        self.veth.veth_back.SetVethInNetns(parent_net_ns_fd.as_raw_fd(), handle).await;

        let ret =unsafe {
            syscall(SYS_clone3,&args as *const CloneArgs,mem::size_of::<CloneArgs>())
        };

        let child_pid = ret as libc::pid_t;

        

        if child_pid == -1 {
            panic!("clone3 failed");
        } else if child_pid == 0 {
            // Child process

            //dont delete kill singal woks  wiht code and ntoing else
            unsafe {
                let mut sa: libc::sigaction = std::mem::zeroed();

                sa.sa_sigaction =KillSignalHnadler as usize;
                sa.sa_flags = 0;

                libc::sigemptyset(&mut sa.sa_mask);

                if libc::sigaction(libc::SIGTERM, &sa, std::ptr::null_mut()) != 0 {
                    panic!("sigaction failed");
                }
            }
            

            self.AddNEtworkingRules(handle, self.veth.veth_front.ip.unwrap().clone(), self.veth.veth_front.GetIndex(handle).await).await?;
            self.EnableLoopInterface(handle).await?;
            loop {
                unsafe {
                    libc::pause();
                }
            }
        } else {
            // Parent process



            let net_fd=self.net_ns.as_ref().expect("msg").GetNetFD(&child_pid)?;
            self.veth.veth_front.SetVethInNetns(net_fd.as_raw_fd(), handle).await;
            println!("Child PID = {}", child_pid);
            
        }
        

        Ok(child_pid)
    }

    

    pub fn Kill(&self){
        let ret = unsafe {
            libc::kill(self.pid.unwrap(), libc::SIGTERM)
        };

        if ret == -1 {

            println!("Failed: {}", std::io::Error::last_os_error());
        }

        println!("Killed the process {}",self.pid.unwrap())
    }

    

    async fn AddNEtworkingRules(&self,handle:&Handle,addr:Ipv4Addr,veth_index:u32)->Result<(),Box<dyn std::error::Error>>{
        handle
            .route()
            .add()
            .v4()
            .gateway(addr)
            .output_interface(veth_index)
            .execute()
            .await?;

        Ok(())
    }   

    async fn EnableLoopInterface(&self,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
        handle
        .link()
        .set(1) // index of lo in that namespace
        .up()
        .execute()
        .await?;

    Ok(())
    }

    pub fn CreateInitFile(&self)->Result<(), Box<std::io::Error>>{
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

    pub fn ReadInitFile()->Result<HashMap<String,Self>,Box<dyn std::error::Error>>{
        let json_str=fs::read_to_string("init.json")?;
        let json:HashMap<String,Self>=serde_json::from_str(json_str.as_str())?;

        Ok(json)
    }

    pub fn WriteToFile(&self,mut obj:HashMap<String,Self>)->Result<(),Box<dyn std::error::Error>>{
    
        fs::write("init.json", serde_json::to_string_pretty(&obj)?)?;
        Ok(())
    } 



    pub fn CreateConFile(&self)->Result<(), Box<std::io::Error>>{
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

    pub fn ReadConFile()->Result<HashMap<String,Self>,Box<dyn std::error::Error>>{
        let json_str=fs::read_to_string("container.json")?;
        let json:HashMap<String,Self>=serde_json::from_str(json_str.as_str())?;

        Ok(json)
    }

    pub fn WriteConFile(&self,mut obj:HashMap<String,Self>)->Result<(),Box<dyn std::error::Error>>{
    
        fs::write("container.json", serde_json::to_string_pretty(&obj)?)?;
        Ok(())
    } 

} 
    




pub extern "C" fn KillSignalHnadler(_: i32) {
        
       unsafe {
        let msg = b"handler called\n";
        libc::write(1, msg.as_ptr() as *const _, msg.len());
        libc::_exit(0);
    }
}