pub fn out() {
    let output_string = str_data();
    println!("{output_string}");
}

pub fn str_data<'a>() -> &'a str {
    "Hello, world!"
}

pub fn boom() {
    eprintln!("3... 2... 1...");
    panic!("boom!");
}

pub fn result(a: u8) -> Result<u8, String> {
    if a < 100 {
        Ok(a)
    } else {
        Err("invalid!".into())
    }
}
