use colored::Colorize;

pub fn scroll(message: &str) {
    let line = "-".repeat(message.chars().count());

    print!(
        "{}\n{}\n{}\n",
        format!("|-{}-|", line).yellow(),
        format!("| {} |", message).yellow(),
        format!("|-{}-|", line).yellow(),
    );
}
