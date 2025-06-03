use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;

mod merkle;
use merkle::{MerkleNode, build_merkle_tree};

/// Returns the raw EXIF blob from a JPEG file (excluding JPEG markers).
pub fn extract_exif_blob(path: &str) -> Option<Vec<u8>> {
    let mut reader = BufReader::new(File::open(path).ok()?);
    let mut buf = [0u8; 2];

    // Read SOI (Start of Image)
    reader.read_exact(&mut buf).ok()?;
    if buf != [0xFF, 0xD8] {
        return None; // Not a JPEG
    }

    loop {
        reader.read_exact(&mut buf).ok()?;
        if buf[0] != 0xFF {
            return None; // Invalid marker
        }

        // Read marker
        let marker = buf[1];
        reader.read_exact(&mut buf).ok()?; // Read segment length
        let len = u16::from_be_bytes(buf) as usize;

        if marker == 0xE1 {
            // APP1 (EXIF) segment
            let mut exif_buf = vec![0u8; len - 2]; // already read 2 bytes for length
            reader.read_exact(&mut exif_buf).ok()?;
            if &exif_buf[..6] == b"Exif\0\0" {
                return Some(exif_buf);
            }
        } else {
            // Skip other segments
            reader.seek_relative((len - 2) as i64).ok()?;
        }
    }
}

pub fn print_exif_tags(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing file: {} \n", path);
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    for f in exif.fields() {
        println!("{} {} {}",
                 f.tag, f.ifd_num, f.display_value().with_unit(&exif));
    }
    Ok(())
}

fn print_ascii(blob: &[u8]) {
    for &b in blob {
        if b.is_ascii_graphic() || b == b' ' {
            print!("{}", b as char);
        }
    }
    println!();
}

pub fn build_exif_merkle_tree(path: &str) -> Result<MerkleNode, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    
    // Collect and sort EXIF fields
    let mut leaves: Vec<Vec<u8>> = exif.fields()
        .map(|f| {
            let value = format!("{}:{}", f.tag, f.display_value());
            value.as_bytes().to_vec()
        })
        .collect();
    
    // Sort leaves by their content to ensure deterministic ordering
    leaves.sort();

    build_merkle_tree(leaves)
        .ok_or_else(|| "Failed to build Merkle tree".into())
}

fn save_merkle_tree(path: &str, merkle_root: &MerkleNode) -> Result<(), Box<dyn std::error::Error>> {
    // Create a merkle tree filename based on the image filename
    let image_path = Path::new(path);
    let image_stem = image_path.file_stem().unwrap().to_str().unwrap();
    let merkle_path = format!("{}_merkle.json", image_stem);
    
    merkle_root.save_to_file(&merkle_path)?;
    println!("Merkle tree saved to: {}", merkle_path);
    Ok(())
}

fn verify_image_merkle_tree(image_path: &str, merkle_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Load the stored Merkle tree
    let stored_tree = MerkleNode::load_from_file(merkle_path)?;
    
    // Get current EXIF data
    let file = std::fs::File::open(image_path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    
    // Convert current EXIF fields to bytes
    let current_leaves: Vec<Vec<u8>> = exif.fields()
        .map(|f| {
            let value = format!("{}:{}", f.tag, f.display_value());
            value.as_bytes().to_vec()
        })
        .collect();

    // Verify the stored tree against current data
    Ok(stored_tree.verify(&current_leaves))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PHOTOS: [&str; 1] = ["../../guttenfelder_photos/06_2015_RF_Guttenfelder_00004.JPG"];
    println!("--------------------------------");
    println!("EXIF Reader and Merkle Tree");
    println!("--------------------------------");

    for path in PHOTOS {
        println!("Processing file: {}", path);
        
        // Print EXIF tags
        print_exif_tags(path)?;
        
        // Build and print Merkle tree
        let merkle_root = build_exif_merkle_tree(path)?;
        println!("\nMerkle Root Hash: {}", hex::encode(&merkle_root.hash));
        
        // Save the Merkle tree
        save_merkle_tree(path, &merkle_root)?;
        
        // Demonstrate verification
        let image_path = Path::new(path);
        let image_stem = image_path.file_stem().unwrap().to_str().unwrap();
        let merkle_path = format!("{}_merkle.json", image_stem);
        
        let is_valid = verify_image_merkle_tree(path, &merkle_path)?;
        println!("\nMerkle tree verification: {}", if is_valid { "VALID" } else { "INVALID" });
    }
    
    Ok(())
}
