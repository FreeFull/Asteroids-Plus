use sdl::{pixels, keycode};
use sdl::rect::Point;
use std::default::Default;

use game::entity::{Entity, Asteroid};
use game::location::{Location, Displacement, Acceleration};
use game::Renderer;

static TURN_SPEED: f32 = 0.01;
static ACCELERATION: f32 = 0.1;

pub struct Loop {
    renderer: Renderer,
    state: State,
}

impl Loop {
    pub fn new(renderer: Renderer) -> Loop {
        let (xmax, ymax) = renderer.get_output_size().unwrap();
        let output_size = (xmax as i32, ymax as i32);
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
        *match key {
            keycode::UpKey => &mut keys.up,
            keycode::DownKey => &mut keys.down,
            keycode::LeftKey => &mut keys.left,
            keycode::RightKey => &mut keys.right,
            keycode::SpaceKey => &mut keys.space,
            _ => &mut dummy // Ignore any other keys
        } = keyDown;
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
        self.state.player1.update();
        for entity in self.state.entities.iter() {
            match entity.update(self.state.entities.as_slice(), &mut self.state.delete_list, &mut self.state.add_list) {
                Some(e) => self.state.updated_entities.push(e),
                None => {}
            }
        }
        ::std::mem::swap(&mut self.state.entities, &mut self.state.updated_entities);
        self.state.updated_entities.truncate(0);
        self.draw();
    }

    pub fn draw(&self) {
        let renderer = &self.renderer;
        renderer.set_draw_color(pixels::RGB(0, 0, 0)).unwrap();
        renderer.clear().unwrap();
        self.state.player1.draw(renderer);
        let screen_centre = self.state.player1.position();
        for entity in self.state.entities.iter() {
            entity.draw(renderer, self.state.output_size, screen_centre);
        }
        renderer.present();
    }
}

struct State {
    keys: KeyState,
    player1: PlayerShip,
    entities: Vec<Entity>,
    updated_entities: Vec<Entity>,
    delete_list: Vec<uint>,
    add_list: Vec<Entity>,
    output_size: (i32, i32),
}

impl State {
    fn new(output_size: (i32, i32)) -> State {
        State {
            keys: Default::default(),
            player1: PlayerShip::new(output_size),
            entities: vec![Asteroid.new_entity(Location::new(0.1, 0.1).unwrap(), Displacement::midpoint(), 4.0)],
            updated_entities: vec![],
            delete_list: vec![],
            add_list: vec![],
            output_size: output_size,
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

struct PlayerShip {
    pos: Location,
    vel: Displacement,
    angle: f32, // Zero points upwards.
    shape: Vec<(f32,f32)>,
    output_size: (i32, i32),
}

impl PlayerShip {
    fn new(output_size: (i32, i32)) -> PlayerShip {
        let mut shape = Vec::new();
        shape.push_all([(0.0,-10.0),(-5.0,10.0),(0.0,3.0),(5.0,10.0),(0.0,-10.0)]);

        PlayerShip {
            pos: Location::midpoint(),
            vel: Displacement::midpoint(),
            angle: 0.0,
            shape: shape,
            output_size: output_size,
        }
    }

    fn accelerate(&mut self) {
        let (x, y) = rotate((0.0, -1.0 * ACCELERATION), self.angle);
        let acc = Acceleration::new(x, y).unwrap();
        self.vel = self.vel + acc;
    }
    fn decelerate(&mut self) {
        let (x, y) = rotate((0.0, 1.0 * ACCELERATION), self.angle);
        let acc = Acceleration::new(x, y).unwrap();
        self.vel = self.vel + acc;
    }
    fn turn_left(&mut self) {
        self.angle = self.angle + TURN_SPEED;
    }
    fn turn_right(&mut self) {
        self.angle = self.angle - TURN_SPEED;
    }

    fn update(&mut self) {
    }

    fn draw(&self, renderer: &Renderer) {
        let rotated_shape = self.shape
            .iter()
            .map(|&point| rotate(point, self.angle))
            .map(|(x,y)| translate(Point::new(x as i32, y as i32),self.midpoint()))
            .collect::<Vec<Point>>();
        renderer.set_draw_color(pixels::RGB(255, 255, 255)).unwrap();
        renderer.draw_lines(rotated_shape.as_slice());
    }

    fn position(&self) -> Point {
        let (xmax, ymax) = self.output_size;
        self.pos.as_point(xmax, ymax)
    }

    fn midpoint(&self) -> Point {
        let (xmax, ymax) = self.output_size;
        Point::new(xmax/2, ymax/2)
    }
}

fn rotate((x,y): (f32,f32), angle: f32) -> (f32,f32) {
    (x * angle.cos() + y * angle.sin(), - x * angle.sin() + y * angle.cos())
}

fn translate(point: Point, by: Point) -> Point {
    Point::new(point.x + by.x, point.y + by.y)
}
