use crate::cpu65p64::addr::PhysAddr;

use std::{
    thread,
    sync::{
        mpsc,
        mpsc::{Sender, Receiver},
        Mutex,
        Arc
    },
    collections::HashMap
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MEMORY_BUFFER: Arc<Mutex<[u8; 256]>> = Arc::new(Mutex::new([0; 256]));
}
/*/
bitflags! {
    pub struct PageTableEntryFlags: u64 {
        /// Specifies whether the mapped frame or page table is loaded in memory.
        const PRESENT =         1;
        /// Controls whether writes to the mapped frames are allowed.
        ///
        /// If this bit is unset in a level 1 page table entry, the mapped frame is read-only.
        /// If this bit is unset in a higher level page table entry the complete range of mapped
        /// pages is read-only.
        const WRITABLE =        1 << 1;
        /// Controls whether accesses from userspace (i.e. ring 3) are permitted.
        const USER_ACCESSIBLE = 1 << 2;
        /// If this bit is set, a “write-through” policy is used for the cache, else a “write-back”
        /// policy is used.
        const WRITE_THROUGH =   1 << 3;
        /// Disables caching for the pointed entry is cacheable.
        const NO_CACHE =        1 << 4;
        /// Set by the CPU when the mapped frame or page table is accessed.
        const ACCESSED =        1 << 5;
        /// Set by the CPU on a write to the mapped frame.
        const DIRTY =           1 << 6;
        /// Specifies that the entry maps a huge frame instead of a page table. Only allowed in
        /// P2 or P3 tables.
        const HUGE_PAGE =       1 << 7;
        /// Indicates that the mapping is present in all address spaces, so it isn't flushed from
        /// the TLB on an address space switch.
        const GLOBAL =          1 << 8;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_9 =           1 << 9;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_10 =          1 << 10;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_11 =          1 << 11;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_52 =          1 << 52;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_53 =          1 << 53;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_54 =          1 << 54;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_55 =          1 << 55;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_56 =          1 << 56;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_57 =          1 << 57;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_58 =          1 << 58;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_59 =          1 << 59;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_60 =          1 << 60;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_61 =          1 << 61;
        /// Available to the OS, can be used to store additional data, e.g. custom flags.
        const BIT_62 =          1 << 62;
        /// Forbid code execution from the mapped frames.
        ///
        /// Can be only used when the no-execute page protection feature is enabled in the EFER
        /// register.
        const NO_EXECUTE =      1 << 63;
    }
}
*/


/*
Frame = 4KiB, discard the least significant 12 bits
Big frame = 512 frames, 2MiB, discard the least significant 21 bits
Huge frame = 256000 frames, 1GiB, discard the least significant 30 bits
*/

#[derive(Debug, Hash)]
pub struct Frame(pub [u8; 4096]);

impl Frame {
    pub fn new() -> Self {
        Self([0u8; 4096])
    }
}

/*

Video Addresses

Text = PhysAddr(0xB8000) Frame(0xB8) BigFrame()
Pixels = PhysAddr(0xA0000) Frame(0xA0)

*/

#[derive(Debug)]
pub enum MappedFrame {
    Memory(Frame),
    Video(usize, Sender<(usize, u8)>)
}

#[derive(Debug)]
pub struct MemMap(HashMap<usize, MappedFrame>);

impl MemMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn map(&mut self, frame_offset: usize, frame: MappedFrame) {
        self.0.insert(frame_offset, frame);
    }

    pub fn read(&self, addr: PhysAddr) -> u8 {
        let frame = addr.as_u64() >> 12;

        match &self.0[&(frame as usize)] {
            MappedFrame::Memory(frame) => {
                (*frame).0[0xfffusize & addr.as_u64() as usize]
            },
            MappedFrame::Video(plane, video_tx) => {
                0
            },
        }
    }

    /// Writes data onto a physical frame identified by a `PhysAddr`.
    pub fn write(&self, addr: PhysAddr, data: u8) -> Result<(), String> {
        let frame = addr.as_u64() >> 12;
        let offset = 0xfffusize & addr.as_u64() as usize;

        if self.0.contains_key(&(frame as usize)) {
            match &self.0[&(frame as usize)] {
                MappedFrame::Memory(frame) => {
                },
                MappedFrame::Video(plane, video_tx) => {
                    let index = ((*plane) << 12) + offset;
                    video_tx.send((index as usize, data)).unwrap();
                },
            };
            Ok(())
        }
        else {
            Err(format!("Physical frame 0x{frame:X} is not present."))
        }
    }
}
