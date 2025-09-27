

use anyhow::anyhow;
use reqwest::{header, Client};
use serde_json::Value;

use crate::bilibili::{
    downloader::stream_downloader, modules::{BiliInfo, BiliStream, PlayerInfo, Upper, Video, VideoPart}, utils, wbi_generater::{encode_wbi, get_wbi_keys}
};

impl Video {
    pub async fn from_bvid(
        bvid: String,
    ) -> Result<Video, anyhow::Error> {
        let video = get_video_details(QueryType::BVID, bvid).await?;
        Ok(video)
    }
    pub async fn from_avid(
        avid: String,
    ) -> Result<Video, anyhow::Error> {
        let video = get_video_details(QueryType::AVID, avid).await?;
        Ok(video)
    }
    pub async fn get_stream(&self) -> Result<Self, anyhow::Error> {
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
    pub async fn get_player_info(&self, bvid: String, cid: i64) -> Result<Self, anyhow::Error> {
        let player_info = get_player_information(bvid, cid, "".to_string()).await?;
        Ok(
            Self {
                player_info: Some(player_info),
                ..self.clone()
            }
        )
    }
    pub async fn download_stream(&self) -> Result<(), anyhow::Error> {
        let stream_url: String;
        if let Some(flac_stream) = &self.flac_stream {
            if let Some(url) = flac_stream.get_stream_url() {
                stream_url = url;
            } else {
                // return Err("URL expired (timeout)".into());
                return Err(anyhow!("URL expired (timeout)"));
            }
        } else if let Some(stream) = &self.stream {
            if let Some(url) = stream.get_stream_url() {
                stream_url = url;
            } else {
                return Err(anyhow!("URL expired (timeout)"));
            }
        } else {
                return Err(anyhow!("Invalid stream URLs"));
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
) -> Result<Video, anyhow::Error> {
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
        return Err(anyhow!(format!(
            "Failed when request: {}",
            json_resp["message"].as_str().unwrap()
        )));
        
    }

    // Write data into struct
    let err_msg = "failed when deserialization response";
    let video = Video {
        info: BiliInfo::from_json(json_resp["data"].clone()).ok_or(anyhow!(err_msg))?,
        player_info: None,
        stream: None,
        flac_stream: None,
        upper: Upper::from_json(json_resp["data"]["owner"].clone()).ok_or(anyhow!(err_msg))?,
        parts: VideoPart::from_json_array(json_resp["data"]["pages"].clone()),
    };

    Ok(video)
}

// get web player data
async fn get_player_information(bvid: String, cid: i64, sessdata: String) -> Result<PlayerInfo, anyhow::Error> {
    // Create client
    let client = Client::new();
    let wbi = get_wbi_keys().await.unwrap(); // TODO: Move to global

    // Write params and generate wbi
    let params = encode_wbi(
        vec![
            ("bvid", bvid),
            ("cid", cid.to_string()),
        ],
        wbi,
    );

    // Write into url and send request
    let url = "https://api.bilibili.com/x/player/wbi/v2";
    let resp = client
        .get(format!("{}?{}", url, params))
        .header(header::REFERER, "https://www.bilibili.com")
        .header(header::USER_AGENT, utils::get_user_agent())
        .header(header::COOKIE, format!("SESSDATA={}", sessdata))
        .send()
        .await?;

    let json_resp: Value = serde_json::from_str(resp.text().await?.as_str()).unwrap();

    Ok(PlayerInfo::from_json(json_resp["data"].clone()).ok_or(anyhow!("failed when deserialization response"))?)
}

// API docs https://socialsisteryi.github.io/bilibili-API-collect/docs/video/videostream_url.html#%E8%8E%B7%E5%8F%96%E8%A7%86%E9%A2%91%E6%B5%81%E5%9C%B0%E5%9D%80-web%E7%AB%AF
async fn get_video_stream(
    bvid: String,
    cid: i64,
) -> Result<(Option<BiliStream>, Option<BiliStream>), anyhow::Error> {
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
        return Err(anyhow!(format!(
            "Failed when request: {}",
            json_resp["message"].as_str().unwrap()
        )));
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