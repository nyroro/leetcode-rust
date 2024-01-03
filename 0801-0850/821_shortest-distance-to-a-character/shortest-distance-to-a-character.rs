
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let mut result: Vec<i32> = vec![0; n];
        let mut prev = -(n as i32);

        // 从左到右遍历，计算每个索引到最近的字符c的距离

        for i in 0..n {
            if s_chars[i] == c {
                prev = i as i32;
            }
            result[i] = (i as i32 - prev).abs();
        }

        prev = 2 * (n as i32);

        // 从右到左遍历，计算每个索引到最近的字符c的距离，并取两次遍历的距离的最小值

        for i in (0..n).rev() {
            if s_chars[i] == c {
                prev = i as i32;
            }
            result[i] = result[i].min(prev - (i as i32));
        }

        result

    }
}
