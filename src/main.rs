#![deny(clippy::all)]

use std::{fs::OpenOptions, io::Read, path::PathBuf};

use clap::Parser;
use emulator::cpu::CPU;

#[derive(Parser)]
struct Arguments {
    /// The binary file to be executed
    file: PathBuf,
}

fn main() {
    let arguments = Arguments::parse();
    let file = arguments.file;

    let mut file = match OpenOptions::new().read(true).open(file) {
        Ok(file) => file,
        Err(err) => {
            panic!("Error opening file: {err}")
        }
    };

    let mut vector = vec![];

    match file.read_to_end(&mut vector) {
        Ok(_) => {}
        Err(err) => {
            panic!("Error reading file: {err}")
        }
    }

    let mut cpu = CPU::new();
    cpu.load_program(&vector);

    loop {
        cpu.fetch_decode_execute();
    }
}
