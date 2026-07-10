use std::{collections::{HashMap, hash_map}, fs::{self, File, OpenOptions}, io::Write, net::Ipv4Addr};

use futures::{TryStreamExt};
use rtnetlink::Handle;
use serde::de::value;
use serde_json::{Map, Value, value::Index};

use crate::container::bridge;




use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Bridge {
    bridge_name: String,
    index: u32,

    status: String,
    ip:String,
    subnet:u8,
    network:String,

    insys: String,
}


pub fn CreateBridgeFile() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("bridge.json")
    {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("Bridge storage file already exists");
            return Ok(());
        }
        Err(e) => return Err(Box::new(e)),
    };

    file.write_all(b"{}")?;
    println!("Created the bridge storage file");

    Ok(())
}

type Bridges_file=HashMap<String,Bridge>;
pub fn ReadBridgeFile()->Result<Bridges_file,Box<dyn std::error::Error>>{
    let paylaod=fs::read_to_string("bridge.json")?;
    let mut obj:Bridges_file=serde_json::from_str(&paylaod)?;
    Ok(obj)
}

pub fn WriteBridgeFile(mut obj:Bridges_file,bridge:Bridge)->Result<(),Box<dyn std::error::Error>>{
    obj.insert(bridge.bridge_name.clone(), bridge);
    fs::write("bridge.json", serde_json::to_string_pretty(&obj)?)?;
    Ok(())
}

pub async fn BridgeStorageStruct(insys_bridge_name:&String,bridge_name :&String,handle:&Handle,obj:&Value)->Bridge{
    let index=GetIndexOfinterface(&insys_bridge_name, handle).await ;
    
    println!("1 {}",obj["subnet"]);
    let obj=Bridge{
        bridge_name:String::from(bridge_name),
        index:index,
        insys:format!("dc-{}", bridge_name),
        ip:obj["ip"].as_str().unwrap().to_string(),
        subnet:obj["subnet"].as_u64().unwrap() as u8,
        status:obj["status"].as_str().unwrap().to_string(),
        network:obj["network"].as_str().unwrap().to_string()
    };
    obj
}





pub async fn CreateBridge(insys_bridge_name:&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    

    handle.link()
    .add()
    .bridge(insys_bridge_name.clone())
    .execute()
    .await?;

    Ok(())
}






pub async fn DeleteBridge(insys_bridge_name :&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    let index=GetIndexOfinterface(insys_bridge_name, handle).await;

    handle.link()
    .del(index)
    .execute()
    .await?;

    Ok(())
}

pub async fn DeleteBridgeFromStruct(){


}

pub async fn GetAllBridges(bridge_name :&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    let index=GetIndexOfinterface(bridge_name, handle).await;

    handle.link()
    .del(index)
    .execute()
    .await?;

    Ok(())
}

pub async  fn AssignBridgeIP(insys_bridge_name :&String,handle:&Handle,data:&Value)->Result<(),Box<dyn std::error::Error>>{
    let index=GetIndexOfinterface(insys_bridge_name, handle).await;
    

    handle
    .address()
    .add(
        index,
        data["ip"].as_str().unwrap().parse::<Ipv4Addr>().unwrap().into(),
        data["subnet"].as_u64().unwrap() as u8,
    )
    .execute()
    .await?;

    Ok(())
}


async fn GetIndexOfinterface(interface_name:&String,handle:&Handle)->u32{
    let mut links = handle
    .link()
    .get()
    .match_name(interface_name.clone())
    .execute();
    let interface =links.try_next().await.unwrap().expect("cant find the interface");
    interface.header.index
}


