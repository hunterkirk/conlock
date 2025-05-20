use std::{
    ffi::OsString,
    os::unix::ffi::{OsStringExt, OsStrExt}, // <-- add this line
    path::{PathBuf},
    sync::{Arc, Mutex},
};


use inotify::{EventMask, Inotify, WatchMask};
use tokio::{sync::mpsc, task};
use anyhow::Result;
//use std::os::unix::ffi::OsStrExt;

struct Event<T> {
    wd: inotify::WatchDescriptor,
    mask: EventMask,
    cookie: u32,
    name: Option<T>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let watch_dir = PathBuf::from(".");

    // Initialize inotify inside a Mutex for interior mutability.
    let inotify = Arc::new(Mutex::new(Inotify::init()?));

    // Add watch using the recommended method.
    {
        let inotify_guard = inotify.lock().unwrap();
        inotify_guard
            .watches()
            .add(&watch_dir, WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE)?;
    }

    let inotify_thread = Arc::clone(&inotify);
    let buffer_size = 4096;

    let (tx, mut rx) = mpsc::channel(32);

    // Spawn blocking thread to read inotify events.
    task::spawn_blocking(move || -> Result<()> {
        let mut buffer = vec![0u8; buffer_size];

        loop {
            let events: Vec<Event<OsString>> = inotify_thread
                .lock()
                .unwrap()
                .read_events_blocking(&mut buffer)?
                .map(|event| Event {
                    wd: event.wd,
                    mask: event.mask,
                    cookie: event.cookie,
                    name: event.name.map(|name| {
                        OsString::from_vec(name.as_bytes().to_vec())
                    }),
                })
                .collect();

            for event in events {
                if tx.blocking_send(event).is_err() {
                    break;
                }
            }
        }

        #[allow(unreachable_code)]
        Ok(())
    });

    // Process events
    while let Some(event) = rx.recv().await {
        let dir_path = watch_dir.clone();
        let name = event.name.unwrap_or_else(|| OsString::from("unknown"));
        println!(
            "Change detected in directory {:?}: file {:?}, mask: {:?}",
            dir_path,
            name,
            event.mask
        );
    }

    Ok(())
}
