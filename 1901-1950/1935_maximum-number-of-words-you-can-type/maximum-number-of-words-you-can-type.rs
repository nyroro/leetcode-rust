
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let words: Vec<&str> = text.split_whitespace().collect(); // 将字符串按空格分割成单词

        let mut count = 0; // 计数器，记录可以完全输入的单词数量


        for word in words {
            let mut is_broken = false; // 标记是否包含损坏字母


            for c in broken_letters.chars() {
                if word.contains(c) {
                    is_broken = true;
                    break;
                }
            }

            if !is_broken {
                count += 1;
            }
        }

        count

    }
}
