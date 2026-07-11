use std::{fs, io::Read};

use futures::{TryStreamExt, future::ok, sink::Buffer};
use rtnetlink::Handle;
use serde_json::Value;


use crate::{container::{self, bridge}, sokcet};




 pub async fn Brain_init(sender:std::sync::mpsc::Sender<()>)->Result<(),Box <dyn std::error::Error>>{
    let tcp_listner= sokcet::CreateBrainSocket()?;
    sender.send(());
    
    println!("STarteda the brain");
    let (mut stream,addr)=tcp_listner.accept()?;
    loop{
        
        let mut op = [0u8; 1];
        stream.read_exact(&mut  op);

        let mut len_buf = [0u8; 1];
        stream.read_exact(&mut len_buf)?;
        
        let len = u8::from_be_bytes(len_buf) as usize;

        let mut msg = vec![0u8; len];
        
        stream.read_exact(&mut msg)?;
        
        handle_client(&msg,op[0]).await?;
    }
    Ok(())
}

async fn handle_client(msg:&Vec<u8>,op:u8)->Result<(),Box<dyn std::error::Error>>{
   
    match op {
        1=>{//Create a bridge
            println!("Creating a bridge");
            
            
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj");
            let handle=Create_RT_Netlink().unwrap();
            let bridge_name=&data["name"].as_str().unwrap().to_string();
            let insys_bridge_name=format!("dc-{}",bridge_name);
            container::bridge::CreateBridge(&insys_bridge_name, &handle).await?;
            println!("Bridge created");

            println!("AssignBridgeIP");
            
            container::bridge::AssignBridgeIP(&insys_bridge_name, &handle,&data).await.expect("failed to assing ip to teh bridge");
           
            println!("Storing in a file");
            container::bridge::CreateBridgeFile()?;
            let obj=container::bridge::ReadBridgeFile()?;
            let bridge=container::bridge::BridgeStorageStruct(&insys_bridge_name,bridge_name, &handle,&data).await;
            
            container::bridge::WriteBridgeFile(obj, bridge)?;
        }
        2=>{//Delete a bridge
            
            
               
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while deleting a bridge");
            let handle=Create_RT_Netlink().unwrap();
            let bridge_name=&data["name"].as_str().unwrap().to_string();
            let insys_bridge_name=format!("dc-{}",bridge_name);


            container::bridge::DeleteBridge(&insys_bridge_name, &handle).await?;

            container::bridge::CreateBridgeFile()?;

            let mut obj=container::bridge::ReadBridgeFile()?;

            obj.remove(bridge_name);
            fs::write("bridge.json", serde_json::to_string_pretty(&obj)?)?;
            
            
        }
        
        
        3=>{//Create veth
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating veth pair");
            let handle=Create_RT_Netlink().unwrap();

           
            let insys_veth_name=format!("dc-{}",data["veth0_name"].as_str().unwrap().to_string());
            let insys_veth_peer_name=format!("dc-{}",data["veth1_name"].as_str().unwrap().to_string());

            
            container::veth::CreateVeth(&insys_veth_name, &insys_veth_peer_name, &handle).await.expect("not able to craete veth pair");

            _=container::veth::CreateVethFile();

            let obj=container::veth::ReadVethFile().unwrap();

            let veth=container::veth::VethStorageStruct(&insys_veth_name,&insys_veth_peer_name,&handle,&data ).await;

            container::veth::WriteVethFile(obj, veth).expect("not able to write to file");

            

        }
        4=>{//Delete a veth
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating veth pair");
            let handle=Create_RT_Netlink().unwrap();

            let veth_name = data["veth0_name"].as_str().unwrap().to_string();
            let insys_veth_name=format!("dc-{}",data["veth0_name"].as_str().unwrap().to_string()); 


            container::veth::DeleteVeth(&insys_veth_name, &handle).await?;

            _=container::veth::CreateVethFile();

            let mut obj=container::veth::ReadVethFile().unwrap();

            let header=format!("{}_{}",veth_name,data["veth1_name"].as_str().unwrap());
            obj.remove(&header);
            fs::write("veth.json", serde_json::to_string_pretty(&obj)?)?;

        }
        
        
        5=>{//Create net ns 
            
        }
        _=>{
            println!("kn hua");
        }
    }
    Ok(())

}




fn Create_RT_Netlink()->Result<Handle,std::io::Error>{
    let (connection,handle,_)=rtnetlink::new_connection()?;

    tokio::spawn(connection);

    Ok(handle)
}






