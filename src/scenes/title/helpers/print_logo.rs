use colored::*;

pub fn print_logo() {
    print!(
        "\n\n{}\n{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n\n",
        "__________                                      __    ".yellow(),
        "\\______   \\ ____   ____  __ __   ____   _______/  |_  ".yellow(),
        " |       _//  _ \\ / ___\\|  |  \\_/ __ \\ /  ___/\\   __\\ ".yellow(),
        " |    |   (  <_> ) /_/  >  |  /\\  ___/ \\___ \\  |  |   ".yellow(),
        " |____|_  /\\____/\\___  /|____/  \\___  >____  > |__|   ".yellow(),
        "        \\/      /_____/             \\/     \\/         ".yellow(),
        "|---------------------------------------------------|".yellow(),
        "|   T H E   R O G U E L I K E   R U S T   G A M E   |".yellow(),
        "|---------------------------------------------------|".yellow(),
    );

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("{} version {}", name, version);
    println!("\x07");
}
