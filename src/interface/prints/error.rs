use colored::Colorize;

pub fn error(message: &str) {
    println!("{}", message.to_string().red());
}
