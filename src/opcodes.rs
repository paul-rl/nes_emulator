use std::collections::HashMap;

use crate::cpu::AddressingMode;
pub struct OpCode {
    pub instruction: u8,
    pub name: String,
    pub num_bytes: u8,
    pub num_cycles: u8,
    pub mode: AddressingMode,
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
        OpCode::new(0xe4, "CPX".to_string(), 2, 3, AddressingMode::ZeroPage),
    );
    retval.insert(
        0xec,
        OpCode::new(0xec, "CPX".to_string(), 3, 4, AddressingMode::Absolute),
    );

    /* Branching */
    retval.insert(
        0x4c,
        OpCode::new(
            0x4c,
            "JMP".to_string(),
            3,
            3,
            AddressingMode::Absolute,
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
            "CLC".to_string(),
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

pub struct Instructions {
    pub map: HashMap<u8, OpCode>,
}

impl Instructions {
    pub fn new() -> Self {
        Instructions {
            map: make_opcode_hashtable(),
        }
    }
}
impl OpCode {
    pub fn new(
        instruction: u8,
        name: String,
        num_bytes: u8,
        num_cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        OpCode {
            instruction,
            name,
            num_bytes,
            num_cycles,
            mode,
        }
    }
}
