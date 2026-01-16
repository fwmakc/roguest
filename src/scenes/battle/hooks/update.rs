use std::time::Duration;

use crate::{
    engine::{Game, Scene},
    scenes::{
        BattleScene,
        battle::helpers::{fight_on_scene, safe_on_scene},
    },
};

pub fn update(scene: &mut BattleScene, game: &mut Game, delta_time: Duration) {
    let Some(ref mut player) = game.player else {
        scene.deactivate();
        game.stop();
        return;
    };

    // let dt = delta_time.as_secs_f32();
    // println!("Delta time: {:.4}s (FPS: {:.1})", dt, 1.0 / dt);

    if !fight_on_scene(&mut game.input, player) {
        game.stop();
        return;
    }

    if !safe_on_scene(&mut game.input, player) {
        game.stop();
        return;
    }
}
