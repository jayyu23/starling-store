use std::fs::File;
use std::io::{Read, BufReader};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PHOTOS: [&str; 1] = ["../guttenfelder_photos/06_2015_RF_Guttenfelder_00004.JPG"];
    println!("--------------------------------");
    println!("EXIF Reader");
    println!("--------------------------------");

    // Parse photo into binary blob
    for path in PHOTOS {
        let exif_blob = extract_exif_blob(path)
            .ok_or("Failed to extract EXIF blob from image")?;
        println!("EXIF Blob: {:02X?}", exif_blob);
        print_ascii(&exif_blob); // Print as ASCII to visually inspect EXIF validity
    }
    Ok(())
}
