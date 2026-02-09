//! Shera Blockchain CLI Main Entry Point
//!
//! Usage: cargo run -- [OPTIONS] <COMMAND>

use crt_zkvm::cli::run_cli;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          Shera Blockchain (שרה)                               ║");
    println!("║          CRT-ZKVM Zero-Knowledge Virtual Machine             ║");
    println!("║          Version 0.2.0 - Shera Edition                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    run_cli();
}
