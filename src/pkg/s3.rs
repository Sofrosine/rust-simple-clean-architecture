use std::error::Error;
use actix_multipart::form::tempfile::TempFile;
use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn create_s3_client() -> Result<Client, Box<dyn Error>> {
    // Custom region for your S3-compatible storage
    let region_provider = RegionProviderChain::default_provider().or_else("custom-region");
    // Provide the credentials manually
    let credentials_provider = Credentials::new(
        std::env::var("AWS_S3_ACCESS_KEY_ID").unwrap_or(String::from("")),    // Access key
        std::env::var("AWS_S3_SECRET_ACCESS_KEY").unwrap_or(String::from("")),    // Access key
        None, None, "Static",
    );
    // Load the custom S3 configuration with provided credentials
    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .endpoint_url("https://is3.cloudhost.id") // Custom S3-compatible endpoint
        .credentials_provider(
            credentials_provider
        )
        .load()
        .await;
    // Create and return the S3 client
    Ok(Client::new(&shared_config))
}

pub async fn upload_file_to_s3(s3: Client, file_param: Option<TempFile>, file_path: String) -> Result<String, Box<dyn Error>> {
    // Initialize S3 client
    let client = s3;

    // Ensure the logo is present
    let temp_file = file_param.ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No file provided",
        ))
    })?;

    // Access the file path
    let temp_path = temp_file.file.path();
    let mut file = File::open(temp_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    // Convert the buffer to ByteStream
    let byte_stream = ByteStream::from(buffer);

    // Upload file to S3
    client
        .put_object()
        .bucket("sekula-storage")
        .key(&file_path)
        .body(byte_stream)
        .send()
        .await
        .map_err(|e| format!("Failed to upload file: {:?}", e))?;

    // Return the S3 URL
    let s3_url = format!("https://{}/{}", "is3.cloudhost.id/sekula-storage", file_path);
    Ok(s3_url)
}
