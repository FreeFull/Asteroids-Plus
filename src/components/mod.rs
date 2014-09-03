use ecs::{Component, World};

component!( ID_Position: Position {
    x: i32,
    y: i32
})

component!( ID_Velocity: Velocity {
    x: i32,
    y: i32
})

component!( ID_Acceleration: Acceleration {
    x: i32,
    y: i32
})

component!( ID_Rotation: Rotation {
    angle: u8
})

component!( ID_Size: Size {
    size: u8
})

component!( ID_Camera: Camera )

pub fn register_components(world: &mut World) {
    world.register_component::<Position>();
    world.register_component::<Velocity>();
    world.register_component::<Acceleration>();
    world.register_component::<Rotation>();
    world.register_component::<Size>();
    world.register_component::<Camera>();
}
