use tokio;

use crate::traits::server::ConnectableService;
mod file_share;
mod traits;
#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| {
        let mut service =
            file_share::main::FileShareService::new(6000, "File Sharing Service Started").unwrap();
        service.start();
    });
}
