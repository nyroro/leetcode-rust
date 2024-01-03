
struct LUPrefix {
    videos: Vec<bool>,
    longest_prefix: i32,
}

impl LUPrefix {
    fn new(n: i32) -> Self {
        LUPrefix {
            videos: vec![false; n as usize],
            longest_prefix: 0,
        }
    }
    
    fn upload(&mut self, video: i32) {
        self.videos[(video - 1) as usize] = true;
        if video == self.longest_prefix + 1 {
            while self.longest_prefix < self.videos.len() as i32 && self.videos[self.longest_prefix as usize] {
                self.longest_prefix += 1;
            }
        }
    }
    
    fn longest(&self) -> i32 {
        self.longest_prefix

    }
}
