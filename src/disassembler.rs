use std::{fs::OpenOptions, io::Read, io::Write, path::PathBuf};

use clap::Parser;
use emulator::{instruction::Instruction, parser::InstructionParser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    input: String,

    #[arg(short = 'o')]
    output: Option<PathBuf>,
}

fn read_file(path: PathBuf) -> Result<Vec<u8>, String> {
    let mut vec = Vec::new();
    let file = OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .open(path);
    if file.is_err() {
        return Err(format!(
            "Could not open file: {:?}",
            file.unwrap_err().to_string()
        ));
    }

    let mut file = file.unwrap();

    match file.read_to_end(&mut vec) {
        Ok(..) => {}
        Err(err) => {
            return Err(format!("Could not open file: {:?}", err.to_string()));
        }
    };

    Ok(vec)
}

fn write_to_file(instructions: &Vec<Instruction>, path: PathBuf) -> Result<(), String> {
    let file = OpenOptions::new()
        .read(false)
        .write(true)
        .append(false)
        .open(path);
    if file.is_err() {
        return Err(format!(
            "Could not open file: {:?}",
            file.unwrap_err().to_string()
        ));
    }
    let mut file = file.unwrap();
    for insn in instructions {
        if let Err(err) = writeln!(file, "{insn}") {
            return Err(format!("Could not write to file: {err}"));
        };
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
    let input_path = args.input;
    let bytes = match read_file(input_path.into()) {
        Ok(bytes) => bytes,
        Err(err) => {
            println!("Error: {err}");
            return;
        }
    };

    let mut parser = InstructionParser::new(bytes);

    let mut instruction_vector = vec![];

    while let Some(insn) = parser.parse() {
        instruction_vector.push(insn);
    }

    if let Some(path) = args.output {
        write_to_file(&instruction_vector, path).unwrap();
    } else {
        for insn in &instruction_vector {
            println!("{insn}");
        }
    }
}
