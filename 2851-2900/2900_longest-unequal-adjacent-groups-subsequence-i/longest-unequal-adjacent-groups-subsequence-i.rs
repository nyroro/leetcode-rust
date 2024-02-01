
impl Solution {
    pub fn get_words_in_longest_subsequence(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();  // 创建一个空的字符串数组

        for i in 0..n {
            if i < n - 1 && groups[i] != groups[i + 1] {
                result.push(words[i as usize].clone());  // 将满足条件的单词添加到结果数组中

            }
        }
        result.push(words[(n - 1) as usize].clone());  // 将最后一个单词添加到结果数组中

        result  // 返回结果数组

    }
}
