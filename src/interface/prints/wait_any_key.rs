use std::io;

use colored::Colorize;

use crate::interface::inputs;

pub fn wait_any_key() {
    let mut input = String::new();
    println!("{}", "Нажмите любую клавишу, чтобы продолжить".blue());

    io::stdin().read_line(&mut input).expect("Ошибка чтения");
}
