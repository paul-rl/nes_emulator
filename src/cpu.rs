pub fn main(){

}
// CPU Registers:
pub struct CPU {
    pub register_a: u8, // Accumulator
    pub register_x: u8,
    // Represents status, each bit is a flag (in order below):
    // Negative, Overflow, ____ (unused, always set), Break,
    // Decimal, Interrupt Disable, Zero, Carry 
    // E.g. if value is 0100011: Zero, Overflow, Negative flags all set
    pub status: u8,
    pub program_counter: u16, // Holds address for next instruction
}

impl CPU {
    pub fn new() -> Self{
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0
        }    
    }

    // Interprets instructions.
    // Takes mutable reference to self to change registers and program instructions.
    pub fn interpret(&mut self, program: Vec<u8>){
        self.program_counter = 0;

        // CPU Cycle:
        // Fetch
        // Decode
        // Execute
        // Repeat
        

        '_cpu_cycle: loop {
            let opcode = program[self.program_counter as usize]; // Fetch
            self.program_counter += 1; // PC UPDATE

            match opcode { // DECODE, then on match EXECUTE 
                0xA9 => { //LDA: Load Accumulator with Memory
                    let param = program[self.program_counter as usize]; // byte after LDA due to PC UPDATE
                    self.program_counter += 1;
                    self.register_a = param;

                    // Set flags depending on Accumulator value
                    // Check if Accumulator is 0
                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010; // Set Zero
                    } else {
                        self.status = self.status & 0b1111_1011; // Unset Zero
                    }
                    // Check if Accumulator is negative (negative bit is set)
                    if self.register_a & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000; // Set Negative
                    } else {
                        self.status = self.status & 0b0111_1111; // Unset Negative
                    }
                }
                0x00 => {// BRK: Break
                    return;
                }
                0xAA => {// TAX: Transfer Accumulator to X
                    self.register_x = self.register_a; // Copy Accumulator onto Register X

                    // Set Zero flag if needed
                    if self.register_x == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    // Set Negative flag if needed
                    if self.register_x & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }
                }
                _ => todo!("")
            }
        } // REPEAT
    }
}

#[cfg(test)]
mod test { 
    use super::*; // all functions in parent

    #[test]
    fn test_0xa9_lda_immediate_load_date(){
        let mut cpu: CPU = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]); // LDA 0X05 BRK
        assert_eq!(cpu.register_a, 0x05); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xa9_lda_zero_flag(){
        let mut cpu: CPU = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);// LDA 0X00 BRK
        assert_eq!(cpu.register_a, 0x00); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010); // Zero flag set
    }
    #[test]
    fn test_0xa9_lda_negative_flag(){
        let mut cpu: CPU = CPU::new();
        cpu.interpret(vec![0xa9, 0b1000_0001, 0x00]); // LDA 0X41 BRK
        assert_eq!(cpu.register_a, 0b1000_0001); // Value loaded onto Accumulator
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000); // Negative flag set
    }
}