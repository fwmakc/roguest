use colored::Colorize;

pub fn info(message: &str) {
    println!("{}", message.to_string().yellow());
}
