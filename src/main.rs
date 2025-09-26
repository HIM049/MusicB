// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// In order to be compatible with both desktop, wasm, and android, the example is both a binary and a library.
// Just forward to the library in main

mod bilibili;
mod frontend;
use std::{sync::Arc, vec};
use image::{ImageBuffer, Rgba};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use slint::{Image, SharedPixelBuffer, ModelRc, VecModel};
use bilibili::modules::Video;
use crate::{bilibili::{task_modules::Task}, frontend::load_image}; 


slint::include_modules!();

static APP_STATE: Lazy<Mutex<AppState>> = Lazy::new(|| {
    Mutex::new(AppState { current_item: None, task_list: vec![], temp_task_list: vec![], })
});

pub struct AppState {
    pub current_item: Option<Video>,
    pub task_list: Vec<Task>,
    pub temp_task_list: Vec<Task>,
}


#[tokio::main]
async fn main() {

    let ui = MainWindow::new().unwrap();
    let ui_handle = Arc::new(ui.as_weak());
    
    let handle = ui_handle.clone();
    ui.on_query_bili_info(move |query_type, input| {
        println!("on_query_bili_info");
        let ui = handle.clone();

        // run async functions
        tokio::spawn(async move {
            match query_type {
                0 => {} // video
                1 => {} // collect
                _ => {}
            }
            let video = Video::from_bvid(input.to_string()).await;
            let mut app_state = APP_STATE.lock().await;
            app_state.current_item = if let Ok(v) = video {Some(v)} else {None};

            let cover: Arc<ImageBuffer<Rgba<u8>, Vec<u8>>>;
            if let Some(item) = &app_state.current_item {
                cover = Arc::new(load_image(item.info.pic.clone()).await);
            } else {
                return;
            }
            
            if let Err(e) = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui.upgrade() {
                    if let Some(video) = app_state.current_item.clone() {
                        ui.invoke_query_bili_info_finish( QueryCardInfo { 
                            author: video.upper.name.into(), 
                            bvid: video.info.bvid.into(), 
                            count: video.info.videos as i32, 
                            cover: Image::from_rgba8(SharedPixelBuffer::clone_from_slice(&cover, cover.width(), cover.height())), 
                            title: video.info.title.into(),
                        })
                    }
                } else {
                    println!("failed to get weak");
                }
            }) {
                println!("{}", e);
            }
        });
    });

    let handle = ui_handle.clone();
    ui.on_add_to_temp_list(move || {
        println!("on_add_to_create");
        let ui = handle.clone();
        tokio::spawn(async move {
            let mut app_state = APP_STATE.lock().await;
            if let Some(video) = &app_state.current_item {
                app_state.temp_task_list = Task::from_video(video.clone());
            }

            let cover: Arc<ImageBuffer<Rgba<u8>, Vec<u8>>>;
            if let Some(item) = &app_state.current_item {
                cover = Arc::new(load_image(item.info.pic.clone()).await);
            } else {
                return;
            }

            if let Err(e) = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui.upgrade() {
                    let model: VecModel<ListItem> = VecModel::default();
                    for task in app_state.temp_task_list.clone() {
                            
                        model.push(ListItem { 
                            cover: Image::from_rgba8(SharedPixelBuffer::clone_from_slice(&cover, cover.width(), cover.height())), 
                            subtitle: "".into(), 
                            title: task.part_data.unwrap().title.into(), 
                        });
                    }

                    ui.invoke_add_to_temp_list_finish(ModelRc::new(model));
                }

            }) {
                println!("{}", e);
            }
        });
    });
    

    // ui.on_start_download(move || {
    //     tokio::spawn(async move {
    //         let app_state = APP_STATE.lock().await;
    //         if let Some(v) = &app_state.current_item {
    //             let video = v.get_stream().await.unwrap();
    //             video.download_stream().await.unwrap();
    //             println!("Download Finished")
    //         }
    
    //     });
    // });

    ui.run().expect("Failed to run the main window");

}