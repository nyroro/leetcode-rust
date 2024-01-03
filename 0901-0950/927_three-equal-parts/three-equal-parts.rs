
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let ones_count: i32 = arr.iter().sum();
        
        if ones_count % 3 != 0 {
            return vec![-1, -1];
        }
        
        if ones_count == 0 {
            return vec![0, n as i32 - 1];
        }
        
        let target_ones = ones_count / 3;
        let mut first = -1;
        let mut second = -1;
        let mut third = -1;
        let mut ones = 0;
        
        for i in 0..n {
            if arr[i] == 1 {
                ones += 1;
                if ones == 1 {
                    first = i as i32;
                }
                if ones == target_ones + 1 {
                    second = i as i32;
                }
                if ones == 2 * target_ones + 1 {
                    third = i as i32;
                }
            }
        }
        
        while third < n as i32 && arr[first as usize] == arr[second as usize] && arr[second as usize] == arr[third as usize] {
            first += 1;
            second += 1;
            third += 1;
        }
        
        if third == n as i32 {
            return vec![first - 1, second];
        }
        
        vec![-1, -1]
    }
}
