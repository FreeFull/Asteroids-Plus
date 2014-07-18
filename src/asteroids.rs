extern crate sdl = "sdl2";
use sdl::{video, event, keycode, timer, render};

use game::Loop;
mod game;

static MS_PER_FRAME : uint = 16; // About 60fps

fn main() {
    sdl::init(sdl::InitAudio | sdl::InitVideo);

    let window = video::Window::new("Asteroids Plus",
                                    video::PosCentered, video::PosCentered,
                                    500, 500,
                                    video::WindowFlags::empty()).unwrap();
    let renderer = render::Renderer::from_window(window,
                                                 render::DriverAuto,
                                                 render::Accelerated).unwrap();
    let mut game_loop = Loop::new(renderer);

    let mut previousTime = timer::get_ticks();
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
                _ => ()
            }
        }

        game_loop.update();

        let time_taken = timer::get_ticks() - previousTime;
        if time_taken > MS_PER_FRAME {
            continue;
        }
        timer::delay(MS_PER_FRAME - time_taken);
        previousTime = timer::get_ticks();
    }

    sdl::quit();
}
