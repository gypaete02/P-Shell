use std::{io::{stdout, Write, stdin}, process::Command};

use crate::parsing::parse;


mod parsing;

fn main() {
    
    loop {

        print!("{}", get_prompt());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let (command, args) = parse(&input[..]);

        let child = Command::new(command)
            .args(args)
            .spawn();

        match child {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    eprintln!("{e}");
                }
            }
            Err(e) => eprintln!("{e}")
        }
    }

}

fn get_prompt() -> String {
    "> ".to_string()
}