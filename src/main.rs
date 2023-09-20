use file_share::sender::send_file;
use tokio;

use crate::traits::server::ConnectableService;
mod file_share;
mod screen_share;
mod traits;
#[tokio::main]
async fn main() {
    let _ = send_file("192.168.1.15".to_string(), 6000, "~/Videos/.erdekes/cohan2.mp4".to_string()).await;
    tokio::task::spawn_blocking(|| {
        let mut service =
            file_share::main::FileShareService::new(6000, "File Sharing Service Started").unwrap();
        service.start();
    });
}
