use anyhow::Error;
use screen_share::server::ScreenShareSenderService;
use std::thread;
use tokio::{self, sync::mpsc, sync::mpsc::Receiver, sync::mpsc::Sender};

use crate::traits::server::ConnectableService;
mod file_share;
mod screen_share;
mod traits;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel(10);

    thread::spawn(|| {
        let mut service =
            file_share::main::FileShareService::new(6000, "File Sharing Service Started").unwrap();
        service.start();
    });

    let screen_share_service = screen_share::server::ScreenShareService::new(
        7000,
        rx,
        "Screen Sharing Service Started".to_string(),
    )?;

    thread::spawn(move || screen_share_service.start());

    Ok(())
}
