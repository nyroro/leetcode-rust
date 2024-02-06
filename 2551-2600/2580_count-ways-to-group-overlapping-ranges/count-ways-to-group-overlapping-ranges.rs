
const MOD: i64 = 1_000_000_007;



impl Solution {
    pub fn count_ways(ranges: Vec<Vec<i32>>) -> i32 {
        let mut sorted_ranges = ranges;
        sorted_ranges.sort(); // Sort the ranges based on the start values


        let mut ret: i64 = 1;
        let mut i = 0;
        while i < sorted_ranges.len() {
            let mut j = i + 1;
            let mut R = sorted_ranges[i][1];
            while j < sorted_ranges.len() && sorted_ranges[j][0] <= R {
                R = R.max(sorted_ranges[j][1]);
                j += 1;
            }
            ret = (ret * 2) % MOD;
            i = j;
        }
        ret as i32

    }
}
