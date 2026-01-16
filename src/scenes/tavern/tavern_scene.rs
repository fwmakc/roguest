use std::time::Duration;

use crate::{
    engine::{Game, Scene, SceneActive, SceneName},
    scenes::tavern::tavern_hooks,
};

pub struct TavernScene {
    active: SceneActive,
    name: SceneName,
}

impl TavernScene {
    pub fn new() -> Self {
        Self {
            active: SceneActive::new(false),
            name: SceneName::new("TavernScene"),
        }
    }
}

impl Scene for TavernScene {
    fn name(&self) -> &str {
        self.name.get()
    }

    fn is_active(&self) -> bool {
        self.active.is_active()
    }

    fn activate(&mut self) {
        self.active.activate();
        tavern_hooks::activated(self);
    }

    fn deactivate(&mut self) {
        self.active.deactivate();
        tavern_hooks::deactivated(self);
    }

    fn mounted(&mut self, game: &mut Game) {
        tavern_hooks::mounted(self, game);
    }

    fn unmounted(&mut self, game: &mut Game) {
        tavern_hooks::unmounted(self, game);
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {
        tavern_hooks::update(self, game, delta_time);
    }

    fn draw(&mut self, game: &mut Game, delta_time: Duration) {
        tavern_hooks::draw(self, game, delta_time);
    }
}
