
impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks: Vec<char> = blocks.chars().collect();
        let n = blocks.len();
        let k = k as usize;
        let mut min_recolors = k as i32;
        
        for i in 0..=n - k {
            let mut recolors = 0;
            for j in i..i + k {
                if blocks[j] == 'W' {
                    recolors += 1;
                }
            }
            min_recolors = min_recolors.min(recolors);
        }
        
        min_recolors

    }
}
