use colored::Colorize;

use crate::{
    engine::Game,
    interface::{inputs, prints},
    objects::entities::player,
};

pub fn create_player(game: &mut Game) {
    let player_name = inputs::input("Введите свое имя:");

    println!("");
    println!("Привет, {}!", player_name.blue());

    let new_player = player(player_name);
    game.set_player(new_player);

    prints::info("Добро пожаловать в игру");
    prints::wait_any_key();
}
