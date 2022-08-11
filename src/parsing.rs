use std::str::SplitWhitespace;

pub fn parse(mut line: &str) -> (&str, SplitWhitespace) {

    line = line.trim();
    let mut parts = line.split_whitespace();

    if parts.any(|x| x.eq("|")) {
        // todo
    }

    let command = parts.next().unwrap_or("");

    let args = parts;

    (command, args)
}