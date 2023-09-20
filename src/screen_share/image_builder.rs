use anyhow::Error;
use flate2::{write::GzEncoder, Compression};
use screenshots::Screen;
use std::{io::Write, time::Instant};

fn take_screenshot(disply_id: usize) -> Result<Vec<u8>, Error> {
    let start = Instant::now();
    let screens = Screen::all()?;
    let screen = screens[disply_id];

    println!("capturer {screen:?}");
    let image = screen.capture()?;

    Ok(image
        .as_raw()
        .to_vec()
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 3 != 0)
        .map(|(_, item)| item.to_owned())
        .collect::<Vec<_>>())
}

// TODO WRITE IT
fn compress_buffer(buffer: &Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
    let _ = encoder.write_all(&buffer);

    let body = encoder.finish()?;

    Ok(body)
}
