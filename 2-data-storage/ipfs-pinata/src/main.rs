use std::path::PathBuf;
use std::env;
use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory containing BLOBs to upload
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Optional: Custom name prefix for uploaded files
    #[arg(long)]
    name_prefix: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PinataResponse {
    #[serde(rename = "IpfsHash")]
    ipfs_hash: String,
    #[serde(rename = "PinSize")]
    pin_size: u64,
    #[serde(rename = "Timestamp")]
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PinataError {
    error: String,
}

struct PinataClient {
    client: reqwest::Client,
    api_key: String,
    secret: String,
}

impl PinataClient {
    fn new(api_key: String, secret: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            secret,
        }
    }

    async fn pin_file(&self, file_path: &PathBuf, custom_name: Option<String>) -> Result<PinataResponse> {
        let file = File::open(file_path)
            .await
            .with_context(|| format!("Failed to open file: {:?}", file_path))?;

        let file_name = custom_name.unwrap_or_else(|| {
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        });

        // Create a stream from the file
        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = reqwest::Body::wrap_stream(stream);

        // Create multipart form
        let form = multipart::Form::new()
            .part("file", multipart::Part::stream(file_body).file_name(file_name));

        println!("Uploading file: {:?}", file_path);

        let response = self
            .client
            .post("https://api.pinata.cloud/pinning/pinFileToIPFS")
            .header("pinata_api_key", &self.api_key)
            .header("pinata_secret_api_key", &self.secret)
            .multipart(form)
            .send()
            .await
            .with_context(|| "Failed to send request to Pinata")?;

        if response.status().is_success() {
            let pinata_response: PinataResponse = response
                .json()
                .await
                .with_context(|| "Failed to parse Pinata response")?;
            
            println!("Successfully uploaded: {:?}", file_path);
            println!("   IPFS Hash: {}", pinata_response.ipfs_hash);
            println!("   Size: {} bytes", pinata_response.pin_size);
            
            Ok(pinata_response)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            
            anyhow::bail!("Pinata API error: {}", error_text);
        }
    }

    async fn test_authentication(&self) -> Result<()> {
        println!("Testing Pinata API authentication...");
        
        let response = self
            .client
            .get("https://api.pinata.cloud/data/testAuthentication")
            .header("pinata_api_key", &self.api_key)
            .header("pinata_secret_api_key", &self.secret)
            .send()
            .await
            .with_context(|| "Failed to test authentication")?;

        if response.status().is_success() {
            println!("✅ Authentication successful!");
            Ok(())
        } else {
            anyhow::bail!("Authentication failed: {}", response.status());
        }
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

fn load_env_vars() -> Result<(String, String)> {
    // Load .env file if it exists
    dotenv().ok();

    let api_key = env::var("PINATA_API_KEY")
        .with_context(|| "PINATA_API_KEY environment variable not found. Please set it in your .env file or environment.")?;
    
    let secret = env::var("PINATA_API_SECRET")
        .with_context(|| "PINATA_API_SECRET environment variable not found. Please set it in your .env file or environment.")?;

    if api_key.is_empty() {
        anyhow::bail!("PINATA_API_KEY cannot be empty");
    }

    if secret.is_empty() {
        anyhow::bail!("PINATA_API_SECRET cannot be empty");
    }

    Ok((api_key, secret))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("🚀 Starting BLOB upload to Pinata IPFS");
    println!("Input directory: {:?}", args.input_dir);

    // Load environment variables
    let (api_key, secret) = load_env_vars()
        .with_context(|| "Failed to load Pinata API credentials from environment")?;

    // Initialize Pinata client
    let client = PinataClient::new(api_key, secret);

    // Test authentication
    client.test_authentication().await?;

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
            format!("{}_{}", prefix, file_path.file_name().unwrap_or_default().to_string_lossy())
        });

        println!("[{}/{}] Processing file: {:?}", index + 1, files.len(), file_path);

        match client.pin_file(file_path, custom_name).await {
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

    Ok(())
}
