extern crate quicksilver;

use quicksilver::{
    input::{Key, Keyboard},
};

use crate::paddle::Paddle;

trait Command {
    fn execute (&self, entity: &mut Paddle);
}

pub struct InputHandler {
    up_command: UpCommand,
    down_command: DownCommand,
    stop_command: StopCommand,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            up_command: UpCommand,
            down_command: DownCommand,
            stop_command: StopCommand
        }
    }

    pub fn handle_input(&self, keyboard: &Keyboard, player_1: &mut Paddle, player_2: &mut Paddle) {
        if keyboard[Key::W].is_down() {
            self.up_command.execute(player_1);
        }
        if keyboard[Key::S].is_down() {
            self.down_command.execute(player_1);
        }
        if keyboard[Key::Space].is_down() {
            self.stop_command.execute(player_1);
        }
        if keyboard[Key::Up].is_down() {
            self.up_command.execute(player_2);
        }
        if keyboard[Key::Down].is_down() {
            self.down_command.execute(player_2);
        }
    }
}

struct UpCommand;
struct DownCommand;
struct StopCommand;

impl Command for UpCommand {
    fn execute (&self, entity: &mut Paddle) {
        if entity.position.y > 0.0 {
            entity.position.y -= 10.0;
        }
    }
} 

impl Command for DownCommand {
    fn execute(&self, entity: &mut Paddle) {
        if entity.position.y < 880.0 {
            entity.position.y += 10.0;
        }
    }
}

impl Command for StopCommand {
    fn execute(&self, entity: &mut Paddle) {
        entity.position.y = 500.0;
    }
}