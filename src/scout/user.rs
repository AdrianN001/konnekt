

use anyhow::Error;
use serde::{Serialize, Deserialize};

use tokio::fs;


use local_ip_address::local_ip;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum State{
    Alive,
    Dead,
}

#[derive(Serialize, Deserialize, Debug )]
pub struct User{
    pub ip_address: String,
    pub username: String,
    pub state: State
}

impl User{
    pub async fn new_from_fs(config_file_path: &str) -> Result<Self, Error>{

        let config_file = fs::read_to_string(config_file_path).await?;
        
        let username = config_file.lines()
                                                .find(|line: &&str| {
                                                    line.starts_with("username=")
                                                })
                                                .unwrap_or_default()
                                                .to_string()
                                                .strip_prefix("username=\"")
                                                .unwrap_or_default()
                                                .strip_suffix("\"")
                                                .unwrap_or_default()
                                                .to_string();
            
        let ip_address = local_ip()?.to_string();

        Ok(Self{
            ip_address,
            username,
            state: State::Alive
        })

    }

   
}
