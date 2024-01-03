
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut num_arr = Vec::new();
        let mut num = num;
        while num > 0 {
            num_arr.push(num % 10);
            num /= 10;
        }
        num_arr.sort();

        let mut min_sum = i32::MAX;
        let mut min_pair = (0, 0);

        for i in 0..24 {
            let mut new1 = 0;
            let mut new2 = 0;
            for j in 0..4 {
                if (i >> j) & 1 == 1 {
                    new1 = new1 * 10 + num_arr[j];
                } else {
                    new2 = new2 * 10 + num_arr[j];
                }
            }
            let sum = new1 + new2;
            if sum < min_sum {
                min_sum = sum;
                min_pair = (new1, new2);
            }
        }

        min_sum

    }
}
