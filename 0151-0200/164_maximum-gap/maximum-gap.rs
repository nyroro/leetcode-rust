
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }
        
        let min_val = *nums.iter().min().unwrap();
        let max_val = *nums.iter().max().unwrap();
        
        let bucket_size = std::cmp::max(1, (max_val - min_val) / (n - 1) as i32);
        let bucket_count = ((max_val - min_val) / bucket_size) as usize + 1;
        
        let mut bucket_min = vec![std::i32::MAX; bucket_count];
        let mut bucket_max = vec![std::i32::MIN; bucket_count];
        
        for num in nums {
            let index = ((num - min_val) / bucket_size) as usize;
            bucket_min[index] = std::cmp::min(bucket_min[index], num);
            bucket_max[index] = std::cmp::max(bucket_max[index], num);
        }
        
        let mut max_gap = 0;
        let mut prev_max = min_val;
        
        for i in 0..bucket_count {
            if bucket_min[i] == std::i32::MAX && bucket_max[i] == std::i32::MIN {
                continue;
            }
            max_gap = std::cmp::max(max_gap, bucket_min[i] - prev_max);
            prev_max = bucket_max[i];
        }
        
        max_gap

    }
}
