
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_count = vec![0; 26];
        let mut t_count = vec![0; 26];
        let mut steps = 0;

        // 统计s和t中每个字符的出现次数

        for ch in s.chars() {
            let index = (ch as u8 - b'a') as usize;
            s_count[index] += 1;
        }

        for ch in t.chars() {
            let index = (ch as u8 - b'a') as usize;
            t_count[index] += 1;
        }

        // 比较s和t中每个字符的出现次数，计算步数

        for i in 0..26 {
            if s_count[i] > t_count[i] {
                steps += s_count[i] - t_count[i];
            }
        }

        steps

    }
}
