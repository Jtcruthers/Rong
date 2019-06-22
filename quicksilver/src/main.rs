extern crate quicksilver;
extern crate rand;

use rand::Rng;

use quicksilver::{
    Result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background, Color, Image},
    input::Key,
    lifecycle::{Asset, Settings, State, Window, run},
};

mod paddle;
use paddle::Paddle;

struct Screen {
    position: Vector,
    velocity: Vector,
    background: Asset<Image>,
    paddles: [Paddle; 2]
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
            paddles: [
                Paddle { position: Vector::new(50, 50), ..Default::default() },
                Paddle { position: Vector::new(1500, 50), ..Default::default() }
            ]
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
                Transform::scale((0.1, 0.1)),
                0
            );
            Ok(())
        });
        for paddle in self.paddles.iter() {
            window.draw(
                &Rectangle::new(paddle.position, paddle.width),
                Background::Col(paddle.background)
            );
        }
        Ok(())
    }
}

fn main() {
    run::<Screen>("Rectangluar", Vector::new(1600, 1200), Settings {
        icon_path: Some("image.png"),
        ..Settings::default()
    });
}