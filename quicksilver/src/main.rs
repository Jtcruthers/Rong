extern crate quicksilver;
extern crate rand;

mod input_handler;
mod paddle;

use rand::Rng;

use quicksilver::{
    Result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background, Color, Image},
    input::Key,
    lifecycle::{Asset, Settings, State, Window, run},
};


use paddle::Paddle;
use input_handler::InputHandler;

struct Screen {
    position: Vector,
    velocity: Vector,
    background: Asset<Image>,
    player_1: Paddle,
    player_2: Paddle,
    input_handler: InputHandler
}

impl Screen {
    fn set_random_position(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range::<f32>(0.0, 1600.0);
        let y = rng.gen_range::<f32>(0.0, 1200.0);
        self.position.x = x;
        self.position.y = y;
    }

}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            position: Vector::new(50, 50),
            velocity: Vector::new(0, 0),
            background: Asset::new(Image::load("image.png")),
            player_1: Paddle { position: Vector::new(50, 50), ..Default::default() },
            player_2: Paddle { position: Vector::new(1500, 50), ..Default::default() },
            input_handler: InputHandler::new()
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Right].is_down() {
            self.velocity.x += 0.1;
        }
        if window.keyboard()[Key::Left].is_down() {
            self.velocity.x -= 0.1;
        }
        if window.keyboard()[Key::Up].is_down() {
            self.velocity.y -= 0.1;
        }
        if window.keyboard()[Key::Down].is_down() {
            self.velocity.y += 0.1;
        }
        if window.keyboard()[Key::Space].is_down() {
            self.velocity.x = 0.0;
            self.velocity.y = 0.0;
        }
        self.input_handler.handle_input(window.keyboard(), &mut self.player_1, &mut self.player_2);
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        println!("position {}\nvelocity {}", self.position, self.velocity);
        let position = self.position.clone(); //Had to clone it because it complained about borrowing. Look into that
        self.background.execute(|image| {
            window.draw_ex(
                &image.area().with_center(position),
                Background::Img(&image),
                Transform::scale((0.05, 0.05)),
                0
            );
            Ok(())
        });
        window.draw(
            &Rectangle::new(self.player_1.position, self.player_1.width),
            Background::Col(self.player_1.background)
        );
        window.draw(
            &Rectangle::new(self.player_2.position, self.player_2.width),
            Background::Col(self.player_2.background)
        );
        Ok(())
    }
}

fn main() {
    run::<Screen>("Rectangluar", Vector::new(1600, 1200), Settings {
        icon_path: Some("image.png"),
        ..Settings::default()
    });
}