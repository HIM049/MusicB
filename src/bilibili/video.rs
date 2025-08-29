

use reqwest::{header, Client};
use serde_json::Value;

use crate::bilibili::{
    downloader::stream_downloader, modules::{BiliInfo, BiliStream, Meta, Upper, Video}, utils, wbi_generater::{encode_wbi, get_wbi_keys}
};

impl Video {
    pub async fn from_bvid(
        bvid: String,
    ) -> Result<Video, Box<dyn std::error::Error + Send + Sync>> {
        let video = get_video_details(QueryType::BVID, bvid).await?;
        Ok(video)
    }
    pub async fn from_avid(
        avid: String,
    ) -> Result<Video, Box<dyn std::error::Error + Send + Sync>> {
        let video = get_video_details(QueryType::AVID, avid).await?;
        Ok(video)
    }
    pub async fn get_stream(&self) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let (stream, flac) =
            get_video_stream(self.info.bvid.clone(), self.info.cid.clone()).await?;
        Ok(
            Video {
                stream: stream,
                flac_stream: flac,
                ..self.clone()
            }
        )
    }
    pub async fn download_stream(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let stream_url: String;
        if let Some(flac_stream) = &self.flac_stream {
            if let Some(url) = flac_stream.get_stream_url() {
                stream_url = url;
            } else {
                return Err("URL expired (timeout)".into());
            }
        } else if let Some(stream) = &self.stream {
            if let Some(url) = stream.get_stream_url() {
                stream_url = url;
            } else {
                return Err("URL expired (timeout)".into());
            }
        } else {
                return Err("Invalid stream URLs".into());
        }
        stream_downloader(stream_url, "C:\\Users\\HIM~\\Desktop\\resp.m4a".to_string()).await?;

        Ok(())
    }
}

enum QueryType {
    BVID,
    AVID,
}

async fn get_video_details(
    query_type: QueryType,
    id: String,
) -> Result<Video, Box<dyn std::error::Error + Send + Sync>> {
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
    let resp = client
        .get(format!("{}?{}", url, params))
        .header(header::REFERER, "https://www.bilibili.com")
        .header(header::USER_AGENT, utils::get_user_agent())
        .send()
        .await?;
    let json_resp: Value = serde_json::from_str(resp.text().await?.as_str()).unwrap();

    // Check response code
    if json_resp["code"].as_i64().unwrap_or(1) != 0 {
        return Err(format!(
            "Failed when request: {}",
            json_resp["message"].as_str().unwrap()
        )
        .into());
    }

    // Write data into struct
    let err_msg = "failed when deserialization response";
    let video = Video {
        info: BiliInfo::from_json(json_resp["data"].clone()).ok_or(err_msg)?,
        stream: None,
        flac_stream: None,
        meta: Meta::from_json(json_resp["data"].clone()).ok_or(err_msg)?,
        upper: Upper::from_json(json_resp["data"]["owner"].clone()).ok_or(err_msg)?,
    };

    Ok(video)
}

// API docs https://socialsisteryi.github.io/bilibili-API-collect/docs/video/videostream_url.html#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E6%B5%81%E5%9C%B0%E5%9D%80-web%E7%AB%AF
async fn get_video_stream(
    bvid: String,
    cid: i64,
) -> Result<(Option<BiliStream>, Option<BiliStream>), Box<dyn std::error::Error + Send + Sync>> {
    // Create client
    let client = Client::new();
    let wbi = get_wbi_keys().await.unwrap(); // TODO: Move to global

    // Write params and generate wbi
    let params = encode_wbi(
        vec![
            ("bvid", bvid),
            ("cid", cid.to_string()),
            ("gaia_source", "view-card".to_string()), // 无cookie时需要
            ("fnval", "16".to_string()),              // For DASH
            ("platform", "pc".to_string()),
        ],
        wbi,
    );

    // Write into url and send request
    let url = "https://api.bilibili.com/x/player/wbi/playurl";
    let resp = client
        .get(format!("{}?{}", url, params))
        .header(header::REFERER, "https://www.bilibili.com")
        .header(header::USER_AGENT, utils::get_user_agent())
        .send()
        .await?;

    let json_resp: Value = serde_json::from_str(resp.text().await?.as_str()).unwrap();

    // Check response code
    if json_resp["code"].as_i64().unwrap_or(1) != 0 {
        return Err(format!(
            "Failed when request: {}",
            json_resp["message"].as_str().unwrap()
        )
        .into());
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