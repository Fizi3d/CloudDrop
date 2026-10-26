use azure_core::{stream::DEFAULT_BUFFER_SIZE, http::{Url,Body}};
use azure_storage_blob::{BlobServiceClient, BlobContainerClient, stream::tokio::FileStream};
use azure_identity::DeveloperToolsCredential;
use anyhow::{Context};
//use tokio::fs;

pub fn connect_to_azure(account_name: &str, container_name: &str) -> anyhow::Result<(BlobServiceClient, BlobContainerClient)> {

    let service_url = Url::parse(&format!("https://{}.blob.core.windows.net", account_name))?;
    let credential = DeveloperToolsCredential::new(None)?;
    let service_client = BlobServiceClient::new(service_url, Some(credential), None)?;
    let container_client = service_client.blob_container_client(container_name);
    Ok((service_client, container_client))
}

#[allow(unused_variables)]
pub async fn upload_file(container_client: &BlobContainerClient, path: &std::path::Path, blob_name: &str) -> anyhow::Result<()> {

    let client = container_client.blob_client(blob_name);
    let file_size = std::fs::metadata(path)?.len();

    let content: Body = if file_size <= DEFAULT_BUFFER_SIZE as u64 {
        tokio::fs::read(path).await
            .context(format!("Failed to read {}",path.as_os_str().to_string_lossy()))?
            .into()
    }
    else {
        let file = tokio::fs::File::open(path).await
        .context(format!("Failed to open large file: {}", path.as_os_str().to_string_lossy()))?;
        let mut builder = FileStream::builder(file);
        builder = builder.with_buffer_size(DEFAULT_BUFFER_SIZE);
        builder.build().await.context(format!("Failed to build stream for large file: {}", path.as_os_str().to_string_lossy()))?.into()
    };

    client.upload(content.into(), None).await.context(format!("Failed to upload body for file: {}", path.as_os_str().to_string_lossy()))?;

    

    Ok(())
}

#[allow(unused_variables)]
pub async fn handle_bad_file(path: &std::path::Path) -> anyhow::Result<()> {

    println!("would handle bad file: {}", path.as_os_str().to_string_lossy());

    Ok(())
}
    