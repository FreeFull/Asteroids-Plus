use sdl::{event,render,pixels,video,keycode};
use sdl::rect::Point;
use std::default::Default;

type Renderer = render::Renderer<video::Window>;

pub struct Loop {
    renderer : Renderer,
    state : State,
}

struct FloatPoint(f64,f64);

impl Loop {
    pub fn new(renderer : Renderer) -> Loop {
        midpoint = {
            let (x,y) = renderer.get_output_size().unwrap();
            (x*32,y*32)
        };
        Loop {
            renderer : renderer,
            state : State {
                keys : Default::default(),
                player1 : PlayerShip {
                    pos : midpoint,
                    vec : FloatPoint(0.0,0.0),
                    dir : FloatPoint(0.0,1.0),
                }
            }
        }
    }

    pub fn handlekey(&mut self, keyDown : bool, key : keycode::KeyCode) {
        let mut dummy = false; 
        let keys = &mut self.state.keys;
        let oldkeys = *keys;
        *match key {
            keycode::UpKey => &mut keys.up,
            keycode::DownKey => &mut keys.down,
            keycode::LeftKey => &mut keys.left,
            keycode::RightKey => &mut keys.right,
            keycode::SpaceKey => &mut keys.space,
            _ => &mut dummy // Ignore any other keys
        } = keyDown;
        if *keys != oldkeys {
            println!("{}",self.state.keys);
        }
    }

    pub fn update(&mut self) {
        let keys = self.keys;
        let player = &mut self.player1;
        if keys.up { player.accelerate() }
        if keys.down { player.decelerate() }
        if keys.left { player.turn_left() }
        if keys.right { player.turn_right() }
        self.draw();
    }

    pub fn draw(&self) {
        let renderer = &self.renderer;
        renderer.set_draw_color(pixels::RGB(0,0,0)).unwrap();
        renderer.clear().unwrap();
        renderer.set_draw_color(pixels::RGB(255,255,255)).unwrap();
        renderer.draw_point(self.player1.real_pos()).unwrap();
        renderer.present();
    }
}

struct State {
    keys : KeyState,
    player1 : PlayerShip
}

#[deriving(Default,PartialEq,Eq,Show)]
struct KeyState {
    down : bool,
    left : bool,
    right : bool,
    up: bool,
    space: bool,
}

// For smoother movement, everything is stored to 64th of a pixel accuracy
struct PlayerShip {
    pos : Point,
    vel : Point,
    angle : f32, // Zero points upwards.
    dimensions : (uint,uint),
}

impl PlayerShip {
    fn real_pos(&self) -> Point {
        Point(0,0) // TODO
    }

    fn update(&mut self) {
    }
}
