
impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut arr: Vec<(i32, usize)> = nums.iter().cloned().enumerate().map(|(i, val)| (val, i)).collect();
        arr.sort_by_key(|&(val, _)| val);
        
        let al = arr[0].1 + 1;
        let ar = arr.len() - arr[0].1;
        
        if arr.len() == 1 {
            return al as i32;
        }
        
        let bl = arr[arr.len() - 1].1 + 1;
        let br = arr.len() - arr[arr.len() - 1].1;
        
        let mut ret = std::i32::MAX;
        ret = ret.min(al.max(bl) as i32);
        ret = ret.min(ar.max(br) as i32);
        ret = ret.min((al.min(bl) + ar.min(br)) as i32);
        
        ret

    }
}
