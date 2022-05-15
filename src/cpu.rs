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
