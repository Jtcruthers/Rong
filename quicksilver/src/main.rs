extern crate quicksilver;
extern crate rand;

use rand::Rng;

use quicksilver::{
    Result,
    geom::{Rectangle, Transform, Vector},
    graphics::{Background, Color},
    input::Key,
    lifecycle::{Settings, State, Window, run},
};

struct Screen {
    position: Vector,
    velocity: Vector
}

impl Screen {
    fn set_random_position(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range::<f32>(0.0, 1600.0);
        let y = rng.gen_range::<f32>(0.0, 1200.0);
        self.position.x = x;
        self.position.y = y;
    }

    fn slide(&mut self) {
        if self.position.x > 1600.0 || self.position.x < 0.0 {
            self.velocity.x *= -1.0;
        }
        if self.position.y > 1200.0 || self.position.y < 0.0 {
            self.velocity.y *= -1.0;
        }
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            position: Vector::new(50, 50),
            velocity: Vector::new(0, 0)
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
        window.clear(Color::WHITE)?;

        println!("position {}\nvelocity {}", self.position, self.velocity);
        window.draw_ex(
            &Rectangle::new(self.position, (100, 100)),
            Background::Col(Color::BLUE),
            Transform::rotate(45),
            0
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