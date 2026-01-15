use std::time::Duration;

use crate::engine::{Game, Scene};

pub trait GameScene {
    fn base(&self) -> &Scene;

    fn base_mut(&mut self) -> &mut Scene;

    fn mounted(&mut self, game: &mut Game);

    fn update(&mut self, game: &mut Game, delta_time: Duration);

    fn rendering(&mut self, game: &mut Game);
}
