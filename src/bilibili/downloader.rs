use std::{fs::File, io::Write};

use reqwest::{header, Client};

use crate::bilibili::utils;
use futures_util::stream::StreamExt;


pub async fn stream_downloader(stream_url: String, save_path: String) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let resp = client.get(stream_url)
        .header(header::REFERER, "https://www.bilibili.com")
        .header(header::USER_AGENT, utils::get_user_agent())
        .send()
        .await?;

    let mut file = File::create(save_path)?;
    let mut stream = resp.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
    }

    Ok(())
}