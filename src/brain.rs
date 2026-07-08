use futures::{TryStreamExt, future::ok};
use rtnetlink::Handle;

use crate::sokcet;



pub fn Brain_init(){
    let tcp_listner= sokcet::CreateBrainSocket().unwrap();
    let _=sokcet::StartBrainListener(tcp_listner);
}



fn Create_RT_Netlink()->Result<Handle,std::io::Error>{
    let (connection,handle,_)=rtnetlink::new_connection()?;

    tokio::spawn(connection);

    Ok(handle)
}

async fn CreateBridge(bridge_name :&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    

    handle.link()
    .add()
    .bridge(bridge_name.clone())
    .execute()
    .await?;

    Ok(())
}
async fn AssignBridgeIP(bridge_name :&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    let index=GetIndexOfinterface(bridge_name, handle).await;
    

    handle
    .address()
    .add(
        index,
        std::net::Ipv4Addr::new(192, 168, 1, 1).into(),
        24,
    )
    .execute()
    .await?;

    Ok(())
}


async fn CreateVeth(veth_name:&String,veth_peer_name:&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    handle.link()
    .add()
    .veth(veth_name.clone(), veth_peer_name.clone())
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
    let interface =links.try_next().await.unwrap().expect("cant find the interface name");
    interface.header.index
     

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

