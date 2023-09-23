use std::{io::Read, sync::Arc};

use anyhow::Error;
use flate2::read;
use tokio::{
    net,
    sync::{mpsc, mpsc::Receiver, mpsc::Sender, Mutex},
};

pub struct ScreenShareService {
    listener: std::net::TcpListener,
    output_channel_for_frames: Receiver<Vec<u8>>,
}

pub struct ScreenShareSenderService {
    socket: std::net::TcpListener,
    endpoint: String,
    frame_buffer: Arc<Mutex<Vec<u8>>>,
}

impl ScreenShareService {
    pub fn new(port: i32, channel: Receiver<Vec<u8>>, debug_text: String) -> Result<Self, Error> {
        let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))?;

        println!("{}", debug_text);

        Ok(ScreenShareService {
            listener: listener,
            output_channel_for_frames: channel,
        })
    }

    async fn decompress_frame(frame: Vec<u8>) -> () {
        let mut gz = read::GzDecoder::new(&frame[..]);

        let mut s: Vec<u8> = Vec::new();
        gz.read_to_end(&mut s).unwrap();
    }

    pub fn start(&self) -> Result<(), Error> {
        loop {
            let (stream, _addr) = self.listener.accept()?;
        }
    }
}

impl ScreenShareSenderService {
    pub fn new(ip_address: String, port: i32, debug_text: String) -> Result<Self, Error> {
        let socket = std::net::TcpListener::bind("[::]:0")?;
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
