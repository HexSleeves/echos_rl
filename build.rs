// build.rs
use std::{env, process::Command};

fn main() {
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    if target.contains("windows") {
        println!("cargo:rerun-if-changed=build/windows/icon.rc");
        println!("cargo:rerun-if-changed=build/windows/icon.ico");
        embed_resource::compile("build/windows/icon.rc", embed_resource::NONE).manifest_optional().unwrap();
    }

    // Always rerun this build script if it changes
    println!("cargo:rerun-if-changed=build.rs");

    let cranelift_check_status = Command::new("rustc")
        .arg("-Z")
        .arg("codegen-backend=cranelift")
        .arg("--version")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    let cranelift_available = match cranelift_check_status {
        Ok(status) => status.success(),
        Err(e) => {
            println!(
                "cargo:warning=Failed to execute rustc for Cranelift check ({}). Assuming Cranelift is not available.",
                e
            );
            false
        }
    };

    if cranelift_available {
        println!("cargo:warning=Cranelift backend available. Will attempt to use via .cargo/config.toml.");
        // Set a custom cfg flag that .cargo/config.toml can read
        println!("cargo:rustc-cfg=has_cranelift");
        println!("cargo:rustc-env=CRANELIFT_COMPILATION_ENABLED=1"); // Informational
    } else {
        println!(
            "cargo:warning=Cranelift backend not available or rustc check failed. Using default LLVM backend."
        );
        println!("cargo:rustc-env=CRANELIFT_COMPILATION_ENABLED=0"); // Informational
    }
}
