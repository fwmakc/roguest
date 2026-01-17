use std::time::Duration;

use crate::{
    engine::{Game, Scene},
    scenes::{TavernScene, tavern::tavern_helpers::select_in_tavern},
};

pub fn update(scene: &mut TavernScene, game: &mut Game, delta_time: Duration) {
    let Some(ref mut player) = game.player else {
        game.stop();
        return;
    };

    select_in_tavern(scene, game);

    let dt = delta_time.as_secs_f32();
    println!("Delta time: {:.4}s (FPS: {:.1})", dt, 1.0 / dt);
}
