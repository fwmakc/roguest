use std::time::Duration;

use crate::{
    engine::{Game, Scene, SceneActive, SceneName},
    scenes::title::hooks,
};

pub struct TitleScene {
    active: SceneActive,
    name: SceneName,
}

impl TitleScene {
    pub fn new() -> Self {
        Self {
            active: SceneActive::new(true),
            name: SceneName::new("TitleScene"),
        }
    }
}

impl Scene for TitleScene {
    fn name(&self) -> &str {
        self.name.get()
    }

    fn is_active(&self) -> bool {
        self.active.is_active()
    }

    fn activate(&mut self) {
        self.active.activate();
        hooks::activated(self);
    }

    fn deactivate(&mut self) {
        self.active.deactivate();
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
