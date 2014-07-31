use sdl::{video, render};

pub use self::gameloop::Loop;

mod entity;
pub mod location;
pub mod gameloop;

pub type Renderer = render::Renderer<video::Window>;
