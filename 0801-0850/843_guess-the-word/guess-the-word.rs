


impl Solution {
    fn check(a: &str, b: &str) -> i32 {
        let mut ct = 0;
        for (i, _) in a.chars().enumerate() {
            if a.chars().nth(i) == b.chars().nth(i) {
                ct += 1;
            }
        }
        ct

    }

    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let mut matches = 0;
        let mut i = 0;
        let mut words = words;

        while i < 30 && matches != 6 {
            let trial = &words[words.len() / 2];
            matches = master.guess(trial.to_string());

            let mut possibilities = Vec::new();
            for word in &words {
                if Solution::check(word, trial) == matches {
                    possibilities.push(word.to_string());
                }
            }
            words = possibilities;
            i += 1;
        }
    }
}
