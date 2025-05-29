use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};

const PACKAGE: &str = "guest";
extern crate alloc;
use alloc::string::String;

fn main() {
    println!("Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone(); // save elf for use with test verification

    // EXIF data blob as a string (this would normally come from an actual image file)
    let exif_blob = "Make: Canon\nModel: 5D Mark III\nDateTime: 2015:05:22 15:07:45\nExposureTime: 1/60\nFNumber: f/8.0".to_string();

    print!("Proving execution of EXIF validation... ");
    let (view, proof) = prover
        .prove_with_input::<(), String>(&(), &exif_blob)
        .expect("failed to prove program");

    assert_eq!(view.exit_code().expect("failed to retrieve exit code"), 0);

    let output: u32 = view
        .public_output::<u32>()
        .expect("failed to retrieve public output");
    assert_eq!(output, 0); // expecting 0 for valid EXIF

    println!("EXIF validation result: {}!", if output == 0 { "VALID" } else { "INVALID" });
    println!(
        ">>>>> Logging\n{}<<<<<",
        view.logs().expect("failed to retrieve debug logs").join("")
    );

    print!("Verifying execution...");
    proof
        .verify_expected::<String, u32>(
            &exif_blob, // private input (the EXIF blob)
            0,          // exit code = 0 (valid EXIF)  
            &0u32,      // output = 0 (valid EXIF)
            &elf,       // expected elf (program binary)
            &[],        // no associated data
        )
        .expect("failed to verify proof");

    println!("  Succeeded!");
}