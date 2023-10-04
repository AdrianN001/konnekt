mod file_share;
mod traits;
mod scout;
mod shared;

use anyhow::Error;
use std::thread;
use ctrlc;
use shared::snapshot::Snapshot;


use crate::traits::server::ConnectableService;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut your_profile = scout::initiator::broadcast_live().await?;


    let snapshot = Snapshot::create("6").await?;
    snapshot.write_to_file().await?;

    let prev_snapshot = Snapshot::read_from_fs("3").await?;

    let differences = snapshot.compare(&prev_snapshot);

    println!("{:?}", snapshot);
    println!("{:?}", prev_snapshot);
   
    for cell in differences {
        println!("{:?} => {:?}", (cell.0).file_name, cell.1);
    }

    let _ = ctrlc::set_handler( move  || {

        let _ = scout::initiator::broadcast_stop(&mut your_profile);
        std::process::exit(1);
    });

    
    
    let file_share_thread = thread::spawn(|| {
        let mut service =
            file_share::server::FileShareService::new(6000, "File Sharing Service Started").unwrap();
        service.start();
    });


    Ok(())

}
