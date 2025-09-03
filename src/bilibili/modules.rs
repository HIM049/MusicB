use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::bilibili::utils::extract_title;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub info: BiliInfo,
    pub player_info: Option<PlayerInfo>,
    pub stream: Option<BiliStream>,
    pub flac_stream: Option<BiliStream>,
    pub meta: Option<Meta>,
    pub upper: Upper,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BiliInfo {
    pub aid: i64,
    pub bvid: String,
    pub cid: i64,
    pub title: String,
    pub pic: String,
    pub author: String,
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
                title: json["title"].as_str()?.to_string(),
                pic: json["pic"].as_str()?.to_string(),
                author: json["title"].as_str()?.to_string(),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    bgm_id: String,
    bgm_title: String,
    bgm_jump_url: String,
    subtitles: Option<Vec<Subtitle>>,
}

impl PlayerInfo {
    pub fn from_json(json: Value) -> Option<Self> {
        // Get list of subtitles
        let mut subtitles: Vec<Subtitle> = Vec::new();
        for subtitle_json in json["subtitle"]["subtitles"].as_array().unwrap() {
            if let Some(sub) = Subtitle::from_json(subtitle_json.clone()) {
        println!("单列表项{:?}", sub);

                if sub.subtitle_type == 0 {
                    // uploaded by user (not by ai)
                    subtitles.push(sub);
                }
            }
        }

        Some(
            Self { 
                bgm_id: json["bgm_info"]["music_id"].as_str()?.to_string(), 
                bgm_title: extract_title(json["bgm_info"]["music_title"].as_str()?)?, 
                bgm_jump_url: json["bgm_info"]["jump_url"].as_str()?.to_string(),
                subtitles: if subtitles.len() <= 0 {None} else {Some(subtitles)},
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subtitle {
    id: i64,
    lang: String,
    lang_doc: String,
    author_mid: i64,
    url: String,
    url_v2: String,
    subtitle_type: i64,
}

impl Subtitle {
    pub fn from_json(json: Value) -> Option<Self> {
        Some(
            Self { 
                id: json["id"].as_i64()?, 
                lang: json["lan"].as_str()?.to_string(), 
                lang_doc: json["lan_doc"].as_str()?.to_string(), 
                author_mid: json["author_mid"].as_i64()?,
                url: json["subtitle_url"].as_str()?.to_string(), 
                url_v2: json["subtitle_url_v2"].as_str()?.to_string(), 
                subtitle_type: json["type"].as_i64()?, 
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            codecs: if let Some(str) = json["codecs"].as_str() { str.to_string() } else { return None; },
            get_time: SystemTime::now(),
        };
        Some(stream)
    }

    pub fn get_stream_url(&self) -> Option<String> {
        let elapsed = self.get_time.elapsed().unwrap();
        if elapsed < Duration::from_secs(60 * 25) {
            return Some(self.base_url.clone());
        } else {
            return None;
        }
        
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// 
pub struct Meta {
    pub title: String,
    pub cover_url: String, // Turn to cover image?
    pub author: String,
    pub lyrics_path: String, // data.subtitle.list[0]. id / lan字幕语言 / lan_doc字幕语言名称 / is_lock / author_mid / subtitle_url
}

impl Meta {
    pub fn from_json(json: Value) -> Option<Self> {
        Some(
            //TODO: Waiting for refactor
            Meta {
                title: json["title"].as_str()?.to_string(),
                cover_url: json["pic"].as_str()?.to_string(),
                author: json["title"].as_str()?.to_string(),
                lyrics_path: "".to_string(),
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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