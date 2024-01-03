
impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let mut ans = i32::MAX;
        let mut childs = vec![0; k as usize];
        Solution::helper(&cookies, &mut childs, 0, &mut ans);
        ans

    }
    
    fn helper(cookies: &Vec<i32>, childs: &mut Vec<i32>, ind: usize, ans: &mut i32) {
        if ind == cookies.len() {
            *ans = (*childs.iter().max().unwrap()).min(*ans);
            return;
        }
        for i in 0..childs.len() {
            childs[i] += cookies[ind];
            Solution::helper(cookies, childs, ind + 1, ans);
            childs[i] -= cookies[ind];
            if childs[i] == 0 {
                break;
            }
        }
    }
}
