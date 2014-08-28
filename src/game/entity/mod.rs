use sdl::rect::Point;
use game::location::{Location, Displacement};
use game::Renderer;

pub use self::asteroid::Asteroid;
mod asteroid;

enum EntityType {
    AsteroidType,
    BulletType,
}

struct EntityData {
    position: Location,
    velocity: Displacement,
    size: f32,
}

pub struct Entity {
    update: fn(self_: &Entity, entities: &[Entity], to_delete: &mut Vec<uint>, to_add: &mut Vec<Entity>) -> Option<Entity>,
    draw_real: fn(self_: &Entity, renderer: &Renderer, position: Point),
    position: fn(self_: &Entity) -> Location,
    kind: fn(self_: &Entity) -> EntityType,
    data: EntityData,
}

impl Entity {
    pub fn new_entity(&self, position: Location, velocity: Displacement, size: f32) -> Entity {
        Entity {
            data: EntityData { position: position, size: size, velocity: velocity },
            ..*self
        }
    }

    pub fn update(&self, entities: &[Entity], to_delete: &mut Vec<uint>, to_add: &mut Vec<Entity>) -> Option<Entity> {
        (self.update)(self, entities, to_delete, to_add)
    }

    pub fn draw(&self, renderer: &Renderer, (max_x, max_y): (i32, i32), screen_view: Point) {
        // TODO: Implement draw.
        let Point { x, y } = self.data.position.as_point(max_x, max_y);
        let position = Point::new(x + max_x/2, y + max_y/2);
        self.draw_real(renderer, position)
    }

    pub fn draw_real(&self, renderer: &Renderer, position: Point) {
        (self.draw_real)(self, renderer, position)
    }

    pub fn position(&self) -> Location {
        (self.position)(self)
    }

    pub fn kind(&self) -> EntityType {
        (self.kind)(self)
    }
}

