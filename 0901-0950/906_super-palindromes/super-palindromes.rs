
impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let left_num: u64 = left.parse().unwrap();
        let right_num: u64 = right.parse().unwrap();
        let mut ans = if 9 >= left_num && 9 <= right_num { 1 } else { 0 };

        fn is_palindrome(num: u64) -> bool {
            let num_str = num.to_string();
            num_str == num_str.chars().rev().collect::<String>()
        }

        for dig in 1..10 {
            let is_odd = dig % 2 != 0 && dig != 1;
            let inner_len = (dig >> 1) - 1;
            let mut inner_lim = std::cmp::max(1, 2u64.pow(inner_len as u32));
            let mid_pos = dig >> 1;
            let mut mid_lim = if is_odd { 3 } else { 1 };

            for edge in 1..3 {
                let mut pal = vec![0; dig as usize];
                pal[0] = edge;
                pal[dig as usize - 1] = edge;

                if edge == 2 {
                    inner_lim = 1;
                    mid_lim = mid_lim.min(2);
                }

                for inner in 0..inner_lim {
                    if inner > 0 {
                        let inner_str = format!("{:0width$b}", inner, width = inner_len as usize);
                        pal[1..(1 + inner_len as usize)].copy_from_slice(&inner_str.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>());
                        pal[(dig as usize - inner_len as usize - 1)..(dig as usize - 1)].copy_from_slice(&inner_str.chars().rev().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>());
                    }

                    for mid in 0..mid_lim {
                        if is_odd {
                            pal[mid_pos as usize] = mid;
                        }

                        let palin: u64 = pal.iter().fold(0, |acc, &x| acc * 10 + x);
                        let square = palin * palin;

                        if square > right_num {
                            return ans;
                        }

                        if square >= left_num && is_palindrome(square) {
                            ans += 1;
                        }
                    }
                }
            }
        }

        ans

    }
}
