// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// In order to be compatible with both desktop, wasm, and android, the example is both a binary and a library.
// Just forward to the library in main

mod bilibili;
mod frontend;
use std::sync::Arc;

use slint::{Image, SharedPixelBuffer, SharedString};
use bilibili::modules::Video;

use crate::frontend::load_image;


slint::include_modules!();


#[tokio::main]
async fn main() {

    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_get_bili_information(move |bvid| {
        let ui = ui_handle.clone();

        tokio::spawn(async move {
            let mut video = Video::from_bvid(bvid.to_string()).await.unwrap();
            let cover = Arc::new(load_image("http://i0.hdslb.com/bfs/archive/026405c07be8d948ec294658651b51e7f3fd7932.jpg".to_string()).await);
            video.get_stream().await.unwrap();
            // video.download_stream().await.unwrap();
            println!("video: {:?}", video); //TODO: Remove it
            


            if let Err(e) = slint::invoke_from_event_loop(move || {
                
                // let video_info = VideoInfo { 
                //     author: video.upper.name.into(), 
                //     count: 1, 
                //     cover: Image::from_rgb8(SharedPixelBuffer::clone_from_slice(&cover, cover.width(), cover.height())), 
                //     title: video.meta.title.into(), 
                // };

                if let Some(ui) = ui.upgrade() {
                    // ui.set_videoInfo(video_info);
                    ui.set_showInfo(true);
                    ui.set_inputImage(Image::from_rgba8(SharedPixelBuffer::clone_from_slice(&cover, cover.width(), cover.height())));
                } else {
                    println!("failed to get weak");
                }
            }) {
                println!("{}", e);
            }

        });
    });

    ui.run().expect("Failed to run the main window");

}

