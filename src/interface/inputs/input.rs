use std::io::{self};

pub fn input(label: &str) -> String {
    let mut input = String::new();
    println!("{}", label);

    io::stdin().read_line(&mut input).expect("Ошибка чтения");

    input.trim().to_string()
}
