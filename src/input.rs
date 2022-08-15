use std::io::{stdout, Write};
use std::env;

use crossterm::cursor::MoveToColumn;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{ClearType, Clear};
use crossterm::{
    cursor,
    event::{self, KeyCode}
};

use crate::completation;
use crate::data::History;


pub fn read_line(string: &mut String, history: &mut History) -> std::io::Result<()> {

    crossterm::terminal::enable_raw_mode()?;

    let prompt = get_prompt();
    let mut position = 0;

    refresh(&prompt, string, &mut position)?;

    read(string, &mut position, &prompt, history)?;

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn read(string: &mut String, position: &mut usize, prompt: &String, history: &mut History) -> std::io::Result<()> {

    loop {

        if let event::Event::Key(event) = event::read()? {
            match event.code {

                KeyCode::Char(c) => {
                    string.insert(*position, c);
                    *position += 1;

                    refresh(prompt, string, position)?;
                }

                KeyCode::Enter => {
                    string.push_str("\n");
                    print!("\r\n");
                    history.add(string);

                    return Ok(());
                }

                KeyCode::Delete => {

                    if *position >= string.len() {
                        continue;
                    }

                    string.remove(*position);

                    refresh(prompt, string, position)?;
                }

                KeyCode::Backspace => {

                    if string.len() == 0 {
                        continue;
                    }

                    let p = match position.checked_sub(1) {
                        Some(v) => v,
                        None => continue
                    };

                    string.remove(p);
                    *position = position.saturating_sub(1);

                    refresh(prompt, string, position)?;
                }

                KeyCode::Left => {
                    *position = position.saturating_sub(1);
                    refresh(prompt, string, position)?;
                }

                KeyCode::Right => {
                    if *position < string.len() {
                        *position += 1;
                        refresh(prompt, string, position)?;
                    }
                }

                KeyCode::Up => {
                    history.step_up(string);
                    *position = string.len();
                    refresh(prompt, string, position)?;
                }

                KeyCode::Down => {
                    history.step_down(string);
                    *position = string.len();
                    refresh(prompt, string, position)?;
                }

                KeyCode::Tab => {
                    // TODO: Just use word under cursor, not whole string.
                    completation::complete(string);
                }

                _ => ()
            }
        }

    }

}

fn get_prompt() -> String {
    let path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let hostname = whoami::hostname();
    let username = whoami::username();

    format!("\r{username}@{hostname}: {path} > ")
}

fn refresh(prompt: &String, string: &String, position: &mut usize) -> std::io::Result<()> {
    execute!(stdout(),
        Clear(ClearType::CurrentLine),
        Print(&prompt),
        Print(&string),
        MoveToColumn((prompt.len() + *position).saturating_sub(1) as u16)
    )
}