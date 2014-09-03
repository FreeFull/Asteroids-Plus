use ecs::World;

type Renderer = ::sdl::render::Renderer<::sdl::video::Window>;

mod rendering {
    use ecs::{Passive, Aspect, World};
    use components::{ID_Position, ID_Velocity, ID_Rotation, ID_Size, ID_Camera};
    use super::Renderer;

    pub struct RenderingSystem {
        renderer: Renderer,
        entities: Aspect,
        camera: Aspect,
    }

    impl RenderingSystem {
        pub fn new(renderer: Renderer) -> Box<Passive + 'static> {
            let entities = Aspect::for_all(vec![*ID_Position, *ID_Velocity, *ID_Rotation, *ID_Size]);
            let camera = Aspect::for_all(vec![*ID_Camera]);

            box RenderingSystem {
                renderer: renderer,
                entities: entities,
                camera: camera,
            } as Box<Passive + 'static>
        }
    }

    impl Passive for RenderingSystem {
        fn process(&mut self, world: &World) {

        }
    }
}

pub fn register_systems(world: &mut World, renderer: Renderer) {
    world.register_passive("render", rendering::RenderingSystem::new(renderer));
}
