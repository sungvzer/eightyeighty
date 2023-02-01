use std::{fs::OpenOptions, io::Read, path::PathBuf};

use clap::Parser;
use emulator::parser::InstructionParser;

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

    while let Some(insn) = parser.parse() {
        println!("{insn:?}");
    }
}
