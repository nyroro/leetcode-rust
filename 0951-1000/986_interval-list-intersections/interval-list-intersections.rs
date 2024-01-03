
impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut i = 0;
        let mut j = 0;
        
        while i < first_list.len() && j < second_list.len() {
            let start1 = first_list[i][0];
            let end1 = first_list[i][1];
            let start2 = second_list[j][0];
            let end2 = second_list[j][1];
            
            if end1 < start2 {
                i += 1;
            } else if end2 < start1 {
                j += 1;
            } else {
                let intersection_start = start1.max(start2);
                let intersection_end = end1.min(end2);
                result.push(vec![intersection_start, intersection_end]);
                
                if end1 < end2 {
                    i += 1;
                } else {
                    j += 1;
                }
            }
        }
        
        result

    }
}
