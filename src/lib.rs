pub mod convert;
pub mod prattl;
pub mod summarize;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
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

#[tokio::main]
async fn term_main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .green()
                .on_black();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
