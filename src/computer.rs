use anyhow::Result;
use std::{path::Path, str};

pub enum ExitCode {
    Halt,
    InvalidOperation,
    OutOfOperations,
}

pub struct Computer {
    mem: Vec<u16>,
    cursor: usize,
    output: Vec<u8>,
    registers: [u16; 8],
    stack: Vec<u16>,
}

impl Computer {
    pub fn from_file(file: impl AsRef<Path>) -> Result<Computer> {
        let nums: Vec<u16> = std::fs::read(file)?
            .chunks_exact(2)
            .map(|n| u16::from_le_bytes(n.try_into().unwrap()))
            .collect();
        Ok(Computer::new(&nums))
    }

    pub fn new(nums: &[u16]) -> Computer {
        Computer {
            mem: nums.to_vec(),
            cursor: 0,
            output: vec![],
            registers: [0; 8],
            stack: vec![],
        }
    }

    pub fn read_instructions_until_terminate(&mut self) -> ExitCode {
        loop {
            if self.cursor >= self.mem.len() {
                break;
            }
            match self.mem[self.cursor] {
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
        ExitCode::OutOfOperations
    }

    fn set_op(&mut self) {
        let (reg, val) = self.get_two_arguments();
        self.set_reg(reg, val);
    }

    fn push_op(&mut self) {
        let val = self.get_one_argument();
        self.stack.push(self.read(val));
    }

    fn pop_op(&mut self) {
        let dest = self.get_one_argument();
        let val = self.stack.pop().expect("must have item in stack");
        self.set(dest, val);
    }

    fn eq_op(&mut self) {
        let (dest, left, right) = self.get_three_arguments();
        self.set(dest, (self.read(left) == self.read(right)) as u16);
    }

    fn gt_op(&mut self) {
        let (dest, left, right) = self.get_three_arguments();
        self.set(dest, (self.read(left) > self.read(right)) as u16);
    }

    fn jmp_op(&mut self) {
        let jump_to = self.get_one_argument();
        self.cursor = self.read(jump_to) as usize;
    }

    fn jt_op(&mut self) {
        let (val, jump_to) = self.get_two_arguments();
        if self.read(val) != 0 {
            self.cursor = self.read(jump_to) as usize;
        }
    }

    fn jf_op(&mut self) {
        let (val, jump_to) = self.get_two_arguments();
        if self.read(val) == 0 {
            self.cursor = self.read(jump_to) as usize;
        }
    }

    fn add_op(&mut self) {
        let (dest, left, right) = self.get_three_arguments();
        self.set_reg(dest, (self.read(left) + self.read(right)) % 32768);
    }

    fn mult_op(&mut self) {
        todo!()
    }

    fn mod_op(&mut self) {
        todo!()
    }

    fn and_op(&mut self) {
        let (dest, left, right) = self.get_three_arguments();
        self.set(dest, self.read(left) & self.read(right));
    }

    fn or_op(&mut self) {
        let (dest, left, right) = self.get_three_arguments();
        self.set(dest, self.read(left) | self.read(right));
    }

    fn not_op(&mut self) {
        let (dest, val) = self.get_two_arguments();
        self.set(dest, !self.read(val));
    }

    fn rmem_op(&mut self) {
        todo!()
    }

    fn wmem_op(&mut self) {
        todo!()
    }

    fn call_op(&mut self) {
        todo!()
    }

    fn ret_op(&mut self) {
        todo!()
    }

    fn out_op(&mut self) {
        let char = self.get_one_argument();
        self.output
            .push(self.read(char).try_into().expect("not valid ascii"));
    }

    fn in_op(&mut self) {
        todo!()
    }

    fn no_op(&mut self) {
        self.get_no_arguments();
    }

    fn read(&self, num: u16) -> u16 {
        assert!(num <= 32775, "num is invalid");
        if num < 32768 {
            return num;
        }
        self.registers[(num % 32768) as usize]
    }

    fn get_no_arguments(&mut self) {
        self.cursor += 1;
    }

    fn get_one_argument(&mut self) -> u16 {
        self.cursor += 2;
        self.mem[self.cursor - 1]
    }

    fn get_two_arguments(&mut self) -> (u16, u16) {
        self.cursor += 3;
        (self.mem[self.cursor - 2], self.mem[self.cursor - 1])
    }

    fn get_three_arguments(&mut self) -> (u16, u16, u16) {
        self.cursor += 4;
        (
            self.mem[self.cursor - 3],
            self.mem[self.cursor - 2],
            self.mem[self.cursor - 1],
        )
    }

    fn set(&mut self, dest: u16, val: u16) {
        if dest >= 32768 {
            self.set_reg(dest, val)
        } else if self.mem.len() <= dest as usize {
            self.mem.resize((dest + 1) as usize, 0);
        }
    }

    fn set_reg(&mut self, reg: u16, val: u16) {
        assert!((32768..=32775).contains(&reg), "invalid register");
        self.registers[(reg % 32768) as usize] = self.read(val);
    }

    pub fn get_output(&self) -> &[u8] {
        &self.output
    }

    pub fn _get_output_as_string(&self) -> Result<String> {
        Ok(str::from_utf8(&self.output)?.to_owned())
    }
}
