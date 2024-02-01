
struct TextEditor {
    text: String,
    cursor: usize,
}

impl TextEditor {
    fn new() -> Self {
        TextEditor {
            text: String::new(),
            cursor: 0,
        }
    }

    fn add_text(&mut self, text: String) {
        self.text.insert_str(self.cursor, &text);
        self.cursor += text.len();
    }

    fn delete_text(&mut self, k: usize) -> usize {
        let prev_pos = self.cursor;
        let start = self.cursor.saturating_sub(k);
        let end = self.cursor;
        let deleted = self.text.drain(start..end).count();
        self.cursor -= deleted;
        prev_pos - self.cursor

    }

    fn cursor_left(&self, k: usize) -> String {
        let new_cursor = self.cursor.saturating_sub(k);
        let start = new_cursor.saturating_sub(10).max(0);
        self.text[start..self.cursor].to_string()
    }

    fn cursor_right(&self, k: usize) -> String {
        let new_cursor = self.cursor + k;
        let start = self.cursor.saturating_sub(10).max(0);
        self.text[start..new_cursor.min(self.text.len())].to_string()
    }
}
