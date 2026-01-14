use colored::*;
use std::time::Duration;

use crate::{
    eco::objects::player,
    engine::{Game, GameScene, Scene, inputs},
};

pub struct TitleScene {
    base: Scene,
}

impl TitleScene {
    pub fn new() -> Self {
        Self {
            base: Scene::new("title", true),
        }
    }
}

#[allow(unused)]
impl GameScene for TitleScene {
    fn base(&self) -> &Scene {
        &self.base
    }

    fn base_mut(&mut self) -> &mut Scene {
        &mut self.base
    }

    fn create(&mut self, game: &mut Game) {
        print!(
            "\n\n{}\n{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n\n",
            "__________                                      __    ".yellow(),
            "\\______   \\ ____   ____  __ __   ____   _______/  |_  ".yellow(),
            " |       _//  _ \\ / ___\\|  |  \\_/ __ \\ /  ___/\\   __\\ ".yellow(),
            " |    |   (  <_> ) /_/  >  |  /\\  ___/ \\___ \\  |  |   ".yellow(),
            " |____|_  /\\____/\\___  /|____/  \\___  >____  > |__|   ".yellow(),
            "        \\/      /_____/             \\/     \\/         ".yellow(),
            "|---------------------------------------------------|".yellow(),
            "|   T H E   R O G U E L I K E   R U S T   G A M E   |".yellow(),
            "|---------------------------------------------------|".yellow(),
        );

        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        print!("{} version {}\n\x07\n", name, version);

        let player_name = inputs::input("Введите свое имя:");

        println!("Привет, {}!", player_name.blue());
        println!("{}", "Добро пожаловать в игру".yellow());
        println!("{} выйти", "[Esc]".blue());

        let new_player = player(player_name);

        game.set_player(new_player);
        game.set_fps(60.0);

        self.deactivate();
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {}

    fn rendering(&mut self, game: &mut Game) {}
}
