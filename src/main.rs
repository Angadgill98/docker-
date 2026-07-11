use libc::{
    syscall,
    SYS_clone3,
    SIGCHLD,
    CLONE_NEWPID,
    CLONE_NEWNET,
    CLONE_NEWNS
};

use std::{default, fs::File, mem, os::fd::AsRawFd};

mod cli;
mod sokcet;
mod brain;
mod ns;
mod container;
fn main() {
    dotenvy::dotenv().ok();
    let (sender,rec)=std::sync::mpsc::channel::<()>();
    std::thread::spawn(move||{
        let rt = tokio::runtime::Runtime::new().unwrap();

        if let Err(e) = rt.block_on(brain::Brain_init(sender)) {
            eprintln!("Brain_init failed: {e}");
        }
    });
    rec.recv().unwrap();
    cli::CLI();
    
    
}





// fn CLI_printer(){
//     println!("")
// }




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

fn ParentProcessCode(){

}

fn ChildProcessCode(){

}