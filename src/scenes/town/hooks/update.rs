use colored::Colorize;
use std::time::Duration;

use crate::engine::{Game, Scene};

pub fn update(scene: &mut Scene, game: &mut Game, delta_time: Duration) {
    let Some(ref mut player) = game.player else {
        scene.deactivate();
        game.stop();
        return;
    };

    println!("{}", "Вы находитесь в городе!".yellow());

    let dt = delta_time.as_secs_f32();
    println!("Delta time: {:.4}s (FPS: {:.1})", dt, 1.0 / dt);
}
