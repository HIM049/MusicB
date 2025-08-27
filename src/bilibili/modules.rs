use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub info: BiliInfo,
    pub stream: Option<BiliStream>,
    pub flac_stream: Option<BiliStream>,
    pub meta: Meta,
    pub upper: Upper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiliInfo {
    pub aid: i64,
    pub bvid: String,
    pub cid: i64,
    pub pic: String,
    pub videos: i64,      // Patrs count
    pub tid: i64,         // 分区信息
    pub tid_v2: i64,      // 分区信息
    pub tname: String,    // 子分区名称
    pub tname_v2: String, // 子分区名称
    pub pubdate: i64,     // publish time (sec)
    pub desc: String,     // 简介
}

impl BiliInfo {
    pub fn from_json(json: Value) -> Option<BiliInfo> {
        Some(
            BiliInfo {
                aid: json["aid"].as_i64()?,
                bvid: json["bvid"].as_str()?.to_string(),
                cid: json["cid"].as_i64()?,
                pic: json["pic"].as_str()?.to_string(),
                videos: json["videos"].as_i64()?,
                tid: json["tid"].as_i64()?,
                tid_v2: json["tid_v2"].as_i64()?,
                tname: json["tname"].as_str()?.to_string(),
                tname_v2: json["tname_v2"].as_str()?.to_string(),
                pubdate: json["pubdate"].as_i64()?,
                desc: json["desc"].as_str()?.to_string(),
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiliStream {
    pub quality: AudioQuality,
    pub base_url: String,
    pub codecs: String,
    pub get_time: SystemTime,
}

impl BiliStream {
    pub fn from_json(json: Value) -> Option<Self> {
        let stream = BiliStream {
            quality: AudioQuality::from_num(if let Some(num) = json["id"].as_i64() { num } else { return None; }),
            base_url: if let Some(str) = json["base_url"].as_str() { str.to_string() } else { return None; },
            codecs: if let Some(str) = json["base_url"].as_str() { str.to_string() } else { return None; },
            get_time: SystemTime::now(),
        };
        Some(stream)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub cover_url: String,
    pub author: String,
    pub lyrics_path: String, // data.subtitle.list[0]. id / lan字幕语言 / lan_doc字幕语言名称 / is_lock / author_mid / subtitle_url
}

impl Meta {
    pub fn from_json(json: Value) -> Option<Self> {
        Some(
            Meta {
            title: json["title"].as_str()?.to_string(),
            cover_url: json["pic"].as_str()?.to_string(),
            author: json["title"].as_str()?.to_string(),
            lyrics_path: "".to_string(), //TODO
        }
    )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upper {
    pub mid: i64,
    pub name: String,
    pub avatar: String,
}

impl Upper {
    pub fn from_json(json: Value) -> Option<Self> {
        Some(
            Upper {
                mid: json["mid"].as_i64()?,
                name: json["name"].as_str()?.to_string(),
                avatar: json["face"].as_str()?.to_string(),
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AudioQuality {
    Unknown,
    Low64K,
    Medium132K,
    High192K,
    Dolby,
    HiRes,
}

impl AudioQuality {
    pub fn from_num(quality: i64) -> AudioQuality {
        match quality {
            30216 => AudioQuality::Low64K,
            30232 =>AudioQuality::Medium132K,
            30280 => AudioQuality::High192K,
            30250 => AudioQuality::Dolby,
            30251 => AudioQuality::HiRes,
            _ => AudioQuality::Unknown,
        }
    }
}