use crate::opcodes::Instructions;
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
    pub stack_ptr: u8,
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
            stack_ptr: 0,
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
        (hi << 8) | (lo as u16)
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
        // Get our instruction set
        let instructions: Instructions = Instructions::new();
        // CPU Cycle:
        // Fetch
        // Decode
        // Execute
        // Repeat
        '_cpu_cycle: loop {
            let opcode: u8 = self.mem_read(self.program_counter); // Fetch
            let operation: &OpCode = instructions
                .map
                .get(&opcode)
                .expect("Failed to get from map");
            let mode: &AddressingMode = &operation.mode;
            self.program_counter += 1; // PC UPDATE
            let first_program_counter: u16 = self.program_counter;

            println!(
                "Instruction: {}, Addressing Mode: {:?}",
                operation.name, operation.mode
            );
            // DECODE, then on match EXECUTE
            match opcode {
                0x00 => return,                                             // BRK: Break
                0xea => {}                                                  // NOP
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {} // ADC
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {} // SBC
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(&mode), // AND
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {} // EOR
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {} // ORA
                0x0a | 0x06 | 0x16 | 0x0e | 0x1e => {}                      // ASL
                0x4a | 0x46 | 0x56 | 0x4e | 0x5e => {}                      // LSR
                0x2a | 0x26 | 0x36 | 0x2e | 0x3e => {}                      // ROL
                0x6a | 0x66 | 0x76 | 0x6e | 0x7e => {}                      // ROR
                0xe6 | 0xf6 | 0xee | 0xfe => {}                             // INC
                0xE8 => self.inx(),                                         // INX
                0xc8 => {}                                                  // INY
                0xc6 | 0xd6 | 0xce | 0xde => {}                             // DEC
                0xca => {}                                                  // DEX
                0x88 => {}                                                  // DEY
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {} // CMP
                0xc0 | 0xc4 | 0xcc => {}                                    // CPY
                0xe0 | 0xe4 | 0xec => {}                                    // CPX
                0x4c | 0x6c => {}                                           // JMP
                0x20 => {}                                                  // JSR
                0x60 => {}                                                  // RTS
                0x40 => {}                                                  // RTI
                0xd0 => {}                                                  // BNE
                0x70 => {}                                                  // BVS
                0x50 => {}                                                  // BVC
                0x30 => {}                                                  // BMI
                0xf0 => {}                                                  // BEQ
                0xb0 => {}                                                  // BCS
                0x90 => {}                                                  // BCC
                0x10 => {}                                                  // BPL
                0x24 | 0x2c => {}                                           // BIT
                0xA9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&mode), // LDA
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => self.ldx(&mode),        // LDX
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => self.ldy(&mode),        // LDY
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(&mode), // STA
                0x86 | 0x96 | 0x8e => self.stx(&mode),                                    // STX
                0x84 | 0x94 | 0x8c => self.sty(&mode),                                    // STY
                0xd8 => {}                                                  // CLD
                0x58 => {}                                                  // CLI
                0xb8 => {}                                                  // CLV
                0x18 => {}                                                  // TODO: CLV
                0x38 => {}                                                  // SEC
                0x78 => {}                                                  // SEI
                0xf8 => {}                                                  // SED
                0xaa => self.tax(),                                         // TAX
                0xa8 => self.tay(),                                         // TAY
                0xba => self.tsx(),                                         // TSX
                0x8A => self.txa(),                                                  // TXA
                0x9a => self.txs(),                                                  // TXS
                0x98 => self.tya(),                                                  // TYA
                0x48 => {}                                                  // PHA
                0x68 => {}                                                  // PLA
                0x08 => {}                                                  // PHP
                0x28 => {}                                                  // PLP

                _ => todo!(""),
            }

            // PC hasn't changed, so no branching
            if first_program_counter == self.program_counter {
                // -1 because already moved up the instruction that was read
                self.program_counter += (operation.num_bytes - 1) as u16;
            }
        } // REPEAT
    }
    // LDA: Load Accumulator to Memory
    fn lda(&mut self, mode: &AddressingMode) {
        self.register_a = self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_a);
    }
    // LDX: Load Index Register X From Memory
    fn ldx(&mut self, mode: &AddressingMode) {
        self.register_x = self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_x);
    }
    // LDY: Load Index Register Y From Memory
    fn ldy(&mut self, mode: &AddressingMode) {
        self.register_y = self.mem_read(self.get_operand_address(mode));
        self.update_zero_and_negative_flags(self.register_y);
    }
    // STA: Store Accumulator in Memory
    fn sta(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_a);
    }
    // STY: Store Accumulator in Memory
    fn sty(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_y);
    }
    // STX: Store Accumulator in Memory
    fn stx(&mut self, mode: &AddressingMode) {
        self.mem_write(self.get_operand_address(mode), self.register_x);
    }
    // TAX: Transfer Accumulator to X
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }
    // TAY: Transfer Accumulator to Y
    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }
    // TSX: Transfer Stack Pointer to X
    fn tsx(&mut self) {
        self.register_x = self.stack_ptr;
        self.update_zero_and_negative_flags(self.register_x);
    }
    // TXA: Transfer X to Accumulator
    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }
    // TXS: Transfer X to Stack Pointer
    fn txs(&mut self) {
        self.stack_ptr = self.register_x;
    }
    // TYA: Transfer Y to Accumulator
    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }
    // INX: Increment index X by one
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);

        self.update_zero_and_negative_flags(self.register_x);
    }
    // AND: "AND" Memory with Accumulator
    fn and(&mut self, mode: &AddressingMode){
        let data: u8 = self.mem_read(self.get_operand_address(mode) as u16);
        println!("data is: {:b}", data);
        println!("rega is: {:b}", self.register_a);
        self.register_a = self.register_a & data;
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
    fn test_lda_from_memory() {
        let mut cpu: CPU = CPU::new();
        cpu.mem_write(0x10, 0x55);
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }
    #[test]
    fn test_0xa9_lda_immediate_load() {
        let mut cpu: CPU = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]); // LDA 0x05 BRK
        assert_eq!(cpu.register_a, 0x05); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xad_lda_absolute_load() {
        let mut cpu: CPU = CPU::new();
        cpu.memory[0x2805] = 22;
        cpu.load(vec![0xad, 0x05, 0x28, 0x00]); // LDA Absolute 0x0005 BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, 22); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xbd_lda_absolute_x_load() {
        let mut cpu: CPU = CPU::new();
        cpu.register_x = 1;
        cpu.mem_write((0x2805 as u16).wrapping_add(cpu.register_x as u16), 0x16);
        cpu.load(vec![0xbd, 0x05, 0x28, 0x00]); // LDA Absolute 0x0005 BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, 0x16); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xb9_lda_absolute_y_load() {
        let mut cpu: CPU = CPU::new();
        cpu.register_y = 1;
        cpu.mem_write((0x2805 as u16).wrapping_add(cpu.register_y as u16), 0x16);
        cpu.load(vec![0xb9, 0x05, 0x28, 0x00]); // LDA Absolute 0x0005 BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, 0x16); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xa5_lda_zero_page_load() {
        let mut cpu: CPU = CPU::new();
        cpu.mem_write(0x05, 0x16);
        cpu.load_and_run(vec![0xa5, 0x05, 0x00]); // LDA Absolute 0x0005 BRK

        assert_eq!(cpu.register_a, 0x16); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xb5_lda_zero_page_x_load() {
        let mut cpu: CPU = CPU::new();
        cpu.register_x = 1;
        cpu.mem_write((0x05 as u8).wrapping_add(cpu.register_x) as u16, 0x16);
        cpu.load(vec![0xb5, 0x05, 0x00]); // LDA Absolute 0x0005 BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, 0x16); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xa1_lda_zero_page_x_indirect_load() {
        let mut cpu: CPU = CPU::new();
        cpu.register_x = 1;

        cpu.load(vec![0xa1, 0x05, 0x00]); // LDA Absolute 0x0005 BRK
        cpu.mem_write(cpu.get_operand_address(&AddressingMode::Indirect_X), 0x16);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, 0x16); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xb1_lda_zero_page_y_indirect_load() {
        let mut cpu: CPU = CPU::new();
        cpu.register_y = 1;
        cpu.load(vec![0xb1, 0x05, 0x00]); // LDA Absolute 0x0005 BRK
        cpu.mem_write(cpu.get_operand_address(&AddressingMode::Indirect_Y), 0x16);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, 0x16); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
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
    // Test for LDX:
    fn test_0xa2_ldx_immediate_load() {
        let mut cpu: CPU = CPU::new();
        cpu.load(vec![0xa2, 0x05, 0x00]); // LDA 0x05 BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_x, 0x05); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    // Since we have already tested addresing modes, we can skip this.
    // If time allows, make test for each opcode
    // Test for LDY:
    #[test]
    fn test_0xa2_ldy_immediate_load() {
        let mut cpu: CPU = CPU::new();
        cpu.load(vec![0xa2, 0x05, 0x00]); // LDA 0x05 BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_x, 0x05); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
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
        assert_eq!(cpu.register_x, 0b1000_0001); // Value loaded onto Accumulator
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
        assert_eq!(cpu.register_x, 0); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 != 0); // Zero flag not set
    }
    #[test]
    fn test_0xa8_tay_move_a_to_y() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 10;
        cpu.load(vec![0xa8, 0x00]); // TAX BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_y, 10); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0xba_tsx_move_stack_ptr_to_x() {
        let mut cpu: CPU = CPU::new();
        cpu.stack_ptr = 10;
        cpu.load(vec![0xba, 0x00]); // TAX BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_x, 10); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0x8a_txa_move_x_to_a() {
        let mut cpu: CPU = CPU::new();
        cpu.register_x = 10;
        cpu.load(vec![0x8a, 0x00]); // TAX BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, 10); // Value loaded onto Accumulator
        assert!(cpu.status & 0b0000_0010 == 0); // Zero flag not set
        assert!(cpu.status & 0b1000_0000 == 0); // Negative flag not set
    }
    #[test]
    fn test_0x9a_txs_move_x_to_stack_ptr() {
        let mut cpu: CPU = CPU::new();
        cpu.register_x = 10;
        cpu.load(vec![0x9a, 0x00]); // TAX BRK
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.stack_ptr, 10); // Value loaded onto Accumulator
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
    fn test_mem_write() {
        let mut cpu: CPU = CPU::new();
        cpu.mem_write(0xe8, 0xe8);
        assert_eq!(cpu.memory[0xe8], 0xe8);
    }
    // Test for AND
    #[test]
    fn test_0x29_and_immediate(){
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1111_0001;
        let and_result: u8 = cpu.register_a & 0b1010_1111;
        cpu.load(vec![0x29, 0b1010_1111, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, and_result);
    }
    #[test]
    fn test_0x8d_sta_absolute(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x8d, 0x34, 0x12, 0x00];

        cpu.register_a = 10;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_a);
    }
    #[test]
    fn test_0x8e_stx_absolute(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x8e, 0x34, 0x12, 0x00];

        cpu.register_x = 10;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_x);
    }
    #[test]
    fn test_0x8c_sty_absolute(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x8c, 0x34, 0x12, 0x00];

        cpu.register_y = 10;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_y);
    }
}
