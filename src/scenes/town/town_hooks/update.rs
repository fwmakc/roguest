use std::time::Duration;

use crate::{
    engine::{Game, Scene},
    interface::prints,
    scenes::{
        TownScene,
        town::town_helpers::{select_in_town, welcome},
    },
};

pub fn update(scene: &mut TownScene, game: &mut Game, delta_time: Duration) {
    prints::fps(delta_time);

    let Some(ref mut player) = game.player else {
        game.stop();
        return;
    };

    select_in_town(scene, game);
}
