use std::{env, io};
use std::net::{TcpListener, TcpStream};


pub fn CreateBrainSocket()-> io::Result<TcpListener> {
    let mut addr:String;    
    match env::var("BRAIN_ADD")  {
        Ok(address)=>{
            println!("addr valueb is {address}");
            addr=address;
            TcpListener::bind(addr)
        }
        Err(e)=>{
            println!("value not present deafulting to 127.0.0.1:8080");
            addr="127.0.0.1:8080".to_string();
            TcpListener::bind(addr)
        }
    }
    
    
}

pub fn StartBrainListener(listener_socket:TcpListener)->Result<(),std::io::Error>{
    loop{
        let (stream,addr)=listener_socket.accept()?;

    }

    Ok(())
} 

pub fn CreateClientSocket()-> io::Result<TcpStream> {
    let mut addr:String; 
    match env::var("BRAIN_ADD")  {
        Ok(address)=>{
            println!("addr valueb is {address}");
            addr=address;
            TcpStream::connect(addr)
        }
        Err(e)=>{
            println!("value not present deafulting to 127.0.0.1:8080");
            addr="127.0.0.1:8080".to_string();
            TcpStream::connect(addr)
        }
    }
    
}


