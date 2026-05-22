use rand::prelude::*;

pub const MEMORY_SIZE: usize = 1024;
pub const MAX_CYCLES: usize = 10_000;
pub const NUM_REGISTERS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Nop,
    Inc(u8),        // Reg
    Dec(u8),        // Reg
    Add(u8, u8),    // Dst, Src
    Sub(u8, u8),    // Dst, Src
    Mul(u8, u8),    // Dst, Src
    Div(u8, u8),    // Dst, Src
    Mov(u8, u8),    // Dst, Src
    Ldi(u8, i32),   // Dst, Imm
    Jmp(i32),       // Offset
    Jz(u8, i32),    // Reg, Offset
    IoOut(u8),      // Port
    Ld(u8, u8),     // Dst, AddrReg
    St(u8, u8),     // AddrReg, Src
    Hlt,
}

pub struct AstVM {
    pub program: Vec<Opcode>,
    pub memory: [u8; MEMORY_SIZE],
    pub registers: [i32; NUM_REGISTERS],
    pub ip: usize,
    pub halted: bool,
    pub cycles: usize,
    pub output_buffer: Vec<i32>,
}

impl AstVM {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
            memory: [0; MEMORY_SIZE],
            registers: [0; NUM_REGISTERS],
            ip: 0,
            halted: false,
            cycles: 0,
            output_buffer: Vec::with_capacity(64),
        }
    }

    pub fn load(&mut self, prog: &[Opcode]) {
        self.program = prog.to_vec();
        self.registers.fill(0);
        self.memory.fill(0);
        self.ip = 0;
        self.halted = false;
        self.cycles = 0;
        self.output_buffer.clear();
    }

    pub fn step(&mut self) {
        if self.halted || self.ip >= self.program.len() {
            self.halted = true;
            return;
        }

        if self.cycles >= MAX_CYCLES {
            self.halted = true;
            return;
        }

        let instr = self.program[self.ip];
        self.ip += 1;
        self.cycles += 1;

        match instr {
            Opcode::Nop => {}
            Opcode::Inc(r) => {
                let r = (r as usize) % NUM_REGISTERS;
                self.registers[r] = self.registers[r].wrapping_add(1);
            }
            Opcode::Dec(r) => {
                let r = (r as usize) % NUM_REGISTERS;
                self.registers[r] = self.registers[r].wrapping_sub(1);
            }
            Opcode::Add(d, s) => {
                let d = (d as usize) % NUM_REGISTERS;
                let s = (s as usize) % NUM_REGISTERS;
                self.registers[d] = self.registers[d].wrapping_add(self.registers[s]);
            }
            Opcode::Sub(d, s) => {
                let d = (d as usize) % NUM_REGISTERS;
                let s = (s as usize) % NUM_REGISTERS;
                self.registers[d] = self.registers[d].wrapping_sub(self.registers[s]);
            }
            Opcode::Mul(d, s) => {
                let d = (d as usize) % NUM_REGISTERS;
                let s = (s as usize) % NUM_REGISTERS;
                self.registers[d] = self.registers[d].wrapping_mul(self.registers[s]);
            }
            Opcode::Div(d, s) => {
                let d = (d as usize) % NUM_REGISTERS;
                let s = (s as usize) % NUM_REGISTERS;
                if self.registers[s] != 0 {
                    self.registers[d] = self.registers[d].wrapping_div(self.registers[s]);
                }
            }
            Opcode::Mov(d, s) => {
                let d = (d as usize) % NUM_REGISTERS;
                let s = (s as usize) % NUM_REGISTERS;
                self.registers[d] = self.registers[s];
            }
            Opcode::Ldi(d, imm) => {
                let d = (d as usize) % NUM_REGISTERS;
                self.registers[d] = imm;
            }
            Opcode::Jmp(offset) => {
                let next_ip = (self.ip as i32).wrapping_add(offset);
                if next_ip >= 0 && next_ip < self.program.len() as i32 {
                    self.ip = next_ip as usize;
                } else {
                    self.halted = true;
                }
            }
            Opcode::Jz(r, offset) => {
                let r = (r as usize) % NUM_REGISTERS;
                if self.registers[r] == 0 {
                    let next_ip = (self.ip as i32).wrapping_add(offset);
                    if next_ip >= 0 && next_ip < self.program.len() as i32 {
                        self.ip = next_ip as usize;
                    } else {
                        self.halted = true;
                    }
                }
            }
            Opcode::IoOut(r) => {
                let r = (r as usize) % NUM_REGISTERS;
                if self.output_buffer.len() < 64 {
                    self.output_buffer.push(self.registers[r]);
                }
            }
            Opcode::Ld(d, a) => {
                let d = (d as usize) % NUM_REGISTERS;
                let a = (a as usize) % NUM_REGISTERS;
                let addr = (self.registers[a] as usize) % MEMORY_SIZE;
                self.registers[d] = self.memory[addr] as i32;
            }
            Opcode::St(a, s) => {
                let a = (a as usize) % NUM_REGISTERS;
                let s = (s as usize) % NUM_REGISTERS;
                let addr = (self.registers[a] as usize) % MEMORY_SIZE;
                self.memory[addr] = (self.registers[s] & 0xFF) as u8;
            }
            Opcode::Hlt => {
                self.halted = true;
            }
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }

    pub fn run_with_radiation(&mut self, intensity: f64) {
        let mut rng = rand::rng();
        while !self.halted {
            if rng.random_bool(intensity) {
                let target_reg = rng.random_range(0..NUM_REGISTERS);
                let target_bit = rng.random_range(0..32);
                self.registers[target_reg] ^= 1 << target_bit;
            }
            if rng.random_bool(intensity * 0.1) {
                let target_addr = rng.random_range(0..MEMORY_SIZE);
                let target_bit = rng.random_range(0..8);
                self.memory[target_addr] ^= 1 << target_bit;
            }
            self.step();
        }
    }
}
