use std::io::{self};

use crate::interface::prints;

pub fn input(label: &str) -> String {
    loop {
        let mut input = String::new();
        println!("{}", label);

        io::stdin().read_line(&mut input).expect("Ошибка чтения");

        let trimmed = input.trim();

        if !trimmed.is_empty() {
            return trimmed.to_string();
        }

        prints::error("Ошибка: ввод не может быть пустым. Пожалуйста, введите текст.");
    }
}
