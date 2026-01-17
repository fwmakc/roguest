use crate::{engine::Game, interface::prints, scenes::TownScene};

pub fn selected_forest(scene: &mut TownScene, game: &mut Game) {
    prints::message("Лес встречает вас тишиной...");
}
