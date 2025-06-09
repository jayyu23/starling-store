use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, primitives::ByteStream};
use anyhow::Result;
use std::env;
use dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    println!("Starting Akave Hot Storage uploader...");

    // Get configuration from environment variables
    let akave_endpoint = env::var("AKAVE_HOSTNAME")
        .unwrap_or_else(|_| "https://o3-rc1.akave.xyz".to_string());
    let bucket_name = env::var("AKAVE_BUCKET")
        .unwrap_or_else(|_| "starling-akave".to_string());
    let _access_key = env::var("AKAVE_USERNAME")
        .expect("AKAVE_USERNAME environment variable is required");
    let _secret_key = env::var("AKAVE_CREDENTIAL")
        .expect("AKAVE_CREDENTIAL environment variable is required");

    // Configure AWS SDK for Akave
    let region_provider = RegionProviderChain::default_provider().or_else("akave-network");
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(region_provider)
        .endpoint_url(&akave_endpoint)
        .load()
        .await;

    let client = Client::new(&config);

    // Sample data to upload
    let sample_data = "{{ \"message\": \"Hello from Akave Hot Storage! This is a test upload.\" }}";
    let object_key = format!("test-upload-{}.json", chrono::Utc::now().format("%Y%m%d-%H%M%S"));

    println!("Uploading data to Akave...");
    println!("Endpoint: {}", akave_endpoint);
    println!("Bucket: {}", bucket_name);
    println!("Object key: {}", object_key);

    // Upload the data
    let result = client
        .put_object()
        .bucket(&bucket_name)
        .key(&object_key)
        .body(ByteStream::from(sample_data.as_bytes().to_vec()))
        .content_type("text/plain")
        .send()
        .await;

    match result {
        Ok(output) => {
            println!("Successfully uploaded data to Akave!");
            println!("ETag: {:?}", output.e_tag());
            
            // Verify the upload by listing objects
            list_objects(&client, &bucket_name).await?;
        }
        Err(e) => {
            eprintln!("Failed to upload data: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

async fn list_objects(client: &Client, bucket_name: &str) -> Result<()> {
    println!("\nListing objects in bucket '{}':", bucket_name);
    
    let result = client
        .list_objects_v2()
        .bucket(bucket_name)
        .max_keys(10)
        .send()
        .await?;

    let objects = result.contents();
    if !objects.is_empty() {
        for object in objects {
            if let (Some(key), Some(size), Some(modified)) = (object.key(), object.size(), object.last_modified()) {
                println!("  {} ({} bytes, modified: {})", key, size, modified);
            }
        }
    } else {
        println!("  No objects found in bucket.");
    }

    Ok(())
}
