use crate::{engine::Game, interface::prints, scenes::TownScene};

pub fn selected_forest(scene: &mut TownScene, game: &mut Game) {
    if let Some(ref mut player) = game.player {
        player.wstate.set("left");
    };

    prints::message("Лес встречает вас тишиной...");
}
