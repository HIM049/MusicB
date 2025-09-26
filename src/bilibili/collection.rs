use reqwest::{header, Client};
use serde_json::Value;

use crate::bilibili::{modules::{Collection, CollectionMedia}, utils, wbi_generater::{encode_wbi, get_wbi_keys}};

impl Collection {
    pub async fn from_mid(mid: i64) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let value = get_collection_details(mid, 1, 1, "".to_string()).await?;
        
        Ok(Collection::from_json(value["data"].clone()).ok_or("failed when deserialization response")?)
    }
}

async fn get_collection_details(mid: i64, ps: i64, pn: i64, sessdata: String) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    // Create client
    let client = Client::new();
    let wbi = get_wbi_keys().await.unwrap(); // TODO: Move to global

    // Write params and generate wbi
    let params = encode_wbi(
        vec![
            ("media_id", mid.to_string()),
            ("order", "mtime".to_string()), // view / pubtime
            ("ps", ps.to_string()),
            ("pn", pn.to_string()),
            ("platform", "web".to_string()),
        ],
        wbi,
    );

    // Write into url and send request
    let url = "https://api.bilibili.com/x/v3/fav/resource/list";
    let resp = client
        .get(format!("{}?{}", url, params))
        .header(header::REFERER, "https://www.bilibili.com")
        .header(header::USER_AGENT, utils::get_user_agent())
        .header(header::COOKIE, format!("SESSDATA={}", sessdata))
        .send()
        .await?;

    let json_resp: Value = serde_json::from_str(resp.text().await?.as_str())?;

    // Check response code
    if json_resp["code"].as_i64().unwrap_or(1) != 0 {
        return Err(format!(
            "Failed when request: {}",
            json_resp["message"].as_str().unwrap()
        )
        .into());
    }

    Ok(json_resp)
}


pub async fn get_collection_list(mid: i64) -> Result<Vec<CollectionMedia>, Box<dyn std::error::Error + Send + Sync>> {
    let mut index = 1;
    let mut media_list: Vec<CollectionMedia> = vec![];

    loop {
        let value = get_collection_details(mid, 20, index, "".to_string()).await?;
        if let Some(medias) = value["data"]["medias"].as_array() {
            for media in medias.iter() {
                if let Some(media) = CollectionMedia::from_json(media.clone()) {
                    media_list.push(media);
                }
            }
        } else {
            break;
        }
        index += 1;
    }
    Ok(media_list)
}