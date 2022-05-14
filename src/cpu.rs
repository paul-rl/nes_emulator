pub fn main() {}
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
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    // Interprets instructions.
    // Takes mutable reference to self to change registers and program instructions.
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        // CPU Cycle:
        // Fetch
        // Decode
        // Execute
        // Repeat

        '_cpu_cycle: loop {
            let opcode = program[self.program_counter as usize]; // Fetch
            self.program_counter += 1; // PC UPDATE

            match opcode {
                // DECODE, then on match EXECUTE
                0xA9 => {
                    //LDA: Load Accumulator with Memory
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);
                }

                0x00 => return, // BRK: Break

                0xAA => self.tax(), // TAX

                0xE8 => self.inx(), // INX
                _ => todo!(""),
            }
        } // REPEAT
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    // TAX: Transfer Accumulator to X
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    // INX: Increment index X by one
    fn inx(&mut self) {
        match self.register_x {
            0xff => self.register_x = 0,
            _ => self.register_x += 1,
        }

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
        cpu.interpret(vec![0xa9, 0x05, 0x00]); // LDA 0X05 BRK
        assert_eq!(cpu.register_a, 0x05); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu: CPU = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]); // LDA 0X00 BRK
        assert_eq!(cpu.register_a, 0x00); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0b0000_0010); // Zero flag set
    }
    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu: CPU = CPU::new();
        cpu.interpret(vec![0xa9, 0b1000_0001, 0x00]); // LDA 0X41 BRK
        assert_eq!(cpu.register_a, 0b1000_0001); // Value loaded onto Accumulator
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000); // Negative flag set
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xAA, 0x00]); // TAX BRK
        assert_eq!(cpu.register_a, 10); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x_negative() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1000_0001;
        cpu.interpret(vec![0xAA, 0x00]); // TAX BRK
        assert_eq!(cpu.register_a, 0b1000_0001); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 != 0); // Negative flag set
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x_zero() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0;
        cpu.interpret(vec![0xAA, 0x00]); // TAX BRK
        assert_eq!(cpu.register_a, 0); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 != 0); // Zero flag not set
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu: CPU = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu: CPU = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1);
    }
}
