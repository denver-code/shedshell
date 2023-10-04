use std::process::{exit, ExitCode};

pub fn main() -> Result<(), ExitCode> {
    println!("Exiting shell...");
    exit(0);
}
