// #![feature(macro_rules)]
#![feature(globs,phase)]
#[phase(plugin,link)]
extern crate ecs;
#[phase(plugin)]
extern crate lazy_static;
extern crate sdl = "sdl2";
use sdl::{video, render};

use ecs::World;

use components::register_components;
use systems::register_systems;
mod components;
mod systems;

fn main() {
    sdl::init(sdl::InitAudio | sdl::InitVideo);

    let window = video::Window::new("Asteroids Plus",
                                    video::PosCentered, video::PosCentered,
                                    512, 512,
                                    video::WindowFlags::empty()).unwrap();
    let renderer = render::Renderer::from_window(window,
                                                 render::DriverAuto,
                                                 render::Accelerated).unwrap();
    let mut world = World::new();
    let (tx, quit) = sync_channel::<()>(1);
    register_components(&mut world);
    register_systems(&mut world, renderer, tx);

    world.finalise();

    loop {
        world.update();
        match quit.try_recv() {
            Ok(_) => break,
            _ => {}
        }
    }

    sdl::quit();
}
