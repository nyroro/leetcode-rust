
struct ExamRoom {
    seats: Vec<i32>,
    n: i32,
}

impl ExamRoom {
    fn new(n: i32) -> Self {
        ExamRoom {
            seats: Vec::new(),
            n,
        }
    }
    
    fn seat(&mut self) -> i32 {
        if self.seats.is_empty() {
            self.seats.push(0);
            return 0;
        }
        
        let mut max_dist = self.seats[0];
        let mut pos = 0;
        
        for i in 0..self.seats.len() - 1 {
            let dist = (self.seats[i + 1] - self.seats[i]) / 2;
            if dist > max_dist {
                max_dist = dist;
                pos = (self.seats[i + 1] + self.seats[i]) / 2;
            }
        }
        
        if self.n - 1 - self.seats[self.seats.len() - 1] > max_dist {
            pos = self.n - 1;
        }
        
        self.seats.push(pos);
        self.seats.sort_unstable();
        pos

    }
    
    fn leave(&mut self, p: i32) {
        if let Some(index) = self.seats.iter().position(|&x| x == p) {
            self.seats.remove(index);
        }
    }
}
