use std::io::stdout;
use std::env;

use crossterm::cursor::MoveToColumn;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{ClearType, Clear};
use crossterm::event::{self, KeyCode};

use crate::completation::Completer;
use crate::data::History;


pub fn read_line(string: &mut String, history: &mut History) -> std::io::Result<()> {

    crossterm::terminal::enable_raw_mode()?;

    let prompt = get_prompt();
    let mut position = 0;
    let mut completer = Completer::new(string.clone());

    refresh(&prompt, string, &mut position)?;

    loop {

        if let event::Event::Key(event) = event::read()? {
            match event.code {

                KeyCode::Char(c) => {
                    string.insert(position, c);
                    position += 1;

                    refresh(&prompt, string, &mut position)?;
                    completer.update(word_at(position, string));
                }

                KeyCode::Enter => {
                    string.push_str("\n");
                    print!("\r\n");
                    history.add(string);

                    break;
                }

                KeyCode::Delete => {

                    if position >= string.len() {
                        continue;
                    }

                    string.remove(position);

                    refresh(&prompt, string, &mut position)?;
                    completer.update(word_at(position, string));
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
                    position = position.saturating_sub(1);

                    refresh(&prompt, string, &mut position)?;
                    completer.update(word_at(position, string));
                }

                KeyCode::Left => {
                    position = position.saturating_sub(1);
                    refresh(&prompt, string, &mut position)?;
                }

                KeyCode::Right => {
                    if position < string.len() {
                        position += 1;
                        refresh(&prompt, string, &mut position)?;
                    }
                }

                KeyCode::Up => {
                    history.step_up(string);
                    position = string.len();
                    refresh(&prompt, string, &mut position)?;
                }

                KeyCode::Down => {
                    history.step_down(string);
                    position = string.len();
                    refresh(&prompt, string, &mut position)?;
                }

                KeyCode::Tab => {
                    string.push_str(completer.complete());
                    // TODO Move cursor to end of completed word
                    refresh(&prompt, string, &mut position)?;
                }

                _ => ()
            }
        }

    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
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

fn word_at(mut position: usize, string: &String) -> String {

    // FIXME: Does not work correctly: when used returns last word...

    if position == 0 {
        return String::new();
    }

    position -= 1;
    let mut beg = position;

    loop {
        match string.chars().nth(beg) {
            Some(c) => {
                if c == ' ' {
                    beg += 1;
                    break;
                }
                match beg.checked_sub(1) {
                    Some(b) => beg = b,
                    None => break
                }
            }
            None => break
        }
    }

    string[beg..=position].to_string()
}

#[test]
fn test_word_at() {
    let s1 = "cat test.tx".to_string();
    let s2 = "cat test.tx > file".to_string();
    let s3 = "test1 test2".to_string();
    let s4 = "test1 test2".to_string();

    assert_eq!(word_at(11, &s1).as_str(), "test.tx");
    assert_eq!(word_at(10, &s2).as_str(), "test.t");
    assert_eq!(word_at(0, &s3).as_str(), "");
    assert_eq!(word_at(3, &s4).as_str(), "tes");

}
