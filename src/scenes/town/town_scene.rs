use colored::*;
use std::time::Duration;

use crate::{
    engine::{Game, GameScene, Scene},
    scenes::town::town_update,
};

pub struct TownScene {
    base: Scene,
}

impl TownScene {
    pub fn new() -> Self {
        Self {
            base: Scene::new("town", true),
        }
    }
}

#[allow(unused)]
impl GameScene for TownScene {
    fn base(&self) -> &Scene {
        &self.base
    }

    fn base_mut(&mut self) -> &mut Scene {
        &mut self.base
    }

    fn create(&mut self, game: &mut Game) {
        println!("{}", "Вы находитесь в городе!".yellow());
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {
        town_update(&mut self.base, game, delta_time);
    }

    fn rendering(&mut self, game: &mut Game) {}
}
