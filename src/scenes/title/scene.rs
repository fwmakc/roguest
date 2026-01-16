use std::time::Duration;

use crate::{
    engine::{Game, Scene, SceneActive, SceneName},
    scenes::title::hooks,
};

pub struct TitleScene {
    active: bool,
    name: String,
}

impl TitleScene {
    pub fn new() -> Self {
        Self {
            active: true,
            name: "TitleScene".to_string(),
        }
    }
}

impl Scene for TitleScene {
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

    fn mounted(&mut self, game: &mut Game) {
        hooks::mounted(self, game);
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {}

    fn draw(&mut self, game: &mut Game, delta_time: Duration) {}
}
