
struct MagicDictionary {
    dictionary: Vec<String>,
}

impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            dictionary: Vec::new(),
        }
    }
    
    fn build_dict(&mut self, dictionary: Vec<String>) {
        self.dictionary = dictionary;
    }
    
    fn search(&self, search_word: String) -> bool {
        for word in &self.dictionary {
            if word.len() != search_word.len() {
                continue;
            }
            let mut diff_count = 0;
            for (c1, c2) in word.chars().zip(search_word.chars()) {
                if c1 != c2 {
                    diff_count += 1;
                }
                if diff_count > 1 {
                    break;
                }
            }
            if diff_count == 1 {
                return true;
            }
        }
        false

    }
}
