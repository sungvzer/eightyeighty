#![deny(clippy::all)]

mod logs;
use std::{fs::OpenOptions, io::Read, path::PathBuf, str::FromStr};

use clap::Parser;
use emulator::cpu::CPU;
use log::trace;
use logs::log_init;

#[derive(Parser)]
struct Arguments {
    /// The binary file to be executed
    file: PathBuf,
}

fn main() {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_owned());
    let level = log::Level::from_str(&log_level).unwrap_or(log::Level::Info);

    match log_init(level) {
        Ok(_) => {
            trace!("Initialized logging");
        }
        Err(err) => {
            panic!("Could not initialize logger: {err}")
        }
    };

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
