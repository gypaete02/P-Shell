pub struct Completer {
    line: String,
    index: usize,
    possibilities: Vec<String>
}

impl Completer {

    pub fn new(line: String) -> Self {
        Self {
            line,
            index: 0,
            possibilities: vec![]
        }
    }

    pub fn update(&mut self, line: String) {
        // TODO:
        // line = "dir1/dir2/file.t"
        //
        // todo: list dir2, filter out all files that don't start with 'file.t',
        // and collect to a Vec.
        // Store that Vec into self.possibilities, and when complete() is called iterate over the
        // 'possibilities' with 'index'

        self.line = line;
    }

    pub fn complete(&mut self) -> &str {
        println!("\r\npath: {}\r", self.line);
        &self.line
    }
}
