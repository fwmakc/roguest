use colored::Colorize;
use console::Key;

use crate::{
    engine::Game,
    interface::{inputs::select, prints},
    scenes::{
        TavernScene,
        tavern::tavern_helpers::{selected_back_to_town, selected_look_around},
    },
};

pub fn select_in_tavern(scene: &mut TavernScene, game: &mut Game) {
    match select(vec!["Осмотреться", "Вернуться в город"]) {
        0 => selected_look_around(),
        1 => selected_back_to_town(scene, game),
        _ => prints::message("Вы стоите на месте."),
    }
}
