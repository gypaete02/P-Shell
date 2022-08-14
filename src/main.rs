mod parsing;
mod executing;
mod input;
mod data;
mod completation;

use std::{io::{stdout, Write}, env};

use parsing::parse;


fn main() {

    //crossterm::terminal::enable_raw_mode().unwrap();

    let mut history = data::History::init();
    
    loop {
        print_prompt();
        stdout().flush().unwrap();

        let mut input = String::new();
        input::read_line(&mut input, &mut history).unwrap();

        let tokens = parse(input.as_str());
        executing::execute(tokens);
    }

}

fn print_prompt() {
    let path = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let hostname = whoami::hostname();
    let username = whoami::username();

    print!("\r{username}@{hostname}: {path} > ");
}