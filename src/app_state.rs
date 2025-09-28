
use image::{ImageBuffer, Rgba};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::bilibili::{modules::Video, task_modules::Task};

pub static APP_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| {
    Mutex::new(AppState {
         current_item: None, 
         task_list: vec![], 
         temp_task_list: vec![], 
         views: Views::default()
        })
});

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_item: Option<Video>,
    pub task_list: Vec<Task>,
    pub temp_task_list: Vec<Task>,
    pub views: Views,
}

#[derive(Debug, Clone, Default)]
pub struct Views {
    pub download_view: DownloadView,
}

#[derive(Debug, Clone, Default)]
pub struct DownloadView {
    pub download_view_index: i32,
    pub create_task_view_index: i32,
    pub query_is_info_showing: bool,
    pub query_is_querying: bool,
    pub query_card_info: QueryCardInfoRust,
}

#[derive(Debug, Clone)]
pub struct QueryCardInfoRust {
    pub bvid: String,
    pub title: String,
    pub cover: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub author: String,
    pub count: i32,
}

impl Default for QueryCardInfoRust {
    fn default() -> Self {
        Self { bvid: "".to_string(), title: "".to_string(), cover: ImageBuffer::new(0, 0), author: "".to_string(), count: 0 }
    }
}