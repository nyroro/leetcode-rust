
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted_score = score.clone();
        sorted_score.sort_unstable_by(|a, b| b.cmp(a));
        
        let mut ranks = vec![String::new(); score.len()];
        
        for (i, &s) in score.iter().enumerate() {
            let rank = match sorted_score.iter().position(|&x| x == s) {
                Some(pos) => pos + 1,
                None => 0,
            };
            
            let rank_str = match rank {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                _ => rank.to_string(),
            };
            
            ranks[i] = rank_str;
        }
        
        ranks

    }
}
