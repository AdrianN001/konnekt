use std::sync::Arc;

use anyhow::Error;
use tokio::{
    net,
    sync::{mpsc, mpsc::Receiver, mpsc::Sender, Mutex},
};

struct ScreenShareService {
    listener: net::TcpListener,
    frame_buffer: Arc<Mutex<Vec<u8>>>,
    output_channel_for_frames: Receiver<Vec<u8>>,
}

struct ScreenShareSenderService {
    socket: net::UdpSocket,
    endpoint: String,
    frame_buffer: Arc<Mutex<Vec<u8>>>,
}

impl ScreenShareService {
    pub async fn new(
        port: i32,
        channel: Receiver<Vec<u8>>,
        debug_text: String,
    ) -> Result<Self, Error> {
        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

        println!("{}", debug_text);

        Ok(ScreenShareService {
            listener: listener,
            output_channel_for_frames: channel,
            frame_buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }
    pub async fn start() -> ! {
        //let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel(10);

        loop {}
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
