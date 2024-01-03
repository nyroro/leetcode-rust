
impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let mut count1 = vec![0; 26];
        let mut count2 = vec![0; 26];
        let mut max_count1 = 0;
        let mut max_count2 = 0;
        
        for ch in a.chars() {
            let idx = ch as usize - 'a' as usize;
            count1[idx] += 1;
            max_count1 = max_count1.max(count1[idx]);
        }
        
        for ch in b.chars() {
            let idx = ch as usize - 'a' as usize;
            count2[idx] += 1;
            max_count2 = max_count2.max(count2[idx]);
        }
        
        let mut ans = a.len() + b.len() - max_count1 - max_count2;
        
        for i in 0..26 {
            if i > 0 {
                count1[i] += count1[i - 1];
                count2[i] += count2[i - 1];
            }
            
            if i < 25 {
                ans = ans.min(a.len() - count1[i] + count2[i]);
                ans = ans.min(b.len() - count2[i] + count1[i]);
            }
        }
        
        ans as i32

    }
}
