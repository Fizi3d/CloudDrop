use std::{time::Duration, path::{Path, PathBuf}};
use notify_debouncer_full::{DebounceEventResult, Debouncer, new_debouncer, FileIdMap, notify::{ReadDirectoryChangesWatcher, RecursiveMode, EventKind}};
use anyhow::{Context, Result};
use tokio::sync::mpsc::Sender;



pub fn start(tx: Sender<PathBuf>, watch_path: &Path) -> Result<Debouncer<ReadDirectoryChangesWatcher, FileIdMap>> {


    let mut debounced_watcher = new_debouncer(
        Duration::from_secs(2),
        None, 
        move |event_result: DebounceEventResult| { match event_result {
            Ok(events) => {
                for event in events {
                    if matches!(event.event.kind, EventKind::Create(_)) {
                        for path in event.event.paths {
                            if path.is_file() {
                                tx.blocking_send(path).unwrap();
                            }
                        }
                    }
                }
            }
            Err(errors) => errors.iter().for_each(|e| eprintln!("watch error: {:?}", e)),
        }
    }).context("Failed to create debounced watcher")?;

    debounced_watcher.watch(watch_path, RecursiveMode::Recursive)
        .context("Failed to start watching the specified path")?;
    anyhow::Ok(debounced_watcher)
}