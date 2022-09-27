pub mod cpu;

use cpu::HS65P64;

use std::{
    thread,
    sync::{Arc, Mutex}
};
use lazy_static::lazy_static;

use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use pixels::{Pixels, SurfaceTexture};

pub const TEST_INSTRUCTIONS: [u16; 3] = [
    0x00, // brk
    0xea, // nop
    0x42, // test
];

lazy_static! {
    pub static ref MEMORY_BUFFER: Arc<Mutex<[u8; 1024]>> = Arc::new(Mutex::new([0; 1024]));
}

fn render(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % 800 as usize) as i16;
        let y = (i / 800 as usize) as i16;

        let inside = x >= 10 && x < 110
            && y > 20 && y < 120;

        let rgba = if inside {
            [0x5e, 0x99, 0x39, 0xff]
        } else {
            [0x48, 0xb2, 0xe8, 0xff]
        };

        pixel.copy_from_slice(&rgba);
    }
}

fn main() -> ! {

    // Initialize emulation

    let _emulation_thread_handle = thread::Builder::new()
        .name("emulation".to_string())
        .spawn(|| {
            let mut core = HS65P64::new();
            { print!("{:?}", MEMORY_BUFFER.lock().unwrap()); }
            loop {
                core.step(0x42)
            }
        });

    { MEMORY_BUFFER.lock().unwrap()[0] = 0x42; }

    // Initialize rendering

    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Yardland")
        //.with_min_inner_size(LogicalSize {width: 800, height: 600})
        .build(&event_loop)
        .expect("Window could not be built.");

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(800, 600, surface_texture).expect("Pixels lib could not be initialized.");

    event_loop.run(move |event, _, control_flow| {
        //control_flow.set_poll();

        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                control_flow.set_exit()
            }
        }

        if let Event::RedrawRequested(_) = event {
            render(pixels.get_frame());
            if pixels.render().is_err() {
                control_flow.set_exit()
            }
        }

        if let Event::WindowEvent{event, ..} = event {
            if let WindowEvent::Resized(new_size) = event {
                pixels.resize_surface(new_size.width, new_size.height)
            }
        }

        window.request_redraw();
    })
}
