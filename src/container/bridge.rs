use std::{collections::HashMap, fs::{self, OpenOptions}, io::Write, net::Ipv4Addr};

use rtnetlink::Handle;
use serde_json::Value;

use crate::interface::{self, Interface};

use serde::{Serialize, Deserialize};
#[derive(Clone,Serialize, Deserialize)]
pub struct Bridge{
    pub name:String,
    pub insys: String,

    pub index: Option<u32>,

    pub status: Option<String>,

    pub ip:Option<String>,
    pub subnet:Option<u8>,
    pub network:Option<String>,

    
}

impl Bridge {
    pub async fn AssignIP(&self,handle:&Handle,data:&Value)->Result<(),Box<dyn std::error::Error>>{
        let index=self.GetIndex(handle).await?;
        handle
        .address()
        .add(
            index,
            data["ip"].as_str().unwrap().parse::<Ipv4Addr>().unwrap().into(),
            data["subnet"].as_u64().unwrap() as u8,
        )
        .execute()
        .await?;

        Ok(())
    }

    pub fn CreateFile(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("bridge.json")
        {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                println!("Bridge storage file already exists");
                return Ok(());
            }
            Err(e) => return Err(Box::new(e)),
        };

        file.write_all(b"{}")?;
        println!("Created the bridge storage file");

        Ok(())
    }

    pub fn ReadFile(&self)->Result<HashMap<String,Self>,Box<dyn std::error::Error>>{
        let paylaod=fs::read_to_string("bridge.json")?;
        let obj:HashMap<String,Self>=serde_json::from_str(&paylaod)?;
        Ok(obj)
    }

    pub fn WriteToFile(&self,mut obj:HashMap<String,Self>)->Result<(),Box<dyn std::error::Error>>{
        obj.insert(self.name.clone(),self.clone());
        fs::write("bridge.json", serde_json::to_string_pretty(&obj)?)?;
        Ok(())
    }    
}



impl interface::Interface for Bridge {
    fn name(&self) -> &str {
        &self.name
    }

    async fn Create(&self,handle:&rtnetlink::Handle)->Result<(), Box<dyn std::error::Error>> {
        handle.link()
            .add()
            .bridge(self.insys.clone())
            .execute()
            .await?;

        Ok(())   
    }

    async fn Delete(&self,handle:&rtnetlink::Handle)->Result<(), Box<dyn std::error::Error>> {
        let index=self.GetIndex(handle).await?;

        handle.link()
        .del(index)
        .execute()
        .await?;

        Ok(())
    }
    
}

