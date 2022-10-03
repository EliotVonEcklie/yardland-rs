//! This module provides 65p64 specific functions and data structures,
//! and access to various processor registers.

#![warn(dead_code)]

pub mod addr;

#[derive(Debug)]
pub struct Cpu65p64 {

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

impl Cpu65p64 {
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
                self.a.0 = self.pc << 31;
                self.b.0 = self.pc;
            },
            _ => {},
        }
        
        self.pc += 1;

        return;
    }
}
