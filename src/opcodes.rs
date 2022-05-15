use crate::cpu::AddressingMode;
pub struct OpCode {
    instruction: u8,
    name: String,
    num_bytes: u8,
    num_cycles: u8,
    mode: AddressingMode
}

impl OpCode {
    pub fn new() -> Self{
        OpCode {
            instruction:0,
            name:String::new(),
            num_bytes:0,
            num_cycles: 0,
            mode: AddressingMode::NoneAddressing
        }
    }
}