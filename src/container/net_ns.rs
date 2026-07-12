use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::Write, mem, os::fd::AsRawFd};

use futures::io;
use libc::{
    CLONE_NEWNET, CLONE_NEWNS, CLONE_NEWPID, MS_BIND, SIGCHLD, SYS_clone3, mount, syscall
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

fn TempProcess()->i64{
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


fn BindMountNetns(pid:i64){
    let old_refrence_path=CString::new("").expect("no file path found for the old refernce");
    
    CreateBindMountFIle(pid.to_string()).expect("Unabel to crete the desired file");

    let new_referce_path=CString::new("").expect("no file path found for the new refernce");

    let status=unsafe{
        mount(old_refrence_path.as_ptr(), new_referce_path.as_ptr(), std::ptr::null(), MS_BIND, std::ptr::null())
    };

    if status!=0{
        panic!("failed to bind mount net ns")
    }


}

fn CreateBindMountFIle(file_name:String)->Result<(), Box<dyn std::error::Error>>{
    fs::create_dir_all("")?;
    File::create("");
    Ok(())
}

fn CreateNetFile()->Result<(), Box<dyn std::error::Error>>{
    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("net_ns.json")
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


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Net_ns_bindmount{
    name:String,
}

type Net_ns_file_map=HashMap<String,Net_ns_bindmount>;
pub fn ReadNet_ns_file()->Result<Net_ns_file_map,Box<dyn std::error::Error>>{
    let paylaod=fs::read_to_string("net_ns.json")?;
    let mut obj:Net_ns_file_map=serde_json::from_str(&paylaod)?;
    Ok(obj)
}

pub fn WriteNet_ns_file(mut obj:Net_ns_file_map,net:Net_ns_bindmount)->Result<(),Box<dyn std::error::Error>>{
    obj.insert(net.name.clone(), net);
    fs::write("net_ns.json", serde_json::to_string_pretty(&obj)?)?;
    Ok(())
} 


