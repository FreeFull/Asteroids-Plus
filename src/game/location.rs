#![allow(unused_code)]
use sdl::rect::Point;

type Contents = i32;
static MAX : Contents = ::std::i32::MAX;

macro_rules! create_struct {
    ($name: ident) => {
        #[deriving(Show)]
        pub struct $name {
            pub x: Contents,
            pub y: Contents,
        }


        impl $name {
            pub fn new(x: f64, y: f64) -> Option<$name> {
                if x >= -1.0 && y >= -1.0 && x <= 1.0 && y <= 1.0 {
                    return Some( $name {
                        x: (x * MAX as f64) as Contents,
                        y: (y * MAX as f64) as Contents,
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

macro_rules! impl_sub {
    ($from: ident, $result: ident) => {
        impl Sub<$from,$result> for $from {
            fn sub(&self, rhs: &$from) -> $result {
                $result {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
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
impl_sub!(Location, Displacement)
impl_sub!(Displacement, Acceleration)

impl Location {
    pub fn as_point(&self, max_x: i32, max_y: i32) -> Point {
        let location_x = self.x as i64 - ::std::i32::MIN as i64;
        let location_y = self.y as i64 - ::std::i32::MIN as i64;
        let x = location_x * max_x as i64 / ::std::u32::MAX as i64;
        let y = location_y * max_y as i64 / ::std::u32::MAX as i64;
        Point::new(x as i32, y as i32)
    }
}
