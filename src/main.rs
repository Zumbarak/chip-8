mod chip8;
mod font;
mod keys;
mod stack;

use std::num::NonZeroU32;
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

const W: usize = 64;
const H: usize = 32;
const SCALE: usize = 12;

struct App {
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    chip8: chip8::Chip8,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("CHIP-8")
            .with_inner_size(winit::dpi::LogicalSize::new(
                (W * SCALE) as f64,
                (H * SCALE) as f64,
            ));
        let window = Rc::new(event_loop.create_window(attrs).unwrap());
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                for _ in 0..10 {
                    let opcode = self.chip8.fetch();
                    self.chip8.execute(opcode);
                }
                self.chip8.update_timers();

                let window = self.window.as_ref().unwrap();
                let surface = self.surface.as_mut().unwrap();
                let size = window.inner_size();
                let ww = size.width.max(1);
                let wh = size.height.max(1);
                surface
                    .resize(NonZeroU32::new(ww).unwrap(), NonZeroU32::new(wh).unwrap())
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for wy in 0..wh as usize {
                    for wx in 0..ww as usize {
                        let cx = wx * W / ww as usize;
                        let cy = wy * H / wh as usize;
                        buffer[wy * ww as usize + wx] = if self.chip8.pixel(cx, cy) {
                            0x00FF_FFFF
                        } else {
                            0
                        };
                    }
                }
                buffer.present().unwrap();
                window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
                    if let Some(idx) = keys::map_key(code) {
                        let pressed = event.state == winit::event::ElementState::Pressed;
                        self.chip8.set_key(idx, pressed);
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: chip-8 <rom-path>");
    let rom = std::fs::read(&path).expect("failed to read ROM file");

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(&rom);

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App {
        window: None,
        surface: None,
        chip8,
    };
    event_loop.run_app(&mut app).unwrap();
}
