use std::time::Duration;

use crate::{
    engine::{Game, Scene, SceneActive, SceneName},
    scenes::battle::hooks,
};

pub struct BattleScene {
    active: bool,
    name: String,
}

impl BattleScene {
    pub fn new() -> Self {
        Self {
            active: false,
            name: "BattleScene".to_string(),
        }
    }
}

impl Scene for BattleScene {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn activate(&mut self) {
        self.active = true;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn mounted(&mut self, game: &mut Game) {}

    fn update(&mut self, game: &mut Game, delta_time: Duration) {
        hooks::update(self, game, delta_time);
    }

    fn draw(&mut self, game: &mut Game, delta_time: Duration) {}
}
