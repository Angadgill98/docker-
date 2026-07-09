use std::{env, io};
use std::net::{TcpListener, TcpStream};


pub fn CreateBrainSocket()-> io::Result<TcpListener> {
    let mut addr:String;    
    match env::var("BRAIN_ADD")  {
        Ok(address)=>{
            println!("listening to addr value {address}");
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


pub fn CreateClientSocket()-> io::Result<TcpStream> {
    let mut addr:String; 
    match env::var("BRAIN_ADD")  {
        Ok(address)=>{
            println!("connecting client socket to addr value  {address}");
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


