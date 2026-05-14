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

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Framebuffer {
    pixels: [u8; WIDTH * HEIGHT],
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            pixels: [0; WIDTH * HEIGHT],
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if x < WIDTH && y < HEIGHT {
            self.pixels[y * WIDTH + x] = value;
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.pixels[y * WIDTH + x]
    }
}

fn draw_char(fb: &mut Framebuffer, char: u8, x: usize, y: usize) {
    let index = char as usize * 5;

    for row in 0..5 {
        let byte = FONTSET[index + row];

        for col in 0..8 {
            let bit = byte >> (7 - col) & 1;

            if bit == 1 {
                fb.set_pixel(x + col, y + row, 1);
            }
        }
    }
}

pub fn draw_str(fb: &mut Framebuffer, text: &str, mut x: usize, y: usize) {
    for c in text.bytes() {
        if c.is_ascii_digit() {
            draw_char(fb, c - b'0', x, y);
        } else if (b'A'..=b'F').contains(&c) {
            draw_char(fb, 10 + c - b'A', x, y);
        }

        x += 8;
    }

    print_fb(fb);
}

pub fn print_fb(fb: &Framebuffer) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = fb.get_pixel(x, y);
            if x % 64 == 0 {
                println!();
            }
            print!("{}", if pixel == 1 { "#" } else { "." });
        }
    }
    println!();
}
