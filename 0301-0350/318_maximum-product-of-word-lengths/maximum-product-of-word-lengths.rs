
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut max_product = 0;
        let mut bitmasks = vec![0; words.len()];

        // 将每个单词表示为位掩码

        for (i, word) in words.iter().enumerate() {
            for ch in word.chars() {
                bitmasks[i] |= 1 << (ch as u8 - b'a');
            }
        }

        // 遍历所有的单词对，计算长度乘积并检查公共字母

        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if bitmasks[i] & bitmasks[j] == 0 {
                    let product = words[i].len() * words[j].len();
                    max_product = max_product.max(product);
                }
            }
        }

        max_product as i32

    }
}
