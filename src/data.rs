
pub struct History {
    history: Vec<String>,
    index: usize,
    temp: Option<String>,
}

impl History {

    pub fn init() -> Self {
        Self {
            history: vec![],
            index: 0,
            temp: None,
        }
    }

    pub fn add(&mut self, string: &String) {
        let mut string = string.clone();
        Self::remove_lf(&mut string);
        self.history.insert(0, string);
    }

    pub fn step_up(&mut self, string: &mut String) {

        // FIXME: After a step_down(), it takes two step_up() to work.

        let last = self.history.get(self.index);

        if last.is_some() {
            if string != last.unwrap() {
                Self::remove_lf(string);
                self.temp = Some(string.clone());
            }
        } else {
            return;
        }

        *string = match self.history.get(self.index) {
            Some(s) => {
                self.index += 1;
                s.clone()
            }
            None => string.clone()
        };
    }

    pub fn step_down(&mut self, string: &mut String) {

        // FIXME: After a step_up(), it takes two step_down() to work.

        self.index = self.index.saturating_sub(1);

        *string = self.history
            .get(self.index)
            .or(self.temp.as_ref())
            .unwrap_or(string)
            .clone();
    }

    fn remove_lf(string: &mut String) {
        if string.chars().last().unwrap_or(' ') == '\n' {
            string.pop();
        }
    }

}









// TODO
//struct Variables {
//    variables: HashMap<String, Value>
//}

//enum Value {
//    Num,
//    Bool,
//    String,
//    // ...
//}

