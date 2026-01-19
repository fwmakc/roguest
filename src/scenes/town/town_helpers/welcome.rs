use colored::Colorize;
use console::Key;

use crate::interface::prints;

pub fn welcome() {
    prints::scroll("Город «Столица»");
    prints::message("Добро пожаловать в город");
}
