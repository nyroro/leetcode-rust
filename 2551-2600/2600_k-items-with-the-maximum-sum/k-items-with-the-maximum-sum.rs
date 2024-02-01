
impl Solution {
    pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
        let ones_to_pick = std::cmp::min(num_ones, k);
        let remaining = k - ones_to_pick;
        let zeros_to_pick = std::cmp::min(num_zeros, remaining);
        let neg_ones_to_pick = remaining - zeros_to_pick;
        
        ones_to_pick + neg_ones_to_pick

    }
}
