use std::time::Duration;

use crate::{
    engine::{Game, Scene, SceneActive, SceneName},
    scenes::town::town_hooks,
};

pub struct TownScene {
    active: SceneActive,
    name: SceneName,
}

impl TownScene {
    pub fn new() -> Self {
        Self {
            active: SceneActive::new(false),
            name: SceneName::new("TownScene"),
        }
    }
}

impl Scene for TownScene {
    fn name(&self) -> &str {
        self.name.get()
    }

    fn is_active(&self) -> bool {
        self.active.is_active()
    }

    fn activate(&mut self) {
        self.active.activate();
        town_hooks::activated(self);
    }

    fn deactivate(&mut self) {
        self.active.deactivate();
        town_hooks::deactivated(self);
    }

    fn mounted(&mut self, game: &mut Game) {
        town_hooks::mounted(self, game);
    }

    fn unmounted(&mut self, game: &mut Game) {
        town_hooks::unmounted(self, game);
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {
        town_hooks::update(self, game, delta_time);
    }

    fn draw(&mut self, game: &mut Game, delta_time: Duration) {
        town_hooks::draw(self, game, delta_time);
    }
}
