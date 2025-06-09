# Akave Hot Storage

Akave uses AWS S3-compatible hot storage for their database. This project provides a Rust implementation of the Akave Hot Storage API.

## Usage

Set up environment variables:
```bash
AKAVE_HOSTNAME=https://o3-rc1.akave.xyz
AKAVE_BUCKET=starling-akave
AKAVE_USERNAME=your-username
AKAVE_CREDENTIAL=your-credential
```

Create bucket:
```bash
aws s3api create-bucket --bucket starling-akave --endpoint-url "https://o3-rc1.akave.xyz"
```

List bucket:
```bash
aws s3api list-buckets --endpoint-url "https://o3-rc1.akave.xyz"
```

View bucket contents:
```bash
aws s3api list-objects --endpoint-url https://o3-rc1.akave.xyz --bucket starling-akave
```

Test upload script on folder:
```bash
cargo run -- --input-dir /dir/to/upload --name-prefix akave-test-guttenfelder-sharding
```

List bucket using script on prefix:

```bash
cargo run -- --list --input-dir akave-test-guttenfelder-sharding
```

Output:
```bash
🗂️  Listing objects in bucket 'starling-akave':
Found 6 objects:
  [1] test-upload-20250609-213547.json (71 bytes, modified: 2025-06-09T21:35:49Z)
  [2] akave-test-guttenfelder-sharding_chunk_002.part (965206 bytes, modified: 2025-06-09T21:47:50Z)
  [3] akave-test-guttenfelder-sharding_06_2015_RF_Guttenfelder_00004.JPG (3062358 bytes, modified: 2025-06-09T21:48:03Z)
  [4] akave-test-guttenfelder-sharding_06_2015_RF_Guttenfelder_00004_metadata.json (657 bytes, modified: 2025-06-09T21:48:16Z)
  [5] akave-test-guttenfelder-sharding_chunk_001.part (1048576 bytes, modified: 2025-06-09T21:48:27Z)
  [6] akave-test-guttenfelder-sharding_chunk_000.part (1048576 bytes, modified: 2025-06-09T21:48:39Z)
```