use colored::Colorize;
use console::Key;
use dialoguer::{Select, theme::ColorfulTheme};

use crate::engine::Game;

pub fn select_in_scene(game: &mut Game) {
    let items = vec![
        "Пойти в таверну",
        "Отправиться в лес",
        "Посмотреть инвентарь",
        "Выйти",
    ];

    // Создаем меню выбора
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Что вы хотите сделать?")
        .items(&items)
        .default(0) // Индекс выбора по умолчанию
        .interact() // Ожидание ввода
        .unwrap(); // Возвращает индекс выбранного элемента (usize)

    // Пример обработки выбора
    match selection {
        0 => println!("Вы заходите в шумную таверну..."),
        1 => println!("Лес встречает вас тишиной..."),
        3 => game.stop(),
        _ => println!("Вы стоите на месте."),
    }
}
