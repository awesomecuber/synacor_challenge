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
                1 => self.set_op(),
                2 => self.push_op(),
                3 => self.pop_op(),
                4 => self.eq_op(),
                5 => self.gt_op(),
                6 => self.jmp_op(),
                7 => self.jt_op(),
                8 => self.jf_op(),
                9 => self.add_op(),
                10 => self.mult_op(),
                11 => self.mod_op(),
                12 => self.and_op(),
                13 => self.or_op(),
                14 => self.not_op(),
                15 => self.rmem_op(),
                16 => self.wmem_op(),
                17 => self.call_op(),
                18 => self.ret_op(),
                19 => self.out_op(),
                20 => self.in_op(),
                21 => self.no_op(),
                _ => return ExitCode::InvalidOperation,
            }
        }
    }

    fn set_op(&mut self) {}

    fn push_op(&mut self) {}

    fn pop_op(&mut self) {}

    fn eq_op(&mut self) {}

    fn gt_op(&mut self) {}

    fn jmp_op(&mut self) {}

    fn jt_op(&mut self) {}

    fn jf_op(&mut self) {}

    fn add_op(&mut self) {}

    fn mult_op(&mut self) {}

    fn mod_op(&mut self) {}

    fn and_op(&mut self) {}

    fn or_op(&mut self) {}

    fn not_op(&mut self) {}

    fn rmem_op(&mut self) {}

    fn wmem_op(&mut self) {}

    fn call_op(&mut self) {}

    fn ret_op(&mut self) {}

    fn out_op(&mut self) {
        self.output.push(
            self.nums[self.cursor + 1]
                .try_into()
                .expect("not valid ascii"),
        );
        self.cursor += 2;
    }

    fn in_op(&mut self) {}

    fn no_op(&mut self) {
        self.cursor += 1
    }

    pub fn get_output(&self) -> &[u8] {
        &self.output
    }

    pub fn get_output_as_string(&self) -> Result<String> {
        Ok(str::from_utf8(&self.output)?.to_owned())
    }
}
