extern crate quicksilver;
extern crate rand;

use quicksilver::{
    geom::Vector,
    graphics::Image,
    lifecycle::Asset,
};

use rand::Rng;

use crate::constants::STEP_VELOCITY_INCREASE;
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
            velocity: Vector::new(-5, 0),
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

    fn set_random_y_velocity(&mut self) {
        let mut rng = rand::thread_rng();
        self.velocity.y = rng.gen_range(-10.0, 10.0);
    }

    fn increase_velocity(&mut self) {
        let mut direction_multiplier = 1.0;
        if self.velocity.x < 0.0 { // If the ball is going 0, then we need to subtract the multiplier
            direction_multiplier *= -1.0;
        }
        self.velocity.x += STEP_VELOCITY_INCREASE * direction_multiplier;
    }

    fn reverse_x_velocity(&mut self) {
        self.velocity.x *= -1.0;
    }

    fn reverse_y_velocity(&mut self) {
        self.velocity.y *= -1.0;
    }

    fn return_ball(&mut self) {
        self.reverse_x_velocity();
        self.set_random_y_velocity();
        self.increase_velocity();
    }

    pub fn update(&mut self, player_1: &Paddle, player_2: &Paddle) {
        if self.position.y <= 0.0 || self.position.y >= 1080.0 {
            self.reverse_y_velocity();
        }
        if self.position.y >= player_1.position.y && self.position.y <= player_1.position.y + player_1.width.y && self.position.x <= 90.0 {
            if self.velocity.x < 0.0 {
                self.return_ball();
            }
        }
        if self.position.y >= player_2.position.y && self.position.y <= player_2.position.y + player_2.width.y && self.position.x >= 1820.0 {
            if self.velocity.x > 0.0 {
                self.return_ball();
            } 
        }

        self.position += self.velocity
    }

    pub fn did_score(&mut self) -> Option<u8> {
        if self.position.x <= 0.0 {
            return Some(2);
        }
        if self.position.x >= 1920.0 {
            return Some(1);
        }
        None
    }
}