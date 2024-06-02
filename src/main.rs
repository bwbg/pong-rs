// TODO: Remove the following line before release!
#![allow(dead_code, unused_imports)]

mod config;
mod error;
mod prelude;

use sdl2::event::Event;
use std::time::Duration;

use prelude::*;

/// Application's entry point.
fn main() -> Result<()> {
    let config = crate::config::get().unwrap();

    let sdl_context = sdl2::init().unwrap();
    let sdl_video_subsystem = sdl_context.video().unwrap();
    let window = sdl_video_subsystem
        .window(
            &config.display_title,
            config.display_size.0,
            config.display_size.1,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(1000) / config.display_fps);
    }
    Ok(())
}
