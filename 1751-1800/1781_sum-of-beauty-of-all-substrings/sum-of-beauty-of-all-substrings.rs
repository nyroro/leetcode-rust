
impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let mut sum = 0;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        
        for i in 0..n {
            let mut freq = vec![0; 26];
            
            for j in i..n {
                freq[(chars[j] as u8 - b'a') as usize] += 1;
                
                let max_freq = *freq.iter().max().unwrap();
                let min_freq = *freq.iter().filter(|&&x| x > 0).min().unwrap();
                
                sum += max_freq - min_freq;
            }
        }
        
        sum

    }
}
