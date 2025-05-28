# Photo Metadata Merkle Proof

A Python implementation for proving photo metadata authenticity using Merkle trees. This project demonstrates how cryptographic proofs can be used to verify specific claims about image metadata without revealing the entire metadata set.

## Overview

This tool extracts EXIF metadata from images, builds a Merkle tree from the metadata entries, and generates cryptographic proofs to verify specific claims about the photo (such as location, timestamp, or device used) while maintaining privacy of other metadata.

Adapted from Jupyter notebook version [here](https://colab.research.google.com/drive/1SFaYxnhX_WdNE3cZDmMJcp9WEVjEgR57#scrollTo=K4R8yrCPPV3f).

## Features

- **EXIF Data Extraction**: Automatically extracts metadata from image files
- **GPS Geocoding**: Converts GPS coordinates to human-readable addresses
- **Merkle Tree Construction**: Builds cryptographic trees from metadata
- **Proof Generation**: Creates inclusion proofs for specific metadata claims
- **Verification**: Validates proofs without revealing other metadata

## Requirements

Install the required dependencies:

```bash
pip install -r requirements.txt
```

## Usage
- `--show-metadata`: Display all extracted metadata
- `--show-image`: Display the image (requires GUI environment)

### Example

```bash
python metadata_merkle.py --show-metadata IMG_2334.jpeg
```

## How It Works

1. **Metadata Extraction**: Extracts EXIF data from the input image
2. **Location Resolution**: Converts GPS coordinates to postal codes and addresses
3. **Merkle Tree Construction**: Creates a Merkle tree with each metadata entry as a leaf
4. **Proof Generation**: Generates inclusion proofs for specific claims
5. **Verification**: Validates proofs cryptographically


## Use Cases

- **Photo Authentication**: Prove when and where a photo was taken
- **Privacy-Preserving Verification**: Verify specific claims without revealing all metadata
- **Digital Forensics**: Authenticate image metadata in legal contexts
- **Social Media**: Verify location claims without exposing exact coordinates

## Technical Details

- Uses SHA-256 for hashing
- Implements the `pymerkle` library for Merkle tree operations
- Supports standard EXIF metadata fields
- Integrates with Nominatim geocoding service

## Limitations

- Requires images with EXIF metadata
- GPS coordinates needed for location-based proofs
- Metadata can be modified or stripped from images
- Relies on external geocoding service for address resolution

## Security Considerations

This tool demonstrates cryptographic concepts but should not be used for high-security applications without additional safeguards:

- EXIF metadata can be easily modified
- No protection against image tampering
- Geocoding service dependency
- No timestamp verification against external sources

## License

This project is for educational purposes as part of EE 292J coursework.
