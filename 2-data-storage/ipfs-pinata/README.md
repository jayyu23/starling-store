# IPFS Pinata BLOB Streamer

A Rust application that streams BLOBs from a local directory to Pinata IPFS for decentralized storage.

## Features

- Asynchronous file streaming to minimize memory usage
- Recursive directory traversal to find all files
- Secure API key-based authentication with Pinata
- Progress tracking and detailed upload statistics
- Fast parallel processing capabilities
- Comprehensive error handling and recovery
- Environment variable support for secure credential management

## Prerequisites

- Rust (latest stable version)
- Pinata account with API credentials

## Setup

### 1. Get Your Pinata API Keys

1. Sign up at [Pinata.cloud](https://pinata.cloud)
2. Navigate to the API Keys section in your dashboard
3. Create a new API key with the following permissions:
   - `pinFileToIPFS`
   - `testAuthentication`
4. Copy your API key and secret

### 2. Configure Environment Variables

Copy the example environment file and add your credentials:

```bash
cp .env.example .env
```

Then edit `.env` and replace the placeholder values with your actual Pinata credentials:

```env
# Pinata API Credentials
# Get these from your Pinata dashboard: https://app.pinata.cloud/keys
PINATA_API_KEY=your_actual_api_key_here
PINATA_SECRET_API_KEY=your_actual_secret_key_here
```

**⚠️ Important**: Never commit your `.env` file to version control. It should be added to `.gitignore`.

## Usage

### Basic Usage

```bash
cargo run -- --input-dir /path/to/your/blobs
```

### With Custom Name Prefix

```bash
cargo run -- --input-dir /path/to/your/blobs --name-prefix "my-project"
```

### Command Line Options

- `--input-dir, -i`: Path to the directory containing files to upload
- `--name-prefix`: Optional prefix for uploaded file names

### Examples

```bash
# Upload all files from ./data directory
cargo run -- -i ./data

# Upload with custom naming
cargo run -- -i ./media --name-prefix "media-backup"

# Upload from absolute path
cargo run -- --input-dir /Users/username/Documents/images
```

## What It Does

1. **Environment Loading**: Loads Pinata credentials from `.env` file
2. **Authentication Test**: Verifies your Pinata credentials before starting
3. **File Discovery**: Recursively scans the input directory for all files
4. **Streaming Upload**: Uploads each file using efficient streaming to handle large files
5. **Progress Tracking**: Shows real-time progress and upload statistics
6. **IPFS Hash**: Returns the IPFS hash for each successfully uploaded file

## Output

The application provides detailed output including:
- Environment variable verification
- Authentication verification
- File discovery count
- Upload progress for each file
- IPFS hashes for successful uploads
- Final summary with success/failure counts

## Example Output

```
🚀 Starting BLOB upload to Pinata IPFS
Input directory: "./data"
Testing Pinata API authentication...
✅ Authentication successful!
📁 Found 3 files to upload

📤 Starting uploads...

[1/3] Processing file: "./data/image1.jpg"
Uploading file: "./data/image1.jpg"
Successfully uploaded: "./data/image1.jpg"
   IPFS Hash: QmXxXxXxXxXxXxXxXxXxXxXxXxXxXxXxXxXxXxXxXxXx
   Size: 2048576 bytes

📊 Upload Summary:
   ✅ Successful: 3
   ❌ Failed: 0
   📁 Total files: 3

🎉 All files uploaded successfully!
```

## Error Handling

The application handles various error scenarios:
- Missing or invalid environment variables
- Invalid or missing API credentials
- Network connectivity issues
- File access permissions
- Pinata API rate limits
- Large file uploads

## Security Best Practices

- ✅ API credentials are stored in `.env` file (not in command line arguments)
- ✅ `.env` file should be added to `.gitignore`
- ✅ Never commit API keys to version control
- ✅ Use environment variables for production deployments
- ✅ Keep your Pinata account secure with strong passwords

## Building for Production

```bash
cargo build --release
```

The optimized binary will be available at `target/release/ipfs-pinata`.

For production deployments, set the environment variables directly in your system instead of using a `.env` file:

```bash
export PINATA_API_KEY="your_api_key"
export PINATA_SECRET_API_KEY="your_secret_key"
./target/release/ipfs-pinata --input-dir /path/to/blobs
```

## Troubleshooting

### "Environment variable not found" error

Make sure you have:
1. Created a `.env` file in the project directory
2. Added your actual Pinata API credentials
3. The credentials are not empty

### Authentication failed

- Verify your API key and secret are correct
- Check that your API key has the required permissions
- Ensure your Pinata account is active and in good standing 