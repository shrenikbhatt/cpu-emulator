pub struct Cpu {
    registers: [u8; 16], // 16 registers
    memory: [u8; 0x1000], // 4 kiB memory
    program_counter: usize, // which memory address to access
}

impl Cpu {
    pub fn new() -> Cpu {
        let registers: [u8; 16] = [0; 16];
        let memory: [u8; 0x1000] = [0; 0x1000];
        Cpu {
            registers,
            memory,
            program_counter: 0,
        }
    }

    pub fn run(&mut self) {
        /*
            opcode split into 4 parts:
            - bits 15-12 represents operation
                - 0: noop if 0x0111, terminate if 0x0
                - 1: integer add involving 2 registers
                - 2: integer add involving 1 register
                - 3: bitwise OR involing 2 registers
                - 4: bitwise OR involving 1 register
                - 5: bitwise AND involving 2 registers
                - 6: bitwise AND involving 1 register
                - 7: Mov value at register Y into register X
                - 8: Mov value into register X
                - 9: Jump to memory address specified by bits 11-0
            - bits 11-8 represent register X
            - bits 7-4 represent register Y if operation involves 2 registers
            - bits 7-4 represent value if operation involves 1 register
            - bits 3-0 represent where to store result (register Z)
        */
        let mut current_opcode: u16 = 0x0111;
        while self.program_counter < (0x1000 - 1) {
            current_opcode = ((self.memory[self.program_counter] as u16) << 8) | (self.memory[self.program_counter + 1] as u16);
            if current_opcode == 0x0 { 
                break;
            }

            // println!("{:x}", current_opcode);
            let opcode_parts: (u8, u8, u8, u8) = self.process_opcode(&current_opcode);

            // println!("{:?}", &opcode_parts);

            match opcode_parts {
                (0, 1, 1, 1) => continue,
                (1, _, _, _) => self.add_y_x(&opcode_parts.1, &opcode_parts.2, &opcode_parts.3),
                (2, _, _, _) => self.add_x(&opcode_parts.1, &opcode_parts.2, &opcode_parts.3),
                (3, _, _, _) => self.or_y_x(&opcode_parts.1, &opcode_parts.2, &opcode_parts.3),
                (4, _, _, _) => self.or_x(&opcode_parts.1, &opcode_parts.2, &opcode_parts.3),
                (5, _, _, _) => self.and_y_x(&opcode_parts.1, &opcode_parts.2, &opcode_parts.3),
                (6, _, _, _) => self.and_x(&opcode_parts.1, &opcode_parts.2, &opcode_parts.3),
                (7, _, _, _) => self.mov_y_x(&opcode_parts.1, &opcode_parts.2),
                (8, _, _, _) => self.mov_x(&opcode_parts.1, &opcode_parts.2),
                (9, _, _, _) => self.jump(&opcode_parts.1, &opcode_parts.2, &opcode_parts.3),
                (_, _, _, _) => println!("Not implemented"),
            }
            self.program_counter += 2; // memory holds u8 but opcode is u16
        }
    }

    pub fn get_value_at_register(&self, register_num: u8) -> u8 {
        self.registers[register_num as usize]
    }
}

impl Cpu {
    fn process_opcode(&self, current_opcode: &u16) -> (u8, u8, u8, u8) {
        let operation: u8 = (current_opcode >> 12) as u8;
        let register_x: u8 = ((current_opcode >> 8) & 0xF) as u8;
        let register_y: u8 = ((current_opcode >> 4) & 0xF) as u8;
        let register_z: u8 = (current_opcode & 0xF) as u8;

        (operation, register_x, register_y, register_z)
    }

    fn add_y_x(&mut self, register_x: &u8, register_y: &u8, register_z: &u8) {
        self.registers[*register_z as usize] = self.registers[*register_x as usize] + self.registers[*register_y as usize];
    }

    fn add_x(&mut self, register_x: &u8, val: &u8, register_z: &u8) {
        self.registers[*register_z as usize] = self.registers[*register_x as usize] + val;
    }

    fn or_y_x(&mut self, register_x: &u8, register_y: &u8, register_z: &u8) {
        self.registers[*register_z as usize] = self.registers[*register_x as usize] | self.registers[*register_y as usize];
    }

    fn or_x(&mut self, register_x: &u8, val: &u8, register_z: &u8) {
        self.registers[*register_z as usize] = self.registers[*register_x as usize] | val;
    }

    fn and_y_x(&mut self, register_x: &u8, register_y: &u8, register_z: &u8) {
        self.registers[*register_z as usize] = self.registers[*register_x as usize] & self.registers[*register_y as usize];
    }

    fn and_x(&mut self, register_x: &u8, val: &u8, register_z: &u8) {
        self.registers[*register_z as usize] = self.registers[*register_x as usize] & val;
    }

    fn mov_y_x(&mut self, register_x: &u8, register_y: &u8) {
        self.registers[*register_x as usize] = self.registers[*register_y as usize];
    }

    fn mov_x(&mut self, register_x: &u8, val: &u8) {
        self.registers[*register_x as usize] = *val;
    }

    fn jump(&mut self, address_part_1: &u8, address_part_2: &u8, address_part_3: &u8) {
        let address: u16 = (((*address_part_1 as u16) << 8) | ((*address_part_2 as u16) << 4)) | (*address_part_3 as u16);
        self.program_counter = address as usize;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_add_y_x() {
        let mut cpu: Cpu = Cpu::new();
        cpu.registers[0] = 3;
        cpu.registers[1] = 4;
        cpu.memory[0] = 0x10;
        cpu.memory[1] = 0x12;
        cpu.memory[2] = 0x00;

        cpu.run();

        assert_eq!(cpu.registers[2], 7);
    }

    #[test]
    fn test_add_x() {
        let mut cpu: Cpu = Cpu::new();
        cpu.registers[0] = 3;
        cpu.memory[0] = 0x20;
        cpu.memory[1] = 0xF0;

        cpu.run();

        assert_eq!(cpu.registers[0], 18);
    }

    #[test]
    fn test_or_y_x() {
        let mut cpu: Cpu = Cpu::new();
        cpu.registers[0] = 0b0010;
        cpu.registers[1] = 0b1010;
        cpu.memory[0] = 0x30;
        cpu.memory[1] = 0x12;
        cpu.memory[2] = 0x00;

        cpu.run();

        assert_eq!(cpu.registers[2], 0b1010);
    }

    #[test]
    fn test_or_x() {
        let mut cpu: Cpu = Cpu::new();
        cpu.registers[0] = 0b0010;
        cpu.memory[0] = 0x40;
        cpu.memory[1] = 0xF2;
        cpu.memory[2] = 0x00;

        cpu.run();

        assert_eq!(cpu.registers[2], 0b1111);
    }

    #[test]
    fn test_and_y_x() {
        let mut cpu: Cpu = Cpu::new();
        cpu.registers[0] = 0b0010;
        cpu.registers[1] = 0b1010;
        cpu.memory[0] = 0x50;
        cpu.memory[1] = 0x12;
        cpu.memory[2] = 0x00;

        cpu.run();

        assert_eq!(cpu.registers[2], 0b0010);
    }

    #[test]
    fn test_and_x() {
        let mut cpu: Cpu = Cpu::new();
        cpu.registers[0] = 0b1010;
        cpu.memory[0] = 0x60;
        cpu.memory[1] = 0xF2;
        cpu.memory[2] = 0x00;

        cpu.run();

        assert_eq!(cpu.registers[2], 0b1010);
    }

    #[test]
    fn test_mov_y_x() {
        let mut cpu: Cpu = Cpu::new();
        cpu.registers[0] = 0b0010;
        cpu.registers[1] = 0b1010;
        cpu.memory[0] = 0x70;
        cpu.memory[1] = 0x1F;
        cpu.memory[2] = 0x00;

        cpu.run();

        assert_eq!(cpu.registers[0], 0b1010);
    }

    #[test]
    fn test_mov_x() {
        let mut cpu: Cpu = Cpu::new();
        cpu.registers[0] = 0b1010;
        cpu.memory[0] = 0x80;
        cpu.memory[1] = 0xF2;
        cpu.memory[2] = 0x00;

        cpu.run();

        assert_eq!(cpu.registers[0], 0b1111);
    }

    #[test]
    fn test_jump() {
        let mut cpu: Cpu = Cpu::new();
        cpu.memory[0] = 0x90;
        cpu.memory[1] = 0x26;
        cpu.memory[0x26] = 0x30;

        cpu.run();

        assert_eq!(cpu.program_counter, 0x28);
    }
}