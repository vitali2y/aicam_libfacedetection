use log::debug;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    multipart::Form,
};
use std::{error::Error, fs::File, io::Read};

use crate::SETTINGS;

pub async fn upload_image(caption: String, file_path: String) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://api.telegram.org/bot{}/sendPhoto",
        SETTINGS.telegram.token
    );
    debug!("uploading {}", file_path);
    let mut file = File::open(file_path)?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;

    let client = reqwest::Client::new();
    let part = reqwest::multipart::Part::bytes(contents).file_name("filename.filetype");
    let form = Form::new()
        .text("chat_id", SETTINGS.telegram.clone().channel)
        .text("caption", caption)
        .part("photo", part);

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("multipart/form-data"),
    );

    let response = client
        .post(&url)
        .headers(headers.clone())
        .multipart(form)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to upload picture: {}", response.status()),
        )))
    }
}
