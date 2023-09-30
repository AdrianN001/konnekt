mod file_share;
mod traits;
mod scout;

use anyhow::Error;
use std::thread;
use file_share::sender::send_file;
use scout::{*, user::User};
use local_ip_address::local_ip;
use ctrlc;



use crate::traits::server::ConnectableService;

#[tokio::main]
async fn main() -> Result<(), Error>{
    //let _ = send_file("192.168.1.15".to_string(), 6000, "~/Videos/test.mp4".to_string())?;
    let mut your_profile = scout::initiator::broadcast_live().await?;
    print!("{:?}", your_profile);

    let _ = ctrlc::set_handler( move  || {

        let _ = scout::initiator::broadcast_stop(&mut your_profile);
        std::process::exit(1);
    });

    
    
    let file_share_thread = thread::spawn(|| {
        let mut service =
            file_share::server::FileShareService::new(6000, "File Sharing Service Started").unwrap();
        service.start();
    });

    let _ = file_share_thread.join();


    Ok(())

}
