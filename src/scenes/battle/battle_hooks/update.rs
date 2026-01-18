use std::time::Duration;

use crate::{
    engine::{Game, Scene},
    scenes::{
        BattleScene,
        battle::battle_helpers::{fight_on_scene, safe_on_scene},
    },
};

pub fn update(scene: &mut BattleScene, game: &mut Game, delta_time: Duration) {
    let Some(ref mut player) = game.player else {
        game.stop();
        return;
    };

    if !fight_on_scene(&mut game.input, player) {
        game.stop();
        return;
    }

    if !safe_on_scene(&mut game.input, player) {
        game.stop();
        return;
    }
}
