
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut papers = vec![0; n + 1];
        
        // 计数每个引用次数的论文数量

        for &citation in &citations {
            if citation >= n as i32 {
                papers[n] += 1;
            } else {
                papers[citation as usize] += 1;
            }
        }
        
        // 找到 h-index

        let mut h = n as i32;
        let mut sum = papers[n];
        while h > sum {
            h -= 1;
            sum += papers[h as usize];
        }
        
        h

    }
}
