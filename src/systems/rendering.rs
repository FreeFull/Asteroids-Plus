use ecs::{Passive, Aspect, World, Entity};
use components::*;
use super::Renderer;

pub struct RenderingSystem {
    renderer: Renderer,
    entities: Aspect,
    camera: Aspect,
    screen_centre: Position,
}

impl RenderingSystem {
    pub fn new(renderer: Renderer) -> Box<Passive + 'static> {
        let entities = Aspect::for_all(vec![*ID_Position, *ID_Velocity, *ID_Rotation, *ID_Size]);
        let camera = Aspect::for_all(vec![*ID_Position, *ID_Camera]);
        let midpoint = {
            let (x, y) = renderer.get_logical_size();
            Position { x: x as i32, y: y as i32 }
        };

        box RenderingSystem {
            renderer: renderer,
            entities: entities,
            camera: camera,
            screen_centre: midpoint,
        } as Box<Passive + 'static>
    }
}

impl Passive for RenderingSystem {
    // Intentionally left empty
    fn process(&mut self, _: &World) {}

    fn activated(&mut self, entity: &Entity, world: &World) {
        if self.camera.check(entity, world) {
            // Update screen centre position
            world.get_component::<Position>(entity)
                .map(|pos| self.screen_centre = pos);
        }
        if self.entities.check(entity,world) {
            // Render
        }
    }
}
