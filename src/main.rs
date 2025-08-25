// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// In order to be compatible with both desktop, wasm, and android, the example is both a binary and a library.
// Just forward to the library in main

mod bilibili;
use slint::SharedString;
use bilibili::modules::Video;


slint::include_modules!();


#[tokio::main]
async fn main() {

    let ui = MainWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_get_bili_information(move |bvid| {
        let ui = ui_handle.clone();

        tokio::spawn(async move {
            let video = Video::from_bvid(bvid.to_string()).await.unwrap();

            if let Err(e) = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui.upgrade() {
                    ui.set_videoTitle(SharedString::from(video.meta.title));
                    ui.set_showInfo(true);
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

