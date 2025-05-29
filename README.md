# Starling Store

Toolkit for Starling Labs' Guttenfelder photos from North Korea.

## Components

### 1. Python Metadata Merkle (`py_metadata_merkle/`)

A Python implementation for proving photo metadata authenticity using Merkle trees. This tool demonstrates how cryptographic proofs can verify specific claims about image metadata without revealing the entire metadata set.

**Features:**
- EXIF data extraction from images
- GPS coordinate geocoding to human-readable addresses
- Merkle tree construction from metadata
- Cryptographic inclusion proof generation and verification
- Privacy-preserving metadata claims

### 2. Rust EXIF Reader (`rust_exif_reader/`)

A high-performance Rust implementation for extracting raw EXIF data blobs from JPEG files. This tool provides low-level access to image metadata for further processing.

**Features:**
- Raw EXIF blob extraction from JPEG files
- Binary data inspection and ASCII visualization
- Fast, memory-efficient processing

### 3. Nexus zkVM Host (`nexus-host/`)

A zero-knowledge proof system that validates EXIF data using the Nexus zkVM. This component enables cryptographic verification of photo metadata without revealing the actual data.

**Features:**
- Zero-knowledge proof generation for EXIF validation
- RISC-V guest program execution
- Cryptographic verification of photo authenticity
- Privacy-preserving validation

## Use Cases

### Photo Authentication
Prove when, where, and with what device a photo was taken using cryptographic methods.

### Privacy-Preserving Verification
Verify specific metadata claims (location, timestamp, device) without exposing complete metadata.

### Digital Forensics
Authenticate image metadata in legal or investigative contexts with cryptographic backing.

### Research Applications
Support academic research on photo provenance and metadata integrity.

## Technical Architecture

### Cryptographic Components
- **SHA-256 hashing** for data integrity
- **Merkle trees** for efficient proof generation
- **Zero-knowledge proofs** for privacy-preserving validation

### Data Processing Pipeline
1. **Extraction**: Raw EXIF data extraction from images
2. **Processing**: Metadata parsing and geocoding
3. **Tree Construction**: Merkle tree building from metadata entries
4. **Proof Generation**: Cryptographic inclusion proofs
5. **Verification**: Proof validation without data exposure

## Security Considerations

⚠️ **Important**: This project is designed for educational and research purposes. For production use, consider:

- EXIF metadata can be easily modified or stripped
- No protection against image tampering beyond metadata
- External service dependencies (geocoding)
- No timestamp verification against authoritative sources
- Zero-knowledge proofs require trusted setup considerations

## Dataset

The project is designed to work with Guttenfelder's North Korea photo collection. Photos should be placed in the `guttenfelder_photos/` directory (gitignored for privacy).

## Contributing

This project is part of EE 292J coursework. Contributions should follow academic integrity guidelines.

## License

Educational use only. Part of Stanford EE 292J coursework.

## Acknowledgments

- Adapted from Jupyter notebook implementations
- Built using pymerkle, Nexus zkVM, and Rust exif libraries
- Supports Starling Labs' mission for digital integrity



