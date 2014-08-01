use sdl::rect::Point;

type Contents = i16;
static MAX : Contents = ::std::i16::MAX;

macro_rules! create_struct {
    ($name: ident) => {
        pub struct $name {
            pub x: Contents,
            pub y: Contents,
        }


        impl $name {
            pub fn new(x: f32, y: f32) -> Option<$name> {
                if x >= -1.0 && y >= -1.0 && x <= 1.0 && y <= 1.0 {
                    return Some( $name {
                        x: (x * MAX as f32) as Contents,
                        y: (y * MAX as f32) as Contents,
                    })
                }
                None
            }

            pub fn midpoint() -> $name {
                $name {
                    x: 0,
                    y: 0,
                }
            }

            pub fn as_point(&self, max_x: i32, max_y: i32) -> Point {
                Point::new(
                    self.x as i32 * max_x / MAX as i32,
                    self.y as i32 * max_y / MAX as i32,
                    )
            }
        }
    }
}

macro_rules! impl_add {
    ($lhs: ident, $rhs: ident) => {
        impl Add<$rhs, $lhs> for $lhs {
            fn add(&self, rhs: &$rhs) -> $lhs {
                $lhs {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
    }
}

create_struct!(Location)
create_struct!(Displacement)
create_struct!(Acceleration)
impl_add!(Location, Displacement)
impl_add!(Displacement, Acceleration)
