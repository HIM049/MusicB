use reqwest::{Client, Request};
use serde_json::Value;

use crate::bilibili::{modules::{BiliInfo, Meta, Upper, Video}, wbi::{encode_wbi, get_wbi_keys}};
// #[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
// #[repr(i32)]
// enum RespCode {
//     Success = 0,
//     Error = -400,
//     PremissionDenied = -403,
    
// }

impl Video {
    pub async fn from_bvid(bvid: String) -> Result<Video, reqwest::Error> {
        let video = get_video_details(QueryType::BVID, bvid).await?;
        Ok(video)
    }
    pub async fn from_avid(avid: String) -> Result<Video, reqwest::Error> {
        let video = get_video_details(QueryType::AVID, avid).await?;
        Ok(video)
    }
}

enum QueryType {
    BVID,
    AVID,
}

async fn get_video_details(query_type: QueryType, id: String) -> Result<Video, reqwest::Error> {
    let client = Client::new();
    let wbi = get_wbi_keys().await.unwrap(); // TODO: Move to global & add a button to refresh

    let params = match query_type {
        QueryType::BVID => encode_wbi(vec![("bvid", id)], wbi),
        QueryType::AVID => encode_wbi(vec![("aid", id)], wbi),
    };

    let resp = client.get(format!("https://api.bilibili.com/x/web-interface/wbi/view?{}", params)).send().await?;
    let json_resp: Value = serde_json::from_str(resp.text().await?.as_str()).unwrap();
    
    let video = Video {
        info: BiliInfo {
            aid: json_resp["data"]["aid"].as_i64().unwrap(),
            bvid: json_resp["data"]["bvid"].to_string(),
            cid: json_resp["data"]["cid"].as_i64().unwrap(),
            pic: json_resp["data"]["pic"].to_string(),
            videos: json_resp["data"]["videos"].as_i64().unwrap(),
            tid: json_resp["data"]["tid"].as_i64().unwrap(),
            tid_v2: json_resp["data"]["tid_v2"].as_i64().unwrap(),
            tname: json_resp["data"]["tname"].to_string(),
            tname_v2: json_resp["data"]["tname_v2"].to_string(),
            pubdate: json_resp["data"]["pubdate"].as_i64().unwrap(),
            desc: json_resp["data"]["desc"].to_string(),
        },
        meta: Meta {
            title: json_resp["data"]["title"].to_string(),
            cover_url: json_resp["data"]["pic"].to_string(),
            author: json_resp["data"]["title"].to_string(),
            lyrics_path: "".to_string(), //TODO
        },
        upper: Upper {
            mid: json_resp["data"]["owner"]["mid"].as_i64().unwrap(),
            name: json_resp["data"]["owner"]["name"].to_string(),
            avatar: json_resp["data"]["owner"]["face"].to_string(),
        },
    };
    
    Ok(video)

}