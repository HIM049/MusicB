use reqwest::{Client, Request};
use serde_json::Value;

use crate::bilibili::modules::{BasicInfo, Meta, Upper, Video};
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(i32)]
// enum RespCode {
//     Success = 0,
//     Error = -400,
//     PremissionDenied = -403,
    
// }

impl Video {
    pub async fn from_bvid(bvid: String) -> Result<Video, reqwest::Error> {
        let client = Client::new();

        let resp = client.get("https://api.bilibili.com/x/web-interface/view")
            .query(&[("bvid", &bvid)])
            .send()
            .await?;

        let json_resp: Value = serde_json::from_str(resp.text().await?.as_str()).unwrap();
        
        let video = Video {
            info: BasicInfo {
                bvid,
                cid: json_resp["data"]["cid"].as_i64().unwrap().try_into().unwrap(),
            },
            meta: Meta {
                title: json_resp["data"]["title"].to_string(),
                cover_url: json_resp["data"]["pic"].to_string(),
                author: json_resp["data"]["title"].to_string(),
                lyrics_path: "".to_string(), //TODO
            },
            upper: Upper {
                mid: json_resp["data"]["owner"]["mid"].as_i64().unwrap().try_into().unwrap(),
                name: json_resp["data"]["owner"]["name"].to_string(),
                avatar: json_resp["data"]["owner"]["face"].to_string(),
            },
        };
        
        Ok(video)
    }
}