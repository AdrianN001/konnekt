use std::sync::Arc;

use crate::traits;
use anyhow::Error;
use tokio::{net, sync::Mutex};

struct ScreenShareService {
    listener: net::TcpListener,
}

struct ScreenShareSenderService {
    socket: net::UdpSocket,
    endpoint: String,
    frame_buffer: Arc<Mutex<Vec<u8>>>,
}

impl ScreenShareService {
    pub async fn new(port: i32, debug_text: String) -> Result<Self, Error> {
        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

        println!("{}", debug_text);

        Ok(ScreenShareService { listener: listener })
    }
}

impl ScreenShareSenderService {
    pub async fn new(ip_address: String, port: i32, debug_text: String) -> Result<Self, Error> {
        let socket = net::UdpSocket::bind("[::]:0").await?;
        let endpoint = format!("{}:{}", ip_address, port);

        let frame_buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));

        println!("{}", debug_text);

        Ok(ScreenShareSenderService {
            socket: socket,
            endpoint,
            frame_buffer,
        })
    }
    // TODO Write the streaming part
}