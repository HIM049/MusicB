use std::time::{Instant, SystemTime, SystemTimeError};

use chrono::Utc;
use rand_agents::user_agent;
use reqwest::{header, Client, Request};
use serde_json::Value;
use tokio::stream;

use crate::bilibili::{modules::{AudioQuality, BiliInfo, BiliStream, Meta, Upper, Video}, wbi_generater::{encode_wbi, get_wbi_keys}};
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(i32)]
// enum RespCode {
//     Success = 0,
//     Error = -400,
//     PremissionDenied = -403,
    
// }

impl Video {
    pub async fn from_bvid(bvid: String) -> Result<Video, Box<dyn std::error::Error + Send + Sync>> {
        let video = get_video_details(QueryType::BVID, bvid).await?;
        Ok(video)
    }
    pub async fn from_avid(avid: String) -> Result<Video, Box<dyn std::error::Error + Send + Sync>> {
        let video = get_video_details(QueryType::AVID, avid).await?;
        Ok(video)
    }
    pub async fn get_stream(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (stream, flac) = get_video_stream(self.info.bvid.clone(), self.info.cid.clone()).await?;
        self.stream = stream;
        self.flac_stream = flac;
        Ok(())
    }
}

enum QueryType {
    BVID,
    AVID,
}

async fn get_video_details(query_type: QueryType, id: String) -> Result<Video, Box<dyn std::error::Error + Send + Sync>> {
    // Create client
    let client = Client::new();
    let wbi = get_wbi_keys().await.unwrap(); // TODO: Move to global & add a button to refresh

    // Write params and generate wbi
    let params = match query_type {
        QueryType::BVID => encode_wbi(vec![("bvid", id)], wbi),
        QueryType::AVID => encode_wbi(vec![("aid", id)], wbi),
    };

    // Write into url and send request
    let url = "https://api.bilibili.com/x/web-interface/wbi/view";
    let resp = client.get(format!("{}?{}", url, params))
        .header(header::REFERER, "https://www.bilibili.com")
        .header(header::USER_AGENT, get_user_agent()) 
        .send()
        .await?;
    let json_resp: Value = serde_json::from_str(resp.text().await?.as_str()).unwrap();

    // Check response code
    if json_resp["code"].as_i64().unwrap_or(1) != 0 {
        return Err(format!("Failed when request: {}", json_resp["message"].as_str().unwrap()).into())
    }
    
    // Write data into struct
    let video = Video {
        info: BiliInfo {
            aid: json_resp["data"]["aid"].as_i64().unwrap(),
            bvid: json_resp["data"]["bvid"].as_str().unwrap().to_string(),
            cid: json_resp["data"]["cid"].as_i64().unwrap(),
            pic: json_resp["data"]["pic"].as_str().unwrap().to_string(),
            videos: json_resp["data"]["videos"].as_i64().unwrap(),
            tid: json_resp["data"]["tid"].as_i64().unwrap(),
            tid_v2: json_resp["data"]["tid_v2"].as_i64().unwrap(),
            tname: json_resp["data"]["tname"].as_str().unwrap().to_string(),
            tname_v2: json_resp["data"]["tname_v2"].as_str().unwrap().to_string(),
            pubdate: json_resp["data"]["pubdate"].as_i64().unwrap(),
            desc: json_resp["data"]["desc"].as_str().unwrap().to_string(),
        },
        // Fill the blank data
        stream: None,
        flac_stream: None,
        meta: Meta {
            title: json_resp["data"]["title"].as_str().unwrap().to_string(),
            cover_url: json_resp["data"]["pic"].as_str().unwrap().to_string(),
            author: json_resp["data"]["title"].as_str().unwrap().to_string(),
            lyrics_path: "".to_string(), //TODO
        },
        upper: Upper {
            mid: json_resp["data"]["owner"]["mid"].as_i64().unwrap(),
            name: json_resp["data"]["owner"]["name"].as_str().unwrap().to_string(),
            avatar: json_resp["data"]["owner"]["face"].as_str().unwrap().to_string(),
        },
    };
    
    Ok(video)

}

// API docs https://socialsisteryi.github.io/bilibili-API-collect/docs/video/videostream_url.html#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E6%B5%81%E5%9C%B0%E5%9D%80-web%E7%AB%AF
async fn get_video_stream(bvid: String, cid: i64) -> Result<(Option<BiliStream>, Option<BiliStream>), Box<dyn std::error::Error + Send + Sync>> {
    // Create client
    let client = Client::new();
    let wbi = get_wbi_keys().await.unwrap(); // TODO: Move to global

    // Write params and generate wbi
    let params = encode_wbi(vec![
        ("bvid", bvid), 
        ("cid", cid.to_string()), 
        ("gaia_source", "view-card".to_string()), // 无cookie时需要
        ("fnval", "16".to_string()), // For DASH
        ("platform", "pc".to_string()), ], wbi);
    
    // Write into url and send request
    let url = "https://api.bilibili.com/x/player/wbi/playurl";
    let resp = client.get(format!("{}?{}", url, params))
        .header(header::REFERER, "https://www.bilibili.com")
        .header(header::USER_AGENT, get_user_agent()) 
        .send()
        .await?;

    let json_resp: Value = serde_json::from_str(resp.text().await?.as_str()).unwrap();

    // Check response code
    if json_resp["code"].as_i64().unwrap_or(1) != 0 {
        return Err(format!("Failed when request: {}", json_resp["message"].as_str().unwrap()).into())
    }

    // Write data into struct
    let mut max_index: usize = 0;
    let mut max_quality: i64 = 0;
    if let Some(qualities) = json_resp["data"]["dash"]["audio"].as_array() {
        for (index, quality) in qualities.iter().enumerate() {
            if quality["id"].as_i64().unwrap() > max_quality {
                max_quality = quality["id"].as_i64().unwrap();
                max_index = index;
            }
        }
    }

    let stream = BiliStream::from_json(json_resp["data"]["dash"]["audio"][max_index].clone());
    let flac_stream = BiliStream::from_json(json_resp["data"]["dash"]["flac"]["audio"].clone());

    Ok((stream, flac_stream))
}

pub fn get_user_agent() -> String{
    // "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36".to_string()
    user_agent()
}