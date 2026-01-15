use std::time::Duration;

use crate::engine::{Game, GameScene, Scene};
use crate::scenes::battle::hooks;

pub struct BattleScene {
    base: Scene,
}

impl BattleScene {
    pub fn new() -> Self {
        Self {
            base: Scene::new("battle", false),
        }
    }
}

impl GameScene for BattleScene {
    fn base(&self) -> &Scene {
        &self.base
    }

    fn base_mut(&mut self) -> &mut Scene {
        &mut self.base
    }

    fn create(&mut self, game: &mut Game) {}

    fn update(&mut self, game: &mut Game, delta_time: Duration) {
        hooks::update(&mut self.base, game, delta_time);
    }

    fn rendering(&mut self, game: &mut Game) {}
}
