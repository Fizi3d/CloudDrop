use azure_core::http::{RequestContent, Url};
use azure_storage_blob::BlobServiceClient;
use azure_identity::DeveloperToolsCredential;
//use azure_identity::
use dotenvy::dotenv;
use anyhow::{Context};
use tokio;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    
    let account_name: &str = &std::env::var("AZURE_STORAGE_ACCOUNT")
        .context("Error loading Azure storage account")?;
    let container_name: &str = "hello_world_container";

    let service_url = Url::parse(&format!("https://{}.blob.core.windows.net", account_name))?;

    let credential = DeveloperToolsCredential::new(None)?;

    let service_client = BlobServiceClient::new(service_url, Some(credential), None)?;

    let container_client = service_client.blob_container_client(container_name);

    container_client.create(None).await?;
    println!("Container created");

    let blob_name = "hello_world.txt";
    let blob_client = container_client.blob_client(blob_name);
    let content = b"Hello, Azure Blob Storage!";
    let data = RequestContent::from(content.to_vec());
    blob_client
        .upload(data, None)
        .await.context("Error uploading blob")?;
    println!("Blob uploaded");

    let response = blob_client.download(None).await.context("Error downloading blob")?;
    println!("Blob downloaded: {}", String::from_utf8_lossy(&response.body.collect().await?));

    container_client.delete(None).await.context("Error deleting container")?;
    println!("Container deleted");

    Ok(())
}
