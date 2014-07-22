use sdl::{render, pixels, video, keycode};
use sdl::rect::Point;
use std::default::Default;

use self::entity::{Entity,Asteroid,Delete};
mod entity;

type Renderer = render::Renderer<video::Window>;

static TURN_SPEED: f32 = 0.001;
static ACCELERATION: f32 = 0.1;

pub struct Loop {
    renderer: Renderer,
    state: State,
}

impl Loop {
    pub fn new(renderer: Renderer) -> Loop {
        let output_size = renderer.get_output_size().unwrap();
        Loop {
            renderer: renderer,
            state: State::new(output_size)
        }
    }

    pub fn handlekey(&mut self, keyDown: bool, key: keycode::KeyCode) {
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
            println!("{}", *keys);
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
        renderer.set_draw_color(pixels::RGB(0, 0, 0)).unwrap();
        renderer.clear().unwrap();
        self.state.player1.draw(renderer);
        let screen_centre = self.state.player1.position();
        for entity in self.state.entities.iter() {
            entity.draw(renderer, screen_centre);
        }
        renderer.present();
    }
}

struct State {
    keys: KeyState,
    player1: PlayerShip,
    entities: Vec<Box<Entity>>,
    delete_list: Vec<Delete>,
    add_list: Vec<Box<Entity>>
}

impl State {
    fn new(dims: (int, int)) -> State {
        State {
            keys: Default::default(),
            player1: PlayerShip::new(dims),
            entities: vec![],
            delete_list: vec![],
            add_list: vec![],
        }
    }
}

#[deriving(Default, PartialEq, Eq, Show)]
struct KeyState {
    down: bool,
    left: bool,
    right: bool,
    up: bool,
    space: bool,
}

// For smoother movement, everything is stored to 64th of a pixel accuracy
struct PlayerShip {
    pos: Point,
    vel: Point,
    angle: f32, // Zero points upwards.
    shape: Vec<(f32,f32)>,
    midpoint: Point,
}

impl PlayerShip {
    fn new((xmax, ymax): (int, int)) -> PlayerShip {
        let midpoint = {
            Point::new((xmax/2) as i32,(ymax/2) as i32)
        };

        let mut shape = Vec::new();
        shape.push_all([(0.0,-10.0),(-5.0,10.0),(0.0,3.0),(5.0,10.0),(0.0,-10.0)]);

        PlayerShip {
            pos: Point::new(0, 0),
            vel: Point::new(0, 0),
            angle: 0.0,
            shape: shape,
            midpoint: midpoint
        }
    }

    fn accelerate(&mut self) {
    }
    fn decelerate(&mut self) {
    }
    fn turn_left(&mut self) {
        self.angle = self.angle - TURN_SPEED;
    }
    fn turn_right(&mut self) {
        self.angle = self.angle + TURN_SPEED;
    }

    fn update(&mut self) {
    }

    fn draw(&self, renderer: &Renderer) {
        let rotated_shape = self.shape
            .iter()
            .map(|&point| rotate(point, self.angle))
            .map(|(x,y)| translate(Point::new(x as i32, y as i32),self.midpoint))
            .collect::<Vec<Point>>();
        renderer.set_draw_color(pixels::RGB(255, 255, 255)).unwrap();
        renderer.draw_lines(rotated_shape.as_slice());
    }

    fn position(&self) -> Point {
        self.pos
    }
}

fn rotate((x,y): (f32,f32), angle: f32) -> (f32,f32) {
    (x * angle.cos() + y * angle.sin(), x * angle.sin() - y * angle.cos())
}

fn translate(point: Point, by: Point) -> Point {
    Point::new(point.x + by.x, point.y + by.y)
}
