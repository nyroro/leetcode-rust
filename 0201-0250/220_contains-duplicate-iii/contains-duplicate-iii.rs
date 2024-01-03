
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let mut bucket_map: HashMap<i64, i64> = HashMap::new();
        let n = nums.len();
        let value_diff = value_diff as i64;
        let index_diff = index_diff as i64;

        for i in 0..n {
            let bucket = (nums[i] as i64 - std::i32::MIN as i64) / (value_diff + 1);
            if bucket_map.contains_key(&bucket) ||
                (bucket_map.contains_key(&(bucket - 1)) && (nums[i] as i64 - *bucket_map.get(&(bucket - 1)).unwrap()) <= value_diff) ||
                (bucket_map.contains_key(&(bucket + 1)) && (*bucket_map.get(&(bucket + 1)).unwrap() - nums[i] as i64) <= value_diff) {
                return true;
            }
            bucket_map.insert(bucket, nums[i] as i64);
            if i as i64 >= index_diff {
                let remove_bucket = (nums[i - index_diff as usize] as i64 - std::i32::MIN as i64) / (value_diff + 1);
                bucket_map.remove(&remove_bucket);
            }
        }
        false

    }
}
