// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// In order to be compatible with both desktop, wasm, and android, the example is both a binary and a library.
// Just forward to the library in main

mod bilibili;
mod frontend;
mod handlers;
mod event_loop;
mod app_state;
use std::{sync::Arc};
use tokio::sync::mpsc;
use crate::event_loop::{event_loop, AppEvent}; 


slint::include_modules!();

#[tokio::main]
async fn main() {

    let ui = MainWindow::new().unwrap();
    let ui_handle = Arc::new(ui.as_weak());
    

    let (_tx, rx) = mpsc::unbounded_channel::<AppEvent>();
    let tx = _tx.clone();
    ui.on_on_goto_create_task(move || {
        tx.send(AppEvent::SetDownloadViewIndex(1)).unwrap();
    });

    let tx = _tx.clone();
    ui.on_download_back_clicked(move || {
        tx.send(AppEvent::SetDownloadViewIndex(0)).unwrap();
    });

    let tx = _tx.clone();
    ui.on_query_bili_info(move |query_type, input| {
        println!("on_query_bili_info");
        tx.send(AppEvent::QueryBiliInfo(input.to_string(), query_type)).unwrap();
    });

    // Start event loop
    let handle = ui_handle.clone();
    tokio::spawn(event_loop(rx, handle));

    // ui.on_query_bili_info(move |query_type, input| {
    //     println!("on_query_bili_info");
    //     let ui = handle.clone();
    //     // run async functions
    //     tokio::spawn(async move {
    //         let response = query_bili_info(input.to_string(), query_type).await;
    //         match response {
    //             Ok((video, image_buf)) => {
    //                 if let Err(e) = slint::invoke_from_event_loop(move || {
    //                     if let Some(ui) = ui.upgrade() {
    //                         ui.invoke_query_bili_info_finish(handle_video_info(video, image_buf));
    //                     } else {
    //                         println!("failed to get weak");
    //                     }
    //                 }) { println!("{}", e); }
    //             }
    //             Err(e) => {println!("{}", e);}
    //         }
    //     });
    // });

    // let handle = ui_handle.clone();
    // ui.on_add_to_temp_list(move || {
    //     println!("on_add_to_create");
    //     let ui = handle.clone();
    //     tokio::spawn(async move {
    //         let mut app_state = APP_STATE.lock().await;
    //         if let Some(video) = &app_state.current_item {
    //             app_state.temp_task_list = Task::from_video(video.clone());
    //         }

    //         let cover: Arc<ImageBuffer<Rgba<u8>, Vec<u8>>>;
    //         if let Some(item) = &app_state.current_item {
    //             cover = Arc::new(load_image(item.info.pic.clone()).await);
    //         } else {
    //             return;
    //         }

    //         if let Err(e) = slint::invoke_from_event_loop(move || {
    //             if let Some(ui) = ui.upgrade() {
    //                 let model: VecModel<ListItem> = VecModel::default();
    //                 for task in app_state.temp_task_list.clone() {
                            
    //                     model.push(ListItem { 
    //                         cover: Image::from_rgba8(SharedPixelBuffer::clone_from_slice(&cover, cover.width(), cover.height())), 
    //                         subtitle: "".into(), 
    //                         title: task.part_data.unwrap().title.into(), 
    //                     });
    //                 }

    //                 ui.invoke_add_to_temp_list_finish(ModelRc::new(model));
    //             }

    //         }) {
    //             println!("{}", e);
    //         }
    //     });
    // });
    

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
    // tokio::spawn(future);

    ui.run().expect("Failed to run the main window");

}
