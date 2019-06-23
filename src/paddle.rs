extern crate quicksilver;

use quicksilver::{
    geom::{Vector},
    graphics::{Color},
};

pub struct Paddle {
    pub position: Vector,
    pub width: Vector,
    pub background: Color
}

impl Default for Paddle {
    fn default() -> Paddle {
        Paddle {
            position: Vector::new(50, 50),
            width: Vector::new(40, 200),
            background: Color::WHITE
        }
    }

}
