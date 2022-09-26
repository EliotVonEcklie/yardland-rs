pub fn get_test_instructions() -> [u16; 3] {
    [
        0x00, // brk
        0xea, // nop
        0x42, // test
        
    ]
}
