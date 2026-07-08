use std::{fs::File, io::{BufRead, BufReader, Read, Write}, net::TcpStream};

use crate::sokcet;


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

}

fn CLI_Input()->String{
    let mut input:String=String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input
}

fn handleInput(input:&String,socket:&mut TcpStream){
    match input.trim() {
        "1"=>{
            let op:u8=input.parse().unwrap();
            CreateBrdige(socket,&op);
            
        }
        _=>{

        }
    }
}

fn CreateBrdige(socket:&mut TcpStream,input:&u8){
    println!("Enter bridge name:");
    let bridge_name=CLI_Input();
    let operation_no:u8=1;
    let len=bridge_name.len();
    let data=bridge_name;
    socket.write_all(&[operation_no]);  
    socket.write_all(&[len as u8]);
    socket.write_all(data.as_bytes());     
}







fn Create_A_Container(input:&String){
    let mut container_name:String=String::new();
    println!("Creating a Container");
    println!("Enter contianer name: ");
    std::io::stdin().read_line(&mut container_name).unwrap();


    let mut app_name:String=String::new();
    println!("Enter the app name");
    std::io::stdin().read_line(&mut app_name).unwrap();


    let env_vars=HandleENVpath();
    let commands=HandleCommands();
    
    let app_commands=HandleAppCommands();
    
    
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