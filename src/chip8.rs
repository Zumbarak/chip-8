use crate::stack::Stack;

struct Chip8 {
    delay_timer: u8,
    sound_timer: u8,
    last_timer_update: std::time::Instant,
    keys: [bool; 16],
    memory: [u8; 4096],
    pc: u16,
    v: [u8; 16],
    i: u16,
    stack: Stack,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            delay_timer: 0,
            sound_timer: 0,
            last_timer_update: std::time::Instant::now(),
            keys: [false; 16],
            memory: [0; 4096],
            pc: 0,
            v: [0; 16],
            i: 0,
            stack: Stack::new(),
        }
    }

    fn fetch(&mut self) -> u16 {
        let mut output: u16 = self.memory[self.pc as usize] as u16;
        output = output << 8;
        output |= self.memory[(self.pc + 1) as usize] as u16;

        self.pc += 2;

        output
    }

    fn execute(&mut self, opcode: u16) {}

    fn load_rom(&mut self, rom: &[u8]) {
        let l = rom.len();
        if l + 512 <= 4096 {
            self.memory[512..512 + l].copy_from_slice(rom);
        } else {
            panic!("Program too large");
        }
    }

    fn update_timers(&mut self) {
        let now = std::time::Instant::now();

        while now.duration_since(self.last_timer_update).as_micros() >= 16_667 {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }

            self.last_timer_update += std::time::Duration::from_micros(16_667);
        }
    }
}
