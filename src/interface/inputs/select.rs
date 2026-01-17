use std::io::{self};

use dialoguer::{Select, theme::ColorfulTheme};

pub fn select(items: Vec<&str>) -> usize {
    println!("");

    Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .unwrap()
}
