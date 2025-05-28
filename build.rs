extern crate embed_resource;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("build/windows/icon.rc");
    }

    // Check if Cranelift is available
    let cranelift_available = std::process::Command::new("rustc")
        .arg("-Z")
        .arg("codegen-backend=cranelift")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if cranelift_available {
        println!("cargo:warning=Cranelift available, using optimized config");
        // Enable Cranelift for faster dev builds
        println!("cargo:rustc-env=CRANELIFT_AVAILABLE=1");

        // Set environment variable that can be used by cargo config
        // This doesn't directly set the codegen backend, but signals availability
        println!("cargo:rustc-env=USE_CRANELIFT=1");
    } else {
        println!("cargo:warning=Cranelift not available, using standard LLVM backend");
        println!("cargo:rustc-env=CRANELIFT_AVAILABLE=0");
        println!("cargo:rustc-env=USE_CRANELIFT=0");
    }
}
