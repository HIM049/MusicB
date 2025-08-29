// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// In order to be compatible with both desktop, wasm, and android, the example is both a binary and a library.
// Just forward to the library in main

mod bilibili;
mod frontend;
use std::{collections::HashMap, sync::Arc};
use image::{ImageBuffer, Rgba};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use slint::{Image, SharedPixelBuffer, SharedString};
use bilibili::modules::Video;

use crate::{bilibili::video, frontend::load_image};


slint::include_modules!();

static APP_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| {
    Mutex::new(AppState { current_item: None })
});

pub struct AppState {
    pub current_item: Option<Video>,
}


#[tokio::main]
async fn main() {

    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_get_bili_information(move |bvid| {
        let ui = ui_handle.clone();

        tokio::spawn(async move {
                    
            let video = Video::from_bvid(bvid.to_string()).await;
            let mut app_state = APP_STATE.lock().await;
            app_state.current_item = if let Ok(v) = video {Some(v)} else {None};

            let cover: Arc<ImageBuffer<Rgba<u8>, Vec<u8>>>;
            if let Some(item) = &app_state.current_item {
                cover = Arc::new(load_image(item.meta.cover_url.clone()).await);
            } else {
                return;
            }
            
            if let Err(e) = slint::invoke_from_event_loop(move || {

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

    ui.on_start_download(move || {
        tokio::spawn(async move {
            let app_state = APP_STATE.lock().await;
            if let Some(v) = &app_state.current_item {
                let video = v.get_stream().await.unwrap();
                video.download_stream().await.unwrap();
                println!("Download Finished")
            }
    
        });
    });

    ui.run().expect("Failed to run the main window");

}

