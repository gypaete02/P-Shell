

pub fn parse(line: &str) -> Vec<Token> {

    let parts = line.split_whitespace();
    let mut current_command = vec![];
    let mut tokens = vec![];

    for part in parts {

        let mut token = None;

        match part {

            "|" => {
                token = Some(Token::Pipe)
            }
            ">" => {
                token = Some(Token::Redirect)
            }
            ">>" => {
                token = Some(Token::Append)
            }
            "<" => {
                token = Some(Token::ReverseRedirect)
            }
            "||" => {
                token = Some(Token::Or)
            }
            "&&" => {
                token = Some(Token::And)
            }
            ";" => {
                token = Some(Token::Separator)
            }

            part => current_command.push(part), // TODO parse $(...) and `...`
        }

        if let Some(token) = token {
            tokens.push(command_from(&current_command));
            tokens.push(token);

            current_command.clear();
        }
    }

    if !current_command.is_empty() {
        tokens.push(command_from(&current_command));
    }

    tokens
}

fn command_from(parts: &Vec<&str>) -> Token {
    Token::Command(
        parts.get(0).map_or(String::new(), ToString::to_string), 
        parts.iter().skip(1).map(ToString::to_string).collect()
    )
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Command(String, Vec<String>),
    Pipe,
    Redirect,
    Append,
    ReverseRedirect,
    Or,
    And,
    Separator,
}

