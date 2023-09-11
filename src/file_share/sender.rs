use anyhow::Error;
use tokio::{self, io::AsyncWriteExt};

use super::filebuilder::compress_file;

pub async fn send_file(
    ip_address: String,
    port: i32,
    file_name: String,
) -> Future<Output = Result<(), Error>> {
    let mut stream = tokio::net::TcpStream::connect(format!("{}:{}", ip_address, port)).await?;

    let compressed_data: Vec<_> = match compress_file(file_path).await {
        Ok(x) => x,
        Err(x) => return x,
    };

    stream.write_all_buf(&mut compressed_data);
    Ok(());
}
