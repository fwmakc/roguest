use colored::Colorize;

use crate::interface::inputs;

pub fn wait_any_key() {
    print!("\n{}", "Нажмите любую клавишу, чтобы продолжить".blue());
    inputs::input("");
}
