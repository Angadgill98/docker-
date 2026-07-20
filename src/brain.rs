use std::{fmt::format, fs, io::Read, net::Ipv4Addr};

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

            let mut veth_pair=container::veth::VethPair{
                veth_front:container::veth::VethEnd{
                    name:data["veth0_name"].as_str().unwrap().to_string(),
                    insys:format!("dc-{}",data["veth0_name"].as_str().unwrap().to_string()),
                    index:None,
                    ip:None,
                    subnet:None
                },
                veth_back:container::veth::VethEnd{
                    name:data["veth1_name"].as_str().unwrap().to_string(),
                    insys:format!("dc-{}",data["veth1_name"].as_str().unwrap().to_string()),
                    index:None,
                    ip:None,
                    subnet:None
                }
            };


            veth_pair.Create(&handle).await?;
            
            veth_pair.veth_back.index=Some(veth_pair.veth_back.GetIndex(&handle).await);
            veth_pair.veth_front.index=Some(veth_pair.veth_front.GetIndex(&handle).await);
            veth_pair.CreateVethFile()?;

            let mut veth_file_map=container::veth::VethPair::ReadVethFile()?;    
            veth_file_map.insert(format!("{}_{}",veth_pair.veth_front.name,veth_pair.veth_back.name), veth_pair.clone());        
            veth_pair.WriteVethFile(&veth_file_map)?;

        }
        4=>{//assign ip to veth 
            
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while assigning the ip to the veth pair");
            let handle=Create_RT_Netlink().unwrap();

            let veth0_name=data["veth0_name"].as_str().unwrap().to_string();
            let veth1_name=data["veth1_name"].as_str().unwrap().to_string();

            let mut vethmap=container::veth::VethPair::ReadVethFile()?;
            let mut veth_pair=vethmap.get_mut(&format!("{}_{}",veth0_name,veth1_name)).ok_or("no veth pair found")?.clone();

            
                
            
            veth_pair.veth_front.ip=Some(data["veth0_ip"]
            .as_str()
            .unwrap()
            .parse::<Ipv4Addr>()?);

            veth_pair.veth_front.subnet=Some(data["subnet"]
            .as_u64()
            .unwrap() as u8);

            veth_pair.veth_back.ip=Some(data["veth1_ip"]
            .as_str()
            .unwrap()
            .parse::<Ipv4Addr>()?);
            
            veth_pair.veth_back.subnet=Some(data["subnet"]
            .as_u64()
            .unwrap() as u8);

            veth_pair.veth_back.AssignIP(&handle).await?;
            veth_pair.veth_front.AssignIP(&handle).await?;


            vethmap.insert(format!("{}_{}",veth0_name,veth1_name), veth_pair.clone());
            
             
            veth_pair.WriteVethFile(&vethmap)?;      

        }
        5=>{//Delete a veth
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while deleting veth pair");
            let handle=Create_RT_Netlink().unwrap();

            let veth_pair=container::veth::VethPair{
                veth_front:container::veth::VethEnd{
                    name:data["veth0_name"].as_str().unwrap().to_string(),
                    insys:format!("dc-{}",data["veth0_name"].as_str().unwrap().to_string()),
                    index:None,
                    ip:None,
                    subnet:None
                },
                veth_back:container::veth::VethEnd{
                    name:data["veth1_name"].as_str().unwrap().to_string(),
                    insys:format!("dc-{}",data["veth1_name"].as_str().unwrap().to_string()),
                    index:None,
                    ip:None,
                    subnet:None
                }
            };

            veth_pair.Delete(&handle).await?;

            
            
            veth_pair.CreateVethFile()?;

            let mut veth_file_map=container::veth::VethPair::ReadVethFile()?;            
            veth_file_map.remove(format!("{}_{}",veth_pair.veth_front.name,veth_pair.veth_back.name).trim());
            veth_pair.WriteVethFile(&veth_file_map)?;            

            

        }   
        6=>{//Create net ns 
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
        7=>{//Delete net ns
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
        8=>{//Create mount ns
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
        9=>{//Delete mount ns 
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

            let container_name=data["name"].as_str().unwrap().to_string();

            let bridge_name=data["bridge_name"].as_str().unwrap().to_string();
            let BridgeMap=container::bridge::Bridge::ReadFile()?;

            let bridge=BridgeMap.get(&bridge_name).ok_or_else(||format!("no bridge found for name {}",bridge_name))?;

            let veth0_name=data["veth0_name"].as_str().unwrap().to_string();
            let veth1_name=data["veth1_name"].as_str().unwrap().to_string();

            let vethmap=container::veth::VethPair::ReadVethFile()?;
            let veth_pair=vethmap.get(&format!("{}_{}",veth0_name,veth1_name)).ok_or("no veth pair found")?;

            let container=container::container{
                name:container_name,
                bridge:bridge.clone(),
                veth:veth_pair.to_owned(),
                mount_ns:Some(namespaces::mount::mount{name:String::from("xyz")}),
                net_ns:Some(namespaces::net::Net_ns{name:String::from("xyz")}),
                pid_ns:Some(namespaces::pid::pid_ns{}),


                pid:None
            };

            container.CreateInitFile()?;

            let mut init_map=container::container::ReadInitFile()?;
            init_map.insert(container.name.clone(), container.clone());
            container.WriteToFile(init_map)?;
        }
        11=>{//Start a container
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while starting a container");
            let handle=Create_RT_Netlink().unwrap();


            let container_name=data["name"].as_str().unwrap().to_string();

            let mut init_map=container::container::ReadInitFile()?;
            let mut con=init_map.get(&container_name).unwrap().clone();
            let pid= con.Init(&handle).await?;
            con.pid=Some(pid);

            con.CreateConFile()?;
            let mut con_map=container::container::ReadConFile()?;
            con_map.insert(container_name.clone(), con.clone());
            

            
            

            
            
            con.WriteConFile(con_map)?;
            con.WriteToFile(init_map)?;
                
            

        }
        12=>{//stop a conatiener
            let data:Value=serde_json::from_slice(msg).expect("failed to get jsonobj while starting a container");
            let handle=Create_RT_Netlink().unwrap();

            let container_name=data["name"].as_str().unwrap().to_string();
            

            let mut con_map=container::container::ReadConFile()?;
            let con=con_map.get(&container_name).unwrap().clone();
            let pid=con.pid.unwrap();

            con.Kill();

            con_map.remove(&container_name);

            con.WriteConFile(con_map)?;
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






