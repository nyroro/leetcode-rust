
impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut merge = String::new();
        let mut word1 = word1.chars().collect::<Vec<char>>();
        let mut word2 = word2.chars().collect::<Vec<char>>();

        while !word1.is_empty() && !word2.is_empty() {
            if word1 >= word2 {
                merge.push(word1.remove(0));
            } else {
                merge.push(word2.remove(0));
            }
        }

        merge.push_str(&word1.iter().collect::<String>());
        merge.push_str(&word2.iter().collect::<String>());

        merge

    }
}
