#![feature(result_option_inspect)]

pub mod cpu65p64;
pub mod memmap;

use std::{
    thread,
    time::Instant,
    sync::{
        mpsc,
        mpsc::{Sender, Receiver}
    }
};

use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::LogicalSize,
};
use winit_input_helper::WinitInputHelper;
use pixels::{Pixels, SurfaceTexture};

pub const TEST_INSTRUCTIONS: [u16; 3] = [
    0x00, // brk
    0xea, // nop
    0x42, // test
];

fn main() -> ! {
    // Initialize thread channels

    let (video_tx, video_rx): (Sender<(usize, u8)>, Receiver<(usize, u8)>) = mpsc::channel();

    // Initialize emulation

    let _emulation_thread_handle = thread::Builder::new()
        .name("emulation".to_string())
        .spawn(move || {
            let mut _emu_perf_instant = Instant::now();

            let mut mmap = memmap::MemMap::new();

            for f in 0..(768 * 432 * 4) {
                mmap.map(0xa0 + f, memmap::MappedFrame::Video(f, video_tx.clone()));
            }
        });

    // Initialize rendering

    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Yardland")
        .with_inner_size(LogicalSize::new(1366, 768))
        .with_transparent(true)
        .build(&event_loop)
        .expect("Window could not be built.");

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(768, 432, surface_texture).expect("Pixels lib could not be initialized.");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        if input.update(&event) {
            if input.quit() {
                control_flow.set_exit()
            }
            if input.key_released(VirtualKeyCode::F11) {
                window.set_inner_size(LogicalSize::new(768, 432))
            }
        }

        if let Event::MainEventsCleared = event {
            for (index, data) in video_rx.try_iter() {
                pixels.get_frame()[index] = data
            }
        }

        if let Event::RedrawRequested(_) = event {
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
