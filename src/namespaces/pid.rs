use crate::{interface, namespaces};





use serde::{Serialize, Deserialize};


#[derive(Clone,Serialize,Deserialize)]
pub struct pid_ns{

}

impl pid_ns {
    
}


impl namespaces::Namespace for pid_ns {
    
    fn Create(&self,net_flag:&bool,pid_flag:&bool,mount_flag:&bool) {
        let child_pid=self.CreateTempProcess(net_flag, pid_flag, mount_flag);
    }

    fn Delete(&self,target_mount:&String) {
        
    }

    fn BindMount(&self,child_pid:i64)->Result<(),Box<dyn std::error::Error>> {
        


        Ok(())
    }
    
    
}