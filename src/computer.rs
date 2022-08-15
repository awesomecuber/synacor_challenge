use anyhow::Result;
use std::{path::Path, str};

pub enum ExitCode {
    Halt,
    InvalidOperation,
}

pub struct Computer {
    nums: Vec<u16>,
    cursor: usize,
    output: Vec<u8>,
}

impl Computer {
    pub fn from_file(file: impl AsRef<Path>) -> Result<Computer> {
        Ok(Computer {
            nums: std::fs::read(file)?
                .chunks_exact(2)
                .map(|n| u16::from_le_bytes(n.try_into().unwrap()))
                .collect(),
            cursor: 0,
            output: vec![],
        })
    }

    pub fn from_nums(nums: &[u16]) -> Computer {
        Computer {
            nums: nums.to_vec(),
            cursor: 0,
            output: vec![],
        }
    }

    pub fn read_instructions_until_terminate(&mut self) -> ExitCode {
        loop {
            match self.nums[self.cursor] {
                0 => return ExitCode::Halt,
                19 => {
                    self.output.push(
                        self.nums[self.cursor + 1]
                            .try_into()
                            .expect("not valid ascii"),
                    );
                    self.cursor += 2;
                }
                21 => self.cursor += 1,
                _ => return ExitCode::InvalidOperation,
            }
        }
    }

    pub fn get_output(&self) -> &[u8] {
        &self.output
    }

    pub fn get_output_as_string(&self) -> Result<String> {
        Ok(str::from_utf8(&self.output)?.to_owned())
    }
}
