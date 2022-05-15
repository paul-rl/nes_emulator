use std::collections::HashMap;

use crate::opcodes::OpCode;
pub fn main() {}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}
// CPU Registers:
pub struct CPU {
    pub register_a: u8, // Accumulator
    pub register_x: u8,
    pub register_y: u8,
    // Represents status, each bit is a flag (in order below):
    // Negative, Overflow, ____ (unused, always set), Break,
    // Decimal, Interrupt Disable, Zero, Carry
    // E.g. if value is 0100011: Zero, Overflow, Negative flags all set
    pub status: u8,
    pub program_counter: u16, // Holds address for next instruction
    pub memory: [u8; 0xFFFF], // 64 KiB array simulating memory
    pub op_code_hashtable: HashMap<u8, OpCode>,
}

pub fn make_opcode_hashtable() -> HashMap<u8, OpCode> {
    let mut retval: HashMap<u8, OpCode> = HashMap::new();

    retval.insert(
        0x00,
        OpCode::new(
            0x00,
            "BRK".to_string(),
            1,
            7,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0xea,
        OpCode::new(
            0xea,
            "NOP".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );

    /*Arithmetic*/
    retval.insert(
        0x69,
        OpCode::new(0x69, "ADC".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0x65,
        OpCode::new(0x65, "ADC".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x75,
        OpCode::new(0x75, "ADC".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0x6d,
        OpCode::new(0x6d, "ADC".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0x7d,
        OpCode::new(
            0x7d,
            "ADC".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
    );
    retval.insert(
        0x79,
        OpCode::new(
            0x79,
            "ADC".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
    );
    retval.insert(
        0x61,
        OpCode::new(
            0x61,
            "ADC".to_string(),
            2,
            6, /*+1 if page crossed*/
            AddressingMode::Indirect_X,
        ),
    );
    retval.insert(
        0x71,
        OpCode::new(
            0x71,
            "ADC".to_string(),
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
    );

    retval.insert(
        0xe9,
        OpCode::new(0xe9, "SBC".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0xe5,
        OpCode::new(0xe5, "SBC".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xf5,
        OpCode::new(0xf5, "SBC".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0xed,
        OpCode::new(0xed, "SBC".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0xfd,
        OpCode::new(
            0xfd,
            "SBC".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
    );
    retval.insert(
        0xf9,
        OpCode::new(
            0xf9,
            "SBC".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
    );
    retval.insert(
        0xe1,
        OpCode::new(
            0xe1,
            "SBC".to_string(),
            2,
            6, /*+1 if page crossed*/
            AddressingMode::Indirect_X,
        ),
    );
    retval.insert(
        0xf1,
        OpCode::new(
            0xf1,
            "SBC".to_string(),
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
    );

    retval.insert(
        0x29,
        OpCode::new(0x29, "AND".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0x25,
        OpCode::new(0x25, "AND".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x35,
        OpCode::new(0x35, "AND".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0x2d,
        OpCode::new(0x2d, "AND".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0x3d,
        OpCode::new(
            0x3d,
            "AND".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
    );
    retval.insert(
        0x39,
        OpCode::new(
            0x39,
            "AND".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
    );
    retval.insert(
        0x21,
        OpCode::new(
            0x21,
            "AND".to_string(),
            2,
            6, /*+1 if page crossed*/
            AddressingMode::Indirect_X,
        ),
    );
    retval.insert(
        0x31,
        OpCode::new(
            0x31,
            "AND".to_string(),
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
    );

    retval.insert(
        0x49,
        OpCode::new(0x49, "EOR".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0x45,
        OpCode::new(0x45, "EOR".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x55,
        OpCode::new(0x55, "EOR".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0x4d,
        OpCode::new(0x4d, "EOR".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0x5d,
        OpCode::new(
            0x5d,
            "EOR".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
    );
    retval.insert(
        0x59,
        OpCode::new(
            0x59,
            "EOR".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
    );
    retval.insert(
        0x41,
        OpCode::new(
            0x41,
            "EOR".to_string(),
            2,
            6, /*+1 if page crossed*/
            AddressingMode::Indirect_X,
        ),
    );
    retval.insert(
        0x51,
        OpCode::new(
            0x51,
            "EOR".to_string(),
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
    );

    retval.insert(
        0x09,
        OpCode::new(0x09, "ORA".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0x05,
        OpCode::new(0x05, "ORA".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x15,
        OpCode::new(0x15, "ORA".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0x0d,
        OpCode::new(0x0d, "ORA".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0x1d,
        OpCode::new(
            0x1d,
            "ORA".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
    );
    retval.insert(
        0x19,
        OpCode::new(
            0x19,
            "ORA".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
    );
    retval.insert(
        0x01,
        OpCode::new(
            0x01,
            "ORA".to_string(),
            2,
            6, /*+1 if page crossed*/
            AddressingMode::Indirect_X,
        ),
    );
    retval.insert(
        0x11,
        OpCode::new(
            0x11,
            "ORA".to_string(),
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
    );

    /*Shifts*/
    retval.insert(
        0x0a,
        OpCode::new(
            0x0a,
            "ASL".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x06,
        OpCode::new(0x06, "ASL".to_string(), 2, 5, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x16,
        OpCode::new(0x16, "ASL".to_string(), 2, 6, AddressingMode::ZeroPage_Y),
    );
    retval.insert(
        0x0e,
        OpCode::new(0x0e, "ASL".to_string(), 3, 6, AddressingMode::Absolute),
    );
    retval.insert(
        0x1e,
        OpCode::new(0x1e, "ASL".to_string(), 3, 7, AddressingMode::Absolute_X),
    );

    retval.insert(
        0x4a,
        OpCode::new(
            0x4a,
            "LSR".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x46,
        OpCode::new(0x46, "LSR".to_string(), 2, 5, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x56,
        OpCode::new(0x56, "LSR".to_string(), 2, 6, AddressingMode::ZeroPage_Y),
    );
    retval.insert(
        0x4e,
        OpCode::new(0x4e, "LSR".to_string(), 3, 6, AddressingMode::Absolute),
    );
    retval.insert(
        0x5e,
        OpCode::new(0x5e, "LSR".to_string(), 3, 7, AddressingMode::Absolute_X),
    );

    retval.insert(
        0x2a,
        OpCode::new(
            0x2a,
            "ROL".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x26,
        OpCode::new(0x26, "ROL".to_string(), 2, 5, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x36,
        OpCode::new(0x36, "ROL".to_string(), 2, 6, AddressingMode::ZeroPage_Y),
    );
    retval.insert(
        0x2e,
        OpCode::new(0x2e, "ROL".to_string(), 3, 6, AddressingMode::Absolute),
    );
    retval.insert(
        0x3e,
        OpCode::new(0x3e, "ROL".to_string(), 3, 7, AddressingMode::Absolute_X),
    );

    retval.insert(
        0x6a,
        OpCode::new(
            0x6a,
            "ROR".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x66,
        OpCode::new(0x66, "ROR".to_string(), 2, 5, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x76,
        OpCode::new(0x76, "ROR".to_string(), 2, 6, AddressingMode::ZeroPage_Y),
    );
    retval.insert(
        0x6e,
        OpCode::new(0x6e, "ROR".to_string(), 3, 6, AddressingMode::Absolute),
    );
    retval.insert(
        0x7e,
        OpCode::new(0x7e, "ROR".to_string(), 3, 7, AddressingMode::Absolute_X),
    );

    retval.insert(
        0xe6,
        OpCode::new(0xe6, "INC".to_string(), 2, 5, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xf6,
        OpCode::new(0xf6, "INC".to_string(), 2, 6, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0xee,
        OpCode::new(0xee, "INC".to_string(), 3, 6, AddressingMode::Absolute),
    );
    retval.insert(
        0xfe,
        OpCode::new(0xfe, "INC".to_string(), 3, 7, AddressingMode::Absolute_X),
    );

    retval.insert(
        0xe8,
        OpCode::new(
            0xe8,
            "INX".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0xc8,
        OpCode::new(
            0xc8,
            "INY".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );

    retval.insert(
        0xc6,
        OpCode::new(0xc6, "DEC".to_string(), 2, 5, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xd6,
        OpCode::new(0xd6, "DEC".to_string(), 2, 6, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0xce,
        OpCode::new(0xce, "DEC".to_string(), 3, 6, AddressingMode::Absolute),
    );
    retval.insert(
        0xde,
        OpCode::new(0xde, "DEC".to_string(), 3, 7, AddressingMode::Absolute_X),
    );

    retval.insert(
        0xca,
        OpCode::new(
            0xca,
            "DEX".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x88,
        OpCode::new(
            0x88,
            "DEY".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );

    retval.insert(
        0xc9,
        OpCode::new(0xc9, "CMP".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0xc5,
        OpCode::new(0xc5, "CMP".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xd5,
        OpCode::new(0xd5, "CMP".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0xcd,
        OpCode::new(0xcd, "CMP".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0xdd,
        OpCode::new(
            0xdd,
            "CMP".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
    );
    retval.insert(
        0xd9,
        OpCode::new(
            0xd9,
            "CMP".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
    );
    retval.insert(
        0xc1,
        OpCode::new(0xc1, "CMP".to_string(), 2, 6, AddressingMode::Indirect_X),
    );
    retval.insert(
        0xd1,
        OpCode::new(
            0xd1,
            "CMP".to_string(),
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
    );

    retval.insert(
        0xc0,
        OpCode::new(0xc0, "CPY".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0xc4,
        OpCode::new(0xc4, "CPY".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xcc,
        OpCode::new(0xcc, "CPY".to_string(), 3, 4, AddressingMode::Absolute),
    );

    retval.insert(
        0xe0,
        OpCode::new(0xe0, "CPX".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0xe4,
        OpCode::new(0xe4, "CPx".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xec,
        OpCode::new(0xec, "CPx".to_string(), 3, 4, AddressingMode::Absolute),
    );

    /* Branching */
    retval.insert(
        0x4c,
        OpCode::new(
            0x4c,
            "JMP".to_string(),
            3,
            3,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x6c,
        OpCode::new(
            0x6c,
            "JMP".to_string(),
            3,
            5,
            AddressingMode::NoneAddressing,
        ),
    );

    retval.insert(
        0x20,
        OpCode::new(
            0x20,
            "JSR".to_string(),
            3,
            6,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x60,
        OpCode::new(
            0x60,
            "RTS".to_string(),
            1,
            6,
            AddressingMode::NoneAddressing,
        ),
    );

    retval.insert(
        0x40,
        OpCode::new(
            0x40,
            "RTI".to_string(),
            1,
            6,
            AddressingMode::NoneAddressing,
        ),
    );

    retval.insert(
        0xd0,
        OpCode::new(
            0xd0,
            "BNE".to_string(),
            2,
            2, /*+1 if branch succeeds +2 if to a new page*/
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x70,
        OpCode::new(
            0x70,
            "BVS".to_string(),
            2,
            2, /*+1 if branch succeeds +2 if to a new page*/
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x50,
        OpCode::new(
            0x50,
            "BVC".to_string(),
            2,
            2, /*+1 if branch succeeds +2 if to a new page*/
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x30,
        OpCode::new(
            0x30,
            "BMI".to_string(),
            2,
            2, /*+1 if branch succeeds +2 if to a new page*/
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0xf0,
        OpCode::new(
            0xf0,
            "BEQ".to_string(),
            2,
            2, /*+1 if branch succeeds +2 if to a new page*/
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0xb0,
        OpCode::new(
            0xb0,
            "BCS".to_string(),
            2,
            2, /*+1 if branch succeeds +2 if to a new page*/
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x90,
        OpCode::new(
            0x90,
            "BCC".to_string(),
            2,
            2, /*+1 if branch succeeds +2 if to a new page*/
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x10,
        OpCode::new(
            0x10,
            "BPL".to_string(),
            2,
            2, /*+1 if branch succeeds +2 if to a new page*/
            AddressingMode::NoneAddressing,
        ),
    );

    retval.insert(
        0x24,
        OpCode::new(0x24, "BIT".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x2c,
        OpCode::new(0x2c, "BIT".to_string(), 3, 4, AddressingMode::Absolute),
    );

    /*Stores, Loads */
    retval.insert(
        0xa9,
        OpCode::new(0xa9, "LDA".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0xa5,
        OpCode::new(0xa5, "LDA".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xb5,
        OpCode::new(0xb5, "LDA".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0xad,
        OpCode::new(0xad, "LDA".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0xbd,
        OpCode::new(
            0xbd,
            "LDA".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
    );
    retval.insert(
        0xb9,
        OpCode::new(
            0xb9,
            "LDA".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_Y,
        ),
    );
    retval.insert(
        0xa1,
        OpCode::new(0xa1, "LDA".to_string(), 2, 6, AddressingMode::Indirect_X),
    );
    retval.insert(
        0xb1,
        OpCode::new(
            0xb1,
            "LDA".to_string(),
            2,
            5, /*+1 if page crossed*/
            AddressingMode::Indirect_Y,
        ),
    );

    retval.insert(
        0xa2,
        OpCode::new(0xa2, "LDX".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0xa6,
        OpCode::new(0xa6, "LDX".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xb6,
        OpCode::new(
            0xb6,
            "LDX".to_string(),
            2,
            4, /*+1 if page crossed*/
            AddressingMode::ZeroPage_Y,
        ),
    );
    retval.insert(
        0xae,
        OpCode::new(
            0xae,
            "LDX".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute,
        ),
    );
    retval.insert(
        0xbe,
        OpCode::new(0xbe, "LDX".to_string(), 3, 4, AddressingMode::Absolute_Y),
    );

    retval.insert(
        0xa0,
        OpCode::new(0xa0, "LDY".to_string(), 2, 2, AddressingMode::Immediate),
    );
    retval.insert(
        0xa4,
        OpCode::new(0xa4, "LDY".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xb4,
        OpCode::new(0xb4, "LDY".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0xac,
        OpCode::new(0xac, "LDY".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0xbc,
        OpCode::new(
            0xbc,
            "LDY".to_string(),
            3,
            4, /*+1 if page crossed*/
            AddressingMode::Absolute_X,
        ),
    );

    retval.insert(
        0x85,
        OpCode::new(0x85, "STA".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x95,
        OpCode::new(0x95, "STA".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0x8d,
        OpCode::new(0x8d, "STA".to_string(), 3, 4, AddressingMode::Absolute),
    );
    retval.insert(
        0x9d,
        OpCode::new(0x9d, "STA".to_string(), 3, 5, AddressingMode::Absolute_X),
    );
    retval.insert(
        0x99,
        OpCode::new(0x99, "STA".to_string(), 3, 5, AddressingMode::Absolute_Y),
    );
    retval.insert(
        0x81,
        OpCode::new(0x81, "STA".to_string(), 2, 6, AddressingMode::Indirect_X),
    );
    retval.insert(
        0x91,
        OpCode::new(0x91, "STA".to_string(), 2, 6, AddressingMode::Indirect_Y),
    );

    retval.insert(
        0x86,
        OpCode::new(0x86, "STX".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x96,
        OpCode::new(0x96, "STX".to_string(), 2, 4, AddressingMode::ZeroPage_Y),
    );
    retval.insert(
        0x8e,
        OpCode::new(0x8e, "STX".to_string(), 3, 4, AddressingMode::Absolute),
    );

    retval.insert(
        0x84,
        OpCode::new(0x84, "STY".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0x94,
        OpCode::new(0x94, "STY".to_string(), 2, 4, AddressingMode::ZeroPage_X),
    );
    retval.insert(
        0x8c,
        OpCode::new(0x8c, "STY".to_string(), 3, 4, AddressingMode::Absolute),
    );

    /* Flags Clear */
    retval.insert(
        0xd8,
        OpCode::new(
            0xd8,
            "CLD".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x58,
        OpCode::new(
            0x58,
            "CLI".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0xb8,
        OpCode::new(
            0xb8,
            "CLV".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x18,
        OpCode::new(
            0x18,
            "CLV".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x38,
        OpCode::new(
            0x38,
            "SEC".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x78,
        OpCode::new(
            0x78,
            "SEI".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0xf8,
        OpCode::new(
            0xf8,
            "SED".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );

    retval.insert(
        0xaa,
        OpCode::new(
            0xaa,
            "TAX".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0xa8,
        OpCode::new(
            0xa8,
            "TAY".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0xba,
        OpCode::new(
            0xba,
            "TSX".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x8a,
        OpCode::new(
            0x8a,
            "TXA".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x9a,
        OpCode::new(
            0x9a,
            "TXS".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x98,
        OpCode::new(
            0x98,
            "TYA".to_string(),
            1,
            2,
            AddressingMode::NoneAddressing,
        ),
    );

    /* Stack */
    retval.insert(
        0x48,
        OpCode::new(
            0x48,
            "PHA".to_string(),
            1,
            3,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x68,
        OpCode::new(
            0x68,
            "PLA".to_string(),
            1,
            4,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x08,
        OpCode::new(
            0x08,
            "PHP".to_string(),
            1,
            3,
            AddressingMode::NoneAddressing,
        ),
    );
    retval.insert(
        0x28,
        OpCode::new(
            0x28,
            "PLP".to_string(),
            1,
            4,
            AddressingMode::NoneAddressing,
        ),
    );

    retval
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
            op_code_hashtable: make_opcode_hashtable(),
        }
    }

    pub fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPage_X => {
                let pos: u8 = self.mem_read(self.program_counter);
                let addr: u16 = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos: u8 = self.mem_read(self.program_counter);
                let addr: u16 = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::Absolute_X => {
                let pos: u16 = self.mem_read_u16(self.program_counter);
                let addr: u16 = pos.wrapping_add(self.register_x as u16) as u16;
                addr
            }
            AddressingMode::Absolute_Y => {
                let pos: u16 = self.mem_read_u16(self.program_counter);
                let addr: u16 = pos.wrapping_add(self.register_y as u16) as u16;
                addr
            }
            AddressingMode::Indirect_X => {
                let pos: u8 = self.mem_read(self.program_counter);
                let ptr: u8 = pos.wrapping_add(self.register_x);
                let lo: u8 = self.mem_read(ptr as u16);
                let hi: u8 = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let pos: u8 = self.mem_read(self.program_counter);
                // Read lo then hi, little endian
                let lo: u8 = self.mem_read(pos as u16);
                let hi: u8 = self.mem_read(pos.wrapping_add(1) as u16);
                let deref_pos: u16 = (hi as u16) << 8 | (lo as u16);
                deref_pos.wrapping_add(self.register_y as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
    // Reads from given address in memory
    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    // Writes data to given address
    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    // NES uses little endian for u16: 0x8000 written as 00 80 (L to R)
    // When reading u16, read two consecutive registers, and switch their order around to get
    // the stored value
    pub fn mem_read_u16(&self, addr: u16) -> u16 {
        let lo: u16 = self.mem_read(addr) as u16;
        let hi: u16 = self.mem_read(addr + 1) as u16;
        (hi << 8) | lo
    }

    // When writing u16, take upper and lower 8 bits and store them in reverse order.
    pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
        let hi: u8 = (data >> 8) as u8; // Extract last 8 bits
        let lo: u8 = (data & 0xff) as u8; // Extract first 8 bits
        self.mem_write(addr, lo);
        self.mem_write(addr + 1, hi);
    }

    // Loads program into memory, resets registers, then runs the program
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    // Loads program into memory and sets PC to value found in 0xFFFC
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]); // Load program into memory
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    // Restore set of all registers and initialize PC to 2 byte value stored in 0xFFFC
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    // Interprets instructions.
    // Takes mutable reference to self to change registers and program instructions.
    pub fn run(&mut self) {
        // CPU Cycle:
        // Fetch
        // Decode
        // Execute
        // Repeat
        '_cpu_cycle: loop {
            let opcode: u8 = self.mem_read(self.program_counter); // Fetch
            self.program_counter += 1; // PC UPDATE

            match opcode {
                // DECODE, then on match EXECUTE
                0xA9 => {
                    //LDA: Load Accumulator with Memory, Immediate
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }

                0xAD => {
                    //LDA: Load Accumulator with Memory, Absolute
                    self.lda(&AddressingMode::Absolute);
                    self.program_counter += 1;
                }

                0xBD => {
                    //LDA: Load Accumulator with Memory, X-Indexed Absolute
                    self.lda(&AddressingMode::Absolute_X);
                    self.program_counter += 1;
                }

                0xB9 => {
                    //LDA: Load Accumulator with Memory, Y-Indexed Absolute
                    self.lda(&AddressingMode::Absolute_Y);
                    self.program_counter += 1;
                }

                0xA5 => {
                    //LDA: Load Accumulator with Memory, Zero Page
                    self.lda(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }

                0xB5 => {
                    //LDA: Load Accumulator with Memory, Zero Page X
                    self.lda(&AddressingMode::ZeroPage_X);
                    self.program_counter += 1;
                }

                0xA1 => {
                    //LDA: Load Accumulator with Memory, Indirect X
                    self.lda(&AddressingMode::Indirect_X);
                    self.program_counter += 1;
                }

                0xB1 => {
                    //LDA: Load Accumulator with Memory, Indirect Y
                    self.lda(&AddressingMode::Indirect_Y);
                    self.program_counter += 1;
                }

                0x8D => {}
                0x85 => {
                    self.sta(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }

                0x95 => {
                    self.sta(&AddressingMode::ZeroPage_X);
                    self.program_counter += 1;
                }

                0x00 => return, // BRK: Break

                0xAA => self.tax(), // TAX

                0xE8 => self.inx(), // INX
                _ => todo!(""),
            }
        } // REPEAT
    }

    fn lda(&mut self, mode: &AddressingMode) {
        self.register_a = self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_a);
    }

    // STA: Store Accumulator in Memory
    fn sta(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_a);
    }

    // TAX: Transfer Accumulator to X
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    // INX: Increment index X by one
    fn inx(&mut self) {
        print!("Pre {}", self.register_x);
        self.register_x = self.register_x.wrapping_add(1);
        print!("Post {}", self.register_x);

        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // Set flags depending on Accumulator value
        // Check if Accumulator is 0
        if result == 0 {
            self.status = self.status | 0b0000_0010; // Set Zero
        } else {
            self.status = self.status & 0b1111_1011; // Unset Zero
        }
        // Check if Accumulator is negative (negative bit is set)
        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000; // Set Negative
        } else {
            self.status = self.status & 0b0111_1111; // Unset Negative
        }
    }
}

#[cfg(test)]
mod test {
    use super::*; // all functions in parent

    // Tests for LDA:
    #[test]
    fn test_0xa9_lda_immediate_load_date() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]); // LDA 0X05 BRK
        assert_eq!(cpu.register_a, 0x05); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]); // LDA 0X00 BRK
        assert_eq!(cpu.register_a, 0x00); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010); // Zero flag set
    }
    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_run(vec![0xa9, 0b1000_0001, 0x00]); // LDA 0X41 BRK
        assert_eq!(cpu.register_a, 0b1000_0001); // Value loaded onto Accumulator
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000); // Negative flag set
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 10;
        cpu.load(vec![0xAA, 0x00]); // TAX BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_x, 10); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x_negative() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1000_0001;
        cpu.load(vec![0xaa, 0x00]); // TAX BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, 0b1000_0001); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 != 0); // Negative flag set
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x_zero() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0;
        cpu.load(vec![0xAA, 0x00]); // TAX BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, 0); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 != 0); // Zero flag not set
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu: CPU = CPU::new();
        cpu.register_x = 0xff;
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_program_load() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xe8, 0xe8, 0xe8, 0xaa, 0x00];
        cpu.load(program.clone());
        let mut i: u16 = 0;
        for data in program.iter() {
            let data_in_memory: u8 = cpu.memory[(0x8000 + i) as usize];
            i += 1;
            assert_eq!(data_in_memory, *data);
        }
    }

    #[test]
    fn test_mem_read() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xe8, 0xe8, 0xe8, 0xaa, 0x00];
        cpu.load(program.clone());
        let mut i: u16 = 0;
        for data in program.iter() {
            let data_in_memory: u8 = cpu.mem_read(0x8000 + i);
            i += 1;
            assert_eq!(data_in_memory, *data);
        }
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu: CPU = CPU::new();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }
}
