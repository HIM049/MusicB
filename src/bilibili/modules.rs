use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub info: BasicInfo,
    pub meta: Meta,
    pub upper: Upper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicInfo {
    pub bvid: String,
    pub cid: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title:  String,
    pub cover_url: String,
    pub author: String,
    pub lyrics_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upper {
    pub mid: i32,
    pub name: String,
    pub avatar: String,
}

