#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! clap = { version = "4.0", features = ["derive"] }
//! colored = "2.0"
//! ```

use clap::{ArgAction, Parser};
use colored::*;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "build_rust")]
#[command(about = "Intelligent Rust build script with Cranelift detection")]
struct Args {
    /// Build in release mode
    #[arg(long)]
    release: bool,

    /// Use specific build profile
    #[arg(long)]
    profile: Option<String>,

    /// Additional cargo arguments
    #[arg(trailing_var_arg = true, action = ArgAction::Append)]
    extra_args: Vec<String>,
}

fn print_status(msg: &str) {
    println!("{} {}", "[INFO]".green().bold(), msg);
}

fn print_warning(msg: &str) {
    println!("{} {}", "[WARN]".yellow().bold(), msg);
}

fn print_error(msg: &str) {
    println!("{} {}", "[ERROR]".red().bold(), msg);
}

fn check_cranelift() -> bool {
    Command::new("rustc")
        .arg("-Z")
        .arg("codegen-backend=cranelift")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn main() {
    let args = Args::parse();

    print_status("Starting build process...");

    // Determine build mode
    let build_mode = if args.release {
        "release"
    } else if let Some(ref profile) = args.profile {
        profile.as_str()
    } else {
        "dev"
    };

    print_status(&format!("Build mode: {}", build_mode));

    // Prepare cargo command
    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.arg("build");

    if args.release {
        cargo_cmd.arg("--release");
    }

    if let Some(ref profile) = args.profile {
        cargo_cmd.arg("--profile").arg(profile);
    }

    // Add extra arguments
    for arg in args.extra_args {
        cargo_cmd.arg(arg);
    }

    // Check Cranelift availability and configure accordingly
    if check_cranelift() {
        print_status("Cranelift codegen backend detected and available");

        if build_mode == "dev" {
            print_status("Using Cranelift for faster dev compilation");
            cargo_cmd.env("CARGO_CONFIG_PROFILE_DEV_CODEGEN_BACKEND", "cranelift");
        } else {
            print_warning(&format!(
                "Cranelift available but not used for {} builds (LLVM is better for optimized builds)",
                build_mode
            ));
        }
    } else {
        print_warning("Cranelift codegen backend not available, using standard LLVM backend");
        print_status("To install Cranelift: rustup component add rustc-codegen-cranelift-preview");
        cargo_cmd.env("CARGO_CONFIG_PROFILE_DEV_CODEGEN_BACKEND", "");
    }

    // Execute the build
    print_status("Executing cargo build...");
    let status = cargo_cmd.status().expect("Failed to execute cargo build");

    if status.success() {
        print_status("Build completed successfully!");
    } else {
        print_error("Build failed!");
        exit(status.code().unwrap_or(1));
    }
}
