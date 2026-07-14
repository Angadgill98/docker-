use std::string;

use futures::TryStreamExt;
use rtnetlink::Handle;

pub trait Interface{
    fn name(&self)->&str;
    async fn GetIndex(&self,handle:&Handle)->Result<u32, Box<dyn std::error::Error>>{
        let mut links=handle.link()
        .get()
        .match_name(self.name().to_string())
        .execute();
        
        let temp=links.try_next().await?.ok_or_else(||format!("no interface fount for {}",self.name()))?;
        Ok(temp.header.index)
    }

    async fn Create(&self,handle:&Handle)->Result<(), Box<dyn std::error::Error>>;

    async fn Delete(&self,handle:&Handle)->Result<(), Box<dyn std::error::Error>>;

}