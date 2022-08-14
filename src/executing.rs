use crate::parsing::Token;

use std::{
    process::{self, Command, Stdio},
    fs::{File, OpenOptions},
    env
};

pub fn execute(tokens: Vec<Token>) {

    let mut previous_command = None;

    let mut iter = tokens.iter().peekable();
    //let mut last_exit_code = None;

    loop {
        let command = match iter.next() {
            Some(c) => c,
            None => break
        };
        let operation = iter.next();

        let stdin;
        let stdout;

        if let Some(op) = operation {

            match op {
                Token::Pipe => stdout = Stdio::piped(),

                Token::Redirect => {
                    let path = match iter.next() {
                        Some(v) => v,
                        None => {
                            eprintln!("Error: path not specified");
                            return;
                        }
                    };

                    if let Token::Command(path, _) = path {
                        let file = File::create(path).expect("Could not find specified file");

                        stdout = Stdio::from(file);
                    } else {
                        eprintln!("Error: after a redirect (>), a path must be specified");
                        return;
                    }
                }

                Token::Append => {
                    let path = match iter.next() {
                        Some(v) => v,
                        None => {
                            eprintln!("Error: path not specified");
                            return;
                        }
                    };

                    if let Token::Command(path, _) = path {
                        let file = OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(path)
                            .expect("Could not find specified file");
                        
                        stdout = Stdio::from(file);
                    } else {
                        eprintln!("Error: after a redirect (>), a path must be specified");
                        return;
                    }
                }

                _ => stdout = Stdio::inherit()
            }

        } else {
            stdout = Stdio::inherit();
        }

        stdin = previous_command.map_or(Stdio::inherit(), |x: process::Child| Stdio::from(x.stdout.unwrap()));

        if let Token::Command(command, args) = command {

            if !execute_if_builtin(&command, args) {
                let output = Command::new(command)
                    .args(args)
                    .stdout(stdout)
                    .stdin(stdin)
                    .stderr(Stdio::inherit())
                    .spawn();
    
                match output {
                    Ok(output) => previous_command = Some(output),
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{e}");
                    }
                }
            } else {
                previous_command = None;
            }


        } else {
            eprintln!("An unexpected error occurred due to a parsing error.");
            return;
        }
    }

    if let Some(mut final_command) = previous_command {
        final_command.wait().unwrap();
    }

}


fn execute_if_builtin(command: &String, args: &Vec<String>) -> bool {

    match command.as_str() {

        "cd" => {
            env::set_current_dir(
                args.get(0).unwrap_or(&".".to_string())
            ).unwrap();
        }
        "exit" => {
            crossterm::terminal::disable_raw_mode().unwrap();
            process::exit(0);
        }

        _ => return false
    }

    true
}