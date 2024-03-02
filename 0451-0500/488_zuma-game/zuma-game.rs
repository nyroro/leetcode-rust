
use std::collections::HashMap;



impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut freq = vec![0; 26];
        for c in hand.chars() {
            freq[(c as u8 - b'A') as usize] += 1;
        }
        let mut cache: HashMap<String, i32> = HashMap::new();
        Solution::dfs(board, &mut freq, &mut cache)
    }

    fn dfs(board: String, freq: &mut Vec<i32>, cache: &mut HashMap<String, i32>) -> i32 {
        let key = format!("{}#{}", board, Solution::serialize(freq));
        if let Some(&val) = cache.get(&key) {
            return val;
        }
        let mut r = i32::MAX;
        if board.is_empty() {
            r = 0;
        } else {
            for (i, ch) in board.chars().enumerate() {
                for j in 0..freq.len() {
                    let mut worth_trying = false;
                    if ch as u8 - b'A' == j as u8 {
                        worth_trying = true;
                    } else if i > 0 && ch == board.chars().nth(i - 1).unwrap() && ch as u8 - b'A' != j as u8 {
                        worth_trying = true;
                    }
                    if freq[j] > 0 && worth_trying {
                        let mut new_board = board.clone();
                        new_board.insert(i, (j as u8 + b'A') as char);
                        freq[j] -= 1;
                        let new_str = Solution::update_board(new_board);
                        let steps = Solution::dfs(new_str, freq, cache);
                        if steps != -1 {
                            r = r.min(steps + 1);
                        }
                        freq[j] += 1;
                    }
                }
            }
        }
        if r == i32::MAX {
            r = -1;
        }
        cache.insert(key, r);
        r

    }

    fn update_board(board: String) -> String {
        let mut new_board = String::new();
        let mut start = 0;
        let mut end = 0;
        let board_chars: Vec<char> = board.chars().collect();
        let board_len = board_chars.len();
        while start < board_len {
            while end < board_len && board_chars[start] == board_chars[end] {
                end += 1;
            }
            if end - start >= 3 {
                new_board.push_str(&board_chars[..start].iter().collect::<String>());
                new_board.push_str(&board_chars[end..].iter().collect::<String>());
                return Solution::update_board(new_board);
            } else {
                start = end;
            }
        }
        board

    }

    fn serialize(freq: &Vec<i32>) -> String {
        let mut key = String::new();
        for (i, &f) in freq.iter().enumerate() {
            if f > 0 {
                key.push((i as u8 + b'A') as char);
                key.push_str(&f.to_string());
            }
        }
        key

    }
}
