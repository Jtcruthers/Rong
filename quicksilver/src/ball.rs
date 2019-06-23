extern crate quicksilver;

use quicksilver::{
    geom::Vector,
    graphics::Image,
    lifecycle::Asset,
};

use crate::paddle::Paddle;

pub struct Ball {
    position: Vector,
    velocity: Vector,
    background: Asset<Image>,
}


impl Ball {
    pub fn new() -> Ball {
        Ball {
            position: Vector::new(50, 50),
            velocity: Vector::new(-10, -10),
            background: Asset::new(Image::load("image.png")),
        }
    }

    pub fn get_background(&mut self) -> &mut Asset<Image> {
        &mut self.background
    }

    pub fn get_position(&self) -> &Vector {
        &self.position
    }

    fn reverse_x(&mut self) {
        self.velocity.x *= -1.0;
    }

    fn reverse_y(&mut self) {
        self.velocity.y *= -1.0;
    }
    pub fn update(&mut self, player_1: &Paddle, player_2: &Paddle) {
        if self.position.x <= 0.0 || self.position.x >= 1920.0 {
            self.velocity.x *= -1.0;
        }
        if self.position.y <= 0.0 || self.position.y >= 1080.0 {
            self.velocity.y *= -1.0;
        }
        if self.position.y >= player_1.position.y && self.position.y <= (player_1.position.y + player_1.width.y) && self.position.x <= 90.0 {
            self.reverse_x();
        }
        if self.position.y >= player_2.position.y && self.position.y <= (player_2.position.y + player_2.width.y) && self.position.x >= 1820.0 {
            self.reverse_x();
        }

        self.position += self.velocity
    }
}