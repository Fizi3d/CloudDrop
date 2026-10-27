# CloudDrop File Archiver
This is a Rust-based cloud archiver that allows easy archiving of files by simply dropping them into a folder, for instance, one on the user's desktop. Powered by Azure Storage Blobs.

## Getting started

### Requirements
- Rust
- An Azure Storage account with an existing blob container
-  Azure CLI (for local development login)
- An identity with the Storage Blob Data Contributor role on the storage account or container. Owner or Contributor on the account is not enough for blob data access.

### Setup
1. Clone the repository and enter the project folder
2. Create a .env file in the project root:
```
   AZURE_STORAGE_ACCOUNT=<your-storage-account-name>
   AZURE_CONTAINER_NAME=<your-container-name>
   WATCHED_FOLDER=<C:/path/to/the/folder/to/watch>
```
3. Compile and run with `cargo run`
Note: Compiling to a binary will work, but detailed error information is displayed only when run via terminal.

## Future improvements
- authenticaiton with a dedicated service principal for long term unattended use
- Basic CLI with the ability to create and connect to different containers without editing .env
- Parallel file uploads

