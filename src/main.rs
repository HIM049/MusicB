// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// In order to be compatible with both desktop, wasm, and android, the example is both a binary and a library.
// Just forward to the library in main

// fn main() {
//     gallery_lib::main();
// }
mod modules;
use std::{fs::File, io::{BufRead, BufReader}, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}};

use slint::{ModelRc, VecModel};

mod watcher;

slint::include_modules!();


fn main() {
    let (stop_tx, stop_rx) = mpsc::channel::<()>();
    let tx_clone = Arc::new(Mutex::new(stop_tx));
    let rx_clone = Arc::new(Mutex::new(stop_rx));

    let app = MainWindow::new().unwrap();
    app.on_get_rust_value(|| {
        let mut cards: Vec<slint::SharedString> = Vec::new();
        for i in 0..10 {
            cards.push(format!("File{}", i).into());
        }

        ModelRc::new(VecModel::from(cards))
    });

    app.on_switch_watcher(move |enable: bool| -> bool {
       watcher_handler(enable, tx_clone.clone(), rx_clone.clone());
       enable
    });

    app.run().expect("Failed to run the main window");
}

fn watcher_handler(enable: bool, tx_clone:Arc<Mutex<Sender<()>>>, rx_clone: Arc<Mutex<Receiver<()>>>) {
    if enable {
        watcher::start_watcher(rx_clone.clone());
    } else {
        watcher::stop_watcher(tx_clone.lock().unwrap());
    }
}

fn read_record() -> std::io::Result<()> {
    let input = File::open("")?;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
    }

    let task: modules::FileChange = serde_json::from_str(&line).unwrap();

}
