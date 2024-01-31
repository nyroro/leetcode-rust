


impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        let mut pos = 0;
        let mut neg = 0;
        let mut k = 0;
        let mut remaining_target = target;

        while remaining_target > 0 {
            let cur = remaining_target % x;
            remaining_target /= x;

            if k > 0 {
                let option1 = cur * k + pos;
                let option2 = (cur + 1) * k + neg;
                let option3 = (x - cur) * k + pos;
                let option4 = (x - cur - 1) * k + neg;

                pos = option1.min(option2);
                neg = option3.min(option4);
            } else {
                pos = cur * 2;
                neg = (x - cur) * 2;
            }

            k += 1;
        }

        pos.min(k + neg) - 1

    }
}
