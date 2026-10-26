mod watcher;

use dotenvy::dotenv;
use std::{path::PathBuf};
use anyhow::{Context};
use tokio::sync::mpsc::channel;

mod naming;
mod uploader;

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    
    let account_name = &std::env::var("AZURE_STORAGE_ACCOUNT")
        .context("Error loading Azure storage account")?;
    let container_name = &std::env::var("AZURE_CONTAINER_NAME")
        .context("Error loading Azure container name")?;
    let watched_folder_name = &std::env::var("WATCHED_FOLDER")
        .context("Error loading watched folder")?;

    let (_, container_client) = uploader::connect_to_azure(account_name, container_name)
        .context("Failed to connect to Azure")?;

    let (tx, mut rx) = channel::<PathBuf>(100);

    let watched_folder = PathBuf::from(&watched_folder_name);

    let _watcher = watcher::start(tx, &watched_folder)?;

    while let Some(path) = rx.recv().await {
        if !path.is_file() {continue;}
        
        let blob_name = naming::blob_name_from_path(&path, &watched_folder_name);
        let result = match blob_name {
            Some(valid_blob_name) => uploader::upload_file(&container_client, &path, &valid_blob_name).await
                .with_context(|| format!("Error uploading file: {} from path: {}", valid_blob_name, path.as_os_str().to_string_lossy())),
            None => uploader::handle_bad_file(&path).await
                .with_context(|| format!("Error handling bad file from path: {}", path.as_os_str().to_string_lossy())),
        };
        if let Err(e) = result {
            eprintln!("{e:#}");
        }
    }
    eprintln!("Application closed");
    
    Ok(())
}