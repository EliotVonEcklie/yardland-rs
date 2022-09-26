//! Emulate the HS65P64.

#![warn(dead_code)]

#[derive(Debug)]
pub struct HS65P64 {

    // Accumulators

    pub a: u64,
    pub b: u64,
    pub c: u64,

    pub x: u64,
    pub y: u64,
    pub z: u64,

    pub r: [u64; 10],

    pub sp: u16,
    pub sb: u64,

    pub pc: u32,
    //pp: u16,
    //p: u16
}

impl HS65P64 {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,

            x: 0,
            y: 0,
            z: 0,

            r: [0; 10],

            sp: 0,
            sb: 0x100u64,
            
            pc: 0,
            //pp: 0,
            //p: 0b0000000000000100
        }
    }

    pub fn step(&mut self, ir: u16) {
        match ir {
            0x00 /* brk */ => {},
            0xea /* nop */ => {},
            0x42 /* test */ => {
                self.a = (self.pc as u64) << 31;
                self.b = self.pc as u64;
            },
            _ => {},
        }

        if self.pc == 0xFFFFFFFF { self.pc = 0 }
        self.pc += 1;

        return;
    }
}
