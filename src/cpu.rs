//! Emulate the HS65P64.

#![warn(dead_code)]

#[derive(Debug)]
pub struct HS65P64 {

    // Accumulators

    pub a: (u64, u64),
    pub b: (u64, u64),

    // Indexes

    pub x: u64, // Canonical, i.e only 48 bits.
    pub y: u64, // Canonical, i.e only 48 bits.

    pub r: [u64; 16],

    pub sp: u64, // Canonical, i.e only 48 bits. Stack Pointer.
    pub pc: u64, // Canonical, i.e only 48 bits. Program Counter.
    pub sr: u16,
}

impl HS65P64 {
    pub fn new() -> Self {
        Self {
            a: (0, 0),
            b: (0, 0),

            x: 0,
            y: 0,

            r: [0; 16],

            sp: 0x100u64,
            
            pc: 0,
            sr: 0b0000000000000100, // - - - - - - - - N V M X D I Z C
                                    // 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0
        }
    }

    pub fn step(&mut self, ir: u16) {
        match ir {
            0x00 /* brk */ => {},
            0xea /* nop */ => {},
            0x42 /* test */ => {
                self.a.0 = (self.pc as u64) << 31;
                self.b.0 = self.pc as u64;
            },
            _ => {},
        }
        
        self.pc += 1;

        return;
    }
}
