use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, primitives::ByteStream};
use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use std::path::PathBuf;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory containing data shards and metadata to upload
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Optional: Custom name prefix for uploaded files
    #[arg(long)]
    name_prefix: Option<String>,

    /// List files in bucket instead of uploading
    #[arg(long)]
    list: bool,
}

struct AkaveClient {
    client: Client,
    bucket_name: String,
}

impl AkaveClient {
    async fn new() -> Result<Self> {
        // Load environment variables from .env file
        dotenv().ok();
        
        let akave_endpoint = env::var("AKAVE_HOSTNAME")
            .unwrap_or_else(|_| "https://o3-rc1.akave.xyz".to_string());
        let bucket_name = env::var("AKAVE_BUCKET")
            .unwrap_or_else(|_| "starling-akave".to_string());
        let _access_key = env::var("AKAVE_USERNAME")
            .context("AKAVE_USERNAME environment variable is required")?;
        let _secret_key = env::var("AKAVE_CREDENTIAL")
            .context("AKAVE_CREDENTIAL environment variable is required")?;

        // Configure AWS SDK for Akave
        let region_provider = RegionProviderChain::default_provider()
            .or_else("akave-network");
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region_provider)
            .endpoint_url(&akave_endpoint)
            .load()
            .await;

        let client = Client::new(&config);

        println!("✅ Connected to Akave endpoint: {}", akave_endpoint);
        println!("📦 Using bucket: {}", bucket_name);

        Ok(Self {
            client,
            bucket_name,
        })
    }

    async fn upload_file(&self, file_path: &PathBuf, custom_name: Option<String>) -> Result<String> {
        let mut file = File::open(file_path)
            .await
            .with_context(|| format!("Failed to open file: {:?}", file_path))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;

        let object_key = custom_name.unwrap_or_else(|| {
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        });

        println!("Uploading file: {:?} -> {}", file_path, object_key);

        let result = self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(&object_key)
            .body(ByteStream::from(buffer))
            .content_type("application/octet-stream")
            .send()
            .await
            .with_context(|| format!("Failed to upload file: {:?}", file_path))?;

        let etag = result.e_tag().unwrap_or("unknown").to_string();
        
        println!("Successfully uploaded: {:?}", file_path);
        println!("   Object key: {}", object_key);
        println!("   ETag: {}", etag);

        Ok(object_key)
    }

    async fn list_objects(&self) -> Result<()> {
        println!("\n🗂️  Listing objects in bucket '{}':", self.bucket_name);
        
        let result = self.client
            .list_objects_v2()
            .bucket(&self.bucket_name)
            .max_keys(100)
            .send()
            .await
            .with_context(|| "Failed to list objects")?;

        let objects = result.contents();
        if !objects.is_empty() {
            println!("Found {} objects:", objects.len());
            for (index, object) in objects.iter().enumerate() {
                if let (Some(key), Some(size), Some(modified)) = 
                    (object.key(), object.size(), object.last_modified()) {
                    println!("  [{}] {} ({} bytes, modified: {})", 
                             index + 1, key, size, modified);
                }
            }
        } else {
            println!("  No objects found in bucket.");
        }

        Ok(())
    }
}

async fn find_files(input_dir: &PathBuf) -> Result<Vec<PathBuf>> {
    if !input_dir.exists() {
        anyhow::bail!("Input directory does not exist: {:?}", input_dir);
    }

    if !input_dir.is_dir() {
        anyhow::bail!("Input path is not a directory: {:?}", input_dir);
    }

    let mut files = Vec::new();
    
    for entry in WalkDir::new(input_dir) {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    if files.is_empty() {
        println!("No files found in directory: {:?}", input_dir);
    } else {
        println!("📁 Found {} files to upload", files.len());
    }

    Ok(files)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚀 Starting Akave Hot Storage uploader");

    // Initialize Akave client
    let client = AkaveClient::new().await
        .with_context(|| "Failed to initialize Akave client")?;

    // If list flag is set, just list objects and exit
    if args.list {
        client.list_objects().await?;
        return Ok(());
    }

    println!("Input directory: {:?}", args.input_dir);

    // Find all files in the input directory
    let files = find_files(&args.input_dir).await?;

    if files.is_empty() {
        println!("No files to upload. Exiting.");
        return Ok(());
    }

    println!("\n📤 Starting uploads...\n");

    let mut successful_uploads = 0;
    let mut failed_uploads = 0;

    // Upload each file
    for (index, file_path) in files.iter().enumerate() {
        let custom_name = args.name_prefix.as_ref().map(|prefix| {
            format!("{}_{}", prefix, 
                   file_path.file_name().unwrap_or_default().to_string_lossy())
        });

        println!("[{}/{}] Processing file: {:?}", index + 1, files.len(), file_path);

        match client.upload_file(file_path, custom_name).await {
            Ok(_) => {
                successful_uploads += 1;
            }
            Err(e) => {
                println!("❌ Failed to upload {:?}: {}", file_path, e);
                failed_uploads += 1;
            }
        }

        println!(); // Add spacing between files
    }

    println!("Upload Summary:");
    println!("   ✅ Successful: {}", successful_uploads);
    println!("   ❌ Failed: {}", failed_uploads);
    println!("   📁 Total files: {}", files.len());

    if failed_uploads > 0 {
        println!("\n⚠️  Some uploads failed. Check the logs above for details.");
    } else {
        println!("\n🎉 All files uploaded successfully!");
    }

    // List uploaded objects to verify
    println!("\n📋 Verification - listing uploaded objects:");
    client.list_objects().await?;

    Ok(())
}
