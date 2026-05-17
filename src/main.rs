mod chip8;
mod font;
mod keys;
mod stack;

fn main() {
    let mut chip8 = chip8::Chip8::new();
    loop {
        let opcode = fetch();
        execute(opcode);
    }
}
