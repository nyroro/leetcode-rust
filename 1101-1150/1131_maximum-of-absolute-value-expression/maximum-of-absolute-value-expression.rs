
impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let n = arr1.len();
        let mut max1 = std::i32::MIN;
        let mut max2 = std::i32::MIN;
        let mut max3 = std::i32::MIN;
        let mut max4 = std::i32::MIN;
        let mut min1 = std::i32::MAX;
        let mut min2 = std::i32::MAX;
        let mut min3 = std::i32::MAX;
        let mut min4 = std::i32::MAX;

        for i in 0..n {
            max1 = max1.max(arr1[i] + arr2[i] + i as i32);
            min1 = min1.min(arr1[i] + arr2[i] + i as i32);
            max2 = max2.max(arr1[i] - arr2[i] + i as i32);
            min2 = min2.min(arr1[i] - arr2[i] + i as i32);
            max3 = max3.max(arr1[i] + arr2[i] - i as i32);
            min3 = min3.min(arr1[i] + arr2[i] - i as i32);
            max4 = max4.max(arr1[i] - arr2[i] - i as i32);
            min4 = min4.min(arr1[i] - arr2[i] - i as i32);
        }

        let diff1 = max1 - min1;
        let diff2 = max2 - min2;
        let diff3 = max3 - min3;
        let diff4 = max4 - min4;

        diff1.max(diff2).max(diff3).max(diff4)
    }
}
