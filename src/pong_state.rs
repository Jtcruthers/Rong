extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background, Color, Font, FontStyle, Image},
    lifecycle::{Asset, State, Window},
};

use std::rc::Rc;
use std::cell::RefCell;

use crate::ball::Ball;
use crate::constants::*;
use crate::paddle::Paddle;
use crate::input_handler::InputHandler;


pub struct Scoring {
    score1: i32,
    score2: i32,
    score1_texture : Option<Image>,
    score2_texture : Option<Image>,
}

impl Scoring {
    pub fn new() -> Scoring {
        Scoring {
            score1: 0,
            score2: 0,
            score1_texture: None,
            score2_texture: None,
        }
    }
}

pub struct PongState {
    background: Asset<Image>,
    ball: Ball,
    font: Rc<RefCell<Asset<Font>>>,
    p1_font_style: FontStyle,
    p2_font_style: FontStyle,
    input_handler: InputHandler,
    player_1: Paddle,
    player_2: Paddle,
    score: Scoring
}

impl State for PongState {
    fn new() -> Result<PongState> {
        Ok(PongState {
            background: Asset::new(Image::load("background.png")),
            ball: Ball::new(),
            font: Rc::new(RefCell::new(Asset::new(Font::load("arcade.ttf")))),
            p1_font_style: FontStyle::new(64.0, Color::from_rgba(P1_R, P1_G, P1_B, RBG_ALPHA)),
            p2_font_style: FontStyle::new(64.0, Color::from_rgba(P2_R, P2_G, P2_B, RBG_ALPHA)),
            input_handler: InputHandler::new(),
            player_1: Paddle {
                position: Vector::new(50, 50),
                background: Color::from_rgba(P1_R, P1_G, P1_B, RBG_ALPHA),
                ..Default::default()
            },
            player_2: Paddle {
                position: Vector::new(1820, 50),
                background: Color::from_rgba(P2_R, P2_G, P2_B, RBG_ALPHA),
                ..Default::default()
            },
            score: Scoring::new()
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.input_handler.handle_input(window.keyboard(), &mut self.player_1, &mut self.player_2);
        self.ball.update(&self.player_1, &self.player_2);

        if let Some(player_who_scored) = self.ball.did_score() {
            if player_who_scored == 1 { self.score.score1 += 1; }
            if player_who_scored == 2 { self.score.score2 += 1; }
            self.ball.reset();
        }
        println!("SCORE {} to {}", self.score.score1, self.score.score2);
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        self.draw_background(window, 0);
        self.draw_score(window, 1);
        self.draw_players(window, 2);
        self.draw_ball(window, 2);
        
        Ok(())
    }
}

impl PongState {
    fn draw_ball(&mut self, window: &mut Window, z_index: u8) {
        let position = self.ball.get_position().clone(); //Had to clone it because it complained about borrowing. Look into that
        self.ball.get_background().execute(|image| {
            window.draw_ex(
                &image.area().with_center(position),
                Background::Img(&image),
                Transform::scale((0.05, 0.05)),
                z_index
            );
            Ok(())
        }).expect("Could not load ball background.");
    }

    fn draw_score(&mut self, window: &mut Window, z_index: u8) {
        let cloned_font = self.font.clone();
        cloned_font.borrow_mut().execute(|font| {
            let score1_texture = font.render(&format!("{:02}", self.score.score1), &self.p1_font_style).unwrap();
            self.score.score1_texture = Some(score1_texture);
            Ok(())
        }).expect("Could not render player 1 score");
        cloned_font.borrow_mut().execute(|font| {
            let score2_texture = font.render(&format!("{:02}", self.score.score2), &self.p2_font_style).unwrap();
            self.score.score2_texture = Some(score2_texture);
            Ok(())
        }).expect("Could not render player 2 score");
        if let Some(ref image) = self.score.score1_texture {
            window.draw_ex(
                &image.area().with_center((400, 100)),
                Background::Img(&image),
                Transform::scale((3.0, 3.0)),
                z_index
            );
        }
        if let Some(ref image) = self.score.score2_texture {
            window.draw_ex(
                &image.area().with_center((1450, 100)),
                Background::Img(&image),
                Transform::scale((3.0, 3.0)),
                z_index
            );
        }
    }

    fn draw_players(&mut self, window: &mut Window, z_index: u8) {
        window.draw_ex(
            &Rectangle::new(self.player_1.position, self.player_1.width),
            Background::Col(self.player_1.background),
            Transform::scale((1.0, 1.0)),
            z_index
        );
        window.draw_ex(
            &Rectangle::new(self.player_2.position, self.player_2.width),
            Background::Col(self.player_2.background),
            Transform::scale((1.0, 1.0)),
            z_index
        );
    }

    fn draw_background(&mut self, window: &mut Window, z_index: u8) {
        self.background.execute(|image| {
            window.draw_ex(
                &image.area().with_center((960, 540)),
                Background::Img(&image),
                Transform::scale((1.0, 1.0)),
                z_index
            );
            Ok(())
        }).expect("Could not load background.");
    }
}