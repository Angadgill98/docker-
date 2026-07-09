use std::io::Read;

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


            println!("Storing in a file");
            container::bridge::CreateBridgeFile()?;
            let bridge=container::bridge::BridgeStorageStruct(&insys_bridge_name,bridge_name, &handle).await;
            let obj=container::bridge::ReadBridgeFile()?;
            container::bridge::WriteBridgeFile(obj, bridge)?;
        }
        2=>{//Delete a bridge
            let payload=Getdata(&msg,2);
            let data:Value=serde_json::from_slice(payload).unwrap();
            let handle=Create_RT_Netlink().unwrap();

        }
        _=>{
            println!("kn hua");
        }
    }
    Ok(())

}

fn Getdata(msg:&Vec<u8>,len:usize)->&[u8]{
   &msg[len..]
}



fn Create_RT_Netlink()->Result<Handle,std::io::Error>{
    let (connection,handle,_)=rtnetlink::new_connection()?;

    tokio::spawn(connection);

    Ok(handle)
}







async fn CreateVeth(veth_name:&String,veth_peer_name:&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    handle.link()
    .add()
    .veth(veth_name.clone(), veth_peer_name.clone())
    .execute()
    .await?;

    Ok(())
}


async fn SetVethEndpoints(veth_index:u32,bridge_index:u32,handle:&Handle){
    handle
    .link()
    .set(veth_index)
    .master(bridge_index)
    .execute()
    .await.unwrap();
}

async fn MoveVethEndToCON_net_ns(container_pid:u32,veth_index:u32,handle:&Handle){
    handle.link()
    .set(veth_index)
    .setns_by_pid(container_pid)
    .execute()
    .await.unwrap();
}

