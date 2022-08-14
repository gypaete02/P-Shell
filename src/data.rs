use std::{collections::HashMap, borrow::Cow};

pub struct History {
    backwards_index: usize,
    history: Vec<String>,
    temp: String,
}

impl History {
    
    pub fn init() -> Self {
        Self { 
            backwards_index: 0, 
            history: vec![],
            temp: String::new()
        }
    }

    pub fn add(&mut self, line: String) {
        self.history.push(line);
    }

    pub fn step_up<'a>(&'a mut self) -> &'a str {

        if self.backwards_index < self.history.len() - 1 {
            self.backwards_index += 1;
        }

        &self.history[self.backwards_index]
    }

    pub fn step_down<'a>(&'a mut self) -> &'a str {

        if self.backwards_index == 0 {
            return &self.temp[..];
        } else {
            self.backwards_index -= 1;
        }

        &self.history[self.backwards_index]
    }

    pub fn reset_index(&mut self) {
        self.backwards_index = 0;
    }

    pub fn set_temp(&mut self, line: String) {
        self.temp = line;
    }

    pub fn get_temp(&self) -> String {
        self.temp.clone()
    }

}


// TODO
struct Variables {
    variables: HashMap<String, Value>
}

enum Value {
    Num,
    Bool,
    String,
    // ...
}

