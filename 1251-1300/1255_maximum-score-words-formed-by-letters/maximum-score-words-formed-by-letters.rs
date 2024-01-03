
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        // Step 1: 计算单词的得分总和

        fn word_score(word: &str, score: &Vec<i32>) -> i32 {
            word.chars().map(|c| score[(c as u8 - b'a') as usize]).sum()
        }

        // Step 2: 检查给定的字母列表是否足够组成一个单词

        fn can_form_word(word: &str, letters: &mut Vec<char>) -> bool {
            let mut word_letters = word.chars().collect::<Vec<char>>();
            for letter in word_letters.iter() {
                if let Some(pos) = letters.iter().position(|&x| x == *letter) {
                    letters.remove(pos);
                } else {
                    return false;
                }
            }
            true

        }

        // Step 3: 使用回溯法生成所有可能的单词组合

        fn backtrack(
            words: &Vec<String>,
            letters: &mut Vec<char>,
            score: &Vec<i32>,
            index: usize,
        ) -> i32 {
            if index == words.len() {
                return 0;
            }
            let mut max_score = 0;
            for i in index..words.len() {
                let mut temp_letters = letters.clone();
                if can_form_word(&words[i], &mut temp_letters) {
                    let current_score = word_score(&words[i], &score)
                        + backtrack(words, &mut temp_letters, score, i + 1);
                    max_score = max_score.max(current_score);
                }
            }
            max_score

        }

        // Step 4: 计算每个单词组合的总分，并返回最大的分数

        let mut letters = letters;
        backtrack(&words, &mut letters, &score, 0)
    }
}
