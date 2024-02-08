


impl Solution {
    pub fn count_complete_substrings(word: String, k: i32) -> i32 {
        let word = word.as_bytes();
        let n = word.len();
        let mut result = 0;
        
        for x in 1..=26 {
            let length = x * k as usize;
            if length > n {
                break;
            }
            
            let mut freq = vec![0; 26];
            let mut count_freq_k = 0;
            let mut l = 0;
            
            for r in 0..n {
                if r > 0 && (word[r - 1] as i32 - word[r] as i32).abs() > 2 {
                    freq.iter_mut().for_each(|f| *f = 0);
                    count_freq_k = 0;
                    l = r;
                }
                
                if freq[(word[r] - b'a') as usize] == k {
                    count_freq_k -= 1;
                }
                freq[(word[r] - b'a') as usize] += 1;
                if freq[(word[r] - b'a') as usize] == k {
                    count_freq_k += 1;
                }
                
                if r - l + 1 > length {
                    if freq[(word[l] - b'a') as usize] == k {
                        count_freq_k -= 1;
                    }
                    freq[(word[l] - b'a') as usize] -= 1;
                    if freq[(word[l] - b'a') as usize] == k {
                        count_freq_k += 1;
                    }
                    l += 1;
                }
                
                if r - l + 1 == length && count_freq_k == x {
                    result += 1;
                }
            }
        }
        
        result

    }
}
