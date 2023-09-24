use anyhow::Error;
use screen_share::server::ScreenShareSenderService;
use std::thread;
use file_share::sender::send_file;
use tokio::{self, sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender};

use crate::traits::server::ConnectableService;
mod file_share;
mod screen_share;
mod traits;
#[tokio::main]
async fn main() -> Result<(), Error>{
    let (sender, receiver) = mpsc::channel::<Vec<u8>>(10);
    //let _ = send_file("192.168.1.15".to_string(), 6000, "~/Videos/test.mp4".to_string())?;
    tokio::task::spawn_blocking(|| {
        let mut service =
            file_share::server::FileShareService::new(6000, "File Sharing Service Started").unwrap();
        service.start();
    });

    let screen_share_service = screen_share::server::ScreenShareService::new(
        7000,
        receiver,
        "Screen Sharing Service Started".to_string(),
    )?;

    thread::spawn(move || screen_share_service.start());

    Ok(())
}
