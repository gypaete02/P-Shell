use std::io::{stdout, Write};

use crossterm::{
    event::{self, KeyCode}
};

use crate::data::History;

pub fn read_line(string: &mut String, history: &mut History) -> std::io::Result<()> {

    crossterm::terminal::enable_raw_mode().unwrap();

    loop {
        read_char(string, history)?;
        if string.chars().last()
            .map_or(false, |c| c == '\n') 
        {
            crossterm::terminal::disable_raw_mode().unwrap();
            return Ok(());
        }
    }
}

fn read_char(string: &mut String, history: &mut History) -> std::io::Result<()> {
    loop {
        if let event::Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char(c) => {
                    print!("{c}");
                    flush()?;
                    string.push(c);

                    return Ok(());
                }
                KeyCode::Enter => {
                    print!("\r\n");
                    flush()?;
                    string.push_str("\r\n");
                    history.add(string.clone());

                    return Ok(());
                }
                KeyCode::Backspace => {
                    backspace();
                    flush()?;
                    string.pop();

                    return Ok(());
                },

                KeyCode::Up => {
                    history.set_temp(string.clone());
                    *string = history.step_up().to_string();
                }

                KeyCode::Down => {
                    *string = history.step_down().to_string();
                }

                _ => ()
            }
        }
    }
}

fn backspace() {
    print!("\x08\x20\x08");
}

fn flush() -> std::io::Result<()> {
    stdout().flush()
}