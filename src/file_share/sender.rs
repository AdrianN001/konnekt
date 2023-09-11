use anyhow::Error;
use tokio::{self, io::AsyncWriteExt};

use super::filebuilder::compress_file;

pub async fn send_file(ip_address: String, port: i32, file_name: String) -> Result<(), Error> {
    let mut stream = tokio::net::TcpStream::connect(format!("{}:{}", ip_address, port)).await?;

    let compressed_data: Vec<u8> = match compress_file(&file_name).await {
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    let _ = stream.write_all(&compressed_data);
    Ok(())
}
