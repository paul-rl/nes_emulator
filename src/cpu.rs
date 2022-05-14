pub fn main(){

}
// CPU Registers:
pub struct CPU {
    pub register_a: u8, // Accumulator
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
                _ => todo!("")
            }
        } // REPEAT
    }
}
