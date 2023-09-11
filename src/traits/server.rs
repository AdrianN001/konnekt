use anyhow::Result;

pub trait ConnectableService {
    fn new(port: i32, debug_text: &str) -> Result<Self, String>
    where
        Self: Sized;
    fn start(&mut self) -> !;
}
