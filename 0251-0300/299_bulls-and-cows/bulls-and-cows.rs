
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut secret_count = std::collections::HashMap::new();
        let mut guess_count = std::collections::HashMap::new();

        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                bulls += 1;
            } else {
                *secret_count.entry(s).or_insert(0) += 1;
                *guess_count.entry(g).or_insert(0) += 1;
            }
        }

        for (digit, count) in secret_count {
            if let Some(guess_count) = guess_count.get(&digit) {
                cows += std::cmp::min(count, *guess_count);
            }
        }

        format!("{}A{}B", bulls, cows)
    }
}
