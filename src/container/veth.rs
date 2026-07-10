use std::{collections::HashMap, fs::{self, OpenOptions}, io::Write};

use futures::TryStreamExt;
use rtnetlink::Handle;


use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Veth {
    name:Option<String>,
    veth0_name:Option<String>,
    veth1_name:Option<String>,
    
    veth0_ip: Option<String>,
    veth1_ip: Option<String>,

    veth0_net_ns_name: Option<String>,
    veth0_net_ns_index: Option<String>,

    veth1_net_ns_name: Option<String>,
    veth1_net_ns_index: Option<String>,


}

pub fn CreateVethFile() -> Result<(), Box<dyn std::error::Error>> {
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

type Veths_file=HashMap<String,Veth>;
pub fn ReadVethFile()->Result<Veths_file,Box<dyn std::error::Error>>{
    let paylaod=fs::read_to_string("Veth.json")?;
    let mut obj:Veths_file=serde_json::from_str(&paylaod)?;
    Ok(obj)
}

pub fn WriteVethFile(mut obj:Veths_file,veth:Veth)->Result<(),Box<dyn std::error::Error>>{
    // obj.insert(veth.name.unwrap().clone(), veth);
    // fs::write("Veth.json", serde_json::to_string_pretty(&obj)?)?;
    Ok(())
}

pub async fn VethStorageStruct(insys_Veth_name:&String,Veth_name :&String,handle:&Handle,obj:&Value)->Veth{
    let index=GetIndexOfinterface(&insys_Veth_name, handle).await ;
    
    println!("1 {}",obj["subnet"]);
    let obj = Veth {
        name:Some(Veth_name.clone()),

        veth0_name: get_optional_string(obj, "veth0_name"),
        veth1_name: get_optional_string(obj, "veth1_name"),

        veth0_ip: get_optional_string(obj, "veth0_ip"),
        veth1_ip: get_optional_string(obj, "veth1_ip"),

        veth0_net_ns_name: get_optional_string(obj, "veth0_net_ns_name"),
        veth0_net_ns_index: get_optional_string(obj, "veth0_net_ns_index"),

        veth1_net_ns_name: get_optional_string(obj, "veth1_net_ns_name"),
        veth1_net_ns_index: get_optional_string(obj, "veth1_net_ns_index"),
    };
    obj
}

fn get_optional_string(obj: &Value, key: &str) -> Option<String> {
    obj.get(key)
        .and_then(|v| v.as_str())
        .map(String::from)
}


pub async fn CreateVeth(veth_name:&String,veth_peer_name:&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    handle.link()
    .add()
    .veth(veth_name.clone(), veth_peer_name.clone())
    .execute()
    .await?;

    Ok(())
}


pub async fn SetVethEndpoints(veth_index:u32,Veth_index:u32,handle:&Handle){
    handle
    .link()
    .set(veth_index)
    .master(Veth_index)
    .execute()
    .await.unwrap();
}

pub async fn MoveVethEndToCON_net_ns(container_pid:u32,veth_index:u32,handle:&Handle){
    handle.link()
    .set(veth_index)
    .setns_by_pid(container_pid)
    .execute()
    .await.unwrap();
}




pub  async fn GetIndexOfinterface(interface_name:&String,handle:&Handle)->u32{
    let mut links = handle
    .link()
    .get()
    .match_name(interface_name.clone())
    .execute();
    let interface =links.try_next().await.unwrap().expect("cant find the interface");
    interface.header.index
}

