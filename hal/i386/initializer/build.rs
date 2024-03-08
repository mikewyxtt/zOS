use std::process::Command;
use std::path::PathBuf;
use std::env;

fn main() {
    // Set the name to be displayed when logging
    println!("cargo:rustc-env=LOG_DISPLAY_NAME=initializer");
    
    // Build assembly source
    assemble("src/start.S", "start.o");
    assemble("src/load_gdt.S", "load_gdt.o");

    // Use our linker script
    println!("cargo:rustc-link-arg=-Tlink.ld");
}

/// Uses clang to assemble .S src into an object file
fn assemble(src_path: &str, output_path: &str) {
    // Find the 'target' directory then create a full path to the output file so we can output our .o file
    let target_dir = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to retrieve the target directory (OUT_DIR): {}", e);
            std::process::exit(1);
        }
    };

    let target_dir = PathBuf::from(target_dir);
    let output_path = target_dir.join(output_path);

    // Run clang to assemble source into an object file
    let clang_output = Command::new("clang")
        .args(&["-ffreestanding", "-c", "-target", "i686-unknown-none", src_path, "-o"])
        .arg(&output_path) //
        .output()
        .expect("Failed to run clang");

    // Check if clang produced any error output
    if !clang_output.stderr.is_empty() {
        eprintln!("clang (assembler) error:\n{}", String::from_utf8_lossy(&clang_output.stderr));
        std::process::exit(1);
    }

    // Link the output file
    println!("cargo:rustc-link-arg={}", output_path.display());
}