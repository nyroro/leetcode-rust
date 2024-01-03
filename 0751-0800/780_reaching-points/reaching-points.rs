
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        if sx > tx || sy > ty {
            return false;
        }
        if sx == tx {
            return (ty - sy) % sx == 0;
        }
        if sy == ty {
            return (tx - sx) % sy == 0;
        }
        if tx > ty {
            Solution::reaching_points(sx, sy, tx - ty, ty)
        } else {
            Solution::reaching_points(sx, sy, tx, ty - tx)
        }
    }
}
