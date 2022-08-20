mod parsing;
mod executing;
mod input;
mod data;
mod completation;
mod tests;

use crate::parsing::parse;

fn main() {

    let _cleanup = Cleanup;

    let mut history = data::History::init();
    
    loop {

        let mut input = String::new();
        input::read_line(&mut input, &mut history).unwrap();

        let tokens = parse(input.as_str());
        executing::execute(tokens);
    }

}

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Could not set terminal to normal mode. Raw mode is enabled.");
    }
}
