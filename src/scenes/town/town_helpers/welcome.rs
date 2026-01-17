use colored::Colorize;
use console::Key;

use crate::interface::prints;

pub fn welcome() {
    prints::scroll("Добро пожаловать в город");
}
