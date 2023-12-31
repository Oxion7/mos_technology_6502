type Byte = u8;
type Word = u16;

#[derive(Debug)]
pub struct MEM{
    pub data: [Byte; MEM::MAX_MEM],
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
pub struct CPU {
    PC: Word, // Program counter
    SP: Byte, // Stack pointer 
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
    pub const INS_LDA_IM: Byte = 0xA9;    // LDA - Load Accumulator, imidiate mode
    pub const INS_LDA_ZP: Byte = 0xA5;    // LDA, zero page mode
    pub const INS_LDA_ZP_X: Byte = 0xB5;  // LDA, zero page, X
    pub const INS_LDA_ABS: Byte = 0xAD;   // LDA, absolute
    pub const INS_LDA_ABS_X: Byte = 0xBD; // LDA, absolute, X
    pub const INS_LDA_ABS_Y: Byte = 0xB9; // LDA, absolute, Y
    pub const INS_JSR: Byte = 0x20;       // JSR - Jump to  Subroutine

    pub fn reset(&mut self,memory: &mut MEM) {
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

    fn read_byte (&mut self, cycles: &mut u32, memory: &MEM, address: Word) -> Byte{
        let data: Byte = memory.data[address as usize];
        *cycles -= 1;

        data
    }



    #[allow(non_snake_case)]
    fn LDAXY_set_status(&mut self) {
        // Z is set if A = 0
        if self.A == 0 {
            self.Z = 1;
        }
        // N is set if bit 7 of A is set
        if self.A & 0b10000000 > 0{
            self.N = 1;
        }
    }

    fn is_page_boundary_crossed(ab_address:Word, ab_address_x:Word) -> bool{
        if (ab_address + ab_address_x) > 0 {
            true
        } else {
            false
        }
    }

    pub fn execute(&mut self, mut cycles: u32, memory: &mut MEM){
        while cycles > 0 {
            let instruction: Byte = self.fetch_byte(&mut cycles, &memory);
            match instruction {
                CPU::INS_LDA_IM => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.A = value;

                    self.LDAXY_set_status();
                }
                CPU::INS_LDA_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(&mut cycles, memory);
                    self.A = self.read_byte(&mut cycles, memory, (zero_page_address) as u16);
                    
                    self.LDAXY_set_status();
                }
                CPU::INS_LDA_ZP_X => {
                    let mut zero_page_adress: Byte = self.fetch_byte(&mut cycles, memory);
                    zero_page_adress += self.X;
                    cycles -= 1;
                    self.A = self.read_byte(&mut cycles, memory, (zero_page_adress) as u16);
                    //TODO: handle the address overflow
                    self.LDAXY_set_status();
                }
                CPU::INS_LDA_ABS => {
                    let ab_address: Word = self.fetch_word(&mut cycles, memory);
                    self.A = self.read_byte(&mut cycles, memory, ab_address);

                    self.LDAXY_set_status();
                }
                CPU::INS_LDA_ABS_X => {
                    let ab_address: Word = self.fetch_word(&mut cycles, memory);
                    let ab_address_x: Word = ab_address + self.X as Word;
                    cycles -= 1;
                    if Self::is_page_boundary_crossed(ab_address, ab_address_x) == true{
                        cycles -= 1;
                    }
                    self.LDAXY_set_status();
                }
                CPU::INS_LDA_ABS_Y => {
                    let ab_address: Word = self.fetch_word(&mut cycles, memory);
                    let ab_address_y: Word = ab_address + self.Y as Word;
                    cycles -= 1;
                    if Self::is_page_boundary_crossed(ab_address, ab_address_y) == true{
                        cycles -= 1;
                    }
                    self.LDAXY_set_status();
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

//TODO: move tests to separate folder
#[allow(unused_imports,non_snake_case, dead_code)]
mod test {
    use super::{MEM, CPU};

    fn assert_LDA_flags (cpu: CPU, cpu_copy: CPU){
        assert_eq!(cpu.C, cpu_copy.C);
        if  cpu.A == 0{
            assert_eq!(cpu.Z, 1)
        } else {
            assert_eq!(cpu.Z, cpu_copy.Z)
        }
        assert_eq!(cpu.I, cpu_copy.I);
        assert_eq!(cpu.D, cpu_copy.B);
        assert_eq!(cpu.V, cpu_copy.V);
        if cpu.A & 0b10000000 > 0{
            assert_eq!(cpu.N, 1);
        } else {
            assert_eq!(cpu.N, cpu_copy.N); 
        }
    }

    #[test]
    fn LDA_IM_load_value_in_register() {
        let mut mem: MEM = MEM::default();
        let mut cpu: CPU = CPU::default();
        cpu.reset(&mut mem);
        let cpu_copy: CPU = cpu.clone();
        let pc_expected = cpu.PC + 2;

        mem.data[0xFFFC] = CPU::INS_LDA_IM;
        mem.data[0xFFFD] = 0x12;

        cpu.execute(2, &mut mem);
        assert_eq!(cpu.A, 0x12);
        assert_eq!(cpu.PC, pc_expected);
        assert_LDA_flags(cpu, cpu_copy);
    }
    #[test]
    fn LDA_ZP_load_value_in_register(){
        let mut mem: MEM = MEM::default();
        let mut cpu: CPU = CPU::default();
        cpu.reset(&mut mem);
        let cpu_copy: CPU = cpu.clone();
        let pc_expected = cpu.PC + 2;

        mem.data[0xFFFC] = CPU::INS_LDA_ZP;
        mem.data[0xFFFD] = 0x42;
        mem.data[0x0042] = 0x12;

        cpu.execute(3, &mut mem);
        assert_eq!(cpu.A, 0x12);
        assert_eq!(cpu.PC, pc_expected);
        assert_LDA_flags(cpu, cpu_copy);
    }
    #[test]
    fn LDA_AB_load_value_in_register(){
        let mut mem: MEM = MEM::default();
        let mut cpu: CPU = CPU::default();
        cpu.reset(&mut mem);
        let cpu_copy: CPU = cpu.clone();
        let pc_expected = cpu.PC + 3;

        mem.data[0xFFFC] = CPU::INS_LDA_ABS;
        mem.data[0xFFFD] = 0x80;
        mem.data[0xFFFE] = 0x44;
        mem.data[0x4480] = 0x12;

        cpu.execute(4, &mut mem);
        assert_eq!(cpu.A, 0x12);
        assert_eq!(cpu.PC, pc_expected);
        assert_LDA_flags(cpu, cpu_copy);
    }
    //TODO: add LDA_ABS_X and LDA_ABS_Y tests
}