
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
    pub stack_start: u16,
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
            stack_ptr: 0xff,
            stack_start: 0x0100,
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
    pub fn print_memory(&self){
        let mut addr = 0;
        for item in self.memory.iter() {
            println!("0x{:x}: 0x{:x}", addr, item);
            addr += 1;
        }
    }
    pub fn stack_push(&mut self, data: u8) {
        self.mem_write(self.stack_start + self.stack_ptr as u16, data);
        self.stack_ptr = self.stack_ptr.wrapping_sub(1);
    }
    pub fn stack_pop(&mut self) -> u8{
        self.stack_ptr = self.stack_ptr.wrapping_add(1);
        self.mem_read(self.stack_start + self.stack_ptr as u16)
    }
    pub fn stack_push_u16(&mut self, data: u16) { 
        self.stack_push((data >> 8) as u8); // push hi
        self.stack_push((data & 0x00ff) as u8); // push lo
    }
    pub fn stack_pop_u16(&mut self) -> u16 {
        let lo: u16 = self.stack_pop() as u16;
        let hi: u16 = self.stack_pop() as u16;
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
        self.load_at(program, 0x8000);
    }
    pub fn load_at(&mut self, program: Vec<u8>, addr: u16) {
        self.memory[addr as usize..(addr as usize + program.len())].copy_from_slice(&program[..]); // Load program into memory
        self.mem_write_u16(0xFFFC, addr);
    }
    // Restore set of all registers and initialize PC to 2 byte value stored in 0xFFFC
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }
    pub fn run(&mut self) { 
        self.run_with_callback(|_| {});
    }
    // Interprets instructions.
    // Takes mutable reference to self to change registers and program instructions.
    pub fn run_with_callback<F>(&mut self, mut callback: F) where F: FnMut(&mut CPU),{
        // Get our instruction set
        let instructions: Instructions = Instructions::new();
        // CPU Cycle:
        // Fetch
        // Decode
        // Execute
        // Repeat
        '_cpu_cycle: loop {
            callback(self);
            
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
                0x00 => return,                                             // BRK
                0xea => println!("NOP!"),                                                  // NOP
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => self.adc(&mode), // ADC
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => self.sbc(&mode), // SBC
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(&mode), // AND
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => self.eor(&mode), // EOR
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => self.ora(&mode), // ORA
                0x0a | 0x06 | 0x16 | 0x0e | 0x1e => self.asl(&mode),        // ASL
                0x4a | 0x46 | 0x56 | 0x4e | 0x5e => self.lsr(&mode),                     // LSR
                0x2a | 0x26 | 0x36 | 0x2e | 0x3e => self.rol(&mode),                      // ROL
                0x6a | 0x66 | 0x76 | 0x6e | 0x7e => self.ror(&mode),                      // ROR
                0xe6 | 0xf6 | 0xee | 0xfe => self.inc(&mode),                             // INC
                0xe8 => self.inx(),                                         // INX
                0xc8 => self.iny(),                                                  // INY
                0xc6 | 0xd6 | 0xce | 0xde => self.dec(&mode),                             // DEC
                0xca => self.dex(),                                                  // DEX
                0x88 => self.dey(),                                                  // DEY
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => self.cmp(&mode), // CMP
                0xc0 | 0xc4 | 0xcc => self.cpy(&mode),                                    // CPY
                0xe0 | 0xe4 | 0xec => self.cpx(&mode),                                    // CPX
                0x4c | 0x6c => self.jmp(&mode),                                           // JMP
                0x20 => self.jsr(),                                                  // JSR
                0x60 => self.rts(),                                                 // RTS
                0x40 => self.rti(),                                                  // RTI
                0xd0 => self.bne(),                                                  // BNE
                0x70 => self.bvs(),                                                  // BVS
                0x50 => self.bvc(),                                                  // BVC
                0x30 => self.bmi(),                                                  // BMI
                0xf0 => self.beq(),                                                  // BEQ
                0xb0 => self.bcs(),                                                  // BCS
                0x90 => self.bcc(),                                                  // BCC
                0x10 => self.bpl(),                                                  // BPL
                0x24 | 0x2c => self.bit(&mode),                                           // BIT
                0xA9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&mode), // LDA
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => self.ldx(&mode),        // LDX
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => self.ldy(&mode),        // LDY
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(&mode), // STA
                0x86 | 0x96 | 0x8e => self.stx(&mode),                      // STX
                0x84 | 0x94 | 0x8c => self.sty(&mode),                      // STY
                0xd8 => self.cld(),                                                  // CLD
                0x58 => self.cli(),                                         // CLI
                0xb8 => self.clv(),                                                  // CLV
                0x18 => self.clc(),                                                  // CLC
                0x38 => self.sec(),                                                  // SEC
                0x78 => self.sei(),                                                  // SEI
                0xf8 => self.sed(),                                                  // SED
                0xaa => self.tax(),                                         // TAX
                0xa8 => self.tay(),                                         // TAY
                0xba => self.tsx(),                                         // TSX
                0x8A => self.txa(),                                         // TXA
                0x9a => self.txs(),                                         // TXS
                0x98 => self.tya(),                                         // TYA
                0x48 => self.pha(),                                         // PHA
                0x68 => self.pla(),                                         // PLA
                0x08 => self.php(),                                         // PHP
                0x28 => self.plp(),                                         // PLP

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
    // PHA: Push Accumulator On Stack
    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }
    // PHA: Push Status On Stack
    fn php(&mut self) {
        self.status = self.status | 0b0011_0000;
        self.stack_push(self.status);
    }
    // PLA: Pull Accumulator From Stack
    fn pla(&mut self) {
        self.register_a = self.stack_pop();
        self.update_zero_and_negative_flags(self.register_a);
    }
    // PLP: Pull Status From Stack
    fn plp(&mut self) {
        self.status = self.stack_pop();
        self.status = self.status | 0b0010_0000; // Set Break2
        self.status = self.status & 0b1110_1111; // Unset Break
    }
    // INC: Increment Memory by One
    fn inc(&mut self, mode: &AddressingMode) {
        let addr: u16 = self.get_operand_address(mode);
        self.mem_write_u16(addr, self.mem_read_u16(addr).wrapping_add(1));
        self.update_zero_and_negative_flags(self.mem_read_u16(addr) as u8);
    }
    // INX: Increment index X by one
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    // INY: Incement Index Y by One
    fn iny(&mut self){
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
    // AND: "AND" Memory with Accumulator
    fn and(&mut self, mode: &AddressingMode) {
        let data: u8 = self.mem_read(self.get_operand_address(mode) as u16);
        self.register_a = self.register_a & data;
        self.update_zero_and_negative_flags(self.register_a);
    }
    // ASL: Arithmetic Shift Left
    fn asl(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::NoneAddressing => {
                let original: u8 = self.register_a;
                self.register_a = self.register_a << 1;

                self.status = self.status | original >> 7;
                self.update_zero_and_negative_flags(self.register_a);
            }
            _ => {
                let addr: u16 = self.get_operand_address(mode);
                let data: u8 = self.mem_read(addr);

                self.mem_write(addr, data << 1);

                self.status = self.status | data >> 7;
                self.update_zero_and_negative_flags(data);

            }
        }
    }
    // LSR: Logical Shift Right
    fn lsr(&mut self, mode: &AddressingMode){
        match mode {
            AddressingMode::NoneAddressing => {
                let original: u8 = self.register_a;
                self.register_a = self.register_a >> 1;

                self.status = self.status | (original & 0b0000_0001);
                self.update_zero_and_negative_flags(self.register_a);
            }
            _ => {
                let addr: u16 = self.get_operand_address(mode);
                let data: u8 = self.mem_read(addr);

                self.mem_write(addr, data >> 1);

                self.status = self.status | (data & 0b0000_0001);
                self.update_zero_and_negative_flags(data);
            }
        }
    }
    // ROL: Rotate Left
    fn rol(&mut self, mode: &AddressingMode) {
        let carry: u8 = self.status & 0b0000_0001;
        // Perform rotation and store carry
        self.asl(mode);
        
        // update 0th bit with carry's value before changed by asl
        match mode {
            AddressingMode::NoneAddressing => self.register_a = self.register_a | carry,
            _ => {
                let addr: u16 = self.get_operand_address(mode);
                self.mem_write(addr, self.mem_read(addr) | carry);
            },
        }  
    }
    // ROR: Rotate Right
    fn ror(&mut self, mode: &AddressingMode) {
        let carry: u8 = self.status & 0b0000_0001;
        // Perform rotation and store carry
        self.lsr(mode);
        
        // update 7th bit with carry's value before changed by asl
        match mode {
            AddressingMode::NoneAddressing => {
                self.register_a = self.register_a | (carry << 7);
                self.update_zero_and_negative_flags(self.register_a);
            }
            _ => {
                let addr: u16 = self.get_operand_address(mode);
                let data: u8 = self.mem_read(addr) | (carry << 7);
                self.mem_write(addr, data);
                self.update_zero_and_negative_flags(data);
            },
        }
       
    }
    // BIT: Test Bits in Memory with Accumulator
    fn bit(&mut self, mode: &AddressingMode) {
        let data: u8 = self.mem_read(self.get_operand_address(mode) as u16);
        let result: u8 = self.register_a & data;
        
        // Set N flag to M7, V flag to M6, Z flag to result of and
        self.status = self.status | (0b1100_0000 & data);
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        }
    }
    // EOR: "Exclusive OR" Memory with Accumulator
    fn eor(&mut self, mode: &AddressingMode) {
        let data: u8 = self.mem_read(self.get_operand_address(mode) as u16);
        self.register_a = self.register_a ^ data;
        self.update_zero_and_negative_flags(self.register_a);
    }
    // ORA: "OR" Memory with Accumulator
    fn ora(&mut self, mode: &AddressingMode) {
        let data: u8 = self.mem_read(self.get_operand_address(mode) as u16);
        self.register_a = self.register_a | data;
        self.update_zero_and_negative_flags(self.register_a);
    }
    // ADC: Add Memory to Accumulator with Carry
    fn adc(&mut self, mode: &AddressingMode) {
        let data: u8 = self.mem_read(self.get_operand_address(mode));
        self.actual_adc(data);
    }
    fn actual_adc(&mut self, to_add: u8){
        let mut u16_result: u16 = self.register_a as u16 + to_add as u16;
        if self.status & 0b0000_0001 != 0 {
            u16_result += 1;
        }
        
        if u16_result > 255 {
            self.status = self.status | 0b000_0001;
        }
        let u8_result: u8 = u16_result as u8;
        
        // data ^ u8_result: do data and u8_result have the same sign
        // u8_result ^ self.register_a: do u8_result and self.register_a have the same sign
        // & 0x80: extract the sign
        // != 0: do two numbers of same sign being added result in different sign
        if (to_add ^ u8_result) & (u8_result ^ self.register_a) & 0x80 != 0 {
            self.status = self.status | 0b0100_0000;
        } else {
            self.status = self.status & 0b1011_1111;
        }

        self.register_a = u8_result;
        self.update_zero_and_negative_flags(self.register_a);
    }
    // CMP: Compare Memory and Accumulator
    fn cmp(&mut self, mode: &AddressingMode) {
        let mem_data: u8 = self.mem_read(self.get_operand_address(mode));
        
        if mem_data <= self.register_a {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
        self.update_zero_and_negative_flags(self.register_a.wrapping_sub(mem_data));
    }
    // CPX: Compare Index Register X To Memory
    fn cpx(&mut self, mode: &AddressingMode) {
        let mem_data: u8 = self.mem_read(self.get_operand_address(mode));
        
        if mem_data <= self.register_x {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
        self.update_zero_and_negative_flags(self.register_x.wrapping_sub(mem_data));
    }
    // CPY: Compare Index Register Y to Memory
    fn cpy(&mut self, mode: &AddressingMode) {
        let mem_data: u8 = self.mem_read(self.get_operand_address(mode));
        
        if mem_data <= self.register_y {
            self.status = self.status | 0b0000_0001;
        } else {
            self.status = self.status & 0b1111_1110;
        }
        self.update_zero_and_negative_flags(self.register_y.wrapping_sub(mem_data));
    }
    // SBC: Subtract Memory from Accumulator with Borrow
    fn sbc(&mut self, mode: &AddressingMode) {
        let data: u8 = self.mem_read(self.get_operand_address(mode));
        self.actual_adc((!data).wrapping_add(1));
    }

    // DEC: Decrement Memory by One
    fn dec(&mut self, mode: &AddressingMode){
        let addr: u16 = self.get_operand_address(mode);
        self.mem_write_u16(addr, self.mem_read_u16(addr).wrapping_sub(1));
        self.update_zero_and_negative_flags(self.mem_read_u16(addr) as u8);
    }
    // DEX: Decrement Register X by One
    fn dex(&mut self){
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    // DEY: Decrement Register Y by One
    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
    // JMP: JMP Indirect
    fn jmp(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::Absolute => self.program_counter = self.mem_read_u16(self.program_counter),
            _ => {
                // Absolute Indirect Addressing only for jmp
                let addr: u16 = self.mem_read_u16(self.program_counter);
                // Get lo byte from memory @ second byte
                
                // Get hi byte from memory @ (second byte + 1)
                // Bug in 6502 makes it so when crossing pages in absolute indirect jmp
                // memory isn't read from next apge but start of page.
                // If address $3000 contains $40, $30FF contains $80, and $3100 contains $50,
                // The result is $4080, not $5080.
                let result: u16 = if addr & 0x00FF == 0xFF {
                    let lo: u8 = self.mem_read(addr);
                    let hi: u8 = self.mem_read(addr & 0xFF00);
                    (hi as u16) << 8 | lo as u16
                } else {
                    self.mem_read_u16(addr)
                };

                self.program_counter = result;
            },
        }
    }
    // JSR: Jump To Subroutine
    fn jsr(&mut self) {
        self.stack_push_u16(self.program_counter + 2 -  1); // -1 since we already incremented PC
        self.program_counter = self.mem_read_u16(self.program_counter);
    }
    // RTI: Return From Interrupt
    fn rti(&mut self) {
        self.status = self.stack_pop();
        self.program_counter = self.stack_pop_u16();
    }
    // RTS: Return From Subroutine
    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16().wrapping_add(1);
    }
    // BCC: Branch on Carry Clear
    fn bcc(&mut self) {
        if self.status & 0b0000_0001 == 0 {
            let jump: i8 = self.mem_read_u16(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
        }
    }
    // BCS: Branch on Carry Set
    fn bcs(&mut self) {
        if self.status & 0b0000_0001 != 0 {
            let jump: i8 = self.mem_read_u16(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
        }
    }
    // BEQ: Branch on Result Zero
    fn beq(&mut self) {
        if self.status & 0b0000_0010 != 0 {
            let jump: i8 = self.mem_read_u16(self.program_counter) as i8;
            self.program_counter = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
        }
    }
    // BMI: Branch on Result Minus
    fn bmi(&mut self) {
        if self.status & 0b1000_0000 != 0 {    
            let jump: i8 = self.mem_read_u16(self.program_counter) as i8;    
            self.program_counter = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
        }
    }
    // BNE: Branch on Result Not Zero
    fn bne(&mut self) {
        if self.status & 0b0000_0010 == 0 {    
            let jump: i8 = self.mem_read_u16(self.program_counter) as i8;    
            self.program_counter = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
        }
    }
    // BPL: Branch on Result Plus
    fn bpl(&mut self) {
        if self.status & 0b1000_0000 == 0 {    
            let jump: i8 = self.mem_read_u16(self.program_counter) as i8;    
            self.program_counter = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
        }
    }
    // BVC: Branch on Overflow Clear
    fn bvc(&mut self) {
        if self.status & 0b0100_0000 == 0 {    
            let jump: i8 = self.mem_read_u16(self.program_counter) as i8;    
            self.program_counter = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
        }
    }
    // BVS: Branch on Overflow Set
    fn bvs(&mut self) {
        if self.status & 0b0100_0000 != 0 {    
            let jump: i8 = self.mem_read_u16(self.program_counter) as i8;    
            self.program_counter = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);
        }
    }
    // CLC: Clear Carry Flag
    fn clc(&mut self) {
        self.status = self.status & 0b1111_1110;
    }
    // CLD: Clear Decimal Mode
    fn cld(&mut self) {
        self.status = self.status & 0b1111_0111;
    }
    // CLI: Clear Interrupt Disable
    fn cli(&mut self) {
        self.status = self.status & 0b1111_1011;
    }
    // CLV: Clear Overflow Flag
    fn clv(&mut self) {
        self.status = self.status & 0b1011_1111;
    }
    // SEC: Set Carry Flag
    fn sec(&mut self) {
        self.status = self.status | 0b0000_0001;
    }
    // SED: Set Decimal Mode
    fn sed(&mut self) {
        self.status = self.status | 0b0000_1000;
    }
    // SEI: Set Interrupt Disable
    fn sei(&mut self) {
        self.status = self.status | 0b0000_0100;
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
    fn test_0xee_inc_absolute() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xee, 0x12, 0x34, 0x00];

        cpu.mem_write_u16(0x3412, 10);

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.mem_read_u16(0x3412), 10 + 1);
    }
    #[test]
    fn test_0xe8_inx_overflow() {
        let mut cpu: CPU = CPU::new();
        cpu.register_x = 0xff;
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_0xc8_iny_overflow() {
        let mut cpu: CPU = CPU::new();
        cpu.register_y = 0xff;
        cpu.load(vec![0xc8, 0xc8, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_y, 1);
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
    fn test_0x29_and_immediate() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1111_0001;
        let and_result: u8 = cpu.register_a & 0b1010_1111;
        cpu.load(vec![0x29, 0b1010_1111, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, and_result);
        assert!(cpu.status & 0b1000_0000 != 0);
    }
    #[test]
    fn test_0x2d_and_absolute() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1111_0001;
        cpu.mem_write(0xaa42, 0b1010_1111);
        let and_result: u8 = cpu.register_a & 0b1010_1111;
        cpu.load(vec![0x2d, 0x42, 0xaa, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();
        assert_eq!(cpu.register_a, and_result);
        assert!(cpu.status & 0b1000_0000 != 0);
    }
    #[test]
    fn test_0x8d_sta_absolute() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x8d, 0x34, 0x12, 0x00];

        cpu.register_a = 10;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_a);
    }
    #[test]
    fn test_0x8e_stx_absolute() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x8e, 0x34, 0x12, 0x00];

        cpu.register_x = 10;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_x);
    }
    #[test]
    fn test_0x8c_sty_absolute() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x8c, 0x34, 0x12, 0x00];

        cpu.register_y = 10;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read(0x1234), cpu.register_y);
    }
    #[test]
    fn test_0x48_pha() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x48, 0x00];

        cpu.register_a = 10;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(
            cpu.mem_read(cpu.stack_start + cpu.stack_ptr.wrapping_add(1) as u16),
            10
        );
    }
    #[test]
    fn test_0x48_pha_underflow() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x48, 0x00];

        cpu.register_a = 10;
        cpu.stack_ptr = 0;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(
            cpu.mem_read(cpu.stack_start + cpu.stack_ptr.wrapping_add(1) as u16),
            10
        );
    }
    #[test]
    fn test_0x08_php() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x08, 0x00];

        cpu.status = 0b1100_0001;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(
            cpu.mem_read(cpu.stack_start + cpu.stack_ptr.wrapping_add(1) as u16),
            0b1111_0001
        );
    }
    #[test]
    fn test_0x08_php_underflow() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x08, 0x00];

        cpu.status = 0b1100_0001;
        cpu.stack_ptr = 0;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(
            cpu.mem_read(cpu.stack_start + cpu.stack_ptr.wrapping_add(1) as u16),
            0b1111_0001
        );
    }
    #[test]
    fn test_0x68_pla_overflow() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x68, 0x00];

        cpu.register_a = 10;
        cpu.stack_ptr = 0;
        cpu.pha();
        cpu.register_a = 0;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, 10);
        assert_eq!(cpu.stack_ptr, 0);
    }
    #[test]
    fn test_0x28_plp_overflow() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x28, 0x00];

        cpu.status = 0b1100_0001;
        cpu.stack_ptr = 0;
        cpu.php();
        cpu.status = 0;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.status, 0b1110_0001);
        assert_eq!(cpu.stack_ptr, 0);
    }
    #[test]
    fn test_0x0a_asl_acc() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x0a, 0x00];

        cpu.register_a = 0b1001_0101;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, 0b0010_1010);
        assert!(cpu.status & 0b0000_0001 != 0);
    }
    #[test]
    fn test_0x0e_asl_memory() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x0e, 0x20, 0x21,0x00];

        cpu.mem_write_u16(0x2120, 0b1001_0101);

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read_u16(0x2120), 0b0010_1010);
        assert!(cpu.status & 0b0000_0001 != 0);
    }
    #[test]
    fn test_0x4a_lsr_acc() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x4a, 0x00];

        cpu.register_a = 0b1001_0101;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, 0b0100_1010);
        assert!(cpu.status & 0b0000_0001 != 0);
    }
    #[test]
    fn test_0x4e_lsr_memory() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x4e, 0x20, 0x21,0x00];

        cpu.mem_write_u16(0x2120, 0b1001_0101);

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read_u16(0x2120), 0b0100_1010);
        assert!(cpu.status & 0b0000_0001 != 0);
    }
    #[test]
    fn test_0x2a_rol_acc() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x2a, 0x00];

        cpu.register_a = 0b1001_0101;
        cpu.status = 0b0000_0001;
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, 0b0010_1011);
        assert!(cpu.status & 0b0000_0001 != 0);
    }
    #[test]
    fn test_0x2e_rol_memory() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x2e, 0x20, 0x21,0x00];

        cpu.mem_write_u16(0x2120, 0b1001_0101);
        cpu.status = 0b0000_0001;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read_u16(0x2120), 0b0010_1011);
        assert!(cpu.status & 0b0000_0001 != 0);
    }
    #[test]
    fn test_0x6a_ror_acc() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x6a, 0x00];

        cpu.register_a = 0b1001_0101;
        cpu.status = 0b0000_0001;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, 0b1100_1010);
        assert!(cpu.status & 0b0000_0001 != 0);
    }
    #[test]
    fn test_0x6e_ror_memory() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x6e, 0x20, 0x21,0x00];

        cpu.mem_write_u16(0x2120, 0b1001_0101);
        cpu.status = 0b0000_0001;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read_u16(0x2120), 0b1100_1010);
        assert!(cpu.status & 0b0000_0001 != 0);
    }
    #[test]
    fn test_0x2c_bit_absolute(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x2c, 0x00, 0xf8,0x00];

        cpu.mem_write_u16(0xf800, 0b1001_0101);
        cpu.register_a = 0b0011_1001;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.mem_read_u16(0xf800), 0b1001_0101);
        assert_eq!(cpu.register_a, 0b0011_1001);
        // Flags properly set
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == ((cpu.mem_read_u16(0xf800) as u8) & 0b1000_0000));
        assert!(cpu.status & 0b0100_0000 == ((cpu.mem_read_u16(0xf800) as u8) & 0b0100_0000));
    }
    #[test]
    fn test_0x49_eor_immediate() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1111_0001;
        let eor_result: u8 = cpu.register_a ^ 0b1010_1111;
        
        cpu.load(vec![0x49, 0b1010_1111, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, eor_result);
    }
    #[test]
    fn test_0x4d_eor_absolute() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1111_0001;
        cpu.mem_write(0x2a42, 0b1010_1111);
        let eor_result: u8 = cpu.register_a ^ 0b1010_1111;

        cpu.load(vec![0x4d, 0x42, 0x2a, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, eor_result);
    }
    #[test]
    fn test_0x09_ora_immediate() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1111_0001;
        let or_result: u8 = cpu.register_a | 0b1010_1111;
        
        cpu.load(vec![0x09, 0b1010_1111, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, or_result);
    }
    #[test]
    fn test_0x0d_ora_absolute() {
        let mut cpu: CPU = CPU::new();
        cpu.register_a = 0b1111_0001;
        cpu.mem_write(0x2a42, 0b1010_1111);
        let or_result: u8 = cpu.register_a | 0b1010_1111;

        cpu.load(vec![0x0d, 0x42, 0x2a, 0x00]);
        cpu.program_counter = cpu.mem_read_u16(0xFFFC);
        cpu.run();

        assert_eq!(cpu.register_a, or_result);
    }
    #[test]
    fn test_0xce_dec_absolute() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xce, 0x12, 0x34, 0x00];

        cpu.mem_write_u16(0x3412, 10);

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.mem_read_u16(0x3412), 10 -1);
    }
    #[test]
    fn test_0xca_dex_absolute() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xca, 0x00];

        cpu.register_x = 10;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 10 -1);
    }
    #[test]
    fn test_0x88_dey_absolute() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x88, 0x00];

        cpu.register_y = 10;

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_y, 10 - 1);
    }
    #[test]
    fn test_0x4c_jmp_absolute() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x4c, 0x57, 0x79, 0x00];
        
        cpu.mem_write_u16(0x7957, 0x00);

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        // -1 since it moves +1 to get BRK instruction
        assert_eq!(cpu.program_counter - 1, 0x7957);
    }
    #[test]
    fn test_0x6c_jmp_absolute_indirect() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x6c, 0x57, 0x79, 0x00];
        
        cpu.mem_write_u16(0x7957, 0x2222);
        cpu.mem_write_u16(0x2222, 0x00);
        
        

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        // -1 since it moves +1 to get BRK instruction
        assert_eq!(cpu.program_counter - 1, 0x2222);        
    }
    #[test]
    fn test_0x6c_jmp_absolute_indirect_page_end() {
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x6c, 0xff, 0x30, 0x00];
        
        cpu.mem_write_u16(0x3000, 0x40);
        cpu.mem_write_u16(0x30ff, 0x80);
        cpu.mem_write_u16(0x3100, 0x50);
        cpu.mem_write(0x4080, 0x00);
        

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        // -1 since it moves +1 to get BRK instruction
        assert_eq!(cpu.program_counter - 1, 0x4080);        
    }
    #[test]
    fn test_0x20_jsr(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x20, 0x20, 0x21, 0x00];
        
        cpu.mem_write(0x2120, 0xe8);

        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        let curr_pc: u16 = cpu.mem_read_u16(0xfffc) + 2;  
        cpu.run();

        assert_eq!(cpu.register_x, 1);
        assert_eq!(cpu.stack_pop_u16(), curr_pc);
    }
    #[test]
    fn test_0x40_rti(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x40, 0x00]; 
        
        cpu.stack_push_u16(0x8523); 
        cpu.stack_push(0b1100_0101);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.program_counter, 0x8523 + 1);
        assert_eq!(cpu.status, 0b1100_0101);
    }
    #[test]
    fn test_0x60_rts(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x60, 0x00];  

        cpu.stack_push_u16(0x8523);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        // + 1 from RTS, +1 from reading next instruction
        assert_eq!(cpu.program_counter, 0x8523 + 1 + 1);
    }
    #[test]
    fn test_0x90_bcc(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x90, 0x45, 0x00];
        cpu.status = 0b1111_1110;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_0xb0_bcs(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xb0, 0x45, 0x00];
        cpu.status = 0b0000_0001;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_0xf0_beq(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xf0, 0x45, 0x00];
        cpu.status = 0b0000_0010;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_0x30_bmi(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x30, 0x45, 0x00];
        cpu.status = 0b1000_0000;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_0xd0_bne(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xd0, 0x45, 0x00];
        cpu.status = 0b1111_1101;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_0x10_bpl(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x10, 0x45, 0x00];
        cpu.status = 0b0111_1111;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_0x50_bvc(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x50, 0x45, 0x00];
        cpu.status = 0b1011_1111;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    }
    #[test]
    fn test_0x70_bvs(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x70, 0x45, 0x00];
        cpu.status = 0b0100_0000;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    } 
    #[test]
    fn test_0x18_clc(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x18, 0x00];
        cpu.status = 0b1111_1111;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.status, 0b1111_1110);
    } 
    #[test]
    fn test_0xd8_cld(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xd8, 0x00];
        cpu.status = 0b1111_1111;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.status, 0b1111_0111);
    } 
    #[test]
    fn test_0x58_cli(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x58, 0x00];
        cpu.status = 0b1111_1111;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.status, 0b1111_1011);
    } 
    #[test]
    fn test_0xb8_clv(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xb8, 0x00];
        cpu.status = 0b1111_1111;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.status, 0b1011_1111);
    } 
    #[test]
    fn test_0x38_sec(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x38, 0x00];
        cpu.status = 0b0000_0000;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.status, 0b0000_0001);
    }
    #[test]
    fn test_0xf8_sec(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0xf8, 0x00];
        cpu.status = 0b0000_0000;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.status, 0b0000_1000);
    }  
    #[test]
    fn test_0x78_sei(){
        let mut cpu: CPU = CPU::new();
        let program: Vec<u8> = vec![0x78, 0x00];
        cpu.status = 0b0000_0000;  

        cpu.mem_write(cpu.program_counter.wrapping_add(0x8047), 0xe8);
        
        cpu.load(program);
        cpu.program_counter = cpu.mem_read_u16(0xfffc);
        cpu.run();

        assert_eq!(cpu.status, 0b0000_0100);
    } 
}
