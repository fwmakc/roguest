use std::io::{self};

use dialoguer::{Select, theme::ColorfulTheme};

pub fn select(items: Vec<&str>) -> usize {
    println!("");

    let items = vec![
        "Осмотреться",
        "Пойти в таверну",
        "Отправиться в лес",
        "Посмотреть инвентарь",
        "Выйти",
    ];

    Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .unwrap()
}
