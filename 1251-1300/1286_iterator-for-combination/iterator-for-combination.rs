
struct CombinationIterator {
    characters: Vec<char>,
    combination_length: usize,
    current_combination: Vec<usize>,
    has_next_combination: bool,
}

impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        let characters: Vec<char> = characters.chars().collect();
        let combination_length = combination_length as usize;
        let current_combination = (0..combination_length).collect();
        let has_next_combination = characters.len() >= combination_length;

        CombinationIterator {
            characters,
            combination_length,
            current_combination,
            has_next_combination,
        }
    }

    fn next(&mut self) -> String {
        let combination: String = self

            .current_combination

            .iter()
            .map(|&index| self.characters[index])
            .collect();

        self.update_current_combination();
        combination

    }

    fn has_next(&self) -> bool {
        self.has_next_combination

    }

    fn update_current_combination(&mut self) {
        let n = self.characters.len();
        let k = self.combination_length;

        let mut i = k - 1;
        while i >= 0 && self.current_combination[i] == n - k + i {
            i -= 1;
        }

        if i >= 0 {
            self.current_combination[i] += 1;
            for j in i + 1..k {
                self.current_combination[j] = self.current_combination[j - 1] + 1;
            }
        } else {
            self.has_next_combination = false;
        }
    }
}
