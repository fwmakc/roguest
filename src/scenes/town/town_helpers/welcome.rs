use colored::Colorize;
use console::Key;

use crate::interface::inputs::InputHandler;

pub fn welcome() {
    let mut input = InputHandler::new(5);

    print!(
        "{}\n{}\n{}\n",
        "|------------------------------|".yellow(),
        "|   ДОБРО ПОЖАЛОВАТЬ В ГОРОД   |".yellow(),
        "|------------------------------|".yellow(),
    );
}
