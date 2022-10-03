//! Physical and virtual addresses.

use core::convert::TryFrom;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysAddr(u64);

impl PhysAddr {
    pub fn new(addr: u64) -> Self {
        Self(addr)
    }
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for PhysAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::LowerHex for PhysAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl std::fmt::UpperHex for PhysAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct VirtAddr (u64);
