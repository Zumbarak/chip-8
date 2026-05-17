mod chip8;
mod font;
mod keys;
mod stack;

fn main() {
    let path = std::env::args().nth(1).expect("usage: chip-8 <rom-path>");
    let rom = std::fs::read(&path).expect("failed to read ROM file");

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(&rom);

    for _ in 0..1000 {
        let opcode = chip8.fetch();
        chip8.execute(opcode);
        chip8.update_timers();
    }

    chip8.render();
}
