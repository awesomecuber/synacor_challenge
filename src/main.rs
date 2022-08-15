use std::fs::write;

use anyhow::Result;
use computer::Computer;

mod computer;

fn main() -> Result<()> {
    let mut comp = Computer::from_file("challenge.bin")?;
    match comp.read_instructions_until_terminate() {
        computer::ExitCode::Halt => {}
        computer::ExitCode::InvalidOperation => {}
    }

    write("output.txt", comp.get_output())?;

    Ok(())
}
