
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

fn main() {
    let mut obj = MagicDictionary::new();
    obj.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
    println!("{}", obj.search("hello".to_string())); // false

    println!("{}", obj.search("hhllo".to_string())); // true

    println!("{}", obj.search("hell".to_string())); // false

    println!("{}", obj.search("leetcoded".to_string())); // false

}
