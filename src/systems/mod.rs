use ecs::World;

type Renderer = ::sdl::render::Renderer<::sdl::video::Window>;

mod rendering;
mod key_handler;

pub fn register_systems(world: &mut World, renderer: Renderer, quit: SyncSender<()>) {
    world.register_system(key_handler::KeyHandler::new(quit));
    world.register_passive("render", rendering::RenderingSystem::new(renderer));
}
