fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--------------------------------");
    println!("EXIF Reader");
    println!("--------------------------------");
    for path in &["../guttenfelder_photos/06_2015_RF_Guttenfelder_00004.JPG"] {
        println!("Processing file: {} \n", path);
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        for f in exif.fields() {
            println!("{} {} {}",
                     f.tag, f.ifd_num, f.display_value().with_unit(&exif));
        }
    }
    Ok(())
}
