
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

    fn delete_text(&mut self, k: i32) -> i32 {
        let k = k as usize; // Convert i32 to usize

        let prev_pos = self.cursor;
        let start = self.cursor.saturating_sub(k);
        let end = self.cursor;
        let deleted = self.text.drain(start..end).count();
        self.cursor -= deleted;
        deleted as i32

    }

    fn cursor_left(&mut self, k: i32) -> String {
        let k = k as usize; // Convert i32 to usize

        self.cursor = self.cursor.saturating_sub(k);
        let start = self.cursor.saturating_sub(10).max(0);
        self.text[start..self.cursor].to_string()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        let k = k as usize; // Convert i32 to usize

        self.cursor = self.cursor.saturating_add(k).min(self.text.len());
        let start = self.cursor.saturating_sub(10).max(0);
        self.text[start..self.cursor].to_string()
    }
}
