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
            position: Vector::new(960, 540),
            velocity: Vector::new(-10, -8),
            background: Asset::new(Image::load("image.png")),
        }
    }

    pub fn get_background(&mut self) -> &mut Asset<Image> {
        &mut self.background
    }

    pub fn get_position(&self) -> &Vector {
        &self.position
    }

    pub fn reset(&mut self) {
        self.position = Vector::new(960, 540);
        self.velocity = Vector::new(-10, -8);
    }

    fn reverse_x(&mut self) {
        self.velocity.x *= -1.0;
    }

    fn reverse_y(&mut self) {
        self.velocity.y *= -1.0;
    }

    pub fn update(&mut self, player_1: &Paddle, player_2: &Paddle) {
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

    pub fn did_score(&mut self, player_1: &Paddle, player_2: &Paddle) -> Option<u8> {
        if self.position.x <= 0.0 {
            return Some(1);
        }
        if self.position.x >= 1920.0 {
            return Some(2);
        }
        None
    }
}