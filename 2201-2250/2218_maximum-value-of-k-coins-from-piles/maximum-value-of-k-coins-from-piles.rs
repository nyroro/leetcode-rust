
impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        fn max_coins(piles: &Vec<Vec<i32>>, start: usize, end: usize, k: i32) -> Vec<i32> {
            if start == end {
                let mut arr = vec![0; std::cmp::min(k as usize + 1, piles[start].len() + 1)];
                for i in 1..arr.len() {
                    arr[i] = piles[start][i - 1] + arr[i - 1];
                }
                return arr;
            }
            let mid = (start + end) / 2;
            let arr_l = max_coins(piles, start, mid, k);
            let arr_r = max_coins(piles, mid + 1, end, k);
            let n = std::cmp::min(k as usize + 1, arr_l.len() + arr_r.len());
            let mut arr = vec![0; n];
            for i in 0..arr_l.len() {
                for j in 0..arr_r.len() {
                    if (i + j) as i32 > k {
                        break;
                    }
                    arr[i + j] = std::cmp::max(arr[i + j], arr_l[i] + arr_r[j]);
                }
            }
            arr

        }
        
        let arr = max_coins(&piles, 0, piles.len() - 1, k);
        arr[k as usize]
    }
}
