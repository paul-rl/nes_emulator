use crate::cpu::AddressingMode;
pub struct OpCode {
    pub instruction: u8,
    pub name: String,
    pub num_bytes: u8,
    pub num_cycles: u8,
    pub mode: AddressingMode,
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
