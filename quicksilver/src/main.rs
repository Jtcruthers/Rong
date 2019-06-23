extern crate quicksilver;

mod ball;
mod input_handler;
mod paddle;

use quicksilver::{
    Result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background, Color, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};


use ball::Ball;
use paddle::Paddle;
use input_handler::InputHandler;

struct Screen {
    background: Asset<Image>,
    ball: Ball,
    player_1: Paddle,
    player_2: Paddle,
    input_handler: InputHandler
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            background: Asset::new(Image::load("background.png")),
            ball: Ball::new(),
            player_1: Paddle {
                position: Vector::new(50, 50),
                background: Color::from_rgba(106, 64, 220, 100.0),
                ..Default::default()
            },
            player_2: Paddle {
                position: Vector::new(1820, 50),
                background: Color::from_rgba(234, 80, 183, 100.0),
                ..Default::default()
            },
            input_handler: InputHandler::new()
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.input_handler.handle_input(window.keyboard(), &mut self.player_1, &mut self.player_2);
        self.ball.update(&self.player_1, &self.player_2);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.background.execute(|image| {
            window.draw_ex(
                &image.area().with_center((960, 540)),
                Background::Img(&image),
                Transform::scale((1.0, 1.0)),
                0
            );
            Ok(())
        }).expect("Could not load background.");
        let position = self.ball.get_position().clone(); //Had to clone it because it complained about borrowing. Look into that
        self.ball.get_background().execute(|image| {
            window.draw_ex(
                &image.area().with_center(position),
                Background::Img(&image),
                Transform::scale((0.05, 0.05)),
                1
            );
            Ok(())
        }).expect("Could not load ball background.");
        window.draw_ex(
            &Rectangle::new(self.player_1.position, self.player_1.width),
            Background::Col(self.player_1.background),
            Transform::scale((1.0, 1.0)),
            1
        );
        window.draw_ex(
            &Rectangle::new(self.player_2.position, self.player_2.width),
            Background::Col(self.player_2.background),
            Transform::scale((1.0, 1.0)),
            1
        );
        Ok(())
    }
}

fn main() {
    run::<Screen>("Rectangluar", Vector::new(1920, 1080), Settings {
        icon_path: Some("image.png"),
        vsync: false,
        ..Settings::default()
    });
}