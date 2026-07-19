// use std::{collections::HashMap, fs::{self, OpenOptions}, io::Write};

// use futures::TryStreamExt;
// use rtnetlink::Handle;


// use serde::{Serialize, Deserialize};
// use serde_json::Value;

// use crate::container::net_ns;

// #[derive(Serialize, Deserialize)]
// pub struct Veth {
   
//     veth0_name:Option<String>,
//     veth1_name:Option<String>,

//     veth0_insys_name:Option<String>,
//     veth1_insys_name:Option<String>,
    
//     veth0_ip: Option<String>,
//     veth1_ip: Option<String>,

//     veth0_net_ns_name: Option<String>,
//     veth0_net_ns_index: Option<String>,

//     veth1_net_ns_name: Option<String>,
//     veth1_net_ns_index: Option<String>,


// }

// pub fn CreateVethFile() -> Result<(), Box<dyn std::error::Error>> {
//     let mut file = match OpenOptions::new()
//         .write(true)
//         .create_new(true)
//         .open("veth.json")
//     {
//         Ok(file) => file,
//         Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
//             println!("Veth storage file already exists");
//             return Ok(());
//         }
//         Err(e) => return Err(Box::new(e)),
//     };

//     file.write_all(b"{}")?;
//     println!("Created the Veth storage file");

//     Ok(())
// }

// type Veths_file=HashMap<String,Veth>;
// pub fn ReadVethFile()->Result<Veths_file,Box<dyn std::error::Error>>{
//     let paylaod=fs::read_to_string("veth.json")?;
//     let mut obj:Veths_file=serde_json::from_str(&paylaod)?;
//     Ok(obj)
// }

// pub fn WriteVethFile(mut obj:Veths_file,veth:Veth)->Result<(),Box<dyn std::error::Error>>{
//     obj.insert(format!("{}_{}",veth.veth0_name.as_ref().unwrap(),veth.veth1_name.as_ref().unwrap()), veth);
//     fs::write("veth.json", serde_json::to_string_pretty(&obj)?)?;
//     Ok(())
// }

// pub async fn VethStorageStruct(insys_Veth_name:&String,insys_veth_peer_name :&String,handle:&Handle,obj:&Value)->Veth{
//     let index=GetIndexOfinterface(&insys_Veth_name, handle).await ;
    
    
//     let obj = Veth {
        

//         veth0_name: get_optional_string(obj, "veth0_name"),
//         veth1_name: get_optional_string(obj, "veth1_name"),

//         veth0_insys_name: Some(insys_Veth_name.clone()),
//         veth1_insys_name: Some(insys_veth_peer_name.clone()),

//         veth0_ip: get_optional_string(obj, "veth0_ip"),
//         veth1_ip: get_optional_string(obj, "veth1_ip"),

//         veth0_net_ns_name: get_optional_string(obj, "veth0_net_ns_name"),
//         veth0_net_ns_index: get_optional_string(obj, "veth0_net_ns_index"),

//         veth1_net_ns_name: get_optional_string(obj, "veth1_net_ns_name"),
//         veth1_net_ns_index: get_optional_string(obj, "veth1_net_ns_index"),
//     };
//     obj
// }

// fn get_optional_string(obj: &Value, key: &str) -> Option<String> {
//     obj.get(key)
//         .and_then(|v| v.as_str())
//         .map(String::from)
// }


// pub async fn CreateVeth(veth_name:&String,veth_peer_name:&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
//     handle.link()
//     .add()
//     .veth(veth_name.into(), veth_peer_name.into())
//     .execute()
//     .await?;

//     Ok(())
// }


// pub async fn SetVethEndpoints(veth_index:u32,net_ns_fd:i32,handle:&Handle){
//     handle
//     .link()
//     .set(veth_index)
//     .setns_by_fd(net_ns_fd)
//     .execute()
//     .await.expect("failed to move veth to the net ns");
// }





// pub  async fn GetIndexOfinterface(interface_name:&String,handle:&Handle)->u32{
//     let mut links = handle
//     .link()
//     .get()
//     .match_name(interface_name.clone())
//     .execute();
//     let interface =links.try_next().await.unwrap().expect("cant find the interface");
//     interface.header.index
// }


// pub async fn DeleteVeth(insys_veth_name :&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
//     let index=GetIndexOfinterface(insys_veth_name, handle).await;

//     handle.link()
//     .del(index)
//     .execute()
//     .await?;

//     Ok(())
// }

use std::{collections::HashMap, fs::{self, OpenOptions}, io::Write};

use futures::TryStreamExt;
use rtnetlink::Handle;

use crate::interface;


use serde::{Serialize, Deserialize};


#[derive(Clone,Serialize, Deserialize)]
pub struct VethPair{
    
    pub veth_front:VethEnd,
    pub veth_back:VethEnd
}
impl VethPair {
    pub fn CreateVethFile(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("veth.json")
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

    pub fn ReadVethFile(&self)->Result<HashMap<String,VethPair>,Box<dyn std::error::Error>>{
        let paylaod=fs::read_to_string("veth.json")?;
        let mut obj:HashMap<String,VethPair>=serde_json::from_str(&paylaod)?;
        Ok(obj)
    }

    pub fn WriteVethFile(&self,mut VethFileMap:HashMap<String,VethPair>)->Result<(),Box<dyn std::error::Error>>{
        
        fs::write("veth.json", serde_json::to_string_pretty(&VethFileMap)?)?;
        Ok(())
    }

}


#[derive(Clone,Serialize, Deserialize)]
pub struct VethEnd{
    pub name: String,
    pub insys: String,

    pub index:Option<u32>
}

impl VethEnd{
    pub async fn GetIndex(&self,handle:&Handle)->u32{
        let mut links = handle
        .link()
        .get()
        .match_name(self.insys.clone())
        .execute();
        let interface =links.try_next().await.unwrap().expect("cant find the interface");
        interface.header.index
    }

    pub async fn SetVethInNetns(&self,net_ns_fd:i32,handle:&Handle){
        handle
        .link()
        .set(self.GetIndex(handle).await)
        .setns_by_fd(net_ns_fd)
        .execute()
        .await.expect("failed to move veth to the net ns");
    }

    pub async fn AssignIP(&self,handle:&Handle,data:&Value)->Result<(),Box<dyn std::error::Error>>{
        
        handle
        .address()
        .add(
            self.index?,
            data["ip"].as_str().unwrap().parse::<Ipv4Addr>().unwrap().into(),
            data["subnet"].as_u64().unwrap() as u8,
        )
        .execute()
        .await?;

        Ok(())
    }
    
}

impl interface::Interface for VethPair{
    fn name(&self)->&str {
        self.veth_front.insys.as_str()
    }
    async fn Create(&self,handle:&rtnetlink::Handle)-> Result<(), Box<dyn std::error::Error>> {
        let veth0_name=self.veth_front.insys.clone();
        let veth1_name=self.veth_back.insys.clone();
        handle.link()
            .add()
            .veth(veth0_name.into(), veth1_name.into())
            .execute()
            .await?;

        Ok(())
    }
    async fn Delete(&self,handle:&rtnetlink::Handle)-> Result<(), Box<dyn std::error::Error>> {
        
        let index=self.GetIndex(handle).await?;

        handle.link()
        .del(index)
        .execute()
        .await?;

        Ok(())
    }
}