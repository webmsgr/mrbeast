#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{num::NonZeroU32};

use rodio::{OutputStream, Source};
use winit::{
    event_loop::{EventLoop},
    window::WindowBuilder, dpi::LogicalSize,
};
use game_loop::game_loop;
use beast_assets::*;

fn main() {
    println!("Loading...");
    load();
    println!("Loaded!");
    let event_loop = EventLoop::new();
    let size = LogicalSize::new(VIDEO.0.0, VIDEO.0.1);
    let window = WindowBuilder::new().with_inner_size(size).with_resizable(false).with_title("mrbeast.exe").build(&event_loop).unwrap();
    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    let frame: usize = 0;
    let (width, height) = {
        let size = window.inner_size();
        (size.width, size.height)
    };
    surface
        .resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        )
        .unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(AUDIO.lock().unwrap().take().unwrap().convert_samples()).unwrap();
    game_loop(event_loop, window, (frame, surface, context), 30, 0.1, |g| {
        g.game.0 += 1;
        if g.game.0 >= VIDEO.1.len() {
            g.exit();
        }
    }, |g| {
        if g.game.0 >= VIDEO.1.len() {
            return;
        }
        let frame = &VIDEO.1[g.game.0];
        let mut buffer = g.game.1.buffer_mut().unwrap();
        buffer.copy_from_slice(frame);
        buffer.present().unwrap();
    }, |_g, _event| {})
    /*event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now()+Duration::from_millis(33));
        if frame >= VIDEO.1.len() {
            *control_flow = ControlFlow::Exit;
            return;
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::NewEvents(winit::event::StartCause::WaitCancelled { .. }) => {
                frame += 1;
                window.request_redraw();
            }
            Event::RedrawRequested(r) if r == window.id() => {
                
            },
            _ => (),
        }
    });*/
}