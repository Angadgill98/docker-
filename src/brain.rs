use std::{fs, io::Read};

use futures::{TryStreamExt, future::ok, sink::Buffer};
use rtnetlink::Handle;
use serde_json::Value;


use crate::{container::{self, bridge, veth}, interface::Interface, sokcet};




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
            let handle: Handle=Create_RT_Netlink().unwrap();
            let bridge_name=data["name"].as_str().unwrap().to_string();
            let insys_bridge_name=format!("dc-{}",bridge_name);
            let mut birdge=container::bridge::Bridge{
                name:bridge_name,
                insys:insys_bridge_name,

                index:None,
                status:None,
                
                ip:None,
                subnet:None,
                network:None

            };
            birdge.Create(&handle).await?;
            println!("Bridge created");


            println!("AssignBridgeIP");
            birdge.AssignIP(&handle, &data).await?;
            

            println!("Storing in a file");
            birdge.ip=Some(data["ip"].as_str().unwrap().to_string());
            birdge.subnet=Some(data["subnet"].as_u64().unwrap() as u8);
            birdge.network=Some(data["status"].as_str().unwrap().to_string());
            birdge.network=Some(data["network"].as_str().unwrap().to_string());

            birdge.CreateFile()?;
            let BridgesMap= birdge.ReadFile()?;
            birdge.WriteToFile(BridgesMap)?;
            
            
            
            
        }
        2=>{//Delete a bridge
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while deleting a bridge");
            let handle=Create_RT_Netlink().unwrap();
            
            
            let bridge_name=data["name"].as_str().unwrap().to_string();
            let insys_bridge_name=format!("dc-{}",bridge_name);


            let mut birdge=container::bridge::Bridge{
                name:bridge_name,
                insys:insys_bridge_name,

                index:None,
                status:None,
                
                ip:None,
                subnet:None,
                network:None

            };
            birdge.Delete(&handle);

            birdge.CreateFile()?;
            let mut BridgesMap= birdge.ReadFile()?;

            BridgesMap.remove(&birdge.name);
            birdge.WriteToFile(BridgesMap)?;
            
            
        }       
        3=>{//Create veth
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating veth pair");
            let handle=Create_RT_Netlink().unwrap();

            let veth_pair=container::veth::VethPair{
                veth_front:container::veth::VethEnd{
                    name:data["veth0_name"].as_str().unwrap().to_string(),
                    insys:format!("dc-{}",data["veth0_name"].as_str().unwrap().to_string())
                },
                veth_back:container::veth::VethEnd{
                    name:data["veth1_name"].as_str().unwrap().to_string(),
                    insys:format!("dc-{}",data["veth1_name"].as_str().unwrap().to_string())
                }
            };


            veth_pair.Create(&handle);
            
            veth_pair.CreateVethFile();

            let veth_file_map=veth_pair.ReadVethFile()?;            
            veth_pair.WriteVethFile(veth_file_map)?;

        }
        4=>{//Delete a veth
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while deleting veth pair");
            let handle=Create_RT_Netlink().unwrap();

            let veth_pair=container::veth::VethPair{
                veth_front:container::veth::VethEnd{
                    name:data["veth0_name"].as_str().unwrap().to_string(),
                    insys:format!("dc-{}",data["veth0_name"].as_str().unwrap().to_string())
                },
                veth_back:container::veth::VethEnd{
                    name:data["veth1_name"].as_str().unwrap().to_string(),
                    insys:format!("dc-{}",data["veth1_name"].as_str().unwrap().to_string())
                }
            };

            veth_pair.Delete(&handle);

            veth_pair.Create(&handle);
            
            veth_pair.CreateVethFile();

            let veth_file_map=veth_pair.ReadVethFile()?;            
            veth_file_map.remove(format!("{}_{}",veth_pair.veth_front.name,veth_pair.veth_back.name).trim());
            veth_pair.WriteVethFile(veth_file_map)?;            

            

        }   
        5=>{//Create net ns 
            
        }

        6=>{//Create a Contanier
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating veth pair");
            let handle=Create_RT_Netlink().unwrap();


            let parent_pid=std::process::id();
            let child_pid= container::pid_ns::CreateContainer();

            let bridge_name=&data["name"].as_str().unwrap().to_string();
            let insys_bridge_name=format!("dc-{}",bridge_name);
           // container::bridge::CreateBridge(&insys_bridge_name, &handle).await?;


            let insys_veth0_name=format!("dc-{}",data["veth0_name"].as_str().unwrap().to_string());
            let insys_veth1_name=format!("dc-{}",data["veth1_name"].as_str().unwrap().to_string());

            
            container::veth::CreateVeth(&insys_veth0_name, &insys_veth1_name, &handle).await.expect("not able to craete veth pair");

            let index_veth0=container::veth::GetIndexOfinterface(&insys_veth0_name, &handle).await;
            let index_veth1=container::veth::GetIndexOfinterface(&insys_veth0_name, &handle).await;


          //  container::veth::SetVethEndpoints(index_veth0, veth_index, &handle).await;
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






