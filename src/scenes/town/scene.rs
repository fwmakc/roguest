use std::time::Duration;

use crate::{
    engine::{Game, Scene, SceneActive, SceneName},
    scenes::town::hooks,
};

pub struct TownScene {
    active: bool,
    name: String,
}

impl TownScene {
    pub fn new() -> Self {
        Self {
            active: false,
            name: "TownScene".to_string(),
        }
    }
}

impl Scene for TownScene {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn activate(&mut self) {
        self.active = true;
        hooks::activated(self);
    }

    fn deactivate(&mut self) {
        self.active = false;
        hooks::deactivated(self);
    }

    fn mounted(&mut self, game: &mut Game) {
        hooks::mounted(self, game);
    }

    fn unmounted(&mut self, game: &mut Game) {
        hooks::unmounted(self, game);
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {
        hooks::update(self, game, delta_time);
    }

    fn draw(&mut self, game: &mut Game, delta_time: Duration) {
        hooks::draw(self, game, delta_time);
    }
}
