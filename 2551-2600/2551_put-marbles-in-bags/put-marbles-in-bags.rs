
impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        let mut arr: Vec<i32> = Vec::new();
        for i in 0..n - 1 {
            arr.push(weights[i] + weights[i + 1]);
        }
        arr.sort();

        let mut ans: i64 = 0;
        for i in 0..k - 1 {
            ans -= arr[i as usize];
            ans += arr[(n - 2 - i) as usize];
        }
        ans

    }
}
