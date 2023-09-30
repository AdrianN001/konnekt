mod file_share;
mod traits;
mod scout;
mod shared;

use anyhow::Error;
use std::thread;
use shared::file::FileDescriptor;
use file_share::sender::send_file;
use scout::{*, user::User};
use local_ip_address::local_ip;
use ctrlc;
use traits::file::Serializable;
use shared::snapshot::Snapshot;


use crate::traits::server::ConnectableService;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut your_profile = scout::initiator::broadcast_live().await?;


    let snapshot = Snapshot::create().await?;
    snapshot.write_to_file("/home/noirangel/Shared/.snapshots/1.txt").await?;
    println!("{:?}", snapshot);

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
