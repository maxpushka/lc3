use std::{
    io::Read,
    ops::{Index, IndexMut},
};

use crate::{
    defs::{FL, MR, R},
    terminal::check_key,
};

pub struct State {
    pub reg: Registers,
    pub mem: Memory,
    pub running: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            mem: Memory::new(),
            running: false,
        }
    }
}

pub struct Registers {
    reg: [u16; R::COUNT as usize],
}

impl Registers {
    fn new() -> Self {
        let mut state = Self {
            reg: [0; R::COUNT as usize],
        };

        // since exactly one condition flag should be set at any given time, set the Z flag
        state.reg[R::COND as usize] = FL::ZRO as u16;

        // set the PC to starting position
        // 0x3000 is the default
        const PC_START: u16 = 0x3000;
        state.reg[R::PC as usize] = PC_START;
        state
    }

    pub fn update_flags(&mut self, r: u16) {
        if self.reg[r as usize] == 0 {
            self.reg[R::COND as usize] = FL::ZRO as u16;
        } else if (self.reg[r as usize] >> 15) != 0 {
            // a 1 in the left-most bit indicates negative
            self.reg[R::COND as usize] = FL::NEG as u16;
        } else {
            self.reg[R::COND as usize] = FL::POS as u16;
        }
    }
}

impl Index<R> for Registers {
    type Output = u16;
    fn index<'a>(&'a self, i: R) -> &'a u16 {
        &self.reg[i as usize]
    }
}

impl IndexMut<R> for Registers {
    fn index_mut<'a>(&'a mut self, i: R) -> &'a mut u16 {
        &mut self.reg[i as usize]
    }
}

impl Index<u16> for Registers {
    type Output = u16;
    fn index<'a>(&'a self, i: u16) -> &'a u16 {
        &self.reg[i as usize]
    }
}

impl IndexMut<u16> for Registers {
    fn index_mut<'a>(&'a mut self, i: u16) -> &'a mut u16 {
        &mut self.reg[i as usize]
    }
}

pub const MEMORY_MAX: usize = 1 << 16;

pub struct Memory {
    data: [u16; MEMORY_MAX],
}

impl Memory {
    fn new() -> Self {
        Self {
            data: [0; MEMORY_MAX],
        }
    }

    pub fn read(&mut self, address: u16) -> u16 {
        if address == MR::KBSR as u16 {
            if check_key().unwrap() {
                self.data[MR::KBSR as usize] = 1 << 15;

                let mut buffer = [0u8; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                self.data[MR::KBDR as usize] = buffer[0] as u16;
            } else {
                self.data[MR::KBSR as usize] = 0;
            }
        }
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.data[address as usize] = value;
    }
}
