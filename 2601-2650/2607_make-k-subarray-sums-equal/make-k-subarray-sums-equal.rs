


impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        fn gcd(a: i32, b: i32) -> i32 {
            let (mut a, mut b) = if a < b { (b, a) } else { (a, b) };
            while b > 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a

        }

        let n = arr.len() as i32;
        let t = gcd(n, k);
        let mut ret = 0;

        fn gao(arr: &mut [i32]) -> i64 {
            let mut ret = 0;
            arr.sort();
            let m = arr.len();
            for i in 0..m / 2 {
                ret += (arr[m - i - 1] - arr[i]) as i64;
            }
            ret

        }

        for i in 0..t {
            ret += gao(&mut arr[i as usize..].iter().step_by(t as usize).copied().collect::<Vec<i32>>());
        }
        ret

    }
}
