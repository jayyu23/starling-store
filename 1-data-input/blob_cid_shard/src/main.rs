use clap::Parser;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use cid::Cid;
use multihash::Multihash;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path to shard
    #[arg(short, long)]
    input: String,
    
    /// Output directory for chunks and metadata
    #[arg(short, long, default_value = "output")]
    output_dir: String,
    
    /// Chunk size in MB (default: 256)
    #[arg(short, long, default_value_t = 256)]
    chunk_size_mb: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChunkInfo {
    filename: String,
    size: u64,
    sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShardMetadata {
    original_file: String,
    total_size: u64,
    chunk_count: u32,
    chunks: Vec<ChunkInfo>,
    cid: String,
}

struct FileSharder {
    chunk_size_bytes: u64,
    output_dir: PathBuf,
}

impl FileSharder {
    fn new(chunk_size_mb: u64, output_dir: &str) -> std::io::Result<Self> {
        let chunk_size_bytes = chunk_size_mb * 1024 * 1024;
        let output_path = PathBuf::from(output_dir);
        
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&output_path)?;
        
        Ok(FileSharder {
            chunk_size_bytes,
            output_dir: output_path,
        })
    }
    
    fn shard_file(&self, input_path: &str) -> std::io::Result<ShardMetadata> {
        let input_file = File::open(input_path)?;
        let file_size = input_file.metadata()?.len();
        let mut reader = BufReader::new(input_file);
        
        let original_filename = Path::new(input_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        
        let chunk_count = (file_size + self.chunk_size_bytes - 1) / self.chunk_size_bytes;
        let mut chunks = Vec::new();
        let mut buffer = vec![0u8; self.chunk_size_bytes as usize];
        
        println!("Sharding file: {} ({} bytes)", input_path, file_size);
        println!("Creating {} chunks of max {} MB each", chunk_count, self.chunk_size_bytes / (1024 * 1024));
        
        for chunk_index in 0..chunk_count {
            let chunk_filename = format!("chunk_{:03}.part", chunk_index);
            let chunk_path = self.output_dir.join(&chunk_filename);
            
            // Read chunk data
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            
            let chunk_data = &buffer[..bytes_read];
            
            // Calculate SHA256 for this chunk
            let mut hasher = Sha256::new();
            hasher.update(chunk_data);
            let chunk_hash = hasher.finalize();
            let chunk_sha256 = hex::encode(chunk_hash);
            
            // Write chunk to file
            let mut chunk_file = File::create(&chunk_path)?;
            chunk_file.write_all(chunk_data)?;
            
            // Store chunk info
            chunks.push(ChunkInfo {
                filename: chunk_filename,
                size: bytes_read as u64,
                sha256: chunk_sha256,
            });
            
            println!("Created chunk {}: {} bytes", chunk_index, bytes_read);
        }
        
        // Generate global CID for the entire file
        let global_cid = self.generate_global_cid(&chunks, &original_filename, file_size)?;
        
        let metadata = ShardMetadata {
            original_file: original_filename,
            total_size: file_size,
            chunk_count: chunks.len() as u32,
            chunks,
            cid: global_cid,
        };
        
        Ok(metadata)
    }
    
    fn generate_global_cid(&self, chunks: &[ChunkInfo], original_filename: &str, total_size: u64) -> std::io::Result<String> {
        // Create a composite hash from all chunk hashes, filename, and size
        let mut global_hasher = Sha256::new();
        
        // Include original filename
        global_hasher.update(original_filename.as_bytes());
        
        // Include total size
        global_hasher.update(&total_size.to_be_bytes());
        
        // Include all chunk hashes in order
        for chunk in chunks {
            global_hasher.update(&chunk.filename.as_bytes());
            global_hasher.update(&chunk.size.to_be_bytes());
            global_hasher.update(hex::decode(&chunk.sha256).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, e)
            })?);
        }
        
        let global_hash = global_hasher.finalize();
        
        // Create multihash using SHA2-256 (code 0x12)
        let multihash = Multihash::wrap(0x12, &global_hash).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Multihash error: {}", e))
        })?;
        
        // Create CID v1 with raw codec
        let cid = Cid::new_v1(0x55, multihash); // 0x55 is raw codec
        
        Ok(cid.to_string())
    }
    
    fn save_metadata(&self, metadata: &ShardMetadata) -> std::io::Result<()> {
        let metadata_filename = format!("{}_metadata.json", 
            metadata.original_file.split('.').next().unwrap_or("file"));
        let metadata_path = self.output_dir.join(metadata_filename);
        
        let json = serde_json::to_string_pretty(metadata)?;
        let mut metadata_file = File::create(metadata_path)?;
        metadata_file.write_all(json.as_bytes())?;
        
        println!("Metadata saved with CID: {}", metadata.cid);
        Ok(())
    }
    
    fn reassemble_file(&self, metadata_path: &str, output_path: &str) -> std::io::Result<()> {
        // Read metadata
        let metadata_content = std::fs::read_to_string(metadata_path)?;
        let metadata: ShardMetadata = serde_json::from_str(&metadata_content)?;
        
        println!("Reassembling file: {}", metadata.original_file);
        println!("Expected total size: {} bytes", metadata.total_size);
        
        let mut output_file = File::create(output_path)?;
        let mut total_written = 0u64;
        
        for (index, chunk_info) in metadata.chunks.iter().enumerate() {
            let chunk_path = self.output_dir.join(&chunk_info.filename);
            let mut chunk_file = File::open(&chunk_path)?;
            let mut chunk_data = Vec::new();
            chunk_file.read_to_end(&mut chunk_data)?;
            
            // Verify chunk integrity
            let mut hasher = Sha256::new();
            hasher.update(&chunk_data);
            let computed_hash = hex::encode(hasher.finalize());
            
            if computed_hash != chunk_info.sha256 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Chunk {} integrity check failed", index)
                ));
            }
            
            output_file.write_all(&chunk_data)?;
            total_written += chunk_data.len() as u64;
            
            println!("Reassembled chunk {}: {} bytes", index, chunk_data.len());
        }
        
        if total_written != metadata.total_size {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Size mismatch: expected {}, got {}", metadata.total_size, total_written)
            ));
        }
        
        println!("File reassembled successfully: {} bytes", total_written);
        println!("Original CID: {}", metadata.cid);
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("=== BLOB CID Shard Tool ===");
    println!("Input file: {}", args.input);
    println!("Output directory: {}", args.output_dir);
    println!("Chunk size: {} MB", args.chunk_size_mb);
    println!();
    
    let sharder = FileSharder::new(args.chunk_size_mb, &args.output_dir)?;
    
    // Check if input is a metadata file for reassembly
    if args.input.ends_with("_metadata.json") {
        let output_filename = args.input
            .trim_end_matches("_metadata.json")
            .split('/')
            .last()
            .unwrap_or("reassembled_file");
        let output_path = format!("{}/{}_reassembled", args.output_dir, output_filename);
        
        println!("Reassembling file from metadata...");
        sharder.reassemble_file(&args.input, &output_path)?;
    } else {
        // Shard the file
        let metadata = sharder.shard_file(&args.input)?;
        
        // Save metadata
        sharder.save_metadata(&metadata)?;
        
        // Print summary
        println!("\n=== Sharding Complete ===");
        println!("Original file: {}", metadata.original_file);
        println!("Total size: {} bytes", metadata.total_size);
        println!("Chunks created: {}", metadata.chunk_count);
        println!("Global CID: {}", metadata.cid);
        println!("\nChunk details:");
        for (i, chunk) in metadata.chunks.iter().enumerate() {
            println!("  {}: {} ({} bytes, sha256: {}...)", 
                i, chunk.filename, chunk.size, &chunk.sha256[..8]);
        }
    }
    
    Ok(())
}
