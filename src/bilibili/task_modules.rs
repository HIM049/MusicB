use serde::{Deserialize, Serialize};

use crate::bilibili::modules::{Video, VideoPart};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskType {
    Video,
    Audio
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub task_type: TaskType,
    pub video: Video,
    pub part_data: Option<VideoPart>,
}

impl Task {
    pub fn from_video(video: Video) -> Vec<Self> {
        let mut task_list: Vec<Self> = vec![];
        // create jobs for every part
        for part in video.parts.clone() {
            task_list.push( 
                Self { 
                    task_type: TaskType::Video,
                    video: video.clone(),
                    part_data: Some(part),
                }
            );
        }
        task_list
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub song_name: String,
    pub cover_url: String, // Turn to cover image?
    pub author: String,
    pub lyrics_path: String, // data.subtitle.list[0]. id / lan字幕语言 / lan_doc字幕语言名称 / is_lock / author_mid / subtitle_url
    // singer, from ...
}

impl Meta {
    pub fn from_video(video: Video) -> Option<Self> {
        None
    }
}

// impl Meta {
//     pub fn from_json(json: Value) -> Option<Self> {
//         Some(
//             //TODO: Waiting for refactor
//             Meta {
//                 title: json["title"].as_str()?.to_string(),
//                 cover_url: json["pic"].as_str()?.to_string(),
//                 author: json["title"].as_str()?.to_string(),
//                 lyrics_path: "".to_string(),
//             }
//         )
//     }
// }