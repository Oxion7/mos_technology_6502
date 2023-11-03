type Byte = u8;
type Word = u16;

#[derive(Debug)]
struct MEM{
    data: [Byte; MEM::MAX_MEM],
}

impl MEM{
    const MAX_MEM: usize = 1024 * 64;

    fn initialise(&mut self){
        for i in 0..MEM::MAX_MEM{
            self.data[i] = 0;
        }
    }
    fn write_word(&mut self, cycles: &mut u32 , word: Word, addr: u32) {
        self.data[addr as usize] = (word & 0xFF) as u8;
        self.data[(addr + 1) as usize] = (word >> 8) as u8; 
        *cycles -= 2;
    }
}

impl Default for MEM {
    fn default () -> MEM {
        MEM{
            data: [0; MEM::MAX_MEM],
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
struct CPU {
    PC: Word, // Program counter
    SP: Byte, // Stack pointer  | probably should be Byte
        // registers
    A: Byte,
    X: Byte,
    Y: Byte,
        // status flags
    C: Byte, 
    Z: Byte,
    I: Byte,
    D: Byte,
    B: Byte,
    V: Byte,
    N: Byte,
}

impl CPU{
        // opcodes
    const INS_LDA_IM: Byte = 0xA9;    // LDA - Load Accumulator, imidiate mode
    const INS_LDA_ZP: Byte = 0xA5;    // LDA, zero page mode
    const INS_LDA_ZP_X: Byte = 0xB5;  // LDA, zero page, X
    const INS_JSR: Byte = 0x20;       // JSR - Jump to  Subroutine

    fn reset(&mut self,memory: &mut MEM) {
        self.PC = 0xFFFC;
        self.SP = 0x00FF;

        self.C = 0;
        self.Z = 0;
        self.I = 0;
        self.D = 0;
        self.B = 0;
        self.V = 0;
        self.N = 0;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        memory.initialise();
    }

    fn fetch_byte (&mut self, cycles: &mut u32, memory: &MEM) -> Byte{
        let data: Byte = memory.data[self.PC as usize];
        self.PC += 1;
        *cycles -= 1;

        data
    }

    fn fetch_word (&mut self, cycles: &mut u32, memory: &MEM) -> Word{
        let mut data: Word = memory.data[self.PC as usize] as Word;
        self.PC += 1;
        data |= (memory.data[self.PC as usize] as Word) << 8; // because 6502 is little endian and my machine is little endian
        self.PC += 1;
        *cycles -= 2;

        data
    }

    // doesn't increment the program counter
    fn read (&mut self, cycles: &mut u32, memory: &MEM, address: Byte) -> Byte{
        let data: Byte = memory.data[address as usize];
        *cycles -= 1;

        data
    }
    #[allow(non_snake_case)]
    fn LDA_set_status(&mut self) {
        // Z is set if A = 0
        if self.A == 0 {
            self.Z = 1;
        }
        // N is set if bit 7 of A is set
        if self.A & 0b10000000 > 0{
            self.N = 1;
        }
    }

    fn execute(&mut self, mut cycles: u32, memory: &mut MEM){
        while cycles > 0 {
            let instruction: Byte = self.fetch_byte(&mut cycles, &memory);
            match instruction {
                CPU::INS_LDA_IM => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.A = value;

                    self.LDA_set_status();
                }
                CPU::INS_LDA_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    self.A = self.read(&mut cycles, memory, zero_page_address);
                    
                    self.LDA_set_status();
                }
                CPU::INS_LDA_ZP_X => {
                    let mut zero_page_adress: Byte = self.fetch_byte(&mut cycles, memory);
                    zero_page_adress += self.X;
                    cycles -= 1;
                    self.A = self.read(&mut cycles, memory, zero_page_adress);
                    //TODO: handle the address overflow
                    self.LDA_set_status();
                }
                CPU::INS_JSR => {
                    let sr_address: Word = self.fetch_word(&mut cycles, memory);
                    memory.write_word(&mut cycles, self.PC - 1, self.SP as u32);
                    self.PC = sr_address;
                    cycles -= 1;
                }
                _ => {println!("No instruction {}", instruction)}
            }
        }
    }
}

impl Default for CPU {
    fn default() -> CPU {
        CPU { PC: 0, SP: 0, A: 0, X: 0, Y: 0, C: 0, Z: 0, I: 0, D: 0, B:0, V: 0, N: 0 }
    }
}

fn main() {
    let mut mem: MEM = MEM::default();
    let mut cpu: CPU = CPU::default();
    cpu.reset(&mut mem);

    mem.data[0xFFFC] = CPU::INS_JSR;
    mem.data[0xFFFD] = 0x42;
    mem.data[0xFFFE] = 0x42;
    mem.data[0x4242] = CPU::INS_LDA_IM;
    mem.data[0x4243] = 0x12;

    cpu.execute(9,&mut mem);

    println!();
}
