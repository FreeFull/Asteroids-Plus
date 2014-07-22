use sdl::{render, pixels, video};
use sdl::rect::Point;

type Renderer = render::Renderer<video::Window>;

enum EntityType {
    Asteroid,
    Bullet,
}

pub trait Entity {
    fn update(&mut self, entities: &Vec<Box<Entity>>, to_delete: &mut Vec<Delete>, to_add: &mut Vec<Box<Entity>>);
    fn draw(&self, renderer: &Renderer, screen_centre: Point) {
        self.draw_real(renderer)
    }
    fn draw_real(&self, renderer: &Renderer);
    fn position(&self) -> (f32,f32);
    fn screen_position(&self, view: Point) -> Point {
        view
    }
    fn kind(&self) -> EntityType;
}

pub struct Asteroid {
    position: (f32,f32),
    size: f32,
}

impl Entity for Asteroid {
    fn update(&mut self, entities: &Vec<Box<Entity>>, to_delete: &mut Vec<Delete>, to_add: &mut Vec<Box<Entity>>) {
    }

    fn draw_real(&self, renderer: &Renderer) {
        renderer.set_draw_color(pixels::RGB(255,255,255)).unwrap();
    }

    fn position(&self) -> (f32,f32) {
        self.position
    }

    fn kind(&self) -> EntityType {
        Asteroid
    }
}

pub struct Delete {
    id: uint,
}
