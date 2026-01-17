use colored::Colorize;
use console::Key;

use crate::{
    engine::Game,
    interface::inputs::select,
    scenes::town::town_helpers::{selected_forest, selected_look_around, selected_tavern},
};

pub fn select_in_scene(game: &mut Game) {
    match select(vec![
        "Осмотреться",
        "Пойти в таверну",
        "Отправиться в лес",
        "Посмотреть инвентарь",
        "Выйти",
    ]) {
        0 => selected_look_around(),
        1 => selected_tavern(game),
        2 => selected_forest(game),
        4 => game.stop(),
        _ => println!("Вы стоите на месте."),
    }
}
