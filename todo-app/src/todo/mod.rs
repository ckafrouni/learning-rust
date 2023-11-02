#[derive(Default)]
pub struct Todo {
    text: String,
    completed: bool,
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self {
            text,
            completed: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.completed { "x" } else { " " };
        write!(f, "[{}] {}", status, self.text)
    }
}
