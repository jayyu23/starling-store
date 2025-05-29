#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;
use alloc::string::String;
use nexus_rt::println;

#[nexus_rt::main]
fn main() -> u32 {
    // Read in a single string
    let blob_str: String = nexus_rt::read_private_input().expect("Failed to read blob string");
    if validate_exif(&blob_str) {
        println!("EXIF is valid.");
        0
    } else {
        println!("EXIF is invalid.");
        1
    }
}

// NOTE: This is a placeholder. Real implementation should parse TIFF IFDs and tag values.
fn validate_exif(blob_str: &str) -> bool {
    let make_ok = blob_str.contains("Canon");
    let model_ok = blob_str.contains("5D Mark III");
    let date_ok = blob_str.contains("2015:05:22 15:07:45");
    make_ok && model_ok && date_ok
}