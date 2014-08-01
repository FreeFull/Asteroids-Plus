extern crate libc;
extern crate sdl = "sdl2";
use sdl::{video, event, keycode, timer, render};

use std::io::BufWriter;

use game::Loop;
mod game;

static MS_PER_FRAME : uint = 15; // About 60fps

fn main() {
    sdl::init(sdl::InitAudio | sdl::InitVideo);

    let window = video::Window::new("Asteroids Plus",
                                    video::PosCentered, video::PosCentered,
                                    512, 512,
                                    video::WindowFlags::empty()).unwrap();
    let renderer = render::Renderer::from_window(window,
                                                 render::DriverAuto,
                                                 render::Accelerated).unwrap();
    let mut game_loop = Loop::new(renderer);
    let mut previous_time = timer::get_ticks();
    let mut sample_time = timer::get_ticks();
    let mut count = 0u;

    'main : loop {
        'event : loop {
            match event::poll_event() {
                event::QuitEvent(_) => break 'main,
                event::KeyDownEvent(_, _, key, _, _) => match key {
                    keycode::EscapeKey => break 'main,
                    _ => game_loop.handlekey(true, key)
                },
                event::KeyUpEvent(_, _, key, _, _) => match key {
                    _ => game_loop.handlekey(false, key)
                },
                event::NoEvent => break 'event,
                _ => (),
            }
        }

        game_loop.update();

        let current_time = timer::get_ticks();
        let time_taken = current_time - previous_time;
        previous_time = current_time;
        if count == 0 {
            println!("FPS: {}", 64000/(current_time - sample_time));
            count = 128;
            sample_time = current_time;
        } else { count -= 1; }
        if time_taken < MS_PER_FRAME {
            timer::delay(MS_PER_FRAME - time_taken);
        }
    }

    println!("Test!");
    sdl::quit();
}
