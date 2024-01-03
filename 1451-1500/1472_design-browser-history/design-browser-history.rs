
struct BrowserHistory {
    history: Vec<String>,
    current: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        let mut history = Vec::new();
        history.push(homepage);
        BrowserHistory {
            history,
            current: 0,
        }
    }
    
    fn visit(&mut self, url: String) {
        self.current += 1;
        self.history.truncate(self.current);
        self.history.push(url);
    }
    
    fn back(&mut self, steps: i32) -> String {
        let back_steps = self.current.saturating_sub(steps as usize);
        self.current = back_steps;
        self.history[back_steps].clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        let forward_steps = (self.current + steps as usize).min(self.history.len() - 1);
        self.current = forward_steps;
        self.history[forward_steps].clone()
    }
}
