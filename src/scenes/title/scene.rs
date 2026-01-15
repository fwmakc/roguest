use std::time::Duration;

use crate::{
    engine::{Game, GameScene, Scene},
    scenes::title::hooks,
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

impl GameScene for TitleScene {
    fn base(&self) -> &Scene {
        &self.base
    }

    fn base_mut(&mut self) -> &mut Scene {
        &mut self.base
    }

    fn create(&mut self, game: &mut Game) {
        hooks::create(&mut self.base, game);
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {}

    fn rendering(&mut self, game: &mut Game) {}
}
