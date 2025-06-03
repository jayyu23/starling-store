# BLOB CID Shard Tool

A Rust implementation of a CID (Content Identifier) + sharding protocol that splits large files into 256MB chunks with cryptographic integrity guarantees.

## Features

- **File Sharding**: Split large files (e.g., 10GB videos) into configurable chunks (default: 256MB)
- **CID Generation**: Create globally unique Content Identifiers using IPFS CID v1 standard
- **Integrity Verification**: SHA256 hash for each chunk and global CID for the entire file
- **Metadata Management**: Comprehensive metadata tracking with JSON serialization
- **File Reassembly**: Reconstruct original files from chunks with integrity verification

## Architecture

The tool implements a complete sharding protocol with the following components:

1. **FileSharder**: Core sharding engine that splits files into chunks
2. **ChunkInfo**: Metadata structure for individual chunks
3. **ShardMetadata**: Complete metadata for the sharded file
4. **CID Generation**: IPFS-compatible content addressing

## Metadata Format

```json
{
  "original_file": "your_video.mp4",
  "total_size": 10737418240,
  "chunk_count": 20,
  "chunks": [
    {
      "filename": "chunk_000.part",
      "size": 536870912,
      "sha256": "c6ee..."
    },
    {
      "filename": "chunk_001.part", 
      "size": 536870912,
      "sha256": "d7ff..."
    }
  ],
  "cid": "bafy2bzaced..."
}
```

## Usage

### Building

```bash
cd 1-data-input/blob_cid_shard
cargo build --release
```

### Sharding a File

```bash
# Shard a video file into 256MB chunks
cargo run -- --input guttenfelder_photos/video.mp4 --output-dir output

# Custom chunk size (e.g., 128MB)
cargo run -- --input guttenfelder_photos/video.mp4 --output-dir output --chunk-size-mb 128

# Using the binary directly
./target/release/blob_cid_shard --input path/to/large_file.mp4 --output-dir shards
```

### Reassembling a File

```bash
# Reassemble from metadata
cargo run -- --input output/video_metadata.json --output-dir output
```

### Command Line Options

- `--input, -i`: Input file path to shard (or metadata file for reassembly)
- `--output-dir, -o`: Output directory for chunks and metadata (default: "output")
- `--chunk-size-mb, -c`: Chunk size in MB (default: 256)

## Example Output

When sharding a 10GB video file:

```
=== BLOB CID Shard Tool ===
Input file: guttenfelder_photos/video.mp4
Output directory: output
Chunk size: 256 MB

Sharding file: guttenfelder_photos/video.mp4 (10737418240 bytes)
Creating 40 chunks of max 256 MB each
Created chunk 0: 268435456 bytes
Created chunk 1: 268435456 bytes
...
Created chunk 39: 67108864 bytes
Metadata saved with CID: bafy2bzaced7k3j2n8x9m5q1p6r4s8t2v5w7x0y3z6a9b2c5d8e1f4g7h0

=== Sharding Complete ===
Original file: video.mp4
Total size: 10737418240 bytes
Chunks created: 40
Global CID: bafy2bzaced7k3j2n8x9m5q1p6r4s8t2v5w7x0y3z6a9b2c5d8e1f4g7h0

Chunk details:
  0: chunk_000.part (268435456 bytes, sha256: c6ee9d2a...)
  1: chunk_001.part (268435456 bytes, sha256: d7ff8b3c...)
  ...
```

## CID Generation Algorithm

The global CID is generated using:

1. **Composite Hash**: SHA256 of (filename + file_size + all_chunk_metadata)
2. **Multihash**: SHA2-256 multihash encoding
3. **CID v1**: IPFS CID v1 with raw codec (0x55)

This ensures:
- **Uniqueness**: Different files produce different CIDs
- **Determinism**: Same file always produces the same CID
- **Integrity**: Any modification changes the CID
- **IPFS Compatibility**: Standard IPFS addressing

## File Integrity

The tool provides multiple layers of integrity verification:

1. **Chunk-level**: SHA256 hash for each individual chunk
2. **File-level**: Global CID incorporating all chunk hashes
3. **Reassembly**: Verification during file reconstruction
4. **Size validation**: Total reassembled size must match original

## Integration with Starling Store

This tool is designed to integrate with the broader Starling Store ecosystem:

- **Part 1**: BLOB CID and sharding (this tool)
- **Part 2**: IPFS storage via Pinata and Akave
- **Part 3**: Merkle tree verification and blockchain metadata

The generated CIDs can be used as global identifiers across the entire storage pipeline.

## Dependencies

- `sha2`: Cryptographic hashing
- `cid`: IPFS Content Identifier implementation
- `multihash`: Multihash encoding for CIDs
- `serde/serde_json`: Metadata serialization
- `clap`: Command-line interface
- `tokio`: Async runtime

## Error Handling

The tool includes comprehensive error handling for:

- File I/O errors
- Hash computation failures
- CID generation issues
- Chunk integrity mismatches
- Size validation failures

## Security Considerations

- All hashes use SHA256 for cryptographic security
- Chunk integrity is verified during reassembly
- CID format follows IPFS standards for interoperability
- No temporary files are created that could leak data 