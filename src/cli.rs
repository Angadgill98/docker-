use std::{fs::File, io::{BufRead, BufReader, Read, Write}, net::{IpAddr, Ipv4Addr, TcpStream}, os::linux::net, path::Prefix};

use libc::statvfs;
use serde_json::{error::Category::Data, json};

use crate::{container::bridge, sokcet};

use ipnetwork::Ipv4Network;

pub fn CLI(){
    println!("Starting with cli approach");
    let mut input:String;
    let mut  socket=sokcet::CreateClientSocket().unwrap();
    loop {
        CLI_printer();
        input=CLI_Input();
        handleInput(&input,&mut socket);   

    }


}



fn CLI_printer(){
    println!("Enter a no.");


    println!("1: Create a Bridge");
    println!("2: Delete a Bridge");

    println!("");

    println!("3: Create a veth apair");
    println!("4: Assignip to veth");
    println!("5: Delete a veth");

    println!("");

    println!("6: Create net ns");
    println!("7: Delete net ns");

    println!("");

    println!("8: Create mount ns");
    println!("9: Delete mount ns");

    println!("10: Create a Contianer");
    println!("11: Start a container");
    println!("12: Stop a container")

}

fn CLI_Input()->String{
    let mut input:String=String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input
}

fn handleInput(input:&String,socket:&mut TcpStream){
    match input.trim() {
        "1"=>{
            let op:u8= input.trim().parse().unwrap();
            CreateBrdige(socket,&op);
            
        }
        "2"=>{
            let op:u8= input.trim().parse().unwrap(); 
            delete_bridge(socket, &op);
        }   
        "3"=>{
            let op:u8= input.trim().parse().unwrap(); 
            CreateVethpair(socket, &op);
        }
        "4"=>{
            let op:u8= input.trim().parse().unwrap(); 
            Assign_veth_ip(socket, &op);    
        }
        "5"=>{
            let op:u8= input.trim().parse().unwrap(); 
            delete_veth(socket, &op);
        }
        "6"=>{
            let op:u8= input.trim().parse().unwrap();
            CreateNet_ns(socket, &op);
        }
        "7"=>{
            let op:u8= input.trim().parse().unwrap();
            DeleteNet_ns(socket, &op);
        }
        "8"=>{
            let op:u8= input.trim().parse().unwrap();
            CreateMount_ns(socket, &op);
        }
        "9"=>{
            let op:u8= input.trim().parse().unwrap();
            DeleteMount_ns(socket, &op);
        }
        "10"=>{
            let op:u8= input.trim().parse().unwrap();
            CreateContainer(socket, &op);
        }
        "11"=>{
            let op:u8= input.trim().parse().unwrap();
            StartContainer(socket, &op);
        }
        "12"=>{
            let op:u8= input.trim().parse().unwrap();
            StopContainer(socket, &op);
        }
        _=>{

        }
    }
}

fn CreateBrdige(socket:&mut TcpStream,input:&u8){
    println!("Enter bridge name:");
    let bridge_name=CLI_Input();

    
    
    println!("Enter the Subnet of the Bridge");
    let subnet=GetSubnet().to_string().trim().parse::<u8>().unwrap();
    println!("Enter ip of the bridge");
    let ip=GetIP().to_string().trim().parse::<Ipv4Addr>().unwrap();

    let network=Ipv4Network::new(ip, subnet).expect("Valid IP and prefix");
    println!("Creating the network of the bridge:{}",network);


    let status=GetStatus();

    let operation_no:u8=input.clone();
    
    let data = json!({
        "name": &bridge_name.trim(),
        "status":&status.trim(),
        "network":format!("{}",network),
        "ip":ip,
        "subnet":subnet
    });
    

    let json = serde_json::to_string(&data).unwrap();
    let len=json.len();

    
    socket.write_all(&[operation_no]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());  
    
}

fn GetSubnet()->u8{
     println!("Enter subnet prefix (e.g. 24):");

    let prefix = loop {
        let input = CLI_Input();
        match input.trim().parse::<u8>() {
            Ok(prefix) if prefix <= 32 => break prefix,
            _ => println!("Invalid prefix. Enter a number between 0 and 32:"),
        }
    };
    prefix
}

fn GetIP()->Ipv4Addr{
    
    let ip =loop{
        let input =CLI_Input();
        match input.trim().parse::<std::net::Ipv4Addr>(){
            Ok(ip) => break ip,
            Err(_) => {
                println!("Invalid IP address. Please enter again:");
            }
        }
    };
    ip
}

fn GetStatus() -> String {
    println!("Enter bridge status (up/down) [default: down]:");

    let status = loop {
        let input = CLI_Input();
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "" => break "down".to_string(),      // User just pressed Enter
            "up" => break "up".to_string(),
            "down" => break "down".to_string(),
            _ => println!("Invalid input. Enter 'up', 'down', or press Enter for the default (down)."),
        }
    };

    status
}

fn delete_bridge(socket:&mut TcpStream,input:&u8){
    println!("Enter bridge name:");
    let bridge_name=CLI_Input();
    let operation_no:u8=input.clone();
    
    let data = json!({
    "name": &bridge_name.trim()
    });
    

    let json = serde_json::to_string(&data).unwrap();
    let len=json.len();
    
    socket.write_all(&[operation_no]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());  
    
}




fn CreateVethpair(socket:&mut TcpStream,input:&u8){
    println!("Enter veth name 1:");
    let veth1=CLI_Input();



    println!("Enter veth name 2:");
    let veth2=CLI_Input();


    let op=input.clone();

    let data=json!({
        "veth0_name":veth1.trim(),
        "veth1_name":veth2.trim(),

    });

    let json = serde_json::to_string(&data).unwrap();
    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());  

}

fn delete_veth(socket:&mut TcpStream,input:&u8){
    println!("Enter veth name 1:");
    let veth1=CLI_Input();



    println!("Enter veth name 2:");
    let veth2=CLI_Input();


    let op=input.clone();

    let data=json!({
        "veth0_name":veth1.trim(),
        "veth1_name":veth2.trim(),

    });
    

    let json = serde_json::to_string(&data).unwrap();
    let len=json.len();
    
    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());  
    
}

fn Assign_veth_ip(socket:&mut TcpStream,input :&u8){
    println!("Enter veth name 1:");
    let veth1=CLI_Input();



    println!("Enter veth name 2:");
    let veth2=CLI_Input();


    
    println!("Enter ip of the veth0");
    let veth0_ip=GetIP().to_string().trim().parse::<Ipv4Addr>().unwrap();

   

    println!("Enter ip of the veth1");
    let veth1_ip=GetIP().to_string().trim().parse::<Ipv4Addr>().unwrap();


    println!("Enter the subnet");
    let subnet=GetSubnet();

    let op=input.clone();

    let data=json!({
        "veth0_name":veth1.trim(),
        "veth1_name":veth2.trim(),
        "veth1_ip":veth1_ip,
        "veth0_ip":veth0_ip,
        "subnet":subnet

    });

    let json = serde_json::to_string(&data).unwrap();
    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes()); 
}

fn CreateNet_ns(socket:&mut TcpStream,input:&u8){
    println!("Enter the name of net ns");
    let net_ns_name=CLI_Input();

    let data=json!({
        "name":&net_ns_name.trim(),
    });

    let op=input.clone();

    let json = serde_json::to_string(&data).unwrap();
    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());  
    
}

fn DeleteNet_ns(socket:&mut TcpStream,input:&u8){
    println!("Enter the name of net ns");
    let net_ns_name=CLI_Input();

    let data=json!({
        "name":&net_ns_name.trim(),
    });

    let op=input.clone();

    let json = serde_json::to_string(&data).unwrap();
    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());  
}



fn CreateMount_ns(socket:&mut TcpStream,input:&u8){
    println!("Enter the name of the mount ns:");
    let mount_name=CLI_Input();

    // println!("Enter the name of the image need to implemen,available images are :");
    // println!("Ubuntu");
    // let mut  image_name=CLI_Input();

    // loop {
    //     match image_name.trim() {
    //         "Ubuntu"=>{
    //             break;
    //         }
    //         _=>{
    //             println!("invalid input enter valid input");
    //             image_name=CLI_Input();
    //         }
    //     }
    // }

    let data=json!({
        "name":mount_name.clone().trim(),
        //"image":image_name.clone().trim()
    });
    let op=input.clone();

    let json=serde_json::to_string(&data).unwrap();

    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());


}

fn DeleteMount_ns(socket:&mut TcpStream,input:&u8){
    println!("Enter the name of the mount ns:");
    let mount_name=CLI_Input();

    // println!("Enter the name of the image need to implemen,available images are :");
    // println!("Ubuntu");
    // let mut  image_name=CLI_Input();

    // loop {
    //     match image_name.trim() {
    //         "Ubuntu"=>{
    //             break;
    //         }
    //         _=>{
    //             println!("invalid input enter valid input");
    //             image_name=CLI_Input();
    //         }
    //     }
    // }

    let data=json!({
        "name":mount_name.clone().trim(),
        //"image":image_name.clone().trim()
    });
    let op=input.clone();

    let json=serde_json::to_string(&data).unwrap();

    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());


}



fn CreateContainer(socket:&mut TcpStream,input:&u8){
    println!("Enter the name of the container");
    let name=CLI_Input();

    println!("Enter the name of the image need to implemen,available images are :");
    println!("Ubuntu");
    let mut  image=CLI_Input();

    loop {
        match image.trim() {
            "Ubuntu"=>{
                break;
            }
            _=>{
                println!("invalid input enter valid input");
                image=CLI_Input();
            }
        }
    }

    println!("Enter the name of the bridge:");
    let bridge_name=CLI_Input();

    println!("Enter the names of the veth0 and veth1 ");
    println!("Name for veth0 (connecting to the container");
    let veth0=CLI_Input();
    
    println!("Name for veth 1(conencted to bridge");
    let veth1=CLI_Input();

    let data=json!({
        "name":name.trim(),
        "image":image.trim(),
        "bridge_name":bridge_name.trim(),
        "veth0_name":veth0.trim(),
        "veth1_name":veth1.trim()
    });

    let op=input.clone();

    let json =serde_json::to_string(&data).unwrap();

    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());

}

fn StartContainer(socket:&mut TcpStream,input:&u8){
    println!("Enter the name of the container");
    let name=CLI_Input();

    let data=json!({
        "name":name.trim(),
    });

    let op=input.clone();

    let json =serde_json::to_string(&data).unwrap();

    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());
}

fn StopContainer(socket:&mut TcpStream,input:&u8){
    println!("Enter the name of the container");
    let name=CLI_Input();

    let data=json!({
        "name":name.trim(),
    });

    let op=input.clone();

    let json =serde_json::to_string(&data).unwrap();

    let len=json.len();

    socket.write_all(&[op]);  
    socket.write_all(&[len as u8]);
    socket.write_all(json.as_bytes());
}


fn HandleENVpath()->Vec<String>{
    let mut env_path:String=String::new();
    println!("Enter the env with the filename");
    std::io::stdin().read_line(&mut env_path).unwrap();

    let file=File::open(env_path).unwrap();
    let reader=BufReader::new(file);


    let mut env_vec:Vec<String>=Vec::new();
    println!("Env var are :");
    for line in reader.lines(){
        let line=line.unwrap();
        println!("{}",line);
        env_vec.push(line);
    }

    return env_vec;
}

fn HandleCommands()->Vec<String>{
    let mut commands:String=String::new();
    let mut comm_vec:Vec<String>=Vec::new();
    println!("Enter the commads to execute(Blank to move forward");
    loop {
        std::io::stdin().read_line(&mut commands).unwrap();

        match commands.trim() {
            ""=>break,
            _=>{
                comm_vec.push(commands.to_string());
            }
            
        }    
    }
    
    return comm_vec;
}

fn HandleAppCommands()->Vec<String>{
    let mut commands:String=String::new();
    let mut comm_vec:Vec<String>=Vec::new();
    println!("Enter the commads to execute in the app path (Blank to move forward");
    loop {
        std::io::stdin().read_line(&mut commands).unwrap();

        match commands.trim() {
            ""=>break,
            _=>{
                comm_vec.push(commands.to_string());
            }
            
        }    
    }
    
    return comm_vec;
}


fn HandleAppPath(){
    let mut app_path:String=String::new();
    println!("Enter the app path");
    std::io::stdin().read_line(&mut app_path).unwrap();
}