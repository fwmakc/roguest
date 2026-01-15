use colored::*;
use std::time::Duration;

use crate::{
    engine::{Game, GameScene, Scene},
    scenes::tavern::tavern_update,
};

pub struct TavernScene {
    base: Scene,
}

impl TavernScene {
    pub fn new() -> Self {
        Self {
            base: Scene::new("tavern", false),
        }
    }
}

#[allow(unused)]
impl GameScene for TavernScene {
    fn base(&self) -> &Scene {
        &self.base
    }

    fn base_mut(&mut self) -> &mut Scene {
        &mut self.base
    }

    fn create(&mut self, game: &mut Game) {
        println!("{}", "Вы находитесь в таверне!".yellow());
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {
        tavern_update(&mut self.base, game, delta_time);
    }

    fn rendering(&mut self, game: &mut Game) {}
}
