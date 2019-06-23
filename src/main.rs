extern crate quicksilver;

mod ball;
mod input_handler;
mod paddle;

use quicksilver::{
    Future, Result,
    combinators::result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};


use ball::Ball;
use paddle::Paddle;
use input_handler::InputHandler;

struct Screen {
    background: Asset<Image>,
    ball: Ball,
    input_handler: InputHandler,
    player_1: Paddle,
    player_2: Paddle,
    score: (u64, u64),
    score_text: Asset<Image>
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            background: Asset::new(Image::load("background.png")),
            ball: Ball::new(),
            input_handler: InputHandler::new(),
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
            score: (0, 0),
            score_text: Asset::new(Font::load("arcade.ttf")
                .and_then(|font| {
                    let style = FontStyle::new(64.0, Color::BLACK);
                    result(font.render("SAMPLE", &style))
                }))
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.input_handler.handle_input(window.keyboard(), &mut self.player_1, &mut self.player_2);
        self.ball.update(&self.player_1, &self.player_2);

        if let Some(player_who_scored) = self.ball.did_score(&self.player_1, &self.player_2) {
            if player_who_scored == 1 { self.score.0 += 1; }
            if player_who_scored == 2 { self.score.1 += 1; }
            self.ball.reset();
        }
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
        self.score_text.execute(|image| {
            window.draw(&image.area().with_center((400, 100)), Background::Img(&image));
            Ok(())
        });
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