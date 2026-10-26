use azure_core::http::{/*RequestContent, */Url};
use azure_storage_blob::{BlobServiceClient, BlobContainerClient};
use azure_identity::DeveloperToolsCredential;
//use tokio::fs;

pub fn connect_to_azure(account_name: &String, container_name: &String) -> anyhow::Result<(BlobServiceClient, BlobContainerClient)> {

    let service_url = Url::parse(&format!("https://{}.blob.core.windows.net", account_name))?;
    let credential = DeveloperToolsCredential::new(None)?;
    let service_client = BlobServiceClient::new(service_url, Some(credential), None)?;
    let container_client = service_client.blob_container_client(container_name);
    Ok((service_client, container_client))
}

#[allow(unused_variables)]
pub async fn upload_file(container_client: &BlobContainerClient, path: &std::path::Path, blob_name: &str) -> anyhow::Result<()> {
    /*let blob_client = container_client.blob_client(blob_name);
    
    let file_stream = tokio::fs::File::open(path).await.context("Failed to open file for upload")?;*/

    println!("would upload blob: {}", blob_name);


    Ok(())
}

#[allow(unused_variables)]
pub async fn handle_bad_file(path: &std::path::Path) -> anyhow::Result<()> {

    println!("would handle bad file: {}", path.as_os_str().to_string_lossy());

    Ok(())
}
    