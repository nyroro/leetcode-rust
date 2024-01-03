
impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let area1 = (ax2 - ax1) * (ay2 - ay1);
        let area2 = (bx2 - bx1) * (by2 - by1);
        
        let overlap_width = (ax2.min(bx2) - ax1.max(bx1)).max(0);
        let overlap_height = (ay2.min(by2) - ay1.max(by1)).max(0);
        let overlap_area = overlap_width * overlap_height;
        
        area1 + area2 - overlap_area

    }
}
