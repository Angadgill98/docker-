use std::{collections::HashMap, ffi::CString, fs::{self, File, OpenOptions}, io::Write, os::fd::AsRawFd, path::Path};

use flate2::{read::GzDecoder};
use libc::MS_BIND;
use tar::Archive;

use crate::namespaces;





use serde::{Serialize, Deserialize};


#[derive(Clone,Serialize, Deserialize)]
pub struct mount {
   pub name :String
}


impl mount {
    fn verify_path(&self,path: &str) -> bool {
        Path::new(path).exists()
    }


    fn verify_file(&self,path: &str) -> bool {
        Path::new(path).is_file()
    }

    fn JoinProcessMount(&self,target_pid:&i64)->Result<(),Box<dyn std::error::Error>>{
        let file = File::open(format!("/proc/{}/ns/net",target_pid))?;
        let fd = file.as_raw_fd();
        
        let result = unsafe {
            libc::setns(fd, libc::CLONE_NEWNET)
        };

        if result != 0 {
            panic!("setns failed");
        }
        Ok(())
    }

    fn JoinNamedMount(&self,target_ns_name:&String)->Result<(),Box<dyn std::error::Error>>{
        //insert path here
        let file = File::open(format!("{}",target_ns_name))?;
        let fd = file.as_raw_fd();

        let result = unsafe {
            libc::setns(fd, libc::CLONE_NEWNET)
        };

        if result != 0 {
            panic!("setns failed");
        }
        Ok(())
    }

    fn ExtractRootfs(&self,image_name:&String)->Result<(),Box<dyn std::error::Error>>{
        //enter the image path
        let file=File::open(format!(""))?;

        // Decompress the gzip layer
        let decoder = GzDecoder::new(file);

        // Read the tar archive
        let mut archive = Archive::new(decoder);

        // Extract everything into destination
        archive.unpack(Path::new(&format!("").to_string()))?;

        Ok(())
    }

    pub fn Unmount(&self){
        let target = CString::new(format!("dc_ns/mount/{}",self.name)).unwrap();

        let ret = unsafe {
            libc::umount2(target.as_ptr(), libc::MNT_DETACH)
        };

        if ret != 0 {
            eprintln!("umount2 failed: {}", std::io::Error::last_os_error());
        } else {
            println!("Successfully unmounted.");
        }
    }

    pub fn NewRoot(&self)->Result<(), Box<dyn std::error::Error>>{
        let new_root = CString::new(format!("dc_ns/mount/{}",self.name))?;
        let put_old = CString::new(format!("dc_ns/mount/{}/old_root",self.name))?;

        // Ensure /container_root/old_root exists before calling pivot_root.

        std::fs::create_dir_all(format!("dc_ns/mount/{}/old_root",self.name))?;


        let ret = unsafe {
            libc::syscall(
                libc::SYS_pivot_root,
                new_root.as_ptr(),
                put_old.as_ptr(),
            )
        };

        if ret != 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        self.RemoveOldRoot()?;

        Ok(())
    }

    fn RemoveOldRoot(&self)->Result<(),Box<dyn std::error::Error>>{
        unsafe {
            let root = CString::new("/")?;
            libc::chdir(root.as_ptr());

            let old = CString::new("/old_root")?;
            libc::umount2(old.as_ptr(), libc::MNT_DETACH);

            std::fs::remove_dir("/old_root")?;
        };
        Ok(())
    }

    pub fn CreateFile(&self)->Result<(), Box<std::io::Error>>{
        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("mount_ns.json")
        {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                println!("mount ns storage file already exists");
                return Ok(());
            }
            Err(e) => return Err(Box::new(e)),
        };

        file.write_all(b"{}")?;
        println!("Created the mount ns storage file");

        Ok(())
    }

    pub fn ReadFile(&self)->Result<HashMap<String,Self>,Box<dyn std::error::Error>>{
        let paylaod=fs::read_to_string("mount_ns.json")?;
        let obj:HashMap<String,Self>=serde_json::from_str(&paylaod)?;
        Ok(obj)
    }

    pub fn WriteToFile(&self,mut obj:HashMap<String,Self>)->Result<(),Box<dyn std::error::Error>>{
    
        fs::write("mount_ns.json", serde_json::to_string_pretty(&obj)?)?;
        Ok(())
    } 
    
}

impl namespaces::Namespace for mount {
    fn Create(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool) {
        let child_pid=self.CreateTempProcess(net_flag, pid_flag, mount_flag);
    }

    fn BindMount(&self,child_pid:i64)->Result<(),Box<dyn std::error::Error>> {
        
        //name is with self 
        let source=CString::new(format!("/proc/{}/ns/mnt",child_pid)).unwrap();
        
        let target=CString::new(format!("dc_ns/mount/{}",self.name)).unwrap();

        self.verify_file(source.to_str()?);
        
        std::fs::create_dir_all(format!("dc_ns/mount"))?;
        File::create(&target.to_str()?)?;

        unsafe {
            libc::mount(
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