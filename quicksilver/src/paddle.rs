extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
    lifecycle::{Asset, Window},
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
            width: Vector::new(50, 200),
            background: Color::BLUE
        }
    }

}
