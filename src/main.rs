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
    SP: Word, // Stack pointer  | probably should be Byte
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
    // LDA - Load Accumulator, imidiate mode
    const INS_LDA_IM: Byte = 0xA9;

    fn reset(&mut self,memory: &mut MEM) {
        self.PC = 0xFFFC;
        self.SP = 0x0100; //maybe 0x00FF ? idk

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

    fn fetch (&mut self, cycles: &mut u32, memory: &MEM) -> Byte{
        let data: Byte = memory.data[self.PC as usize];
        self.PC += 1;
        *cycles -= 1;

        data
    }

    fn execute(&mut self, mut cycles: u32, memory: &MEM){
        while cycles > 0 {
            let instruction: Byte = self.fetch(&mut cycles, &memory);
            match instruction {
                CPU::INS_LDA_IM => {
                    let value: Byte= self.fetch(&mut cycles, memory);
                    self.A = value;
                    // Z is set if A = 0
                    if self.A == 0 {
                        self.Z = 1;
                    }
                    // N is set if bit 7 of A is set
                    if self.A & 0b10000000 > 0{
                        self.N = 1;
                    }
                }
                _ => {}
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
    cpu.execute(2,&mut mem);
}
