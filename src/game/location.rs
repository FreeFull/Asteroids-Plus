use sdl::rect::Point;

static MAXu16 : u16 = ::std::u16::MAX;

pub struct Location {
    pub x: u16,
    pub y: u16,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Option<Location> {
        if x >= 0.0 && y >= 0.0 && x <= 1.0 && y <= 1.0 {
                return Some( Location {
                    x: (x * MAXu16 as f32) as u16,
                    y: (y * MAXu16 as f32) as u16,
                })
            }
        None
    }

    pub fn as_point(&self, max_x: i32, max_y: i32) -> Point {
        Point::new(
            self.x as i32 * max_x / MAXu16 as i32,
            self.y as i32 * max_y / MAXu16 as i32,
            )
    }
}
