use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    // Set the name to be displayed when logging
    println!("cargo:rustc-env=LOG_DISPLAY_NAME=initializer");
    
    // Path to the NASM source file
    let nasm_src_path = "src/start.asm";

    // Retrieve the target directory where the object file will be placed
    let target_dir = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to retrieve the target directory (OUT_DIR): {}", e);
            std::process::exit(1);
        }
    };

    // Convert target_dir to a String
    let target_dir = PathBuf::from(target_dir);

    // Construct the path to the target object file
    let target_start_o_path = target_dir.join("start.o");

    // Run NASM to assemble start.asm into start.o
    let nasm_output = Command::new("nasm")
        .args(&["-f", "elf32", nasm_src_path, "-o"])
        .arg(&target_start_o_path)
        .output()
        .expect("Failed to run NASM");

    // Check if NASM produced any error output
    if !nasm_output.stderr.is_empty() {
        // Print NASM's error output
        eprintln!("NASM error:\n{}", String::from_utf8_lossy(&nasm_output.stderr));
        // Terminate the build process
        std::process::exit(1);
    }

    // build _load_gdt.asm

    // Path to the GAS source file
    let gas_src_path = "src/load_gdt.S";

    // Retrieve the target directory where the object file will be placed
    let target_dir = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to retrieve the target directory (OUT_DIR): {}", e);
            std::process::exit(1);
        }
    };

    // Convert target_dir to a String
    let target_dir = PathBuf::from(target_dir);

    // Construct the path to the target object file
    let target_load_gdt_o_path = target_dir.join("load_gdt.o");

    // Run NASM to assemble start.asm into start.o
    let nasm_output = Command::new("clang")
        .args(&["-ffreestanding", "-c", "-target", "i686-unknown-none", gas_src_path, "-o"])
        .arg(&target_load_gdt_o_path) //
        .output()
        .expect("Failed to run NASM");

    // Check if NASM produced any error output
    if !nasm_output.stderr.is_empty() {
        // Print NASM's error output
        eprintln!("clang (assembler) error:\n{}", String::from_utf8_lossy(&nasm_output.stderr));
        // Terminate the build process
        std::process::exit(1);
    }

    // Link start.o
    println!("cargo:rustc-link-arg={}", target_start_o_path.display());
    // Link load_gdt.o
    println!("cargo:rustc-link-arg={}", target_load_gdt_o_path.display());
    // Use our linker script
    println!("cargo:rustc-link-arg=-Tlink.ld");
}
