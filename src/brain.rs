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

