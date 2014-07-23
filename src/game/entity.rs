use sdl::{render, pixels, video};
use sdl::rect::{Point, Rect};

type Renderer = render::Renderer<video::Window>;

enum EntityType {
    Asteroid,
    Bullet,
}

pub trait Entity {
    fn update(&self, entities: &[Box<Entity>], to_delete: &mut Vec<uint>, to_add: &mut Vec<Box<Entity>>) -> Option<Box<Entity>>;
    fn draw(&self, renderer: &Renderer, screen_centre: Point) {
        self.draw_real(renderer, Point::new(10,10))
    }
    fn draw_real(&self, renderer: &Renderer, position: Point);
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

impl Asteroid {
    pub fn new_entity(position: (f32,f32), size: f32) -> Box<Entity> {
        box Asteroid {
            position: position,
            size: size,
        } as Box<Entity>
    }
}

impl Entity for Asteroid {
    fn update(&self, entities: &[Box<Entity>], to_delete: &mut Vec<uint>, to_add: &mut Vec<Box<Entity>>) -> Option<Box<Entity>> {
        Some(box *self as Box<Entity>)
    }

    fn draw_real(&self, renderer: &Renderer, position : Point) {
        let size = self.size as i32;
        let rect = Rect::new(position.x - size, position.y - size, size*2, size*2);
        renderer.set_draw_color(pixels::RGB(255,255,255)).unwrap();
        renderer.fill_rect(&rect);
    }

    fn position(&self) -> (f32,f32) {
        self.position
    }

    fn kind(&self) -> EntityType {
        Asteroid
    }
}
