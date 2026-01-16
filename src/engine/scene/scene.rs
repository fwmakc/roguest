use std::time::Duration;

use crate::engine::Game;

pub trait Scene {
    fn name(&self) -> &str;

    fn is_active(&self) -> bool;

    fn activate(&mut self);

    fn deactivate(&mut self);

    fn mounted(&mut self, game: &mut Game);

    fn update(&mut self, game: &mut Game, delta_time: Duration);

    fn draw(&mut self, game: &mut Game, delta_time: Duration);
}
