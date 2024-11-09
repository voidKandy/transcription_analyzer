pub mod convert;
pub mod prattl;
pub mod summarize;
use std::io::{self, Write};

pub fn get_user_confirmation() -> bool {
    println!("(Y/N)?");
    io::stdout().flush().unwrap();
    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input
            .trim()
            .to_lowercase()
            .chars()
            .next()
            .and_then(|ch| Some(ch == 'y'))
        {
            Some(valid_input) => return valid_input,
            None => println!("{input} is not valid input (Y/N)"),
        }
    }
}
