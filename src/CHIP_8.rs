pub enum Opcode {
    SYS(u16),
    CLS,
    RET,     // RET
    JP(u16), // GOTO
    _2NNN,
    _3XNN,
    _4XNN,
    _5XY0,
    _6XNN,
    _7XNN,
    _8XY0,
    _8XY1,
    _8XY2,
    _8XY3,
    _8XY4,
    _8XY5,
    _8XY6,
    _8XY7,
    _8XYE,
    _9XY0,
    LD_I(u16),
    _BNNN,
    _CXNN,
    _DXYN,
    _EX9E,
    _EXA1,
    _FX07,
    _FX0A,
    _FX15,
    _FX18,
    _FX1E,
    _FX29,
    _FX33,
    _FX55,
    _FX65,
}

impl Opcode {
    fn from(opcode: u16) -> Self {
        let opcode_parts = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = opcode & 0x0FFF;
        let _nn = opcode & 0x00FF;
        let _x = opcode_parts.1;
        let _y = opcode_parts.2;

        match opcode_parts {
            (0x00, 0x00, 0x0e, 0x00) => Opcode::CLS,
            (0x00, 0x00, 0x0e, 0x0e) => Opcode::RET,
            (0x00, _, _, _) => Opcode::SYS(nnn),
            (0x01, _, _, _) => Opcode::JP(nnn),
            (0x0A, _, _, _) => Opcode::LD_I(nnn),
            _ => panic!("Unknown opcode"),
        }
    }
}

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct CPU {
    pub memory: Vec<u8>,
    pub registers: Vec<u8>,
    pub index_register: u16,
    pub program_counter: usize,
    pub screen: Vec<u8>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: Vec<u16>,
    pub stack_ptr: u16,
    pub keypad: Vec<u8>,
    pub draw_flag: bool,
}

impl CPU {
    pub fn new() -> Self {
        let mut memory: Vec<u8> = vec![0; 4096];
        memory[..80].clone_from_slice(&FONTSET[..80]);

        CPU {
            memory,
            registers: vec![0; 16],
            index_register: 0,      // I
            program_counter: 0x200, // pc
            screen: vec![0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![0; 16],
            stack_ptr: 0, // sp
            keypad: vec![9; 16],
            draw_flag: false,
        }
    }

    pub fn load_memory(&mut self, memory: Vec<u8>) {
        let start_location = 512;
        self.memory[start_location..(memory.len() + start_location)].clone_from_slice(&memory[..]);
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        let opcode = (self.memory[self.program_counter] as u16) << 8
            | self.memory[self.program_counter + 1] as u16;
        // Decode Opcode

        // Execute Opcode

        match Opcode::from(opcode) {
            Opcode::LD_I(nnn) => {
                self.index_register = nnn;
                self.program_counter += 2;
            }
            _ => panic!("Unknown opcode {}", opcode),
        }

        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }
}
