extern crate quicksilver;
extern crate rand;

use rand::Rng;

use quicksilver::{
    Result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background, Color, Image},
    input::Key,
    lifecycle::{Asset, Settings, Window},
};

pub struct Ball {
    position: Vector,
    velocity: Vector,
    background: Asset<Image>,
}


impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: Vector::new(50, 50),
            velocity: Vector::new(0, 0),
            background: Asset::new(Image::load("image.png")),
        }
    }

    pub fn get_background(&mut self) -> &mut Asset<Image> {
        &mut self.background
    }

    pub fn get_position(&self) -> &Vector {
        &self.position
    }
}