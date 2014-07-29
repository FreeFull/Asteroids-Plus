use sdl::{render, video};
use sdl::rect::Point;
pub use self::asteroid::Asteroid;

type Renderer = render::Renderer<video::Window>;

enum EntityType {
    AsteroidType,
    BulletType,
}

struct EntityData {
    position: (f32, f32),
    size: f32,
}

pub struct Entity {
    update: fn(self_: &Entity, entities: &[Entity], to_delete: &mut Vec<uint>, to_add: &mut Vec<Entity>) -> Option<Entity>,
    draw_real: fn(self_: &Entity, renderer: &Renderer, position: Point),
    position: fn(self_: &Entity) -> (f32,f32),
    kind: fn(self_: &Entity) -> EntityType,
    data: EntityData,
}

impl Entity {
    pub fn new_entity(&self, position: (f32, f32), size: f32) -> Entity {
        Entity {
            data: EntityData { position: position, size: size },
            ..*self
        }
    }

    pub fn update(&self, entities: &[Entity], to_delete: &mut Vec<uint>, to_add: &mut Vec<Entity>) -> Option<Entity> {
        (self.update)(self, entities, to_delete, to_add)
    }

    pub fn draw(&self, renderer: &Renderer, screen_view: Point) {
        // TODO: Implement draw.
        let (x,y) = self.data.position;
        let position = Point::new(x as i32, y as i32);
        self.draw_real(renderer, position)
    }

    pub fn draw_real(&self, renderer: &Renderer, position: Point) {
        (self.draw_real)(self, renderer, position)
    }

    pub fn position(&self) -> (f32,f32) {
        (self.position)(self)
    }

    pub fn kind(&self) -> EntityType {
        (self.kind)(self)
    }
}

mod asteroid {
    use super::{Entity, EntityData, EntityType, AsteroidType, Renderer};
    use sdl::rect::{Point, Rect};
    use sdl::pixels;

    fn update(self_: &Entity, entities: &[Entity], to_delete: &mut Vec<uint>, to_add: &mut Vec<Entity>) -> Option<Entity> {
        Some(*self_)
    }

    fn draw_real(self_: &Entity, renderer: &Renderer, position: Point) {
        let size = self_.data.size as i32;
        let rect = Rect::new(position.x - size, position.y - size, size*2, size*2);
        renderer.set_draw_color(pixels::RGB(255,255,255)).unwrap();
        renderer.fill_rect(&rect);
    }

    fn position(self_: &Entity) -> (f32, f32) {
        self_.data.position
    }

    fn kind(_: &Entity) -> EntityType {
        AsteroidType
    }

    pub static Asteroid: Entity = Entity {
        update: update,
        draw_real: draw_real,
        position: position,
        kind: kind,
        data: EntityData {
            position: (0.0, 0.0),
            size: 5.0,
        }
    };
}
