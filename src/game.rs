use sdl::{event,render,pixels,video,keycode};
use sdl::rect::Point;
use std::default::Default;

type Renderer = render::Renderer<video::Window>;

pub struct Loop {
    renderer : Renderer,
    state : State,
}

impl Loop {
    pub fn new(renderer : Renderer) -> Loop {
        let output_size = renderer.get_output_size().unwrap();
        Loop {
            renderer : renderer,
            state : State::new(output_size)
        }
    }

    pub fn handlekey(&mut self, keyDown : bool, key : keycode::KeyCode) {
        // I'm feeling a bit iffy about this. Ideally, no dummy variable
        // should be needed.
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
            println!("{}",*keys);
        }
    }

    pub fn update(&mut self) {
        {
            let keys = self.state.keys;
            let player = &mut self.state.player1;
            if keys.up { player.accelerate() }
            if keys.down { player.decelerate() }
            if keys.left { player.turn_left() }
            if keys.right { player.turn_right() }
        }
        self.draw();
    }

    pub fn draw(&self) {
        let renderer = &self.renderer;
        renderer.set_draw_color(pixels::RGB(0,0,0)).unwrap();
        renderer.clear().unwrap();
        self.state.player1.draw(renderer);
        renderer.present();
    }
}

struct State {
    keys : KeyState,
    player1 : PlayerShip
}

impl State {
    fn new(dims : (int,int)) -> State {
        let midpoint = {
        };
        State {
            keys : Default::default(),
            player1 : PlayerShip::new(dims)
        }
    }
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
    dimensions : (int,int),
}

impl PlayerShip {
    fn new((xmax,ymax) : (int,int)) -> PlayerShip {
        PlayerShip {
            pos : Point::new(0,0),
            vel : Point::new(0,0),
            angle : 0.0,
            dimensions : (xmax*64,ymax*64)
        }
    }

    fn accelerate(&mut self) {
    }
    fn decelerate(&mut self) {
    }
    fn turn_left(&mut self) {
    }
    fn turn_right(&mut self) {
    }

    fn update(&mut self) {
    }

    fn draw(&self, renderer : &Renderer) {
        renderer.set_draw_color(pixels::RGB(255,255,255)).unwrap();
    }
}
