

pub fn parse(line: &str) -> Vec<Token> {

    let parts = split(line).into_iter();
    let mut current_command = vec![];
    let mut tokens = vec![];

    for part in parts.as_ref() {

        let mut token = None;

        match part.as_str() {

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

/// Splits the given strings at whitespaces, handling quotes, single quotes and backslash escaping.
fn split(line: &str) -> Vec<String> {

    if line.len() == 0 {
        return vec![];
    }

    let mut escaped = false;
    let mut quoted = false;

    let mut current = String::new();
    let mut vec = vec![];

    for ch in line.chars() {

        if escaped {

            current.push(ch);
            escaped = false;
            continue;

        } else if quoted {

            if ch == '"' {
                quoted = false;
            } else if ch == '\\' {
                escaped = true;
            } else {
                current.push(ch);
                escaped = false;
            }

            continue;

        }

        match ch {
            '\\' => escaped = true,
            '"'  => quoted = true,
            ' '  => {
                vec.push(current.clone());
                current.clear();
            }
            '\n' => continue,
            c => current.push(c)
        }
        
    }

    vec.push(current);

    vec
}

/// Returns a command-token from the given parts.
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_command_from() {
        let v1 = vec!["echo", "Hello,", "World"];
        let v2 = vec!["ls"];
        let v3 = vec![""];

        if let Token::Command(command, args) = command_from(&v1) {
            assert_eq!(command, "echo");
            assert_eq!(args, vec!["Hello,", "World"]);
        }

        if let Token::Command(command, args) = command_from(&v2) {
            assert_eq!(command, "ls");
            assert_eq!(args, Vec::<String>::new());
        }

        if let Token::Command(command, args) = command_from(&v3) {
            assert_eq!(command, String::new());
            assert_eq!(args, Vec::<String>::new());
        }
    }

    #[test]
    fn test_split() {
        let s1 = "echo Hello, World";
        let s2 = r#"echo "Hello, World""#;
        let s3 = r#"echo \"Hello,\ World\", Hello,\ World"#;

        assert_eq!(split(s1), vec!["echo", "Hello,", "World"]);
        assert_eq!(split(s2), vec!["echo", "Hello, World"]);
        assert_eq!(split(s3), vec!["echo", "\"Hello, World\",", "Hello, World"]);
    }

    #[test]
    fn test_parse() {
        
       let s1 = "echo Hello, World";
       let s2 = "echo Hello > file.txt";
       let s3 = "echo Hello | cat";
       let s4 = "echo Hello | cat | cat";

       assert_eq!(parse(s1), vec![
                Token::Command("echo".to_string(), 
                    vec!["Hello,".to_string(), "World".to_string()]
                ) 
       ]);

        assert_eq!(parse(s2), vec![
            Token::Command("echo".to_string(),
                vec!["Hello".to_string()]),
            Token::Redirect,
            Token::Command("file.txt".to_string(), vec![])
        ]);
        
        assert_eq!(parse(s3), vec![
            Token::Command("echo".to_string(),
                vec!["Hello".to_string()]),
            Token::Pipe,
            Token::Command("cat".to_string(), vec![])
        ]);

         assert_eq!(parse(s4), vec![
            Token::Command("echo".to_string(),
                vec!["Hello".to_string()]),
            Token::Pipe,
            Token::Command("cat".to_string(), vec![]),
            Token::Pipe,
            Token::Command("cat".to_string(), vec![])
        ]);       
    }


}
