
impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        if !forts.contains(&1) || !forts.contains(&-1) {
            return 0;
        }

        let mut out1 = Vec::new();
        let mut flag = false;
        let mut count1 = 0;

        for &fort in &forts {
            if fort == 1 {
                flag = true;
                count1 = 0;
            } else if fort == -1 {
                flag = false;
                out1.push(count1);
                count1 = 0;
            } else if flag && fort == 0 {
                count1 += 1;
            }
        }

        let mut out2 = Vec::new();
        let mut fla = false;
        let mut count2 = 0;

        for &fort in forts.iter().rev() {
            if fort == 1 {
                fla = true;
                count2 = 0;
            } else if fort == -1 {
                fla = false;
                out2.push(count2);
                count2 = 0;
            } else if fla && fort == 0 {
                count2 += 1;
            }
        }

        out1.extend(out2);
        *out1.iter().max().unwrap_or(&0)
    }
}
