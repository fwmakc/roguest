use std::time::Duration;

use crate::{
    engine::{Game, Scene, SceneActive, SceneName},
    scenes::battle::battle_hooks,
};

pub struct BattleScene {
    active: SceneActive,
    name: SceneName,
}

impl BattleScene {
    pub fn new() -> Self {
        Self {
            active: SceneActive::new(false),
            name: SceneName::new("BattleScene"),
        }
    }
}

impl Scene for BattleScene {
    fn name(&self) -> &str {
        self.name.get()
    }

    fn is_active(&self) -> bool {
        self.active.is_active()
    }

    fn activate(&mut self, game: &mut Game) {
        self.active.activate();
        battle_hooks::activated(self, game);
    }

    fn deactivate(&mut self, game: &mut Game) {
        self.active.deactivate();
        battle_hooks::deactivated(self, game);
    }

    fn mounted(&mut self, game: &mut Game) {
        battle_hooks::mounted(self, game);
    }

    fn unmounted(&mut self, game: &mut Game) {
        battle_hooks::unmounted(self, game);
    }

    fn update(&mut self, game: &mut Game, delta_time: Duration) {
        battle_hooks::update(self, game, delta_time);
    }

    fn draw(&mut self, game: &mut Game, delta_time: Duration) {
        battle_hooks::draw(self, game, delta_time);
    }
}
