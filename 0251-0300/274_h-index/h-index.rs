
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut papers = vec![0; n + 1];
        
        // 计数每个引用次数的论文数量

        for &citation in &citations {
            papers[usize::min(n, citation as usize)] += 1;
        }
        
        // 找到 h-index

        let mut h = n as i32;
        let mut sum = 0;
        while h > sum {
            sum += papers[h as usize];
            h -= 1;
        }
        
        h

    }
}
