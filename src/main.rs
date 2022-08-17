use std::io::stdin;

use anyhow::Result;
use computer::Computer;

use crate::computer::ExitCode;

mod computer;

fn main() {
    let mut comp = Computer::from_bytes(include_bytes!("../challenge.bin"));

    comp.read_instructions_until_terminate();
    println!("{}", comp.get_output_as_string());
    loop {
        let result = comp.read_instructions_until_terminate();
        println!("{}", comp.get_output_as_string());
        match result {
            ExitCode::Continue => {
                println!("Computer not complete");
                break;
            }
            ExitCode::Halt => {
                println!("Computer halted successfully");
                break;
            }
            ExitCode::InvalidOperation => {
                println!("Computer encountered invalid operation");
                break;
            }
            ExitCode::OutOfOperations => {
                println!("Computer ran out of operations");
                break;
            }
            ExitCode::NeedInput => match get_line() {
                Ok(line) => comp.add_input(&line),
                Err(_) => println!("Something went wrong, try again?"),
            },
        }
    }

    // print!("{}", comp.get_output_as_string());
    // write("output.txt", comp.get_output());
}

fn get_line() -> Result<Vec<u8>> {
    let mut line = "".to_string();
    stdin().read_line(&mut line)?;
    Ok(line.trim_end().try_into()?)
}
