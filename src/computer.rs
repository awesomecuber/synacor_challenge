use std::{collections::VecDeque, str};

pub enum ExitCode {
    Continue,
    Halt,
    InvalidOperation,
    OutOfOperations,
    NeedInput,
}

pub struct Computer {
    mem: Vec<u16>,
    cursor: usize,
    output: Vec<u8>,
    registers: [u16; 8],
    stack: Vec<u16>,
    input: VecDeque<u8>,
}

impl Computer {
    pub fn from_bytes(bytes: &[u8]) -> Computer {
        let nums: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|n| u16::from_le_bytes(n.try_into().unwrap()))
            .collect();
        Computer::new(&nums)
    }

    pub fn new(nums: &[u16]) -> Computer {
        let mut mem = nums.to_vec();
        mem.resize((1 << 15) - 1, 0);
        Computer {
            mem,
            cursor: 0,
            output: Vec::new(),
            registers: [0; 8],
            stack: Vec::new(),
            input: VecDeque::new(),
        }
    }

    pub fn add_input(&mut self, line: &[u8]) {
        self.input.extend(line);
        self.input.push_back('\n'.try_into().unwrap());
    }

    pub fn read_instructions_until_terminate(&mut self) -> ExitCode {
        loop {
            if self.cursor >= self.mem.len() {
                break;
            }
            let exit_code = match self.mem[self.cursor] {
                0 => ExitCode::Halt,
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
                _ => ExitCode::InvalidOperation,
            };
            match exit_code {
                ExitCode::Continue => {}
                _ => return exit_code,
            }
        }
        ExitCode::OutOfOperations
    }

    fn set_op(&mut self) -> ExitCode {
        let (reg, val) = self.get_two_arguments();
        self.set_reg(reg, val);
        ExitCode::Continue
    }

    fn push_op(&mut self) -> ExitCode {
        let val = self.get_one_argument();
        self.stack.push(self.read(val));
        ExitCode::Continue
    }

    fn pop_op(&mut self) -> ExitCode {
        let dest = self.get_one_argument();
        let val = self.stack.pop().expect("must have item in stack");
        self.set(dest, val);
        ExitCode::Continue
    }

    fn eq_op(&mut self) -> ExitCode {
        let (dest, left, right) = self.get_three_arguments();
        self.set(dest, (self.read(left) == self.read(right)) as u16);
        ExitCode::Continue
    }

    fn gt_op(&mut self) -> ExitCode {
        let (dest, left, right) = self.get_three_arguments();
        self.set(dest, (self.read(left) > self.read(right)) as u16);
        ExitCode::Continue
    }

    fn jmp_op(&mut self) -> ExitCode {
        let jump_to = self.get_one_argument();
        self.cursor = self.read(jump_to) as usize;
        ExitCode::Continue
    }

    fn jt_op(&mut self) -> ExitCode {
        let (val, jump_to) = self.get_two_arguments();
        if self.read(val) != 0 {
            self.cursor = self.read(jump_to) as usize;
        }
        ExitCode::Continue
    }

    fn jf_op(&mut self) -> ExitCode {
        let (val, jump_to) = self.get_two_arguments();
        if self.read(val) == 0 {
            self.cursor = self.read(jump_to) as usize;
        }
        ExitCode::Continue
    }

    fn add_op(&mut self) -> ExitCode {
        let (dest, left, right) = self.get_three_arguments();
        self.set_reg(
            dest,
            u16::wrapping_add(self.read(left), self.read(right)) % (1 << 15),
        );
        ExitCode::Continue
    }

    fn mult_op(&mut self) -> ExitCode {
        let (dest, left, right) = self.get_three_arguments();
        self.set_reg(
            dest,
            u16::wrapping_mul(self.read(left), self.read(right)) % (1 << 15),
        );
        ExitCode::Continue
    }

    fn mod_op(&mut self) -> ExitCode {
        let (dest, left, right) = self.get_three_arguments();
        self.set_reg(dest, self.read(left) % self.read(right));
        ExitCode::Continue
    }

    fn and_op(&mut self) -> ExitCode {
        let (dest, left, right) = self.get_three_arguments();
        self.set(dest, self.read(left) & self.read(right));
        ExitCode::Continue
    }

    fn or_op(&mut self) -> ExitCode {
        let (dest, left, right) = self.get_three_arguments();
        self.set(dest, self.read(left) | self.read(right));
        ExitCode::Continue
    }

    fn not_op(&mut self) -> ExitCode {
        let (dest, val) = self.get_two_arguments();
        let first_bit = 1 << 15;
        let flipped = if self.read(val) < first_bit {
            !self.read(val) - first_bit
        } else {
            !self.read(val) + first_bit
        };
        self.set(dest, flipped);
        ExitCode::Continue
    }

    fn rmem_op(&mut self) -> ExitCode {
        let (dest, val) = self.get_two_arguments();
        self.set(dest, self.mem[self.read(val) as usize]);
        ExitCode::Continue
    }

    fn wmem_op(&mut self) -> ExitCode {
        let (dest, val) = self.get_two_arguments();
        self.set(self.read(dest), self.read(val));
        ExitCode::Continue
    }

    fn call_op(&mut self) -> ExitCode {
        let jump_to = self.get_one_argument();
        self.stack.push(self.cursor as u16);
        self.cursor = self.read(jump_to) as usize;
        ExitCode::Continue
    }

    fn ret_op(&mut self) -> ExitCode {
        self.get_no_arguments();
        match self.stack.pop() {
            Some(addr) => self.cursor = addr as usize,
            None => return ExitCode::Halt,
        }
        ExitCode::Continue
    }

    fn out_op(&mut self) -> ExitCode {
        let char = self.get_one_argument();
        self.output
            .push(self.read(char).try_into().expect("not valid ascii"));
        ExitCode::Continue
    }

    fn in_op(&mut self) -> ExitCode {
        let dest = self.get_one_argument();
        match self.input.pop_front() {
            Some(char) => self.set(dest, char as u16),
            None => {
                self.cursor -= 2; // get back in original state
                return ExitCode::NeedInput;
            }
        }
        ExitCode::Continue
    }

    fn no_op(&mut self) -> ExitCode {
        self.get_no_arguments();
        ExitCode::Continue
    }

    fn read(&self, num: u16) -> u16 {
        if num < 32768 {
            return num;
        }
        if num <= 32775 {
            return self.registers[(num % 32768) as usize];
        }
        0
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
        } else {
            self.mem[dest as usize] = val;
        }
    }

    fn set_reg(&mut self, reg: u16, val: u16) {
        assert!((32768..=32775).contains(&reg), "invalid register");
        self.registers[(reg % 32768) as usize] = self.read(val);
    }

    pub fn get_output(&self) -> &[u8] {
        &self.output
    }

    pub fn get_output_as_string(&mut self) -> String {
        let to_return = str::from_utf8(&self.output).unwrap().to_owned();
        self.output = Vec::new();
        to_return
    }
}
