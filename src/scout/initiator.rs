use std::net::{ UdpSocket};
use tokio::{net, fs};
use std::env;

use anyhow::Error;

use super::user::{User, State};

const LISTENING_ADDRESS : &str = "0.0.0.0";

/*
 - Binds itself to 
 - sends a message to broadcast about the current state of the app

*/

pub async fn broadcast_live( ) -> Result<User, Error> {
   let socket = net::UdpSocket::bind("[::]:0").await?;
   socket.set_broadcast(true)?;
   let home_dir = env::home_dir().unwrap().to_str().unwrap().to_string();
   
   let your_profile =  User::new_from_fs(&format!("{}{}", home_dir, "/.konn.conf".to_string())).await?;


   let user_json_data = serde_json::to_string(&your_profile)?;


   socket.send_to(user_json_data.as_bytes(), "255.255.255.255:25300").await?;

   Ok(your_profile)
   
}

pub fn broadcast_stop( user_to_kill: &mut User ) -> Result<(), Error> {
    let socket = std::net::UdpSocket::bind("[::]:0")?;
    socket.set_broadcast(true)?;
    
    user_to_kill.state = State::Dead;
    let user_json_data = serde_json::to_string(&user_to_kill)?;

 
 
    socket.send_to(user_json_data.as_bytes(), "255.255.255.255:25300")?;
 
    Ok(())

}