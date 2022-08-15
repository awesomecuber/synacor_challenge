use std::fs::write;

use anyhow::Result;
use computer::Computer;

mod computer;

fn main() -> Result<()> {
    let mut comp = Computer::from_file("challenge.bin")?;
    // let mut comp = Computer::new(&[9, 32768, 32769, 65, 19, 32768]);
    match comp.read_instructions_until_terminate() {
        computer::ExitCode::Halt => println!("Computer halted successfully"),
        computer::ExitCode::InvalidOperation => println!("Computer encountered invalid operation"),
        computer::ExitCode::OutOfOperations => println!("Computer ran out of operations"),
    }

    write("output.txt", comp.get_output())?;

    Ok(())
}
