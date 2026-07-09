use std::io::Read;

use futures::{TryStreamExt, future::ok, sink::Buffer};
use rtnetlink::Handle;
use serde_json::Value;


use crate::{container, sokcet};




pub fn Brain_init(sender:std::sync::mpsc::Sender<()>)->Result<(),Box <dyn std::error::Error>>{
    let tcp_listner= sokcet::CreateBrainSocket().unwrap();
    sender.send(());
    let mut msg=Vec::new();
    loop{
        let (mut stream,addr)=tcp_listner.accept()?;
        loop{
            let mut buffer=[0u8;1024];
            let len=stream.read(&mut buffer).unwrap();
            if len==0{
                break;
            }
            msg.extend_from_slice(&buffer[..len]);
        }

    }
    Ok(())
}

fn handle_client(msg:Vec<u8>){
    match msg[0] {
        1=>{//Create a bridge
            let payload=Getdata(&msg,2);
            let data:Value=serde_json::from_slice(payload).unwrap();
            let handle=Create_RT_Netlink().unwrap();
            container::bridge::CreateBridge(bridge_name, &handle);
        }
        2=>{//Delete a bridge
            let payload=Getdata(&msg,2);
            let data:Value=serde_json::from_slice(payload).unwrap();
            let handle=Create_RT_Netlink().unwrap();

        }
        _=>{

        }
    }

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

