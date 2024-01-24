
impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut l = vec![0; fruits.len()];
        let mut r = vec![0; fruits.len()];
        let mut ret = 0;
        let mut c = 0;
        let mut rx = -1;

        for i in 0..fruits.len() {
            if fruits[i][0] >= start_pos {
                if rx < 0 {
                    rx = i as i32;
                }
                if fruits[i][0] - start_pos <= k {
                    r[i] = fruits[i][1];
                    if i > 0 {
                        r[i] += r[i - 1];
                    }
                    if fruits[i][0] == start_pos {
                        c = fruits[i][1];
                    }
                    ret = ret.max(r[i]);
                } else {
                    break;
                }
            }
        }

        for i in (0..fruits.len()).rev() {
            if start_pos >= fruits[i][0] {
                if start_pos - fruits[i][0] <= k {
                    l[i] = fruits[i][1];
                    if i + 1 < fruits.len() {
                        l[i] += l[i + 1];
                    }
                    ret = ret.max(l[i]);
                } else {
                    break;
                }
            }
        }

        if rx >= 0 {
            let mut j = rx as usize;
            let mut j2 = rx as usize;
            for i in 0..fruits.len() {
                if fruits[i][0] >= start_pos {
                    break;
                }
                if l[i] == 0 {
                    continue;
                }
                while j < fruits.len() && 2 * (start_pos - fruits[i][0]) + fruits[j][0] - start_pos <= k {
                    j += 1;
                }
                if j - 1 >= rx as usize {
                    ret = ret.max(l[i] + r[j - 1] - c);
                }
                while j2 < fruits.len() && (start_pos - fruits[i][0]) + 2 * (fruits[j2][0] - start_pos) <= k {
                    j2 += 1;
                }
                if j2 - 1 >= rx as usize {
                    ret = ret.max(l[i] + r[j2 - 1] - c);
                }
            }
        }

        ret

    }
}
