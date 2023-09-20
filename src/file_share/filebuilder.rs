use std::fs;

use anyhow::Error;
use flate2::write::GzEncoder;
use flate2::{read, Compression};
use std::io::prelude::*;
use std::path::Path;
use tokio;
use tokio::io::AsyncReadExt;

// Creadit : https://github.com/rust-lang/flate2-rs/blob/main/examples/gzdecoder-read.rs

fn get_file_name(file_path: &str) -> String {
    let path = Path::new(file_path);
    let filename = path.file_name().unwrap();

    filename.to_str().unwrap().to_owned()
}

// TODO TEST
// Reads from fs and
pub async fn compress_file(file_path: &str) -> Result<Vec<u8>, Error> {
    let mut file = tokio::fs::File::open(file_path).await?;

    let size = file.metadata().await?.len();
    let name = get_file_name(file_path);

    let mut file_buffer: Vec<u8> = Vec::new();
    file.read_exact(&mut file_buffer).await?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
    let _ = encoder.write_all(&mut file_buffer);

    let mut body = encoder.finish()?;

    let mut file_size_in_bytes = u64::to_be_bytes(size as u64).to_vec();

    let mut file_name_in_bytes = name.as_bytes().to_vec();
    file_name_in_bytes.resize(60 - file_name_in_bytes.len(), 0);

    let mut result = Vec::new();
    result.append(&mut file_size_in_bytes);
    result.append(&mut file_name_in_bytes);
    result.append(&mut body);

    Ok(result)
}

// TODO TEST
pub fn decompress_file(
    compressed_file: &Vec<u8>,
    file_metadata: crate::file_share::main::FileMetaData,
) {
    let file_name = file_metadata.file_name;

    let mut new_file = fs::File::create(file_name).unwrap();

    let mut gz = read::GzDecoder::new(&compressed_file[..]);

    let mut s = Vec::new();
    gz.read_to_end(&mut s).unwrap();
    let _ = new_file.write_all(&s);
}
