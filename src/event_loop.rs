use std::sync::{Arc};

use slint::{Image, SharedPixelBuffer, Weak};
use tokio::sync::{mpsc};

use crate::{app_state::{AppState, QueryCardInfoRust, APP_STATE}, bilibili::video, handlers::query_bili_info, MainWindow, QueryCardInfo};

pub enum AppEvent {
    QueryBiliInfo(String, i32)
}

pub async fn event_loop(mut rx: mpsc::UnboundedReceiver<AppEvent>, ui_weak: Arc<Weak<MainWindow>>) {
    println!("event_loop");
    while let Some(event) = rx.recv().await {
        match event {
            AppEvent::QueryBiliInfo(input, query_type) => {
                println!("query_bili_info_event");
                // set querying state
                let mut app_state = APP_STATE.lock().await;
                app_state.views.download_view.query_is_querying = true;
                push_state(ui_weak.clone(), app_state.clone());

                // get and write data
                if let Ok((video, image_buf)) = query_bili_info(input, query_type).await {
                    app_state.views.download_view.query_card_info = QueryCardInfoRust {
                        bvid: video.info.bvid,
                        title: video.info.title,
                        cover: image_buf,
                        author: video.upper.name,
                        count: video.info.videos as i32,
                    };
                    app_state.views.download_view.query_is_info_showing = true;
                } else {
                    // TODO: failed to get (error msg)
                }

                // set state
                app_state.views.download_view.query_is_querying = false;
                // push state
                push_state(ui_weak.clone(), app_state.clone());
            },
        }
    }
}

pub fn push_state(ui_weak: Arc<Weak<MainWindow>>, app_state: AppState) {
    println!("push_state");

    if let Err(e) = slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let query_card_info = app_state.views.download_view.query_card_info.clone();
            ui.set_query_card(QueryCardInfo{
                author: query_card_info.author.into(),
                bvid: query_card_info.bvid.into(),
                count: query_card_info.count,
                title: query_card_info.title.into(),
                cover: Image::from_rgba8(SharedPixelBuffer::clone_from_slice(
                    &query_card_info.cover, query_card_info.cover.width(), query_card_info.cover.height()
                )),
            });
            ui.set_query_is_info_showing(app_state.views.download_view.query_is_info_showing);
            ui.set_query_is_querying(app_state.views.download_view.query_is_querying);
        } else {
            println!("failed to update ui");
        }
    }) {println!("{}", e)}

}