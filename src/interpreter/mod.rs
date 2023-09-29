
pub type Display = chip8_base::Display;
pub type Pixel = chip8_base::Pixel;
pub type Keys = chip8_base::Keys;

pub type Memory = [u8; 4096];

pub struct Chip8VM {
    pc: u16,
    sp: u8,
    general: [u8; 16],
    i: u16,

    delay: u8,
    sound: u8,
    
    memory: Memory,

    display: Display,
}

impl Chip8VM {
    pub fn new() -> Chip8VM {
        Chip8VM {
            general: [0; 16],
            i: 0x0,
            delay: 0x0,
            sound: 0x0,
            memory: [0; 4096],
            pc: 0x00,
            sp: 0x0,
            display: [[Pixel::Black; 64]; 32]
        }
    }
}

impl chip8_base::Interpreter for Chip8VM {
    fn step(&mut self, keys: &Keys) -> Option<Display> {
        return Some([[Pixel::Black; 64]; 32]);
    }

    fn speed(&self) -> std::time::Duration {
        return std::time::Duration::from_millis(300);
    }

    fn buzzer_active(&self) -> bool {
        return false;
    }
}
