extern crate quicksilver;

use quicksilver::{
    geom::Vector,
    lifecycle::{Settings, run},
};

mod ball;
mod constants;
mod input_handler;
mod paddle;
mod pong_state;


use std::rc::Rc;
use std::cell::RefCell;

use constants::*;
use pong_state::PongState;

fn main() {
    run::<PongState>("Rong", Vector::new(SCREEN_WIDTH, SCREEN_HEIGHT), Settings {
        icon_path: Some("image.png"),
        vsync: false,
        ..Settings::default()
    });
}