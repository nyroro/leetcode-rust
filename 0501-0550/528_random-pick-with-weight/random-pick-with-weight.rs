
struct Solution {
    acc_sum: Vec<i32>,
    total: i32,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let mut acc_sum = vec![];
        let mut total = 0;

        for weight in w {
            total += weight;
            acc_sum.push(total);
        }

        Solution { acc_sum, total }
    }

    fn pick_index(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(1, self.total + 1);

        let mut left = 0;
        let mut right = self.acc_sum.len() as i32 - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if self.acc_sum[mid as usize] < r {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left

    }
}
