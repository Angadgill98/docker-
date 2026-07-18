use std::{fmt::format, fs, io::Read};

use futures::{TryStreamExt, future::ok, sink::Buffer};
use rtnetlink::Handle;
use serde_json::Value;


use crate::{container::{self, bridge, veth}, interface::Interface, namespaces::{self, Namespace}, sokcet};




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
            birdge.status=Some(data["status"].as_str().unwrap().to_string());
            birdge.network=Some(data["network"].as_str().unwrap().to_string());

            birdge.CreateFile()?;
            let mut BridgesMap= container::bridge::Bridge::ReadFile()?;
            BridgesMap.insert(birdge.name.clone(), birdge.clone());
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
            birdge.Delete(&handle).await?;

            birdge.CreateFile()?;
            let mut BridgesMap= container::bridge::Bridge::ReadFile()?;

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


            veth_pair.Create(&handle).await?;
            
            veth_pair.CreateVethFile()?;

            let mut veth_file_map=veth_pair.ReadVethFile()?;    
            veth_file_map.insert(format!("{}_{}",veth_pair.veth_front.name,veth_pair.veth_back.name), veth_pair.clone());        
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

            veth_pair.Delete(&handle).await?;

            
            
            veth_pair.CreateVethFile()?;

            let mut veth_file_map=veth_pair.ReadVethFile()?;            
            veth_file_map.remove(format!("{}_{}",veth_pair.veth_front.name,veth_pair.veth_back.name).trim());
            veth_pair.WriteVethFile(veth_file_map)?;            

            

        }   
        5=>{//Create net ns 
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating net ns");
            let handle=Create_RT_Netlink().unwrap();

            let name=data["name"].as_str().unwrap().to_string();
            let net_ns=namespaces::net::Net_ns{
                name:name.clone()
            };
            let child_pid=net_ns.CreateTempProcess(&true, &false, &false);
            net_ns.BindMount(child_pid)?;

            net_ns.KillChild(child_pid.clone());

            net_ns.CreateFile()?;
            let mut ns= net_ns.ReadFile()?;
            ns.insert(net_ns.name.clone(), net_ns.clone());
            net_ns.WriteToFile(ns)?;

            
        }
        6=>{//Delete net ns
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating veth pair");
            let handle=Create_RT_Netlink().unwrap();


            let name=data["name"].as_str().unwrap().to_string();
            let net_ns=namespaces::net::Net_ns{
                name:name.clone()
            };

            net_ns.Delete(&name);

            net_ns.CreateFile()?;
            let mut ns= net_ns.ReadFile()?;
            ns.remove(&net_ns.name);
            net_ns.WriteToFile(ns)?;


        }
        7=>{//Create mount ns
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating veth pair");
            let handle=Create_RT_Netlink().unwrap();

            let name=data["name"].as_str().unwrap().to_string();

            let mount=namespaces::mount::mount{
                name:name.clone()
            };

            let child_pid=mount.CreateTempProcess(&false, &false, &true);

            mount.BindMount(child_pid)?;

            mount.KillChild(child_pid);


            mount.CreateFile()?;
            let mut ns= mount.ReadFile()?;
            ns.insert(mount.name.clone(), mount.clone());
            mount.WriteToFile(ns)?;

        }
        8=>{//Delete mount ns 
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating veth pair");
            let handle=Create_RT_Netlink().unwrap();

            let name=data["name"].as_str().unwrap().to_string();
            let mount=namespaces::mount::mount{
                name:name.clone()
            };

            mount.Unmount();

            mount.CreateFile()?;
            let mut ns= mount.ReadFile()?;
            ns.remove(&name);
            mount.WriteToFile(ns)?;
        }

        
        10=>{//Create a Contanier
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while creating veth pair");
            let handle=Create_RT_Netlink().unwrap();

            let name=data["name"].as_str().unwrap().to_string();

            let bridge_name=data["bridge_name"].as_str().unwrap().to_string();
            let BridgeMap=container::bridge::Bridge::ReadFile()?;

            let bridge=BridgeMap.get(&bridge_name).ok_or_else(||format!("no bridge found for name {}",bridge_name))?;


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

            let veth12=veth_pair.ReadVethFile()?;

            let pair=veth12.get(format!("{}_{}",veth_pair.veth_front.name,veth_pair.veth_back.name).as_str()).ok_or_else(||format!("no veth found for {}_{}",veth_pair.veth_front.name,veth_pair.veth_back.name))?;

            // let container=container::container{
            //     name:name.clone(),
            //     bridge:bridge.clone(),
            //     veth:pair.clone(),

            //     mount_ns:namespaces::mount::mount{},
            //     net_ns:namespaces::net::Net_ns{
            //         name:"".to_string()
            //     },
            //     pid_ns:namespaces::pid::pid_ns{}
            // };

            // container.Init();
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






