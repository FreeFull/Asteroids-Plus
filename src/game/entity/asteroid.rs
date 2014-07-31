use super::{Entity, EntityData, EntityType, AsteroidType};
use game::location::Location;
use game::Renderer;
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

fn position(self_: &Entity) -> Location {
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
        position: Location { x: 0, y: 0 },
        size: 5.0,
    }
};
