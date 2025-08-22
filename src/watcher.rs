mod modules;
use std::{fs::OpenOptions, path::Path, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex, MutexGuard}, thread};
use notify::{Event, RecursiveMode, Result, Watcher};
use serde::{Deserialize, Serialize};
use std::io::Write;


pub fn start_watcher(stop_rx: Arc<Mutex<Receiver<()>>>) {
    thread::spawn(move || {
        let rx = stop_rx.lock().unwrap();
        let _ = watch_dir(rx);
    });
}

pub fn stop_watcher(stop_tx: MutexGuard<'_, Sender<()>>) {
    stop_tx.send(()).unwrap();
}

fn watch_dir(stop_rx: MutexGuard<'_, Receiver<()>>) -> Result<()> {
    // Init channel & watcher
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;

    // Watch paths
    let paths = ["D:/Project/Test/TestPath"];
    for path in paths {
        watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
    }

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("changes_log.jsonl")
        .unwrap();

    loop {
        // Stop thread when receve
        if let Ok(_) = stop_rx.try_recv() {
            break;
        }

        match rx.recv() {
            Ok(Ok(event)) => {
                match event.kind {
                    notify::EventKind::Any => todo!(),
                    notify::EventKind::Access(access_kind) => todo!(),
                    notify::EventKind::Create(create_kind) => {
                        let record = modules::FileChange {
                            kind: format!("{:?}", event.kind),
                            path: event.paths[0].to_string_lossy().replace("\\", "/"),
                        };
                        let json = serde_json::to_string(&record).unwrap();
                        writeln!(log_file, "{}", json).unwrap();
                    },
                    notify::EventKind::Modify(modify_kind) => todo!(),
                    notify::EventKind::Remove(remove_kind) => todo!(),
                    notify::EventKind::Other => todo!(),
                }

            }
            Ok(Err(e)) => eprint!("Error: {:?}", e),
            Err(e) => eprint!("Error: {:?}", e),
        }
    }
    Ok(())
}