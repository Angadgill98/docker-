use futures::future::ok;
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

async fn CreateVeth(veth_name:&String,veth_peer_name:&String,handle:&Handle)->Result<(),Box<dyn std::error::Error>>{
    handle.link()
    .add()
    .veth(veth_name.clone(), veth_peer_name.clone())
    .execute()
    .await?;

    Ok(())
}

async fn GetIndexOfinterface(interface_name:&String){
    let mut links = handle
    .link()
    .get()
    .match_name(interface_name.clone())
    .execute();
    links.try_next().await.unwrap()
}

async fn SetVethEndpoints(veth_index:i64,interface_index:i64){
    handle
    .link()
    .set(veth_index)
    .master(interface_index)
    .execute()
    .await?;
}