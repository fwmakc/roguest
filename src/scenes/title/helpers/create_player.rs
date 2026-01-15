use colored::Colorize;

use crate::{
    eco::objects::player,
    engine::{Game, inputs},
};

pub fn create_player(game: &mut Game) {
    let player_name = inputs::input("Введите свое имя:");

    println!("Привет, {}!", player_name.blue());
    println!("{}", "Добро пожаловать в игру".yellow());
    println!("{} выйти", "[Esc]".blue());

    let new_player = player(player_name);

    game.set_player(new_player);
}
