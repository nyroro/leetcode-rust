
impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let mut hamsters_vec: Vec<char> = hamsters.chars().collect();
        let mut count: i32 = 0;
        
        if hamsters_vec.len() == 1 {
            if hamsters_vec[0] == 'H' {
                return -1;
            } else {
                return 0;
            }
        }
        
        if hamsters_vec.starts_with(&['H', 'H']) || hamsters_vec.ends_with(&['H', 'H']) {
            return -1;
        }
        
        for i in 1..hamsters_vec.len() - 1 {
            if hamsters_vec[i] == 'H' && hamsters_vec[i - 1] == 'H' && hamsters_vec[i + 1] == 'H' {
                return -1;
            }
        }
        
        for i in 0..hamsters_vec.len() - 2 {
            if hamsters_vec[i] == 'H' && hamsters_vec[i + 1] == '.' && hamsters_vec[i + 2] == 'H' {
                hamsters_vec[i] = '.';
                hamsters_vec[i + 2] = '.';
                count += 1;
            }
        }
        
        count += hamsters_vec.iter().filter(|&c| *c == 'H').count() as i32;
        
        return count;
    }
}
