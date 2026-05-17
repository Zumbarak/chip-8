use crate::font::Framebuffer;
use crate::stack::Stack;

pub struct Chip8 {
    delay_timer: u8,
    sound_timer: u8,
    last_timer_update: std::time::Instant,
    keys: [bool; 16],
    memory: [u8; 4096],
    pc: u16,
    v: [u8; 16],
    i: u16,
    stack: Stack,
    fb: Framebuffer,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            delay_timer: 0,
            sound_timer: 0,
            last_timer_update: std::time::Instant::now(),
            keys: [false; 16],
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: Stack::new(),
            fb: Framebuffer::new(),
        }
    }

    pub fn render(&self) {
        for y in 0..32 {
            let mut line = String::with_capacity(64);
            for x in 0..64 {
                line.push(if self.fb.get_pixel(x, y) == 1 {
                    '#'
                } else {
                    '.'
                });
            }
            println!("{line}");
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let mut output: u16 = self.memory[self.pc as usize] as u16;
        output = output << 8;
        output |= self.memory[(self.pc + 1) as usize] as u16;

        self.pc += 2;

        output
    }

    pub fn execute(&mut self, opcode: u16) {
        let nnn: u16 = opcode & 0xFFF;
        let first_nibble: u8 = (opcode >> 12) as u8;
        let x: usize = ((opcode >> 8) & 0xF) as usize;
        let y: usize = ((opcode >> 4) & 0xF) as usize;
        let nn: u8 = (opcode & 0xFF) as u8;
        let n: u8 = (opcode & 0xF) as u8;

        match first_nibble {
            0x0 => {
                //Clear Screen
                if opcode == 0x00E0 {
                    self.fb.clear();
                } else {
                    self.pc = self.stack.pop();
                }
            }
            0x1 => {
                //jump
                self.pc = nnn;
            }
            0x2 => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            0x6 => {
                //set vx
                self.v[x] = nn;
            }
            0x7 => {
                //add value to vx
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0xA => {
                //Set I
                self.i = nnn;
            }
            0xD => {
                //Draw
                let px = self.v[x] as usize % 64;
                let py = self.v[y] as usize;
                self.v[0xF] = 0;

                for row in 0..n as usize {
                    if row + py >= 32 {
                        break;
                    }
                    let byte = self.memory[self.i as usize + row] as u8;
                    for col in 0..8 {
                        if col + px >= 64 {
                            break;
                        }
                        let bit = byte >> (7 - col) & 1;
                        if bit == 1 {
                            let value = self.fb.get_pixel(px + col, py + row);
                            self.fb.set_pixel(px + col, py + row, value ^ 1);
                            if value == 1 {
                                self.v[0xF] = 1;
                            }
                        }
                    }
                }
            }

            _ => panic!("Undefined instruction"),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let l = rom.len();
        if l + 512 <= 4096 {
            self.memory[512..512 + l].copy_from_slice(rom);
        } else {
            panic!("Program too large");
        }
    }

    pub fn update_timers(&mut self) {
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

    pub fn pixel(&self, x: usize, y: usize) -> bool {
        self.fb.get_pixel(x, y) == 1
    }
}
