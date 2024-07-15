use serde_json::Value;

use crate::error::SharedError;

#[cfg(target_arch = "wasm32")]
pub fn download_bytes(url: &str) -> Result<Vec<u8>, SharedError> {
    let response = futures::executor::block_on(reqwest::get(url))?;
    Ok(futures::executor::block_on(response.bytes())?.to_vec())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn download_bytes(url: &str) -> Result<Vec<u8>, SharedError> {
    use std::io::Read;
    let mut buf = vec![];
    let _ = reqwest::blocking::get(url)?.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(target_arch = "wasm32")]
pub fn download_json(url: &str) -> Result<Value, SharedError> {
    let response = futures::executor::block_on(reqwest::get(url))?;
    let json = futures::executor::block_on(response.json())?;
    Ok(json)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn download_json(url: &str) -> Result<Value, SharedError> {
    Ok(reqwest::blocking::get(url)?.json()?)
}
