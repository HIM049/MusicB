use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub info: BiliInfo,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub cover_url: String,
    pub author: String,
    pub lyrics_path: String, // data.subtitle.list[0]. id / lan字幕语言 / lan_doc字幕语言名称 / is_lock / author_mid / subtitle_url
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upper {
    pub mid: i64,
    pub name: String,
    pub avatar: String,
}
