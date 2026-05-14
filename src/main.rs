mod font;
mod stack;

fn main() {
    let mut fb = font::Framebuffer::new();

    font::draw_str(&mut fb, "C0DE", 10, 10);
    font::draw_str(&mut fb, "C0DE", 10, 10);
}
