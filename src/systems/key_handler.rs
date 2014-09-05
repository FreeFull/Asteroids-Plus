use ecs::{System, World, Components};
use std::default::Default;

#[deriving(Default)]
struct KeyState {
    accel: bool,
    deccel: bool,
    clockwise: bool,
    counterclockwise: bool,
    shoot: bool,
}

pub struct KeyHandler {
    quit: SyncSender<()>,
    state: KeyState,
}

impl KeyHandler {
    pub fn new(quit: SyncSender<()>) -> Box<System + 'static> {
        box KeyHandler {
            quit: quit,
            state: Default::default(),
        }
    }
}

impl System for KeyHandler {
    fn process(&self, world: &World, components: &mut Components) {
    }
}
