use anyhow::anyhow;
use image::{ImageBuffer, Rgba};
use slint::{Image, SharedPixelBuffer};

use crate::{bilibili::modules::Video, frontend::load_image, QueryCardInfo};


pub async fn query_bili_info(input: String, query_type: i32) -> Result<(Video, ImageBuffer<Rgba<u8>, Vec<u8>>), anyhow::Error> {
    match query_type {
        // Video
        0 => {
            let video = Video::from_bvid(input).await?;
            
            // Parse image url
            let image_buf = load_image(video.info.pic.clone()).await;
            Ok((video, image_buf))
        }
        _ => {Err(anyhow!("invalid query_type"))}
    }
}

pub fn handle_video_info(video: Video, image_buf: ImageBuffer<Rgba<u8>, Vec<u8>>) -> QueryCardInfo {
    let cover_image = Image::from_rgba8(
        SharedPixelBuffer::clone_from_slice(&image_buf, image_buf.width(), image_buf.height())
    );

    QueryCardInfo{ 
        author: video.upper.name.into(), 
        bvid: video.info.bvid.into(), 
        count: video.info.videos as i32, 
        cover: cover_image, 
        title: video.info.title.into(),
    }
}